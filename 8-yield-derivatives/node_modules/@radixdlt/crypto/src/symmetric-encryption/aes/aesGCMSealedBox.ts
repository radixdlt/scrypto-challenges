import { combine, Result } from 'neverthrow'
import { AES_GCM_SealedBoxProps, AES_GCM_SealedBoxT } from './_types'
import { buffersEquals, readBuffer } from '@radixdlt/util'
import { validateLength, validateMinLength } from '../../utils'

const tagLength = 16
const nonceLength = 12

const cipherMinLength = 1

const __validateNonce: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateLength.bind(
	null,
	nonceLength,
	'nonce (IV)',
)

const __validateTag: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateLength.bind(null, tagLength, 'auth tag')

const __validateAESSealedBoxCiphertext: (
	buffer: Buffer,
) => Result<Buffer, Error> = validateMinLength.bind(
	null,
	cipherMinLength,
	'ciphertext',
)

/*
 * returns combined buffers: `nonce || tag || cipher`
 * */
const combineSealedBoxProps = (input: AES_GCM_SealedBoxProps): Buffer =>
	Buffer.concat([input.nonce, input.authTag, input.ciphertext])

const create = (
	input: AES_GCM_SealedBoxProps,
): Result<AES_GCM_SealedBoxT, Error> =>
	combine([
		__validateNonce(input.nonce),
		__validateTag(input.authTag),
		__validateAESSealedBoxCiphertext(input.ciphertext),
	]).map(_ => ({
		...input,
		combined: (): Buffer => combineSealedBoxProps(input),
		equals: (other: AES_GCM_SealedBoxT): boolean =>
			buffersEquals(other.nonce, input.nonce) &&
			buffersEquals(other.authTag, input.authTag) &&
			buffersEquals(other.ciphertext, input.ciphertext),
	}))

/* Buffer is: `nonce || tag || cipher` */
const aesSealedBoxFromBuffer = (
	buffer: Buffer,
): Result<AES_GCM_SealedBoxT, Error> => {
	const readNextBuffer = readBuffer.bind(null, buffer)()
	return combine([
		readNextBuffer(nonceLength),
		readNextBuffer(tagLength),
		readNextBuffer(buffer.length - nonceLength - tagLength),
	])
		.map((parsed: Buffer[]) => {
			const nonce = parsed[0]
			const authTag = parsed[1]
			const ciphertext = parsed[2]

			return {
				nonce,
				authTag,
				ciphertext,
			}
		})
		.andThen(create)
}

export const AES_GCM_SealedBox = {
	fromCombinedBuffer: aesSealedBoxFromBuffer,
	create,
	nonceLength,
	tagLength,
}
