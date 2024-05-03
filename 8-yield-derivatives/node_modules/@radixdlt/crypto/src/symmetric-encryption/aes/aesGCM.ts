import { cipher as forgeCipher, util as forgeUtil } from 'node-forge'
import {
	AES_GCM_OPEN_Input,
	AES_GCM_SEAL_Input,
	AES_GCM_SealedBoxT,
} from './_types'
import { err, ok, Result } from 'neverthrow'
import { secureRandomGenerator } from '@radixdlt/util'
import { AES_GCM_SealedBox } from './aesGCMSealedBox'

const AES_GCM_256_ALGORITHM = 'AES-GCM'

export const aesGCMSealDeterministic = (
	input: Omit<AES_GCM_SEAL_Input, 'secureRandom'> & {
		readonly nonce: Buffer
	},
): Result<AES_GCM_SealedBoxT, Error> => {
	const { nonce } = input

	const aesCipher = forgeCipher.createCipher(
		AES_GCM_256_ALGORITHM,
		forgeUtil.createBuffer(input.symmetricKey),
	)

	const iv = forgeUtil.createBuffer(nonce)
	const startOptions = { iv }

	if (input.additionalAuthenticationData) {
		aesCipher.start({
			...startOptions,
			additionalData: forgeUtil.hexToBytes(
				input.additionalAuthenticationData.toString('hex'),
			),
		})
	} else {
		aesCipher.start(startOptions)
	}

	aesCipher.update(forgeUtil.createBuffer(input.plaintext))

	if (!aesCipher.finish()) {
		throw new Error(`AES encryption failed, error unknown...`)
	}

	const ciphertext = Buffer.from(aesCipher.output.toHex(), 'hex')
	const authTag = Buffer.from(aesCipher.mode.tag.toHex(), 'hex')

	return AES_GCM_SealedBox.create({
		ciphertext,
		authTag,
		nonce,
	})
}

const seal = (input: AES_GCM_SEAL_Input): Result<AES_GCM_SealedBoxT, Error> => {
	const secureRandom = input.secureRandom ?? secureRandomGenerator
	const nonce =
		input.nonce ??
		Buffer.from(
			secureRandom.randomSecureBytes(AES_GCM_SealedBox.nonceLength),
			'hex',
		)
	return aesGCMSealDeterministic({ ...input, nonce })
}

const open = (input: AES_GCM_OPEN_Input): Result<Buffer, Error> => {
	const { ciphertext, additionalAuthenticationData, symmetricKey } = input

	return AES_GCM_SealedBox.create(input).andThen(
		(box: AES_GCM_SealedBoxT) => {
			const nonce = box.nonce
			const authTag = box.authTag

			const decipher = forgeCipher.createDecipher(
				AES_GCM_256_ALGORITHM,
				forgeUtil.createBuffer(symmetricKey),
			)

			const iv = forgeUtil.createBuffer(nonce)
			const tag = forgeUtil.createBuffer(authTag)

			const startOptions = {
				iv,
				tag,
			}

			if (additionalAuthenticationData) {
				const additionalData = forgeUtil.hexToBytes(
					additionalAuthenticationData.toString('hex'),
				)
				decipher.start({
					...startOptions,
					additionalData,
				})
			} else {
				decipher.start(startOptions)
			}

			decipher.update(forgeUtil.createBuffer(ciphertext))

			if (!decipher.finish()) {
				return err(new Error(`AES decryption failed.`))
			}

			return ok(Buffer.from(decipher.output.toHex(), 'hex'))
		},
	)
}

export const AES_GCM = {
	seal,
	open,
	tagLength: AES_GCM_SealedBox.tagLength,
	nonceLength: AES_GCM_SealedBox.nonceLength,
	algorithm: AES_GCM_256_ALGORITHM,
}
