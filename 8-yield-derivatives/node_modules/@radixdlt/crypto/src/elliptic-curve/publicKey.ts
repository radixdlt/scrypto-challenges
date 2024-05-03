import { err, ok, Result } from 'neverthrow'
import { curve, ec } from 'elliptic'
import BN from 'bn.js'
import { buffersEquals, msgFromError } from '@radixdlt/util'
import { bnFromUInt256 } from '@radixdlt/primitives'
import { UInt256 } from '@radixdlt/uint256'
import { __pointOnCurveFromEllipticShortPoint } from './ecPointOnCurve'
import { ECPointOnCurveT, PrivateKeyT, PublicKeyT, SignatureT } from './_types'

const thirdPartyLibEllipticSecp256k1 = new ec('secp256k1')

export const isPublicKey = (something: unknown): something is PublicKeyT => {
	const inspection = something as PublicKeyT

	return (
		inspection.asData !== undefined &&
		inspection.isValidSignature !== undefined &&
		inspection.decodeToPointOnCurve !== undefined &&
		inspection.equals !== undefined &&
		inspection.toString !== undefined
	)
}

// eslint-disable-next-line max-lines-per-function
const publicKeyFromEllipticKey = (
	ecKeyPair: ec.KeyPair,
): Result<PublicKeyT, Error> => {
	const validation = ecKeyPair.validate()

	if (!validation.result) {
		return err(new Error(`Invalid privateKey: ${validation.reason}`))
	}

	const newKeyAsData = (input: { readonly compressed: boolean }): Buffer =>
		Buffer.from(ecKeyPair.getPublic(input.compressed, 'array'))

	const isValidSignature = (
		input: Readonly<{
			signature: SignatureT
			hashedMessage: Buffer
		}>,
	): boolean => {
		const message = input.hashedMessage
		const signature = input.signature
		const r = bnFromUInt256(signature.r)
		const s = bnFromUInt256(signature.s)
		return ecKeyPair.verify(new BN(message), { r, s })
	}

	const equals = (other: PublicKeyT): boolean => {
		const comparePubKeyBytes = (compressed: boolean): boolean => {
			const newKeyBytes = newKeyAsData({ compressed })
			const otherBytes = other.asData({ compressed })
			return buffersEquals(newKeyBytes, otherBytes)
		}
		return comparePubKeyBytes(true) && comparePubKeyBytes(false)
	}

	const toString = (compressed?: boolean): string =>
		newKeyAsData({ compressed: compressed ?? true }).toString('hex')

	const publicKey: PublicKeyT = {
		__hex: toString(),
		asData: newKeyAsData,
		toString,
		isValidSignature: isValidSignature,
		equals: equals,
		decodeToPointOnCurve: (): ECPointOnCurveT => {
			const shortPoint = ecKeyPair.getPublic() as curve.short.ShortPoint
			const pointOnCurveResult = __pointOnCurveFromEllipticShortPoint(
				shortPoint,
			)
			if (pointOnCurveResult.isErr())
				throw new Error(
					`Incorrect implementation, should always be able to decode a valid public key
					 into a point on the curve, but got error ${pointOnCurveResult.error.message}`,
				)
			return pointOnCurveResult.value
		},
	}

	return ok(publicKey)
}

const fromPrivateKey = (
	input: Readonly<{
		privateKey: PrivateKeyT
	}>,
): PublicKeyT => fromPrivateKeyScalar({ scalar: input.privateKey.scalar })

const fromPrivateKeyScalar = (
	input: Readonly<{
		scalar: UInt256
	}>,
): PublicKeyT => {
	const result = publicKeyFromEllipticKey(
		thirdPartyLibEllipticSecp256k1.keyFromPrivate(
			input.scalar.toString(16),
		),
	)

	if (result.isErr()) {
		throw new Error(
			`Failed to derive public key from private key, this should never happend since you passed in an 'PrivateKeyT' type value, which should have been validated. You must somehow have bypassed validation, or our implementation is incorrect, which is a fatal error.`,
		)
	}
	return result.value
}

const fromBuffer = (publicKeyBytes: Buffer): Result<PublicKeyT, Error> => {
	try {
		const ecKeyPairElliptic = thirdPartyLibEllipticSecp256k1.keyFromPublic(
			publicKeyBytes,
		)
		return publicKeyFromEllipticKey(ecKeyPairElliptic)
	} catch (e) {
		const underlyingError = msgFromError(e)
		const errMsg = `Failed to decode bytes into public key, underlying error: ${underlyingError}. bytes: '${publicKeyBytes.toString(
			'hex',
		)}'`
		console.error(errMsg)
		return err(new Error(errMsg))
	}
}

const compressedByteCount = 33

export const PublicKey = {
	compressedByteCount,
	fromBuffer,
	fromPrivateKey,
	fromPrivateKeyScalar,
}
