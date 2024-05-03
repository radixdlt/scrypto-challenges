import {
	resultToAsync,
	secureRandomUInt256,
	uint256Max,
} from '@radixdlt/primitives'

import { UInt256 } from '@radixdlt/uint256'

import { err, errAsync, ok, okAsync, Result, ResultAsync } from 'neverthrow'

import { SecureRandom, secureRandomGenerator } from '@radixdlt/util'
import { Secp256k1 } from './secp256k1'
import { sha256Twice } from '../hash'
import { ec } from 'elliptic'
import { PublicKey } from './publicKey'
import {
	DiffieHellman,
	ECPointOnCurveT,
	PrivateKeyT,
	PublicKeyT,
	SignatureT,
} from './_types'
import { Hasher } from '../_types'
import { Signature } from './signature'
import { toPrivateKeyHex } from '../utils'

const __signDataWithPrivateKey = (
	input: Readonly<{
		privateKey: UInt256
		data: Buffer
	}>,
): Result<SignatureT, Error> => {
	// log.info(`Signing ${input.data.toString()} with private key.`)
	const thirdPartyLibEllipticSecp256k1 = new ec('secp256k1')

	const privateKey = thirdPartyLibEllipticSecp256k1.keyFromPrivate(
		input.privateKey.toString(16),
	)

	const ellipticSignature: ec.Signature = privateKey.sign(input.data, {
		canonical: true,
	})

	return Signature.fromIndutnyElliptic(ellipticSignature)
}

const __privateKeyFromValidatedScalar = (scalar: UInt256): PrivateKeyT => {
	const sign = (hashedMessage: Buffer): ResultAsync<SignatureT, Error> => {
		if (hashedMessage.length !== 32) {
			return errAsync(
				new Error(
					'Incorrect length of message to sign, expected 32 bytes.',
				),
			)
		}
		return resultToAsync(
			__signDataWithPrivateKey({
				privateKey: scalar,
				data: hashedMessage,
			}),
		)
	}

	const diffieHellman: DiffieHellman = (
		publicKeyOfOtherParty: PublicKeyT,
	): ResultAsync<ECPointOnCurveT, Error> =>
		okAsync(
			publicKeyOfOtherParty
				.decodeToPointOnCurve()
				.multiplyWithPrivateKey(privateKey),
		)

	const privateKey = {
		sign,
		diffieHellman: diffieHellman,
		signUnhashed: (
			input: Readonly<{
				msgToHash: Buffer | string
				hasher?: Hasher
			}>,
		): ResultAsync<SignatureT, Error> => {
			const hasher = input.hasher ?? sha256Twice

			const hashedMessage = hasher(input.msgToHash)

			return sign(hashedMessage)
		},

		publicKey: () => {
			throw new Error('Overridden below.')
		},

		toString: () => toPrivateKeyHex(scalar),
		scalar: scalar,
	}

	return {
		...privateKey,
		publicKey: (): PublicKeyT => PublicKey.fromPrivateKey({ privateKey }),
	}
}

export const fromBuffer = (buffer: Buffer): Result<PrivateKeyT, Error> =>
	fromHex(buffer.toString('hex'))

export const fromHex = (
	privateKeyHexString: string,
): Result<PrivateKeyT, Error> =>
	fromScalar(new UInt256(privateKeyHexString, 16))

const fromScalar = (scalar: UInt256): Result<PrivateKeyT, Error> => {
	if (!validateSecp256k1PrivateKey(scalar))
		return err(new Error('Invalid private key scalar.'))

	return ok(__privateKeyFromValidatedScalar(scalar))
}

const validateSecp256k1PrivateKey = (scalar: UInt256): boolean =>
	scalar.gte(UInt256.valueOf(1)) && scalar.lte(Secp256k1.order)

const generateNew = (
	secureRandom: SecureRandom = secureRandomGenerator,
): PrivateKeyT => {
	// eslint-disable-next-line functional/no-let
	let scalar: UInt256 = uint256Max
	// eslint-disable-next-line functional/no-loop-statement
	while (!validateSecp256k1PrivateKey(scalar)) {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment,@typescript-eslint/no-unsafe-call
		scalar = secureRandomUInt256(secureRandom)
	}
	return __privateKeyFromValidatedScalar(scalar)
}

export const PrivateKey = {
	generateNew,
	fromScalar,
	fromHex,
	fromBuffer,
}
