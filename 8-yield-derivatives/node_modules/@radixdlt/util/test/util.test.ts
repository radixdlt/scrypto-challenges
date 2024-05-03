import { secureRandomGenerator } from '../src'

const testGenerateBytes = (byteCount: number): string => {
	const bytes = secureRandomGenerator.randomSecureBytes(byteCount)
	expect(bytes.length).toBe(2 * byteCount)
	return bytes
}

describe('util', () => {
	it('can securely generate random bytes', () => {
		const byteCount = 8
		const byteStrings = [...Array(1024)].map((_, i) => {
			return testGenerateBytes(byteCount)
		})
		const uniqueByteStrings = new Set(byteStrings)
		// Probability of collision is: 2^10/2^64 <=> 1/2^54 = 5e-17 <=> VERY low probability.
		expect(uniqueByteStrings.size).toBe(byteStrings.length)
	})
})
