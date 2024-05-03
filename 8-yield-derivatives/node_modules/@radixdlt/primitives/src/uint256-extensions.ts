import BN from 'bn.js'
import { err, Result, ok } from 'neverthrow'
import { UInt256 } from '@radixdlt/uint256'
import {
	isNumberArray,
	SecureRandom,
	secureRandomGenerator,
} from '@radixdlt/util'

const bnUInt256Max: BN = new BN(2).pow(new BN(256)).sub(new BN(1))

export const fitsInUInt256 = (number: BN | number): boolean => {
	const bn = new BN(number)
	const isNotTooBig = bn.lte(bnUInt256Max)
	const isNonNegative = bn.gte(new BN(0))
	return isNotTooBig && isNonNegative
}

/**
 * Converts a big number (BN) into a UInt256
 *
 * @param {BN} bn - A big number to be converted into a UInt256.
 * @returns {UInt256} A 256 bit wide unsigned integer.
 */
export const uint256FromBN = (bn: BN): Result<UInt256, Error> => {
	if (!fitsInUInt256(bn)) {
		return err(
			new Error(
				`BN is either less than 0 or larger than 2^256 - 1, which does not fit in a UInt256.`,
			),
		)
	}
	return ok(new UInt256(bn.toString('hex'), 16))
}

export type UInt256InputUnsafe =
	| number
	| string
	| number[]
	| Uint8Array
	| Buffer

// eslint-disable-next-line complexity
export const isUnsafeInputForUInt256 = (
	something: unknown,
): something is UInt256InputUnsafe => {
	if (typeof something === 'number') {
		return true
	} else if (typeof something === 'string') {
		return true
	} else if (isNumberArray(something)) {
		return true
	} else if (something instanceof Uint8Array) {
		return true
	} else return something instanceof Buffer
}

export const uint256FromUnsafe = (
	unsafe: UInt256InputUnsafe,
): Result<UInt256, Error> => {
	// eslint-disable-next-line functional/no-try-statement
	try {
		const bn = new BN(unsafe)
		return uint256FromBN(bn)
	} catch (e) {
		return err(e as Error)
	}
}

export const bnFromUInt256 = (uint256: UInt256): BN =>
	new BN(uint256.toString(16), 'hex')

export const uint256Max = uint256FromBN(bnUInt256Max)._unsafeUnwrap()

export const secureRandomUInt256 = (
	secureRandom: SecureRandom = secureRandomGenerator,
): UInt256 => {
	const randomBytes = secureRandom.randomSecureBytes(32)
	return new UInt256(randomBytes, 16)
}

export const isUInt256 = (something: unknown): something is UInt256 =>
	something instanceof UInt256
