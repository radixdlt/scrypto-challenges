import { UInt256 } from '@radixdlt/uint256'
import { ECPointOnCurve } from './ecPointOnCurve'
import { ECPointOnCurveT } from './_types'

const generator = ECPointOnCurve.fromXY({
	x: new UInt256(
		'79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
		16,
	),
	y: new UInt256(
		'483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8',
		16,
	),
})._unsafeUnwrap()

const order = new UInt256(
	'FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141',
	16,
)

const fieldSize = new UInt256(
	'FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F',
	16,
)

export enum CurveForm {
	/// Short Weierstrass (Weierstraß) form (`𝑆`), commonly used by `secp256k1`
	SHORT_WEIERSTRASS = 'ShortWeierstrass',
}

export type Curve = Readonly<{
	name: string
	/// Form, ShortWeierstrass, Edwards, TwistedEdwards or Hessian.
	form: CurveForm
	/// a.k.a. `n`
	order: UInt256
	/// a.k.a. `P` or `mod`
	fieldSize: UInt256
	/// a.k.a. `G`
	generator: ECPointOnCurveT
}>

/// The curve E: `y² = x³ + ax + b` over Fp
/// `secp256k1` Also known as the `Bitcoin curve` (though used by us at Radix, Ethereum, Zilliqa)
export const Secp256k1: Curve = {
	name: 'secp256k1',
	form: CurveForm.SHORT_WEIERSTRASS,
	order: order,
	fieldSize: fieldSize,
	generator: generator,
}
