import { err, ResultAsync, ok, Result } from 'neverthrow'
import { KeystoreCryptoT, KeystoreT } from './_types'
import {
	AES_GCM,
	AES_GCM_SealedBox,
	AES_GCM_OPEN_Input,
	AES_GCM_SealedBoxT,
} from '../symmetric-encryption'
import {
	SecureRandom,
	secureRandomGenerator,
	log,
	msgFromError,
} from '@radixdlt/util'
import {
	Scrypt,
	ScryptParams,
	ScryptParamsT,
} from '../key-derivation-functions'
import { sha256 } from '../hash'

const validatePassword = (password: string): Result<string, Error> =>
	ok(password) // no validation for now...

const encryptSecret = (
	input: Readonly<{
		secret: Buffer
		password: string
		memo?: string // e.g. 'Business wallet' or 'My husbands wallet' etc.
		kdf?: string
		kdfParams?: ScryptParamsT
		secureRandom?: SecureRandom
	}>,
): ResultAsync<KeystoreT, Error> => {
	const secureRandom = input.secureRandom ?? secureRandomGenerator

	const kdf = input.kdf ?? 'scrypt'
	const params = input.kdfParams ?? ScryptParams.create({ secureRandom })
	const memo = input.memo ?? Date.now().toLocaleString()

	return validatePassword(input.password)
		.map(p => ({ kdf, params, password: Buffer.from(p) }))
		.asyncAndThen(inp => Scrypt.deriveKey(inp))
		.map(derivedKey => ({
			plaintext: input.secret,
			symmetricKey: derivedKey,
		}))
		.andThen(inp => AES_GCM.seal(inp))
		.map(sealedBox => {
			const cipherText = sealedBox.ciphertext
			const mac = sealedBox.authTag

			const id = sha256(cipherText).toString('hex').slice(-16)

			log.debug(
				`Created Keystore with id='${id}' and memo='${memo}' (non of these are sensisitve).`,
			)
			return {
				memo,
				crypto: {
					cipher: AES_GCM.algorithm,
					cipherparams: {
						nonce: sealedBox.nonce.toString('hex'),
					},
					ciphertext: cipherText.toString('hex'),
					kdf,
					kdfparams: params,
					mac: mac.toString('hex'),
				},
				id,
				version: 1,
			}
		})
}

const decrypt = (
	input: Readonly<{
		keystore: KeystoreT
		password: string
	}>,
): ResultAsync<Buffer, Error> => {
	const { keystore, password } = input
	const kdf = keystore.crypto.kdf
	const encryptedPrivateKey = Buffer.from(keystore.crypto.ciphertext, 'hex')
	const params = keystore.crypto.kdfparams

	return AES_GCM_SealedBox.create({
		nonce: Buffer.from(keystore.crypto.cipherparams.nonce, 'hex'),
		authTag: Buffer.from(keystore.crypto.mac, 'hex'),
		ciphertext: encryptedPrivateKey,
	}).asyncAndThen((aesSealBox: AES_GCM_SealedBoxT) => {
		const aesOpenInput: Omit<AES_GCM_OPEN_Input, 'symmetricKey'> = {
			...aesSealBox,
		}

		return validatePassword(password)
			.map((p: string) => ({ kdf, params, password: Buffer.from(p) }))
			.asyncAndThen(inp => Scrypt.deriveKey(inp))
			.map(
				(derivedKey: Buffer): AES_GCM_OPEN_Input => {
					log.info(
						`[Decrypting Keystore] successfully derived key using KDF ('${keystore.crypto.kdf}')`,
					)
					return {
						...aesOpenInput,
						symmetricKey: derivedKey,
					}
				},
			)
			.andThen(
				(inp): Result<Buffer, Error> =>
					AES_GCM.open(inp).mapErr(e => {
						const underlyingError = msgFromError(e)
						const errMsg = `Failed to decrypt keystore, wrong password? Underlying error: '${underlyingError}'.`
						console.error(errMsg)
						return new Error(errMsg)
					}),
			)
			.map(decrypted => {
				log.debug(
					`Successfully decrypted Keystore with id='${keystore.id}'`,
				)
				return decrypted
			})
	})
}

const fromBuffer = (keystoreBuffer: Buffer): Result<KeystoreT, Error> => {
	try {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment,@typescript-eslint/no-unsafe-call
		const keystore = JSON.parse(keystoreBuffer.toString())
		if (isKeystore(keystore)) return ok(keystore)
		const errMsg = 'Parse object, but is not a keystore'
		log.error(errMsg)
		return err(new Error(errMsg))
	} catch (e: unknown) {
		const underlying = msgFromError(e)
		const errMsg = `Failed to parse keystore from JSON data, underlying error: ${underlying}`
		log.error(errMsg)
		return err(new Error(errMsg))
	}
}

const isScryptParams = (something: unknown): something is ScryptParamsT => {
	const inspection = something as ScryptParamsT
	return (
		inspection.blockSize !== undefined &&
		inspection.costParameterC !== undefined &&
		inspection.costParameterN !== undefined &&
		inspection.lengthOfDerivedKey !== undefined &&
		inspection.parallelizationParameter !== undefined &&
		inspection.salt !== undefined
	)
}

const isKeystoreCrypto = (something: unknown): something is KeystoreCryptoT => {
	const inspection = something as KeystoreCryptoT
	return (
		inspection.cipher !== undefined &&
		inspection.cipherparams !== undefined &&
		inspection.ciphertext !== undefined &&
		inspection.kdf !== undefined &&
		inspection.kdfparams !== undefined &&
		isScryptParams(inspection.kdfparams)
	)
}

const isKeystore = (something: unknown): something is KeystoreT => {
	const inspection = something as KeystoreT
	return (
		inspection.crypto !== undefined &&
		isKeystoreCrypto(inspection.crypto) &&
		inspection.id !== undefined &&
		inspection.version !== undefined
	)
}

export const Keystore = {
	fromBuffer,
	decrypt,
	validatePassword,
	encryptSecret,
}
