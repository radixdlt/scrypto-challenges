import { combine, err, Result } from 'neverthrow'
import { readBuffer } from '@radixdlt/util'
import { AES_GCM, AES_GCM_SealedBoxT } from '../symmetric-encryption'
import { SealedMessageT } from './_types'
import { validateLength } from '../utils'
import { PublicKey, PublicKeyT } from '../elliptic-curve'

const create = (
	input: Readonly<{
		ephemeralPublicKey: PublicKeyT
		nonce: Buffer
		authTag: Buffer
		ciphertext: Buffer
	}>,
): Result<SealedMessageT, Error> =>
	combine([__validateNonce(input.nonce), __validateTag(input.authTag)]).map(
		_ => ({
			...input,
			combined: (): Buffer =>
				Buffer.concat([
					input.ephemeralPublicKey.asData({ compressed: true }),
					input.nonce,
					input.authTag,
					input.ciphertext,
				]),
		}),
	)

const sealedMessageNonceLength = AES_GCM.nonceLength
const sealedMessageAuthTagLength = AES_GCM.tagLength

export const __validateTag: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateLength.bind(
	null,
	sealedMessageAuthTagLength,
	'auth tag',
)

export const __validateNonce: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateLength.bind(
	null,
	sealedMessageNonceLength,
	'nonce',
)

const sealedMessageFromBuffer = (
	buffer: Buffer,
): Result<SealedMessageT, Error> => {
	const sealedMessageLength = buffer.length
	const lengthOfCiphertext =
		sealedMessageLength -
		PublicKey.compressedByteCount -
		sealedMessageNonceLength -
		sealedMessageAuthTagLength

	if (lengthOfCiphertext <= 0)
		return err(new Error('Ciphertext cannot be empty'))

	const readNextBuffer = readBuffer.bind(null, buffer)()

	return combine([
		readNextBuffer(PublicKey.compressedByteCount).andThen(
			PublicKey.fromBuffer,
		),
		readNextBuffer(sealedMessageNonceLength),
		readNextBuffer(sealedMessageAuthTagLength),
		readNextBuffer(lengthOfCiphertext),
	]).andThen(resultList => {
		const ephemeralPublicKey = resultList[0] as PublicKeyT
		const nonce = resultList[1] as Buffer
		const authTag = resultList[2] as Buffer
		const ciphertext = resultList[3] as Buffer

		return create({
			ephemeralPublicKey,
			nonce,
			authTag,
			ciphertext,
		})
	})
}

const sealedMsgFromAESSealedBox = (
	aesSealedBox: AES_GCM_SealedBoxT,
	ephemeralPublicKey: PublicKeyT,
): Result<SealedMessageT, Error> =>
	create({ ...aesSealedBox, ephemeralPublicKey })

export const SealedMessage = {
	nonceByteCount: sealedMessageNonceLength,
	authTagByteCount: sealedMessageAuthTagLength,
	create,
	fromAESSealedBox: sealedMsgFromAESSealedBox,
	fromBuffer: sealedMessageFromBuffer,
}
