/**
 * @group integration
 */

/* eslint-disable */
import { Radix } from '../../src/radix'
import { ValidatorAddressT } from '@radixdlt/account'
import { firstValueFrom, interval, Subject, Subscription } from 'rxjs'
import {
	delay,
	map,
	mergeMap,
	retry,
	retryWhen,
	take,
	tap,
	toArray,
} from 'rxjs/operators'
import {
	PendingTransaction,
	TransactionIdentifierT,
	TransactionStateSuccess,
	TransactionStatus,
} from '../../src/dto/_types'
import { Amount, AmountT, Network } from '@radixdlt/primitives'
import {
	TransactionTrackingEventType,
	KeystoreT,
	log,
	restoreDefaultLogLevel,
} from '../../src'
import { UInt256 } from '@radixdlt/uint256'
import { AccountT } from '../../src'
import { keystoreForTest, makeWalletWithFunds } from '../util'
import {
	AccountBalancesEndpoint,
	Decoded,
	StakePositionsEndpoint,
} from '../../src/api/open-api/_types'
import { retryOnErrorCode } from '../../src/api/utils'

const fetch = require('node-fetch')

const network = Network.STOKENET

// local
// const NODE_URL = 'http://localhost:8080'

// RCNet
//const NODE_URL = 'https://54.73.253.49'

// release net
//const NODE_URL = 'https://18.168.73.103'

const NODE_URL = 'https://stokenet-gateway.radixdlt.com'

// const NODE_URL = 'https://milestonenet-gateway.radixdlt.com'

const loadKeystore = (): Promise<KeystoreT> =>
	Promise.resolve(keystoreForTest.keystore)

const requestFaucet = async (address: string) => {
	let request = {
		params: {
			address,
		},
	}

	await fetch(`${NODE_URL}/faucet/request`, {
		method: 'POST',
		body: JSON.stringify(request),
		headers: { 'Content-Type': 'application/json' },
	})
}

let subs: Subscription

let radix: ReturnType<typeof Radix.create>
let accounts: AccountT[]
let balances: AccountBalancesEndpoint.DecodedResponse
let nativeTokenBalance: Decoded.TokenAmount

