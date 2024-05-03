import { Wallet, WalletT } from '../src'
import { Network } from '@radixdlt/primitives'
import { SigningKeychain } from '@radixdlt/account'
import { KeystoreT, Mnemonic } from '@radixdlt/crypto'
import { makeSigningKeyChainWithFunds } from '@radixdlt/account/test/utils'

export const createWallet = (
	input?: Readonly<{
		network?: Network
		startWithInitialSigningKey?: boolean
	}>,
): WalletT => {
	const mnemonic = Mnemonic.fromEnglishPhrase(
		'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
	)._unsafeUnwrap()
	const startWithInitialSigningKey = input?.startWithInitialSigningKey ?? true
	const signingKeychain = SigningKeychain.create({
		mnemonic,
		startWithInitialSigningKey,
	})

	const network = input?.network ?? Network.MAINNET

	return Wallet.create({
		signingKeychain,
		network,
	})
}

export const makeWalletWithFunds = (network: Network): WalletT => {
	return Wallet.create({
		signingKeychain: makeSigningKeyChainWithFunds(),
		network,
	})
}

export type KeystoreForTest = {
	keystore: KeystoreT
	password: string
	expectedSecret: string
	expectedMnemonicPhrase: string
	publicKeysCompressed: string[]
}

export const keystoreForTest: KeystoreForTest = {
	password: 'my super strong passaword',
	expectedMnemonicPhrase:
		'legal winner thank year wave sausage worth useful legal winner thank yellow',
	expectedSecret: '7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f',
	keystore: {
		crypto: {
			cipher: 'AES-GCM',
			cipherparams: {
				nonce: 'd82fd275598b9b288b8c376d',
			},
			ciphertext: '208e520802bd17d7a569333df41dfd2d',
			kdf: 'scrypt',
			kdfparams: {
				costParameterN: 8192,
				costParameterC: 262144,
				blockSize: 8,
				parallelizationParameter: 1,
				lengthOfDerivedKey: 32,
				salt:
					'cb2227c6782493df3e822c9f6cd1131dea14e135751215d66f48227383b80acd',
			},
			mac: '68bc72c6a6a89c7fe4eb5fda4f4163e0',
		},
		id: 'b34999409a491037',
		version: 1,
	},
	// 1. input seed at https://iancoleman.io/bip39/
	// 2. change to BIP32 and enter derivation path: m/44'/1022'/0'/0
	// 3. Check 'use hardened addresses' checkbox
	// 4. Copy Public Key from table
	publicKeysCompressed: [
		'036d39bd3894fa2193f1ffc62236bfadf3d3c051e8fe9ca5cc02677ea5e1ad34e8',
		'020eb0759d87beb9f97056dc8b3aee12c4b02ad37dd4d259e163a87b273cea8b54',
		'0319ed42cc998f7cfa60e568b3c9f631b47582051affc478f68ea3727a977012e0',
		'028d31597419a690f369a079dfc54276b643836189a375b56d8c1983bffbb53c36',
		'0269f0794113243f60cf8a0ceceffcce93220d7e8531883eac24054d95998dd942',
		'025974fa70072cba176a89afeb81b2a93ec8cde196014ff97f4d0a9da8c11ceca1',
	],
}
