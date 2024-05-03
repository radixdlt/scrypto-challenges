import { Hasher, sha256 } from '../src'
import { sha256Twice } from '../src/hash/sha'

const testHash = (testVector: {
	hasher: Hasher
	input: Buffer
	expected: string
}): void => {
	const digest = testVector.hasher(testVector.input)
	const calculated = digest.toString('hex')
	const expected = testVector.expected
	expect(calculated).toBe(expected)
}

const testHashText = (testVector: {
	hasher: Hasher
	plainText: string
	expected: string
}): void => {
	const plainText = testVector.plainText
	const input = Buffer.from(plainText, 'utf-8')
	testHash({
		...testVector,
		input,
	})
}

describe('hashing', () => {
	// https://www.di-mgt.com.au/sha_testvectors.html
	it('can produce sha256 digests', () => {
		testHashText({
			hasher: sha256,
			plainText: 'abc',
			expected:
				'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad',
		})
	})

	it('can produce sha256 twice digests', () => {
		testHashText({
			hasher: sha256Twice,
			plainText: 'hello',
			expected:
				'9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50',
		})
	})
})
