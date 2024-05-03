import { secureRandomGenerator } from '@radixdlt/util'
import {
	combine,
	err,
	errAsync,
	ok,
	okAsync,
	Result,
	ResultAsync,
} from 'neverthrow'
import {
	EncryptedMessageT,
	EncryptionScheme,
	MessageDecryptionInput,
	MessageEncryptionInput,
	MessageType,
	PlaintextMessageT,
	SealedMessageT,
} from './_types'
import { Scrypt, ScryptParams } from '../key-derivation-functions'
import {
	AES_GCM_SealedBoxT,
	AES_GCM_SealedBox,
	AES_GCM,
	aesGCMSealDeterministic,
} from '../symmetric-encryption'
import { sha256 } from '../hash'
import { Message } from './message'
import { ECPointOnCurveT, KeyPair, PublicKeyT } from '../elliptic-curve'
import { SealedMessage } from './sealedMessage'

type CalculateSharedSecretInput = Readonly<{
	ephemeralPublicKey: PublicKeyT
	diffieHellmanPoint: () => ResultAsync<ECPointOnCurveT, Error>
}>

const calculateSharedSecret = (
	input: CalculateSharedSecretInput,
): ResultAsync<Buffer, Error> => {
	const { diffieHellmanPoint } = input
	return diffieHellmanPoint().map((dhKey: ECPointOnCurveT) => {
		const ephemeralPoint = input.ephemeralPublicKey.decodeToPointOnCurve()
		const sharedSecretPoint = dhKey.add(ephemeralPoint)
		const data = sharedSecretPoint.x.toString(16)
		const buf = Buffer.alloc(32)
		buf.write(data.padStart(64, '0'), 'hex')
		return buf
	})
}

const kdf = (secret: Buffer, nonce: Buffer): ResultAsync<Buffer, Error> => {
	const salt = sha256(nonce)
	return Scrypt.deriveKey({
		password: secret,
		kdf: 'scrypt',
		params: ScryptParams.create({ salt }),
	})
}

const decryptAESSealedBox = (
	input: Readonly<{
		aesSealedBox: AES_GCM_SealedBoxT
		sharedSecret: Buffer
		additionalAuthenticationData: Buffer
	}>,
): ResultAsync<Buffer, Error> => {
	const nonce = input.aesSealedBox.nonce
	const { additionalAuthenticationData } = input

	return kdf(input.sharedSecret, nonce)
		.map(symmetricKey => ({
			...input.aesSealedBox,
			symmetricKey,
			additionalAuthenticationData,
		}))
		.andThen(AES_GCM.open)
}

const aesSealedBoxFromSealedMessage = (
	sealedMessage: SealedMessageT,
): Result<AES_GCM_SealedBoxT, Error> =>
	AES_GCM_SealedBox.create({
		authTag: sealedMessage.authTag,
		ciphertext: sealedMessage.ciphertext,
		nonce: sealedMessage.nonce,
	})

const decryptMessage = (
	input: Readonly<{
		sealedMessage: SealedMessageT
		diffieHellmanPoint: () => ResultAsync<ECPointOnCurveT, Error>
	}>,
): ResultAsync<Buffer, Error> => {
	const ephemeralPublicKey = input.sealedMessage.ephemeralPublicKey

	const additionalAuthenticationData = ephemeralPublicKey.asData({
		compressed: true,
	})

	return combine([
		aesSealedBoxFromSealedMessage(input.sealedMessage).asyncAndThen(
			okAsync,
		),
		calculateSharedSecret({
			...input,
			ephemeralPublicKey,
		}),
	])
		.map(resultList => {
			const aesSealedBox = resultList[0] as AES_GCM_SealedBoxT
			const sharedSecret = resultList[1] as Buffer
			return {
				aesSealedBox,
				sharedSecret,
				additionalAuthenticationData,
			}
		})
		.andThen(decryptAESSealedBox)
}

const decryptEncryptedMessageBuffer = (
	input: Readonly<{
		messageBuffer: Buffer
		diffieHellmanPoint: () => ResultAsync<ECPointOnCurveT, Error>
	}>,
): ResultAsync<Buffer, Error> =>
	Message.fromBuffer(input.messageBuffer)
		.andThen(
			(
				message: EncryptedMessageT | PlaintextMessageT,
			): Result<Parameters<typeof decryptMessage>, Error> =>
				message.kind === 'ENCRYPTED'
					? ok([
							{
								diffieHellmanPoint: input.diffieHellmanPoint,
								sealedMessage: message.sealedMessage,
							},
					  ])
					: err(
							Error(
								`Expected an encrypted message, but got a plaintext message.`,
							),
					  ),
		)
		.asyncAndThen(a => decryptMessage(...a))

const decrypt = (input: MessageDecryptionInput): ResultAsync<Buffer, Error> =>
	Buffer.isBuffer(input.encryptedMessage)
		? decryptEncryptedMessageBuffer({
				...input,
				messageBuffer: input.encryptedMessage,
		  })
		: decryptMessage({
				...input,
				sealedMessage: input.encryptedMessage.sealedMessage,
		  })

type DeterministicMessageEncryptionInput = MessageEncryptionInput &
	Readonly<{
		nonce: Buffer
		ephemeralPublicKey: PublicKeyT
	}>

const encodePlaintext = (plaintext: Buffer | string): Buffer =>
	typeof plaintext === 'string' ? Buffer.from(plaintext, 'utf-8') : plaintext

const __encryptDeterministic = (
	input: DeterministicMessageEncryptionInput,
): ResultAsync<EncryptedMessageT, Error> => {
	const { nonce, ephemeralPublicKey } = input

	const additionalAuthenticationData = ephemeralPublicKey.asData({
		compressed: true,
	})

	const plaintext = encodePlaintext(input.plaintext)

	if (plaintext.length > Message.maxLengthOfCipherTextOfSealedMsg) {
		const errMsg = `Plaintext is too long, expected max #${Message.maxLengthOfCipherTextOfSealedMsg}, but got: #${plaintext.length}`
		return errAsync(new Error(errMsg))
	}

	return calculateSharedSecret({
		...input,
	}).andThen(sharedSecret =>
		kdf(sharedSecret, nonce)
			.andThen(symmetricKey =>
				aesGCMSealDeterministic({
					nonce,
					plaintext,
					additionalAuthenticationData,
					symmetricKey,
				}),
			)
			.andThen(s => SealedMessage.fromAESSealedBox(s, ephemeralPublicKey))
			.andThen((sealedMessage: SealedMessageT) =>
				Message.createEncrypted(
					EncryptionScheme.DH_ADD_EPH_AESGCM256_SCRYPT_000,
					sealedMessage,
				),
			),
	)
}

const encrypt = (
	input: MessageEncryptionInput,
): ResultAsync<EncryptedMessageT, Error> => {
	const secureRandom = input.secureRandom ?? secureRandomGenerator

	const nonce = Buffer.from(
		secureRandom.randomSecureBytes(AES_GCM.nonceLength),
		'hex',
	)

	const ephemeralKeyPair = KeyPair.generateNew(secureRandom)

	const ephemeralPublicKey = ephemeralKeyPair.publicKey

	return __encryptDeterministic({
		...input,
		nonce,
		ephemeralPublicKey,
	})
}

export const MessageEncryption = {
	__encryptDeterministic,
	encodePlaintext,
	encrypt,
	decrypt,
}
