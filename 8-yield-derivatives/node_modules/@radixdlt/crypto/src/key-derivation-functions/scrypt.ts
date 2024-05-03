import { scrypt } from 'scrypt-js'
import { ResultAsync, errAsync } from 'neverthrow'
import { ScryptParamsT } from './_types'
import {
	msgFromError,
	SecureRandom,
	secureRandomGenerator,
} from '@radixdlt/util'

const deriveKey = (
	input: Readonly<{
		password: Buffer
		kdf: string
		params: ScryptParamsT
	}>,
): ResultAsync<Buffer, Error> => {
	if (input.kdf !== 'scrypt')
		return errAsync(new Error('Wrong KDF, expected scrypt'))
	const { params, password: key } = input
	const {
		lengthOfDerivedKey: dklen,
		costParameterN: n,
		blockSize: r,
		parallelizationParameter: p,
	} = params
	const salt = Buffer.from(params.salt, 'hex')

	return ResultAsync.fromPromise(
		scrypt(key, salt, n, r, p, dklen).then(uint8array =>
			Buffer.from(uint8array),
		),
		(e: unknown) => {
			const underlyingErrorMessage = msgFromError(e)
			return new Error(
				`Failed to derive data using scrypt, underlying error: '${underlyingErrorMessage}'`,
			)
		},
	)
}

export const Scrypt = {
	deriveKey,
}

const create = (
	input: Readonly<{
		salt?: Buffer
		secureRandom?: SecureRandom
	}>,
): ScryptParamsT => {
	const secureRandom = input.secureRandom ?? secureRandomGenerator
	if (input.salt && input.salt.length !== 32)
		throw new Error('Incorrect implementatin expected 32 bytes salt')
	const salt =
		input.salt?.toString('hex') ?? secureRandom.randomSecureBytes(32)

	return {
		costParameterN: 8192,
		costParameterC: 262144,
		blockSize: 8,
		parallelizationParameter: 1,
		lengthOfDerivedKey: 32,
		salt: salt,
	}
}

export const ScryptParams = {
	create,
}
