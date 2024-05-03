import {
	SigningKeysT,
	SigningKeyT,
	SigningKeychain,
	SigningKeychainT,
} from '../src'
import { map, skip, take, toArray } from 'rxjs/operators'
import { KeystoreT, Mnemonic, PrivateKey, PublicKeyT } from '@radixdlt/crypto'
import { combineLatest, Subscription } from 'rxjs'
import { LogLevel, restoreDefaultLogLevel } from '@radixdlt/util'
import { mockErrorMsg } from '../../util/test/util'
import { log } from '@radixdlt/util'
import { UInt256 } from '@radixdlt/uint256'

const createSigningKeychain = (
	input?: Readonly<{ startWithInitialSigningKey?: boolean }>,
): SigningKeychainT => {
	const mnemonic = Mnemonic.generateNew()
	const startWithInitialSigningKey = input?.startWithInitialSigningKey ?? true
	return SigningKeychain.create({ startWithInitialSigningKey, mnemonic })
}

const createSpecificSigningKeychain = (
	input?: Readonly<{ startWithInitialSigningKey?: boolean }>,
): SigningKeychainT => {
	const mnemonic = Mnemonic.fromEnglishPhrase(
		'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about',
	)._unsafeUnwrap()
	const startWithInitialSigningKey = input?.startWithInitialSigningKey ?? true
	return SigningKeychain.create({ mnemonic, startWithInitialSigningKey })
}

const expectSigningKeychainsEqual = (
	signingKeychains: {
		signingKeychain1: SigningKeychainT
		signingKeychain2: SigningKeychainT
	},
	done: jest.DoneCallback,
): void => {
	const subs = new Subscription()
	const { signingKeychain1, signingKeychain2 } = signingKeychains
	const signingKeychain1SigningKey1PublicKey$ = signingKeychain1
		.deriveNextLocalHDSigningKey()
		.pipe(map(a => a.publicKey))
	const signingKeychain2SigningKey1PublicKey$ = signingKeychain2
		.deriveNextLocalHDSigningKey()
		.pipe(map(a => a.publicKey))

	subs.add(
		combineLatest(
			signingKeychain1SigningKey1PublicKey$,
			signingKeychain2SigningKey1PublicKey$,
		).subscribe({
			next: (keys: PublicKeyT[]) => {
				expect(keys.length).toBe(2)
				const a = keys[0]
				const b = keys[1]
				expect(a.equals(b)).toBe(true)
				done()
			},
			error: e => done(e),
		}),
	)
}

