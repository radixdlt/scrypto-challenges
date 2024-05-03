import {
	BIP32,
	hardenedIncrement,
	HDMasterSeed,
	HDNode,
	HDPathRadix,
} from '../src'
import { fromString } from 'long'

const publicKeyToAddress = require('ethereum-public-key-to-address')

type BIP32VectorDerivation = {
	path: string
	xpub: string
	xprv: string
}

type BIP32Vector = {
	name: string
	info?: string
	seed: string
	derivations: BIP32VectorDerivation[]
}

/// https://github.com/bitcoin/bips/blob/b0521f076c0b40208e82208f5476c48071aab785/bip-0032.mediawiki
const bip32Vectors: BIP32Vector[] = [
	{
		name: 'Test vector 1',
		seed: '000102030405060708090a0b0c0d0e0f',
		derivations: [
			{
				path: "m/0'",
				xpub:
					'xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw',
				xprv:
					'xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7',
			},
			{
				path: "m/0'/1",
				xpub:
					'xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ',
				xprv:
					'xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs',
			},
			{
				path: "m/0'/1/2'",
				xpub:
					'xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5',
				xprv:
					'xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM',
			},
			{
				path: "m/0'/1/2'/2",
				xpub:
					'xpub6FHa3pjLCk84BayeJxFW2SP4XRrFd1JYnxeLeU8EqN3vDfZmbqBqaGJAyiLjTAwm6ZLRQUMv1ZACTj37sR62cfN7fe5JnJ7dh8zL4fiyLHV',
				xprv:
					'xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334',
			},
			{
				path: "m/0'/1/2'/2/1000000000",
				xpub:
					'xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy',
				xprv:
					'xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76',
			},
		],
	},
	{
		name: 'Test vector 2',
		seed:
			'fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542',
		derivations: [
			{
				path: 'm',
				xpub:
					'xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB',
				xprv:
					'xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U',
			},
			{
				path: 'm/0',
				xpub:
					'xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH',
				xprv:
					'xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt',
			},
			{
				path: "m/0/2147483647'",
				xpub:
					'xpub6ASAVgeehLbnwdqV6UKMHVzgqAG8Gr6riv3Fxxpj8ksbH9ebxaEyBLZ85ySDhKiLDBrQSARLq1uNRts8RuJiHjaDMBU4Zn9h8LZNnBC5y4a',
				xprv:
					'xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9',
			},
			{
				path: "m/0/2147483647'/1",
				xpub:
					'xpub6DF8uhdarytz3FWdA8TvFSvvAh8dP3283MY7p2V4SeE2wyWmG5mg5EwVvmdMVCQcoNJxGoWaU9DCWh89LojfZ537wTfunKau47EL2dhHKon',
				xprv:
					'xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef',
			},
			{
				path: "m/0/2147483647'/1/2147483646'",
				xpub:
					'xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL',
				xprv:
					'xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc',
			},
			{
				path: "m/0/2147483647'/1/2147483646'/2",
				xpub:
					'xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt',
				xprv:
					'xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j',
			},
		],
	},
	{
		name: 'Test vector 3',
		info:
			'These vectors test for the retention of leading zeros. See https://github.com/bitpay/bitcore-lib/issues/47 and https://github.com/iancoleman/bip39/issues/58 for more information.',
		seed:
			'4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be',
		derivations: [
			{
				path: 'm',
				xpub:
					'xpub661MyMwAqRbcEZVB4dScxMAdx6d4nFc9nvyvH3v4gJL378CSRZiYmhRoP7mBy6gSPSCYk6SzXPTf3ND1cZAceL7SfJ1Z3GC8vBgp2epUt13',
				xprv:
					'xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6',
			},
			{
				path: "m/0'",
				xpub:
					'xpub68NZiKmJWnxxS6aaHmn81bvJeTESw724CRDs6HbuccFQN9Ku14VQrADWgqbhhTHBaohPX4CjNLf9fq9MYo6oDaPPLPxSb7gwQN3ih19Zm4Y',
				xprv:
					'xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L',
			},
		],
	},
	{
		name: 'Test vector 4',
		info:
			'These vectors test for the retention of leading zeros. See https://github.com/btcsuite/btcutil/issues/172 for more information.',
		seed:
			'3ddd5602285899a946114506157c7997e5444528f3003f6134712147db19b678',
		derivations: [
			{
				path: 'm',
				xpub:
					'xpub661MyMwAqRbcGczjuMoRm6dXaLDEhW1u34gKenbeYqAix21mdUKJyuyu5F1rzYGVxyL6tmgBUAEPrEz92mBXjByMRiJdba9wpnN37RLLAXa',
				xprv:
					'xprv9s21ZrQH143K48vGoLGRPxgo2JNkJ3J3fqkirQC2zVdk5Dgd5w14S7fRDyHH4dWNHUgkvsvNDCkvAwcSHNAQwhwgNMgZhLtQC63zxwhQmRv',
			},
			{
				path: "m/0'",
				xpub:
					'xpub69AUMk3qDBi3uW1sXgjCmVjJ2G6WQoYSnNHyzkmdCHEhSZ4tBok37xfFEqHd2AddP56Tqp4o56AePAgCjYdvpW2PU2jbUPFKsav5ut6Ch1m',
				xprv:
					'xprv9vB7xEWwNp9kh1wQRfCCQMnZUEG21LpbR9NPCNN1dwhiZkjjeGRnaALmPXCX7SgjFTiCTT6bXes17boXtjq3xLpcDjzEuGLQBM5ohqkao9G',
			},
			{
				path: "m/0'/1'",
				xpub:
					'xpub6BJA1jSqiukeaesWfxe6sNK9CCGaujFFSJLomWHprUL9DePQ4JDkM5d88n49sMGJxrhpjazuXYWdMf17C9T5XnxkopaeS7jGk1GyyVziaMt',
				xprv:
					'xprv9xJocDuwtYCMNAo3Zw76WENQeAS6WGXQ55RCy7tDJ8oALr4FWkuVoHJeHVAcAqiZLE7Je3vZJHxspZdFHfnBEjHqU5hG1Jaj32dVoS6XLT1',
			},
		],
	},
]

