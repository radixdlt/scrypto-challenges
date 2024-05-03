import { secureRandomGenerator } from '@radixdlt/util'
import { AES_GCM } from '../src'

describe('aes gcm', () => {
	it('should decrypt message from Swift lib', () => {
		const decrypted = AES_GCM.open({
			authTag: Buffer.from('baa17313cf02d4f34d135c34643426c8', 'hex'),
			ciphertext: Buffer.from(
				'd2d309e9d7c9c5f5ca692b3e51e4f84670e8ee3dfce4d183389e6fcea2f46a707101e38a6aff1754e57c533b6bea0f620d5bac85ec9a2f86352111e2fc2879c839b6ae0d931c30364d5245bcad69',
				'hex',
			),
			nonce: Buffer.from('bf9e592c50183db30afff24d', 'hex'),
			symmetricKey: Buffer.from(
				'fbe8c9f0dcbdbf52ee3038b18ca378255c35583bae12eb50151d7458d43dc3e1',
				'hex',
			),
			additionalAuthenticationData: Buffer.from(
				'03B7F98F3FB16527A8F779C326A7A57261F1341EAC191011F7A916D01D668F4549',
				'hex',
			),
		})._unsafeUnwrap()

		const plaintext = decrypted.toString('utf-8')
		expect(plaintext).toBe(
			`Guten tag Joe! My nukes are 100 miles south west of Münich, don't tell anyone`,
		)
	})

	it('should be able to decrypt what was just encrypted', () => {
		const plaintext = 'Hey TypeScript lib, this is TypeScript lib!'
		const symmetricKey = Buffer.from(
			secureRandomGenerator.randomSecureBytes(32),
			'hex',
		)
		const sealedBox = AES_GCM.seal({
			plaintext: Buffer.from(plaintext),
			symmetricKey,
		})._unsafeUnwrap()

		const decrypted = AES_GCM.open({
			...sealedBox,
			symmetricKey,
		})._unsafeUnwrap()

		expect(decrypted.toString('utf-8')).toBe(plaintext)
	})
})
