import { AccountAddress, SigningKey, ValidatorAddress } from '../src'

import {
	HDMasterSeed,
	HDPathRadix,
	Mnemonic,
	PrivateKey,
	PublicKey,
	sha256Twice,
} from '@radixdlt/crypto'
import { msgFromError } from '@radixdlt/util'
import { Network } from '@radixdlt/primitives'

describe('account_address_on_bech32_format', () => {
	describe('addr from seeded private key', () => {
		type PrivateKeySeedVector = {
			privateKeySeed: string
			expectedAddr: string
			network: Network
		}
		const privateKeySeedVectors: PrivateKeySeedVector[] = [
			{
				privateKeySeed: '00',
				expectedAddr:
					'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
				network: Network.MAINNET,
			},
			{
				privateKeySeed: 'deadbeef',
				expectedAddr:
					'rdx1qspsel805pa0nhtdhemshp7hm0wjcvd60a8ulre6zxtd2qh3x4smq3srqshyn',
				network: Network.MAINNET,
			},
			{
				privateKeySeed: 'deadbeefdeadbeef',
				expectedAddr:
					'rdx1qsp7gnv7g60plkk9lgskjghdlevyve6rtrzggk7x3fwmp4yfyjza7gcux96y8',
				network: Network.MAINNET,
			},
			{
				privateKeySeed: 'bead',
				expectedAddr:
					'rdx1qsppw0z477r695m9f9qjs3nj2vmdkd3rg4mfx7tf5v0gasrhz32jefqwm9la3',
				network: Network.MAINNET,
			},
			{
				privateKeySeed: 'aaaaaaaaaaaaaaaa',
				expectedAddr:
					'rdx1qspqsfad7e5k2k9agq74g40al0j9cllv7w0ylatvhy7m64wyrwymy5g7xqym7',
				network: Network.MAINNET,
			},
		]

		const doTest = (vector: PrivateKeySeedVector, index: number): void => {
			it(`vector_index${index}`, () => {
				const seed = Buffer.from(vector.privateKeySeed, 'hex')
				const hash = sha256Twice(seed)
				const privateKey = PrivateKey.fromBuffer(hash)._unsafeUnwrap()
				const publicKey = privateKey.publicKey()

				const addr = AccountAddress.fromPublicKeyAndNetwork({
					publicKey,
					network: vector.network,
				})
				expect(addr.toString()).toBe(vector.expectedAddr)
				expect(addr.network).toBe(vector.network)

				const parsedAddress = AccountAddress.fromUnsafe(
					vector.expectedAddr,
				)._unsafeUnwrap()
				expect(parsedAddress.toString()).toBe(vector.expectedAddr)
				expect(parsedAddress.toString()).toBe(addr.toString())
				expect(parsedAddress.publicKey.equals(publicKey)).toBe(true)

				expect(parsedAddress.equals(addr)).toBe(true)
				expect(addr.equals(parsedAddress)).toBe(true)
			})
		}

		privateKeySeedVectors.forEach((v, i) => doTest(v, i))
	})

	describe.skip('rri roundtrip', () => {
		type RRIDesVector = {
			address: string
			data: string
		}

		const reAddressToRri: RRIDesVector[] = [
			{
				address:
					'rdx1qspqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqs7cr9az',
				data: '02'.repeat(PublicKey.compressedByteCount),
			},
		]
		const doTest = (vector: RRIDesVector, index: number): void => {
			it(`rri_deserialization_vector_index${index}`, () => {
				const address = AccountAddress.fromUnsafe(
					vector.address,
				)._unsafeUnwrap()
				expect(address.toString()).toBe(vector.address)
				expect(address.publicKey.toString(true)).toBe(vector.data)
			})
		}

		reAddressToRri.forEach((v, i) => doTest(v, i))
	})

	it('addresses_for_readme', () => {
		const doTest = (input: {
			path: string
			network: Network
			isValidatorAddress: boolean
			expectedBech32: string
		}): void => {
			const { path, network, isValidatorAddress, expectedBech32 } = input
			// const publicKey = PublicKey.fromBuffer(Buffer.from(publicKeyHex, 'hex'))._unsafeUnwrapErr()

			const mnemonic = Mnemonic.fromEnglishPhrase(
				'equip will roof matter pink blind book anxiety banner elbow sun young',
			)._unsafeUnwrap()
			const hdMasterSeed = HDMasterSeed.fromMnemonic({ mnemonic })
			const hdPath = HDPathRadix.fromString(path)._unsafeUnwrap()

			const signingKey = SigningKey.fromHDPathWithHDMasterSeed({
				hdPath,
				hdMasterSeed,
			})

			const publicKey = signingKey.publicKey

			const addressInput = { publicKey, network }
			if (isValidatorAddress) {
				const address = ValidatorAddress.fromPublicKeyAndNetwork(
					addressInput,
				)
				expect(address.toString()).toBe(expectedBech32)
			} else {
				const address = AccountAddress.fromPublicKeyAndNetwork(
					addressInput,
				)
				expect(address.toString()).toBe(expectedBech32)
			}
		}

		doTest({
			path: `m/44'/1022'/0'/0/0`,
			network: Network.STOKENET,
			isValidatorAddress: false,
			expectedBech32:
				'tdx1qspmctkg7dngep54w7lkdda537x7u4acxwgk4fcfvmay55pfkcamrrcwnd0nz',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qspmctkg7dngep54w7lkdda537x7u4acxwgk4fcfvmay55pfkcamrrc0lcarp',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1qw7zaj8nv6xgd9thhant0dy03hh90wpnj942wztxlf99q2dk8wcc7ajtlks',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1qw7zaj8nv6xgd9thhant0dy03hh90wpnj942wztxlf99q2dk8wcc7ajtlks',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0'`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qsplg0a6v4qsx8hjr904h2txwu6562q50ezmgrx7ge3tajgk9smp74gh88as2',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0'`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qsplg0a6v4qsx8hjr904h2txwu6562q50ezmgrx7ge3tajgk9smp74gh88as2',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0'`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1q06rlwn9gyp3ausetad6jenhx4xjs9r7gk6qehjxv2lvj93vxc042lwzhg3',
		})

		doTest({
			path: `m/44'/1022'/0'/0/0'`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1q06rlwn9gyp3ausetad6jenhx4xjs9r7gk6qehjxv2lvj93vxc042lwzhg3',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qspa0ypecs52dwp4uym0hdvzayjemu3lses0j2pk0sls66gjw29gg3q0cpfrg',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qspa0ypecs52dwp4uym0hdvzayjemu3lses0j2pk0sls66gjw29gg3q0cpfrg',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1q0teqwwy9zntsd0pxmamtqhfykwly0uxvrujsdnu8uxkjynj32zyg4redur',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1q0teqwwy9zntsd0pxmamtqhfykwly0uxvrujsdnu8uxkjynj32zyg4redur',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3'`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qsp56t7ezjakq3043v3e662fm567ww7x0fnla9nga5xecpd0lcwpy2cvxyjm9',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3'`,
			network: Network.MAINNET,
			isValidatorAddress: false,
			expectedBech32:
				'rdx1qsp56t7ezjakq3043v3e662fm567ww7x0fnla9nga5xecpd0lcwpy2cvxyjm9',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3'`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1qdxjlkg5hdsytavtywwkjjwaxhnnh3n6vllfv68dpkwqttl7rsfzkhzl823',
		})

		doTest({
			path: `m/44'/1022'/2'/1/3'`,
			network: Network.MAINNET,
			isValidatorAddress: true,
			expectedBech32:
				'rv1qdxjlkg5hdsytavtywwkjjwaxhnnh3n6vllfv68dpkwqttl7rsfzkhzl823',
		})
	})

	describe('test non happy paths', () => {
		beforeAll(() => {
			jest.spyOn(console, 'error').mockImplementation(() => {})
		})

		afterAll(() => {
			jest.clearAllMocks()
		})

		type InvalidVector = {
			invalidAddr: string
			failureReason: string
		}

		const invalidVectors: InvalidVector[] = [
			{
				invalidAddr:
					'vb1qvz3anvawgvm7pwvjs7xmjg48dvndczkgnufh475k2tqa2vm5c6cq9u3702',
				failureReason: 'invalid hrp',
			},
			{
				invalidAddr: 'brx1xhv8x3',
				failureReason: 'invalid address length 0',
			},
			{
				invalidAddr: 'brx1qqnrjk8r',
				failureReason: 'invalid address type (0)',
			},
			{
				invalidAddr: 'brx1qsqsyqcyq5rqzjh9c6',
				failureReason: 'invalid length for address type 4',
			},
		]

		const doTest = (invalidVector: InvalidVector, index: number): void => {
			it(`invalid_vector_index${index}`, () => {
				AccountAddress.fromUnsafe(invalidVector.invalidAddr).match(
					_ => {
						throw new Error(
							`Got success, but expected failure, rri: ${invalidVector.invalidAddr}`,
						)
					},
					e => {
						expect(msgFromError(e).length).toBeGreaterThan(1)
					},
				)
			})
		}
		invalidVectors.forEach((v, i) => doTest(v, i))
	})

	it('address_fromPublicKeyAndNetwork', () => {
		const address = AccountAddress.fromPublicKeyAndNetwork({
			publicKey: PublicKey.fromBuffer(
				Buffer.from(
					'03f43fba6541031ef2195f5ba96677354d28147e45b40cde4662bec9162c361f55',
					'hex',
				),
			)._unsafeUnwrap(),
			network: Network.MAINNET,
		})
		expect(address.toString()).toBe(
			'rdx1qsplg0a6v4qsx8hjr904h2txwu6562q50ezmgrx7ge3tajgk9smp74gh88as2',
		)
	})
})
