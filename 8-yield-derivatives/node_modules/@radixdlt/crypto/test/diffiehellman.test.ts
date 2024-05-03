import { KeyPair } from '../src'

describe('diffiehellman', () => {
	it('works between two', async () => {
		const alice = KeyPair.generateNew()
		const bob = KeyPair.generateNew()

		const dhAB = (
			await alice.privateKey.diffieHellman(bob.publicKey)
		)._unsafeUnwrap()
		const dhBA = (
			await bob.privateKey.diffieHellman(alice.publicKey)
		)._unsafeUnwrap()

		expect(dhAB.equals(dhBA)).toBe(true)
	})
})