describe('integration API tests', () => {
	beforeAll(async () => {
		radix = Radix.create()
		await radix
			.__withWallet(makeWalletWithFunds(network))
			.connect(`${NODE_URL}`)
		accounts = (
			await firstValueFrom(radix.restoreLocalHDAccountsToIndex(2))
		).all
		balances = await firstValueFrom(radix.tokenBalances)
		const maybeTokenBalance = balances.account_balances.liquid_balances.find(
			a => a.token_identifier.rri.name.toLowerCase() === 'xrd',
		)
		if (!maybeTokenBalance) {
			throw Error('no XRD found')
		}
		nativeTokenBalance = maybeTokenBalance
		log.setLevel('INFO')
	})

	beforeEach(() => {
		subs = new Subscription()
	})
	afterEach(() => {
		subs.unsubscribe()
	})
	afterAll(() => {
		restoreDefaultLogLevel()
	})

	it('can connect and is chainable', async () => {
		const radix = Radix.create()
		await radix.connect(`${NODE_URL}`)

		expect(radix).toBeDefined()
		expect(radix.ledger.nativeToken).toBeDefined()
		expect(radix.ledger.tokenBalancesForAddress).toBeDefined() // etc
	})

	it('emits node connection without wallet', async done => {
		const radix = Radix.create()
		await radix.connect(`${NODE_URL}`)

		subs.add(
			radix.__node.subscribe(
				node => {
					expect(node.url.host).toBe(new URL(NODE_URL).host)
					done()
				},
				error => done(error),
			),
		)
	})

	it('can switch networks', async done => {
		await radix
			.login(keystoreForTest.password, loadKeystore)
			.connect(`${NODE_URL}`)

		const address1 = await firstValueFrom(radix.activeAddress)
		expect(address1.network).toBeDefined()

		await radix.connect('https://mainnet-gateway.radixdlt.com')

		const address2 = await firstValueFrom(radix.activeAddress)
		expect(address2.network).toBeDefined()

		await radix.connect('https://stokenet-gateway.radixdlt.com')

		const address3 = await firstValueFrom(radix.activeAddress)
		expect(address3.network).toBeDefined()

		done()
	})

	it('returns native token without wallet', async done => {
		const radix = Radix.create()
		radix.connect(`${NODE_URL}`)

		subs.add(
			radix.ledger.nativeToken(network).subscribe(
				token => {
					expect(token.symbol).toBe('xrd')
					done()
				},
				error => done(error),
			),
		)
	})

	/*

		it('deriveNextSigningKey method on radix updates accounts', done => {
		const expected = [1, 2, 3]

		subs.add(
			radix.accounts
				.pipe(
					map(i => i.size()),
					take(expected.length),
					toArray(),
				)
				.subscribe(values => {
					expect(values).toStrictEqual(expected)
					done()
				}),
		)

		radix.deriveNextAccount({ alsoSwitchTo: true })
		radix.deriveNextAccount({ alsoSwitchTo: false })
	})

	
	it('deriveNextSigningKey alsoSwitchTo method on radix updates activeSigningKey', done => {
		const expected = [0, 1, 3]

		subs.add(
			radix.activeAccount
				.pipe(
					map(account => account.hdPath!.addressIndex.value()),
					take(expected.length),
					toArray(),
				)
				.subscribe(values => {
					expect(values).toStrictEqual(expected)
					done()
				}),
		)

		radix.deriveNextAccount({ alsoSwitchTo: true })
		radix.deriveNextAccount({ alsoSwitchTo: false })
		radix.deriveNextAccount({ alsoSwitchTo: true })
	})

	it('deriveNextSigningKey alsoSwitchTo method on radix updates activeAddress', done => {
		const expectedCount = 3

		subs.add(
			radix.activeAddress
				.pipe(take(expectedCount), toArray())
				.subscribe(values => {
					expect(values.length).toBe(expectedCount)
					done()
				}),
		)

		radix.deriveNextAccount({ alsoSwitchTo: true })
		radix.deriveNextAccount({ alsoSwitchTo: false })
		radix.deriveNextAccount({ alsoSwitchTo: true })
	})
*/
	// 🟢
	it('should compare token balance before and after transfer', async done => {
		const getTokenBalanceSubject = new Subject<number>()

		radix.withTokenBalanceFetchTrigger(getTokenBalanceSubject)

		getTokenBalanceSubject.next(1)

		let transferDone = false
		const amountToSend = Amount.fromUnsafe(
			`1${'0'.repeat(18)}`,
		)._unsafeUnwrap()

		let initialBalance: AmountT
		let balanceAfterTransfer: AmountT
		let fee: AmountT

		radix.activeAddress.subscribe(async address => {
			await requestFaucet(address.toString())
			subs.add(
				radix.tokenBalances.subscribe(balance => {
					const getXRDBalanceOrZero = (): AmountT => {
						const maybeTokenBalance = balance.account_balances.liquid_balances.find(
							a =>
								a.token_identifier.rri.name.toLowerCase() ===
								'xrd',
						)
						return maybeTokenBalance !== undefined
							? maybeTokenBalance.value
							: UInt256.valueOf(0)
					}

					if (transferDone) {
						balanceAfterTransfer = getXRDBalanceOrZero()

						expect(
							initialBalance
								.sub(balanceAfterTransfer)
								.eq(amountToSend.add(fee)),
						).toBe(true)
						done()
					} else {
						initialBalance = getXRDBalanceOrZero()
					}
				}),
			)

			subs.add(
				radix
					.transferTokens({
						transferInput: {
							to_account: accounts[2].address,
							amount: amountToSend,
							tokenIdentifier:
								nativeTokenBalance.token_identifier.rri,
						},
						userConfirmation: 'skip',
						pollTXStatusTrigger: interval(500),
					})
					.completion.subscribe(txID => {
						transferDone = true
						subs.add(
							radix.ledger
								.getTransaction(txID, network)
								.subscribe(tx => {
									fee = tx.fee
									getTokenBalanceSubject.next(1)
								}),
						)
					}),
			)
		})
	})

	// 🟢 can only test this on localnet
	it.skip('should increment transaction history with a new transaction after transfer', async done => {
		const pageSize = 15

		const fetchTxHistory = (cursor: string) => {
			return new Promise<[string, number]>((resolve, _) => {
				const sub = radix
					.transactionHistory({
						size: pageSize,
						cursor,
					})
					.subscribe(txHistory => {
						sub.unsubscribe()
						resolve([
							txHistory.cursor,
							txHistory.transactions.length,
						])
					})
			})
		}

		const getLastCursor = async () => {
			return new Promise<string>((resolve, _) => {
				radix
					.transactionHistory({
						size: pageSize,
					})
					.subscribe(async txHistory => {
						let cursor = txHistory.cursor
						let prevTxCount = 0
						let txCount = 0

						while (cursor) {
							prevTxCount = txCount
							;[cursor, txCount] = await fetchTxHistory(cursor)
						}

						resolve(cursor)
					})
			})
		}

		const cursor = await getLastCursor()

		subs.add(
			radix
				.transactionHistory({
					size: pageSize,
					cursor,
				})
				.subscribe(txHistory => {
					const countBeforeTransfer = txHistory.transactions.length
					subs.add(
						radix
							.transferTokens({
								transferInput: {
									to_account: accounts[2].address,
									amount: 1,
									tokenIdentifier:
										nativeTokenBalance.token_identifier.rri,
								},
								userConfirmation: 'skip',
								pollTXStatusTrigger: interval(500),
							})
							.completion.subscribe(tx => {
								subs.add(
									radix
										.transactionHistory({
											size: pageSize,
											cursor,
										})
										.subscribe(newTxHistory => {
											expect(
												newTxHistory.transactions
													.length - 1,
											).toEqual(countBeforeTransfer)
											done()
										}),
								)
							}),
					)
				}),
		)
	})

	it('should be able to get transaction history', async () => {
		const txID1 = await firstValueFrom(
			radix.transferTokens({
				transferInput: {
					to_account: accounts[2].address,
					amount: 1,
					tokenIdentifier: nativeTokenBalance.token_identifier.rri,
				},
				userConfirmation: 'skip',
			}).completion,
		)

		const txID2 = await firstValueFrom(
			radix.transferTokens({
				transferInput: {
					to_account: accounts[2].address,
					amount: 1,
					tokenIdentifier: nativeTokenBalance.token_identifier.rri,
				},
				userConfirmation: 'skip',
			}).completion,
		)

		const txHistory = await firstValueFrom(
			radix.transactionHistory({ size: 2 }),
		)

		expect(txHistory.transactions[0].txID.equals(txID1))
		expect(txHistory.transactions[1].txID.equals(txID2))
	})

	it('should be able to get recent transactions', async () => {
		const recentTX = await firstValueFrom(
			radix.ledger.recentTransactions({ network }),
		)

		expect(recentTX.transactions.length).toBeGreaterThan(0)
	})

	// 🟢
	it('should handle transaction status updates', done => {
		const txTracking = radix.transferTokens({
			transferInput: {
				to_account: accounts[2].address,
				amount: 1,
				tokenIdentifier: nativeTokenBalance.token_identifier.rri,
			},
			userConfirmation: 'skip',
			pollTXStatusTrigger: interval(1000),
		})

		txTracking.events.subscribe(event => {
			if (
				event.eventUpdateType === TransactionTrackingEventType.SUBMITTED
			) {
				const txID: TransactionIdentifierT = (event as TransactionStateSuccess<PendingTransaction>)
					.transactionState.txID

				subs.add(
					radix
						.transactionStatus(txID, interval(1000))
						.pipe(
							// after a transaction is submitted there is a delay until it appears in transaction status
							retryWhen(retryOnErrorCode({ errorCodes: [404] })),
						)
						.subscribe(({ status }) => {
							expect(status).toEqual(TransactionStatus.CONFIRMED)
							done()
						}),
				)
			}
		})
	})

	it('can lookup tx', async () => {
		const { completion } = radix.transferTokens({
			transferInput: {
				to_account: accounts[2].address,
				amount: 1,
				tokenIdentifier: nativeTokenBalance.token_identifier.rri,
			},
			userConfirmation: 'skip',
			pollTXStatusTrigger: interval(3000),
		})

		const txID = await firstValueFrom(completion)
		const tx = await firstValueFrom(radix.getTransaction(txID))

		expect(txID.equals(tx.txID)).toBe(true)
		expect(tx.actions.length).toEqual(2)
	})

	it('can lookup validator', async () => {
		const validator = (
			await firstValueFrom(radix.ledger.validators(network))
		).validators[0]
		const validatorFromLookup = await firstValueFrom(
			radix.ledger.lookupValidator(validator.address),
		)

		expect(validatorFromLookup.address.equals(validator.address)).toBe(true)
	})

	it('should get validators', async () => {
		const validators = await firstValueFrom(
			radix.ledger.validators(network),
		)

		expect(validators.validators.length).toBeGreaterThan(0)
	})

	const getValidators = async () =>
		(await firstValueFrom(radix.ledger.validators(network))).validators

	const getValidatorStakeAmountForAddress = (
		{ stakes, pendingStakes }: StakePositionsEndpoint.DecodedResponse,
		validatorAddress: ValidatorAddressT,
	) => {
		const validatorStake = stakes.find(values =>
			values.validator.equals(validatorAddress),
		)
		const validatorPendingStake = pendingStakes.find(values =>
			values.validator.equals(validatorAddress),
		)

		const stakeAmount = validatorStake
			? validatorStake.amount
			: Amount.fromUnsafe(0)._unsafeUnwrap()

		const pendingStakeAmount = validatorPendingStake
			? validatorPendingStake.amount
			: Amount.fromUnsafe(0)._unsafeUnwrap()

		return stakeAmount.add(pendingStakeAmount)
	}

	it('can fetch stake positions', async done => {
		const triggerSubject = new Subject<number>()

		radix.withStakingFetchTrigger(triggerSubject)

		const stakeAmount = Amount.fromUnsafe(
			'100000000000000000000',
		)._unsafeUnwrap()

		const [validator] = await getValidators()

		const initialStake = await firstValueFrom(
			radix.stakingPositions.pipe(
				map(res =>
					getValidatorStakeAmountForAddress(res, validator.address),
				),
			),
		)

		const expectedStake = initialStake.add(stakeAmount).toString()

		subs.add(
			(
				await radix.stakeTokens({
					stakeInput: {
						amount: stakeAmount,
						to_validator: validator.address,
					},
					userConfirmation: 'skip',
					pollTXStatusTrigger: interval(1000),
				})
			).completion
				.pipe(
					tap(() => {
						triggerSubject.next(0)
					}),
					delay(1000),
					mergeMap(_ =>
						radix.stakingPositions.pipe(
							map(res =>
								getValidatorStakeAmountForAddress(
									res,
									validator.address,
								),
							),
							map(actualStake => {
								if (actualStake.eq(initialStake)) {
									log.info(
										'radix.stakingPositions is not done fetching lets retry 🔄',
									)
									throw { error: { code: 999 } }
								} else {
									return actualStake.toString()
								}
							}),
							retryWhen(retryOnErrorCode({ errorCodes: [999] })),
						),
					),
				)
				.subscribe(actualStake => {
					expect(actualStake).toEqual(expectedStake)
					done()
				}),
		)
	})

	it('can fetch unstake positions', async () => {
		const triggerSubject = new Subject<number>()

		radix.withStakingFetchTrigger(triggerSubject)

		const stakeAmount = Amount.fromUnsafe(
			'100000000000000000000',
		)._unsafeUnwrap()

		const validator = (await firstValueFrom(radix.validators()))
			.validators[30]

		const stake = await radix.stakeTokens({
			stakeInput: {
				amount: stakeAmount,
				to_validator: validator.address,
			},
			userConfirmation: 'skip',
			pollTXStatusTrigger: interval(1000),
		})

		await firstValueFrom(stake.completion)

		const unstake = await radix.unstakeTokens({
			unstakeInput: {
				unstake_percentage: 100,
				from_validator: validator.address,
			},
			userConfirmation: 'skip',
			pollTXStatusTrigger: interval(1000),
		})

		await firstValueFrom(unstake.completion)

		triggerSubject.next(0)

		const positions = await firstValueFrom(radix.unstakingPositions)

		expect(positions.unstakes[0]).toBeDefined()
	})
	/*
		// 🟢
		it('should be able to paginate validator result', async () => {
			const twoValidators = await firstValueFrom(
				radix.ledger.validators({ size: 2 }),
			)
			const firstValidator = await firstValueFrom(
				radix.ledger.validators({ size: 1 }),
			)
			const secondValidator = await firstValueFrom(
				radix.ledger.validators({ size: 1, cursor: firstValidator.cursor }),
			)
	
			expect(firstValidator.validators[0].address.toString()).toEqual(
				twoValidators.validators[0].address.toString(),
			)
	
			expect(secondValidator.validators[0].address.toString()).toEqual(
				twoValidators.validators[1].address.toString(),
			)
		})
	
		describe('make tx single transfer', () => {
			const tokenTransferInput: TransferTokensInput = {
				to: accounts[2].address,
				amount: 1,
				tokenIdentifier: nativeTokenBalance.token.rri,
			}
	
			let pollTXStatusTrigger: Observable<unknown>
	
			const transferTokens = (): TransferTokensOptions => ({
				transferInput: tokenTransferInput,
				userConfirmation: 'skip',
				pollTXStatusTrigger: pollTXStatusTrigger,
			})
	
			let subs: Subscription
	
			beforeEach(() => {
				subs = new Subscription()
				pollTXStatusTrigger = interval(500)
			})
	
			afterEach(() => {
				subs.unsubscribe()
			})
	
			it.skip('events emits expected values', done => {
				// can't see pending state because quick confirmation
	
				const expectedValues = [
					TransactionTrackingEventType.INITIATED,
					TransactionTrackingEventType.BUILT_FROM_INTENT,
					TransactionTrackingEventType.ASKED_FOR_CONFIRMATION,
					TransactionTrackingEventType.CONFIRMED,
					TransactionTrackingEventType.SIGNED,
					TransactionTrackingEventType.FINALIZED,
					TransactionTrackingEventType.SUBMITTED,
					TransactionTrackingEventType.UPDATE_OF_STATUS_OF_PENDING_TX,
					TransactionTrackingEventType.UPDATE_OF_STATUS_OF_PENDING_TX,
					TransactionTrackingEventType.COMPLETED,
				]
	
				subs.add(
					radix
						.transferTokens(transferTokens())
						.events.pipe(
							map(e => e.eventUpdateType),
							tap(x => console.log(x)),
							take(expectedValues.length),
							toArray(),
						)
						.subscribe({
							next: values => {
								expect(values).toStrictEqual(expectedValues)
								done()
							},
							error: e => {
								done(
									new Error(
										`Tx failed, even though we expected it to succeed, error: ${e.toString()}`,
									),
								)
							},
						}),
				)
			})
	
			it('automatic confirmation', done => {
				subs.add(
					radix.transferTokens(transferTokens()).completion.subscribe({
						next: _txID => {},
						complete: () => {
							done()
						},
						error: e => {
							done(
								new Error(
									`Tx failed, but expected to succeed. Error ${JSON.stringify(
										e,
										null,
										2,
									)}`,
								),
							)
						},
					}),
				)
			})
	
			it('manual confirmation', done => {
				//@ts-ignore
				let transaction
				//@ts-ignore
				let userHasBeenAskedToConfirmTX
	
				const confirmTransaction = () => {
					//@ts-ignore
					transaction.confirm()
				}
	
				const shouldShowConfirmation = () => {
					userHasBeenAskedToConfirmTX = true
					confirmTransaction()
				}
	
				const userConfirmation = new ReplaySubject<ManualUserConfirmTX>()
	
				const transactionTracking = radix.transferTokens({
					...transferTokens(),
					userConfirmation,
				})
	
				subs.add(
					userConfirmation.subscribe(txn => {
						//@ts-ignore
						transaction = txn
						shouldShowConfirmation()
					}),
				)
	
				subs.add(
					transactionTracking.completion.subscribe({
						next: _txID => {
							//@ts-ignore
							expect(userHasBeenAskedToConfirmTX).toBe(true)
							done()
						},
						error: e => {
							done(e)
						},
					}),
				)
			})
		})
		*/
})