describe('signingKeychain_type', () => {
	it('can be created via keystore', async done => {
		const mnemonic = Mnemonic.generateNew()

		const password = 'super secret password'

		let load: () => Promise<KeystoreT>
		await SigningKeychain.byEncryptingMnemonicAndSavingKeystore({
			mnemonic,
			password,
			save: (keystoreToSave: KeystoreT) => {
				load = () => Promise.resolve(keystoreToSave)
				return Promise.resolve(undefined)
			},
		})
			.andThen(signingKeychain1 =>
				SigningKeychain.byLoadingAndDecryptingKeystore({
					password,
					load,
				}).map(signingKeychain2 => ({
					signingKeychain1,
					signingKeychain2,
				})),
			)
			.match(
				signingKeychains => {
					expectSigningKeychainsEqual(signingKeychains, done)
				},
				e => done(e),
			)
	})

	it('mnemonic can be retrieved with password', () => {
		const mnemonicPhrase =
			'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about'
		const mnemonic = Mnemonic.fromEnglishPhrase(
			mnemonicPhrase,
		)._unsafeUnwrap()
		const signingKeychain = SigningKeychain.create({ mnemonic })
		const mnemonicRevealed = signingKeychain.revealMnemonic()
		expect(mnemonicRevealed.equals(mnemonic)).toBe(true)
		expect(mnemonicRevealed.phrase).toBe(mnemonicPhrase)
	})

	it('the accounts derived after restoreSigningKeysUpToIndex has correct index', done => {
		const subs = new Subscription()
		const signingKeychain = createSigningKeychain({
			startWithInitialSigningKey: false,
		})

		const indexToRestoreTo = 3

		const assertSigningKeyHasIndex = (
			signingKey: SigningKeyT,
			index: number,
		): void => {
			expect(signingKey.hdPath!.addressIndex.value()).toBe(index)
		}

		subs.add(
			signingKeychain
				.restoreLocalHDSigningKeysUpToIndex(indexToRestoreTo)
				.subscribe(
					accounts => {
						expect(accounts.size()).toBe(indexToRestoreTo)

						let next = 0
						const assertSigningKeyHasCorrectIndex = (
							signingKey: SigningKeyT,
						): void => {
							assertSigningKeyHasIndex(signingKey, next)
							next += 1
						}

						for (const signingKey of accounts.all) {
							assertSigningKeyHasCorrectIndex(signingKey)
						}

						signingKeychain.deriveNextLocalHDSigningKey().subscribe(
							another0 => {
								assertSigningKeyHasCorrectIndex(another0)

								signingKeychain
									.deriveNextLocalHDSigningKey()
									.subscribe(
										another1 => {
											assertSigningKeyHasCorrectIndex(
												another1,
											)
											done()
										},
										e => done(e),
									)
							},
							e => done(e),
						)
					},
					e => {
						done(e)
					},
				),
		)
	})

	describe('failing signingKeychain scenarios', () => {
		beforeAll(() => {
			log.setLevel(LogLevel.SILENT)
		})

		afterAll(() => {
			restoreDefaultLogLevel()
		})

		it('save errors are propagated', async done => {
			const mnemonic = Mnemonic.generateNew()
			const password = 'super secret password'

			const errMsg = mockErrorMsg('SaveError')

			await SigningKeychain.byEncryptingMnemonicAndSavingKeystore({
				mnemonic,
				password,
				save: _ => Promise.reject(new Error(errMsg)),
			}).match(
				_ => done(new Error('Expected error but got none')),
				error => {
					expect(error.message).toBe(
						`Failed to save keystore, underlying error: '${errMsg}'`,
					)
					done()
				},
			)
		})

		it('load errors are propagated', async done => {
			const password = 'super secret password'

			const errMsg = mockErrorMsg('LoadError')

			await SigningKeychain.byLoadingAndDecryptingKeystore({
				password,
				load: () => Promise.reject(new Error(errMsg)),
			}).match(
				_ => done(new Error('Expected error but got none')),
				error => {
					expect(error.message).toBe(
						`Failed to load keystore, underlying error: '${errMsg}'`,
					)
					done()
				},
			)
		})
	})

	it('signingKeychain can observe accounts', done => {
		const subs = new Subscription()
		const signingKeychain = createSigningKeychain({
			startWithInitialSigningKey: true,
		})
		const expected = [1, 2]

		subs.add(
			signingKeychain
				.observeSigningKeys()
				.pipe(
					map(a => a.all.length),
					take(expected.length),
					toArray(),
				)
				.subscribe(values => {
					expect(values).toStrictEqual(expected)
					done()
				}),
		)

		subs.add(signingKeychain.deriveNextLocalHDSigningKey().subscribe())
	})

	it('can observe active signingKey', done => {
		const subs = new Subscription()
		const signingKeychain = createSigningKeychain()

		subs.add(
			signingKeychain.observeActiveSigningKey().subscribe(active => {
				expect(active.hdPath!.addressIndex.value()).toBe(0)
				expect(active.hdPath!.toString()).toBe(`m/44'/1022'/0'/0/0'`)
				expect(
					signingKeychain
						.__unsafeGetSigningKey()
						.hdPath!.equals(active.hdPath),
				).toBe(true)
				done()
			}),
		)
	})

	it('should derive next but not switch to it by default', done => {
		const signingKeychain = createSigningKeychain()
		const subs = new Subscription()

		subs.add(signingKeychain.deriveNextLocalHDSigningKey().subscribe())

		subs.add(
			signingKeychain.observeActiveSigningKey().subscribe(active => {
				expect(active.hdPath!.addressIndex.value()).toBe(0)
				done()
			}),
		)
	})

	it('should derive next and switch to it if specified', async done => {
		const subs = new Subscription()
		const signingKeychain = createSigningKeychain()

		const expectedValues = [0, 1] // we start at 0 by default, then switch to 1

		subs.add(
			signingKeychain
				.observeActiveSigningKey()
				.pipe(
					map(a => a.hdPath!.addressIndex.value()),
					take(2),
					toArray(),
				)
				.subscribe({
					next: values => {
						expect(values).toStrictEqual(expectedValues)
						done()
					},
					error: e => done(e),
				}),
		)

		subs.add(
			signingKeychain
				.deriveNextLocalHDSigningKey({ alsoSwitchTo: true })
				.subscribe(),
		)
	})

	it('can list all accounts that has been added', done => {
		const testSigningKeysList = (
			mapSigningKeysToNum: (accounts: SigningKeysT) => number,
		): void => {
			const subs = new Subscription()
			const signingKeychain = createSigningKeychain()
			const expectedValues = [1, 2, 3]

			subs.add(
				signingKeychain
					.observeSigningKeys()
					.pipe(
						map(acs => mapSigningKeysToNum(acs)),
						take(expectedValues.length),
						toArray(),
					)
					.subscribe(values => {
						expect(values).toStrictEqual(expectedValues)
						done()
					}),
			)

			subs.add(
				signingKeychain.deriveNextLocalHDSigningKey().subscribe(() => {
					subs.add(
						signingKeychain
							.deriveNextLocalHDSigningKey()
							.subscribe(),
					)
				}),
			)
		}

		testSigningKeysList(acs => acs.localHDSigningKeys().length)
		testSigningKeysList(acs => acs.all.length)
		testSigningKeysList(acs => acs.size())
	})

	it('can switch signingKey by number', done => {
		const subs = new Subscription()
		const signingKeychain = createSigningKeychain()

		const expectedAccountAddressIndices = [0, 1, 0]

		subs.add(
			signingKeychain
				.observeActiveSigningKey()
				.pipe(take(expectedAccountAddressIndices.length), toArray())
				.subscribe({
					next: accountList => {
						expect(
							accountList.map(a =>
								a.hdPath!.addressIndex.value(),
							),
						).toStrictEqual(expectedAccountAddressIndices)
						done()
					},
					error: e => done(e),
				}),
		)

		subs.add(
			signingKeychain
				.deriveNextLocalHDSigningKey({ alsoSwitchTo: true })
				.subscribe(),
		)

		signingKeychain.switchSigningKey({ toIndex: 0 })
	})

	it('signingKeychain can add private key signingKey', done => {
		const privateKeyFromNum = (privateKeyScalar: number) =>
			PrivateKey.fromScalar(
				UInt256.valueOf(privateKeyScalar),
			)._unsafeUnwrap()

		const signingKeychain = createSigningKeychain({
			startWithInitialSigningKey: true,
		})
		const subs = new Subscription()

		const expectedValues = [1, 2]

		subs.add(
			signingKeychain
				.observeSigningKeys()
				.pipe(
					map(acs => acs.size()),
					take(expectedValues.length),
					toArray(),
				)
				.subscribe(
					values => {
						expect(values).toStrictEqual(expectedValues)
						subs.add(
							signingKeychain
								.observeActiveSigningKey()
								.pipe(skip(1))
								.subscribe(signingKey => {
									const expPubKey =
										'0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'
									expect(
										signingKey.publicKey.toString(true),
									).toBe(expPubKey)
									expect(signingKey.uniqueIdentifier).toBe(
										`Non_hd_pubKey${expPubKey}`,
									)
									expect(signingKey.isHDSigningKey).toBe(
										false,
									)
									expect(signingKey.isLocalHDSigningKey).toBe(
										false,
									)
									expect(
										signingKey.isHardwareSigningKey,
									).toBe(false)
									done()
								}),
						)
					},
					e => {
						done(e)
					},
				),
		)

		const privateKey = privateKeyFromNum(1)

		signingKeychain.addSigningKeyFromPrivateKey({
			privateKey,
			alsoSwitchTo: true,
		})
	})
})