describe('BIP32', () => {
	it('can be created', () => {
		const bip32Path = BIP32.unsafeFromSimpleComponents([
			{ index: 12, isHardened: true },
			{ index: 23, isHardened: false },
			{ index: 34, isHardened: true },
			{ index: 45, isHardened: false },
		])._unsafeUnwrap()
		bip32Path.pathComponents.forEach((pc, index) => {
			expect(pc.level).toBe(index)
		})
		expect(bip32Path.toString()).toBe(`m/12'/23/34'/45`)
	})

	it('can be created from string', () => {
		const derivationPath = BIP32.fromString(
			`m/44'/0'/0'/0/0`,
		)._unsafeUnwrap()
		expect(derivationPath.toString()).toBe(`m/44'/0'/0'/0/0`)
	})

	it('can be created from string', () => {
		const derivationPath = BIP32.fromString(`44'`)._unsafeUnwrap()
		expect(derivationPath.toString()).toBe(`m/44'`)
	})

	it('bip32 test vectors', () => {
		bip32Vectors.forEach(vector => {
			const masterSeed = HDMasterSeed.fromSeed(
				Buffer.from(vector.seed, 'hex'),
			)
			const masterNode = masterSeed.masterNode()
			vector.derivations.forEach((vectorDerivation, index) => {
				const path = BIP32.fromString(
					vectorDerivation.path,
				)._unsafeUnwrap()
				const node = masterNode.derive(path)
				const extendedKeys = node.toJSON()
				expect(extendedKeys.xpriv).toBe(vectorDerivation.xprv)
				expect(extendedKeys.xpub).toBe(vectorDerivation.xpub)
			})
		})
	})

	it('bip32 large values', () => {
		const hdPath = HDPathRadix.create({
			account: 0x66aabbcc, // automatically hardened
			change: 1,
			address: { index: 0x55ffeedd, isHardened: false },
		})

		expect(hdPath.account.isHardened).toBe(true)
		expect(hdPath.account.value()).toBe(0x66aabbcc)
		expect(hdPath.account.value()).toBe(1722465228)
		// expect(
		// 	hdPath.account.index.equals(
		// 		fromString('0x66aabbcc', false, 16).add(hardenedIncrement),
		// 	),
		// ).toBe(true)
		expect(hdPath.account.index).toBe(0x66aabbcc + hardenedIncrement)
		expect(hdPath.account.index.toString(10)).toBe('3869948876')
		expect(hdPath.account.index.toString(16)).toBe('e6aabbcc')
		expect(hdPath.account.toString()).toBe(`1722465228'`)
	})

	/// These tests seems great because they use bip44 paths which consists of fewer than 5 components
	/// AND also test "nested" derivations, with 'sub signingKeychains'.
	/// https://github.com/wuminzhe/bip44/blob/master/test/bip44_test.rb
	describe('wuminzhe/bip44 tests', () => {
		/// https://github.com/wuminzhe/bip44/blob/master/test/bip44_test.rb#L10-L16
		it('test_get_addresses_from_wallet', () => {
			const seed =
				'895f9fe6ea00db7f97ff181e21b4303c0246e8342c3775b935ed41e7841e4234aa0331866f1931a8b5d56d7f49f6323ffb91aaaccc9522b2dbfb14aaf6960f73'
			const masterSeed = HDMasterSeed.fromSeed(Buffer.from(seed, 'hex'))
			const masterNode = masterSeed.masterNode()

			const firstHDPath = BIP32.fromString(`m/44'/60'/0'`)._unsafeUnwrap()
			const first_wallet = masterNode.derive(firstHDPath)
			const first_address_hdpath = BIP32.fromString(
				`M/0/0`,
			)._unsafeUnwrap()
			const first_address_pubKey = first_wallet.derive(
				first_address_hdpath,
			).publicKey
			const first_address = publicKeyToAddress(
				first_address_pubKey.toString(false),
			)

			expect(first_address).toBe(
				'0xF35d683226dA7CC652299Ea5F8798f1A8E8812cF',
			)
		})

		/// https://github.com/wuminzhe/bip44/blob/master/test/bip44_test.rb#L36-L44
		it('test_create_wallet_from_xprv', () => {
			const seed =
				'895f9fe6ea00db7f97ff181e21b4303c0246e8342c3775b935ed41e7841e4234aa0331866f1931a8b5d56d7f49f6323ffb91aaaccc9522b2dbfb14aaf6960f73'
			const masterSeed = HDMasterSeed.fromSeed(Buffer.from(seed, 'hex'))
			const masterNode = masterSeed.masterNode()

			const firstHDPath = BIP32.fromString(
				`m/44'/60'/0'/0`,
			)._unsafeUnwrap()

			const first_wallet = masterNode.derive(firstHDPath)

			const signingKeychain = HDNode.fromExtendedPrivateKey(
				first_wallet.toJSON().xpriv,
			)._unsafeUnwrap()
			const subwallet = signingKeychain.derive(
				BIP32.fromString(`M/0`)._unsafeUnwrap(),
			)

			const first_address = publicKeyToAddress(
				subwallet.publicKey.toString(false),
			)

			expect(first_address).toBe(
				'0xF35d683226dA7CC652299Ea5F8798f1A8E8812cF',
			)
		})
	})
})
