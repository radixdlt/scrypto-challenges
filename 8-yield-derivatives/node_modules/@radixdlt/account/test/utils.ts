import { UInt256 } from '@radixdlt/uint256'
import { Mnemonic, PrivateKey } from '@radixdlt/crypto'
import { SigningKeychain, SigningKeychainT } from '../src'

export const makeSigningKeyChainWithFunds = (): SigningKeychainT => {
	const signingKeychain = SigningKeychain.create({
		startWithInitialSigningKey: false,
		mnemonic: Mnemonic.generateNew(), // not used,
	})

	const addPK = (privateKeyScalar: number): void => {
		const privateKey = PrivateKey.fromScalar(
			UInt256.valueOf(privateKeyScalar),
		)._unsafeUnwrap()
		signingKeychain.addSigningKeyFromPrivateKey({
			privateKey,
			alsoSwitchTo: true,
			name: `SigningKey with funds, privateKey: ${privateKeyScalar}`,
		})
	}

	addPK(1)
	addPK(2)
	addPK(3)
	addPK(4)
	addPK(5)

	signingKeychain.switchSigningKey('first')
	if (
		signingKeychain.__unsafeGetSigningKey().publicKey.toString(true) !==
		'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'
	) {
		throw new Error('incorrect imple')
	}

	return signingKeychain
}
