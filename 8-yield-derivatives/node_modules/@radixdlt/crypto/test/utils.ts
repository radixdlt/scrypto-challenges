import { UInt256 } from '@radixdlt/uint256'
import { SignatureT } from '../'

export const signatureFromHexStrings = (input: {
	r: string
	s: string
}): SignatureT => {
	const r = new UInt256(input.r, 16)
	const s = new UInt256(input.s, 16)
	return {
		r,
		s,
		toDER: () => 'not_impl',
		equals: (other: SignatureT): boolean => r.eq(other.r) && s.eq(other.s),
	}
}
