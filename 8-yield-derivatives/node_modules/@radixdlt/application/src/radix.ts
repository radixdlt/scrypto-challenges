import {
	AccountAddressT,
	DeriveHWSigningKeyInput,
	DeriveNextInput,
	SigningKeychain,
	SigningKeychainT,
} from '@radixdlt/account'
import { Network } from '@radixdlt/primitives'
import { apiVersion } from '@radixdlt/networking'
import { nodeAPI, NodeT, radixCoreAPI, RadixCoreAPI } from './api'

import {
	catchError,
	distinctUntilChanged,
	filter,
	map,
	mergeMap,
	retryWhen,
	share,
	shareReplay,
	skipWhile,
	switchMap,
	take,
	tap,
	withLatestFrom,
} from 'rxjs/operators'
import {
	combineLatest,
	EMPTY,
	firstValueFrom,
	interval,
	merge,
	Observable,
	of,
	ReplaySubject,
	Subject,
	Subscription,
	throwError,
} from 'rxjs'
import { KeystoreT, Message, MnemomicT } from '@radixdlt/crypto'
import {
	AddAccountByPrivateKeyInput,
	AccountsT,
	WalletT,
	AccountT,
	MakeTransactionOptions,
	ManualUserConfirmTX,
	SwitchAccountInput,
	TransactionConfirmationBeforeFinalization,
	TransferTokensOptions,
} from './_types'
import {
	APIError,
	APIErrorObject,
	buildTxFromIntentErr,
	finalizeTxErr,
	lookupTxErr,
	lookupValidatorErr,
	nativeTokenErr,
	networkIdErr,
	nodeError,
	recentTransactionsErr,
	stakesForAddressErr,
	submitSignedTxErr,
	tokenBalancesErr,
	transactionHistoryErr,
	unstakesForAddressErr,
	validatorsErr,
	walletError,
} from './errors'
import { log, LogLevel, msgFromError, isArray } from '@radixdlt/util'
import {
	BuiltTransaction,
	ExecutedTransaction,
	FinalizedTransaction,
	flatMapAddressesOf,
	PendingTransaction,
	SignedTransaction,
	SimpleExecutedTransaction,
	SimpleTransactionHistory,
	singleRecipientFromActions,
	Token,
	TransactionHistory,
	TransactionHistoryActiveAccountRequestInput,
	TransactionIdentifierT,
	TransactionIntent,
	TransactionIntentBuilder,
	TransactionIntentBuilderOptions,
	TransactionIntentBuilderT,
	TransactionStateError,
	TransactionStateUpdate,
	TransactionTracking,
	TransactionTrackingEventType,
	TransactionType,
	TransactionStatus,
} from './dto'
import {
	ActionType,
	ExecutedAction,
	TransferTokensAction,
	StakeTokensInput,
	UnstakeTokensInput,
} from './actions'
import { Wallet } from './wallet'
import { tokenInfoErr } from '.'
import { retryOnErrorCode } from './api/utils'

const txTypeFromActions = (
	input: Readonly<{
		actions: ExecutedAction[]
		activeAddress: AccountAddressT
	}>,
): TransactionType => {
	const { activeAddress } = input
	const myAddress = activeAddress.toString()
	const fromUnique = flatMapAddressesOf({
		...input,
		includeTo: false,
	}).map(a => a.toString())
	const toUnique = flatMapAddressesOf({
		...input,
		includeFrom: false,
	}).map(a => a.toString())

	const toMe = toUnique.includes(myAddress)
	const fromMe = fromUnique.includes(myAddress)

	if (toMe && fromMe) {
		return TransactionType.FROM_ME_TO_ME
	} else if (toMe) {
		return TransactionType.INCOMING
	} else if (fromMe) {
		return TransactionType.OUTGOING
	} else {
		return TransactionType.UNRELATED
	}
}

const decorateSimpleExecutedTransactionWithType = (
	simpleExecutedTX: SimpleExecutedTransaction,
	activeAddress: AccountAddressT,
): ExecutedTransaction => ({
	...simpleExecutedTX,
	transactionType: txTypeFromActions({
		actions: simpleExecutedTX.actions,
		activeAddress,
	}),
})

const shouldConfirmTransactionAutomatically = (
	confirmationScheme: TransactionConfirmationBeforeFinalization,
): confirmationScheme is 'skip' => confirmationScheme === 'skip'

const create = () => {
	const subs = new Subscription()
	const radixLog = log // TODO configure child loggers

	const nodeSubject = new ReplaySubject<NodeT>()
	const coreAPISubject = new ReplaySubject<RadixCoreAPI>()
	const walletSubject = new ReplaySubject<WalletT>()
	const errorNotificationSubject = new Subject<APIError>()

	const deriveNextLocalHDAccountSubject = new Subject<DeriveNextInput>()
	const addAccountByPrivateKeySubject = new Subject<AddAccountByPrivateKeyInput>()
	const switchAccountSubject = new Subject<SwitchAccountInput>()

	const tokenBalanceFetchSubject = new Subject<number>()
	const stakingFetchSubject = new Subject<number>()
	const wallet$ = walletSubject.asObservable()

	const networkSubject = new ReplaySubject<Network>()
	const nativeTokenSubject = new ReplaySubject<Token>()

	let walletSubscription: Subscription

	const coreAPIViaNode$ = nodeSubject
		.asObservable()
		.pipe(map((n: NodeT) => radixCoreAPI(n, nodeAPI(n.url))))

	const coreAPI$ = merge(coreAPIViaNode$, coreAPISubject.asObservable()).pipe(
		shareReplay(1),
	)
	// Forwards calls to RadixCoreAPI, return type is a function: `(input?: I) => Observable<O>`
	const fwdAPICall = <I extends unknown[], O>(
		pickFn: (api: RadixCoreAPI) => (...input: I) => Observable<O>,
		errorFn: (error: APIErrorObject) => APIError,
	) => (...input: I) =>
		coreAPI$.pipe(
			mergeMap(a => pickFn(a)(...input)),
			take(1), // Important!
			catchError((error: unknown) => {
				throw errorFn(isArray(error) ? (error as any)[0] : error)
			}),
		)

	const api = {
		networkId: fwdAPICall(
			a => a.networkId,
			m => networkIdErr(m),
		),

		tokenBalancesForAddress: fwdAPICall(
			a => a.tokenBalancesForAddress,
			m => tokenBalancesErr(m),
		),

		transactionHistory: fwdAPICall(
			a => a.transactionHistory,
			m => transactionHistoryErr(m),
		),

		recentTransactions: fwdAPICall(
			a => a.recentTransactions,
			m => recentTransactionsErr(m),
		),

		nativeToken: fwdAPICall(
			a => a.nativeToken,
			m => nativeTokenErr(m),
		),

		tokenInfo: fwdAPICall(
			a => a.tokenInfo,
			m => tokenInfoErr(m),
		),

		stakesForAddress: fwdAPICall(
			a => a.stakesForAddress,
			m => stakesForAddressErr(m),
		),

		unstakesForAddress: fwdAPICall(
			a => a.unstakesForAddress,
			m => unstakesForAddressErr(m),
		),

		validators: fwdAPICall(
			a => a.validators,
			m => validatorsErr(m),
		),

		lookupValidator: fwdAPICall(
			a => a.lookupValidator,
			m => lookupValidatorErr(m),
		),

		getTransaction: fwdAPICall(
			a => a.transactionStatus,
			m => lookupTxErr(m),
		),
		buildTransaction: fwdAPICall(
			a => a.buildTransaction,
			m => buildTxFromIntentErr(m),
		),

		finalizeTransaction: fwdAPICall(
			a => a.finalizeTransaction,
			m => finalizeTxErr(m),
		),
		submitSignedTransaction: fwdAPICall(
			a => a.submitSignedTransaction,
			m => submitSignedTxErr(m),
		),
	}

	const activeAddress = wallet$.pipe(
		mergeMap(a => a.observeActiveAccount()),
		map(a => a.address),
		shareReplay(1),
	)

	const revealMnemonic = (): Observable<MnemomicT> =>
		wallet$.pipe(
			map((wallet: WalletT): MnemomicT => wallet.revealMnemonic()),
		)

	const activeAddressToAPIObservableWithTrigger = <O>(
		trigger: Observable<number>,
		pickFn: (
			api: RadixCoreAPI,
		) => (address: AccountAddressT) => Observable<O>,
		errorFn: (error: APIErrorObject) => APIError,
	): Observable<O> =>
		merge(
			trigger.pipe(
				withLatestFrom(activeAddress),
				map(result => result[1]),
			),
			activeAddress,
		).pipe(
			withLatestFrom(coreAPI$),
			switchMap(([address, api]) =>
				pickFn(api)(address).pipe(
					catchError(error => {
						console.error(error)
						errorNotificationSubject.next(errorFn(error))
						return EMPTY
					}),
				),
			),
			shareReplay(1),
		)

	const tokenBalances = activeAddressToAPIObservableWithTrigger(
		tokenBalanceFetchSubject,
		a => a.tokenBalancesForAddress,
		tokenBalancesErr,
	)

	/*
		const decorateSimpleTokenBalanceWithTokenInfo = (
			simpleTokenBalance: SimpleTokenBalance,
		): Observable<TokenBalance> =>
			api.tokenInfo(simpleTokenBalance.tokenIdentifier).pipe(
				map(
					(tokenInfo: Token): TokenBalance => ({
						amount: simpleTokenBalance.amount,
						token: tokenInfo,
					}),
				),
			)
	*/
	const stakingPositions = activeAddressToAPIObservableWithTrigger(
		stakingFetchSubject,
		a => a.stakesForAddress,
		stakesForAddressErr,
	)

	const unstakingPositions = activeAddressToAPIObservableWithTrigger(
		stakingFetchSubject,
		a => a.unstakesForAddress,
		unstakesForAddressErr,
	)

	const transactionHistory = (
		input: TransactionHistoryActiveAccountRequestInput,
	): Observable<TransactionHistory> =>
		activeAddress.pipe(
			take(1),
			switchMap(activeAddress =>
				api
					.transactionHistory({ ...input, address: activeAddress })
					.pipe(
						map(
							(
								simpleTxHistory: SimpleTransactionHistory,
							): TransactionHistory => ({
								...simpleTxHistory,
								transactions: simpleTxHistory.transactions.map(
									(
										simpleExecutedTX: SimpleExecutedTransaction,
									): ExecutedTransaction =>
										decorateSimpleExecutedTransactionWithType(
											simpleExecutedTX,
											activeAddress,
										),
								),
							}),
						),
					),
			),
		)

	const node$ = merge(
		nodeSubject.asObservable(),
		coreAPISubject.asObservable().pipe(map(api => api.node)),
	)

	const activeAccount: Observable<AccountT> = wallet$.pipe(
		mergeMap(wallet => wallet.observeActiveAccount()),
		shareReplay(1),
		distinctUntilChanged((prev, cur) => prev.equals(cur)),
	)

	const accounts = wallet$.pipe(
		mergeMap(wallet => wallet.observeAccounts()),
		shareReplay(1),
	)

	const __makeTransactionFromIntent = (
		transactionIntent$: Observable<TransactionIntent>,
		options: MakeTransactionOptions,
	): TransactionTracking => {
		const txLog = radixLog // TODO configure child loggers
		const txSubs = new Subscription()

		txLog.debug(
			`Start of transaction flow, inside constructor of 'TransactionTracking'.`,
		)

		const signUnsignedTx = (
			unsignedTx: BuiltTransaction,
		): Observable<SignedTransaction> => {
			txLog.debug('Starting signing transaction (async).')
			return combineLatest(
				transactionIntent$,
				activeAccount.pipe(take(1)),
			).pipe(
				mergeMap(
					([
						transactionIntent,
						account,
					]): Observable<SignedTransaction> => {
						const nonXRDHRPsOfRRIsInTx: string[] = transactionIntent.actions
							.filter(a => a.type === ActionType.TOKEN_TRANSFER)
							.map(a => a as TransferTokensAction)
							.filter(t => t.rri.name !== 'xrd')
							.map(t => t.rri.name)

						const uniquenonXRDHRPsOfRRIsInTx = [
							...new Set(nonXRDHRPsOfRRIsInTx),
						]

						if (uniquenonXRDHRPsOfRRIsInTx.length > 1) {
							const errMsg = `Error cannot sign transction with multiple non-XRD RRIs. Unsupported by Ledger app.`
							log.error(errMsg)
							return throwError(new Error(errMsg))
						}

						const nonXRDHrp =
							uniquenonXRDHRPsOfRRIsInTx.length === 1
								? uniquenonXRDHRPsOfRRIsInTx[0]
								: undefined

						return account
							.sign(unsignedTx.transaction, nonXRDHrp)
							.pipe(
								map(
									(signature): SignedTransaction => {
										const publicKeyOfSigner =
											account.publicKey
										txLog.debug(
											`Finished signing transaction`,
										)
										return {
											transaction: unsignedTx.transaction,
											signature,
											publicKeyOfSigner,
										}
									},
								),
							)
					},
				),
			)
		}

		const pendingTXSubject = new Subject<PendingTransaction>()

		const askUserToConfirmSubject = new ReplaySubject<BuiltTransaction>()
		const userDidConfirmTransactionSubject = new ReplaySubject<0>()

		if (shouldConfirmTransactionAutomatically(options.userConfirmation)) {
			txLog.debug(
				'Transaction has been setup to be automatically confirmed, requiring no final confirmation input from user.',
			)
			txSubs.add(
				askUserToConfirmSubject.subscribe(() => {
					txLog.debug(
						`askUserToConfirmSubject got 'next', calling 'next' on 'userDidConfirmTransactionSubject'`,
					)
					userDidConfirmTransactionSubject.next(0)
				}),
			)
		} else {
			txLog.debug(
				`Transaction has been setup so that it requires a manual final confirmation from user before being finalized.`,
			)
			const twoWayConfirmationSubject: Subject<ManualUserConfirmTX> =
				options.userConfirmation

			txSubs.add(
				askUserToConfirmSubject.subscribe(ux => {
					txLog.info(
						`Forwarding signedUnconfirmedTX and 'userDidConfirmTransactionSubject' to subject 'twoWayConfirmationSubject' now (inside subscribe to 'askUserToConfirmSubject')`,
					)

					const confirmation: ManualUserConfirmTX = {
						txToConfirm: ux,
						confirm: () => userDidConfirmTransactionSubject.next(0),
					}
					twoWayConfirmationSubject.next(confirmation)
				}),
			)
		}

		const trackingSubject = new ReplaySubject<TransactionStateUpdate>()

		const track = (event: TransactionStateUpdate): void => {
			trackingSubject.next(event)
		}

		const completionSubject = new Subject<TransactionIdentifierT>()

		const trackError = (
			input: Readonly<{
				error: Error
				inStep: TransactionTrackingEventType
			}>,
		): void => {
			const errorEvent: TransactionStateError = {
				eventUpdateType: input.inStep,
				error: input.error,
			}
			txLog.debug(`Forwarding error to 'errorSubject'`)
			track(errorEvent)
			completionSubject.error(errorEvent.error)
		}

		const builtTransaction$ = transactionIntent$.pipe(
			withLatestFrom(activeAddress),
			switchMap(
				([intent, address]): Observable<BuiltTransaction> => {
					txLog.debug(
						'Transaction intent created => requesting 🛰 API to build it now.',
					)
					track({
						transactionState: intent,
						eventUpdateType: TransactionTrackingEventType.INITIATED,
					})
					return api.buildTransaction(intent, address)
				},
			),
			catchError((e: Error) => {
				txLog.error(`API failed to build transaction`)
				trackError({
					error: e,
					inStep: TransactionTrackingEventType.BUILT_FROM_INTENT,
				})
				return EMPTY
			}),
			tap(builtTx => {
				txLog.debug(
					'TX built by API => asking for confirmation to sign...',
				)
				track({
					transactionState: builtTx,
					eventUpdateType:
						TransactionTrackingEventType.BUILT_FROM_INTENT,
				})
				askUserToConfirmSubject.next(builtTx)
			}),
			tap(builtTx => {
				track({
					transactionState: builtTx,
					eventUpdateType:
						TransactionTrackingEventType.ASKED_FOR_CONFIRMATION,
				})
			}),
		)

		const signedTransaction$ = combineLatest([
			builtTransaction$,
			userDidConfirmTransactionSubject,
		]).pipe(
			map(([signedTx, _]) => signedTx),
			tap(unsignedTx => {
				track({
					transactionState: unsignedTx,
					eventUpdateType: TransactionTrackingEventType.CONFIRMED,
				})
			}),
			mergeMap(unsignedTx => signUnsignedTx(unsignedTx)),
			shareReplay(1),
			catchError((e: Error) => {
				txLog.error(
					`API failed to sign transaction, error: ${JSON.stringify(
						e,
						null,
						4,
					)}`,
				)
				trackError({
					error: e,
					inStep: TransactionTrackingEventType.SIGNED,
				})
				return EMPTY
			}),
		)

		const finalizedTx$ = signedTransaction$.pipe(
			mergeMap(
				(
					signedTx: SignedTransaction,
				): Observable<FinalizedTransaction> => {
					txLog.debug(
						`Finished signing tx => submitting it to 🛰  API.`,
					)
					track({
						transactionState: signedTx,
						eventUpdateType: TransactionTrackingEventType.SIGNED,
					})
					return networkSubject.pipe(
						mergeMap(network =>
							api.finalizeTransaction(network, signedTx),
						),
					)
				},
			),
			catchError((e: Error) => {
				txLog.error(
					`API failed to submit transaction, error: ${JSON.stringify(
						e,
						null,
						4,
					)}`,
				)
				trackError({
					error: e,
					inStep: TransactionTrackingEventType.FINALIZED,
				})
				return EMPTY
			}),
			tap<FinalizedTransaction>(finalizedTx => {
				track({
					transactionState: finalizedTx,
					eventUpdateType: TransactionTrackingEventType.FINALIZED,
				})
			}),
		)

		txSubs.add(
			finalizedTx$
				.pipe(
					mergeMap(
						(finalizedTx): Observable<PendingTransaction> =>
							networkSubject.pipe(
								mergeMap(network =>
									api.submitSignedTransaction(network, {
										blob: finalizedTx.blob,
										txID: finalizedTx.txID,
									}),
								),
							),
					),
					catchError((e: Error) => {
						txLog.error(
							`API failed to submit transaction, error: ${JSON.stringify(
								e,
								null,
								4,
							)}`,
						)
						trackError({
							error: e,
							inStep: TransactionTrackingEventType.SUBMITTED,
						})
						return EMPTY
					}),
					tap({
						next: (pendingTx: PendingTransaction) => {
							txLog.debug(
								`Submitted transaction with txID='${pendingTx.txID.toString()}', it is now pending.`,
							)
							track({
								transactionState: pendingTx,
								eventUpdateType:
									TransactionTrackingEventType.SUBMITTED,
							})
							pendingTXSubject.next(pendingTx)
						},
						error: (submitTXError: Error) => {
							// TODO would be great to have access to txID here, hopefully API includes it in error msg?
							txLog.error(
								`Submission of signed transaction to API failed with error: ${submitTXError.message}`,
							)
							pendingTXSubject.error(submitTXError)
						},
					}),
				)
				.subscribe(),
		)

		const pollTxStatusTrigger = (
			options.pollTXStatusTrigger ?? interval(1000)
		).pipe(share())

		const transactionStatus$ = combineLatest([
			pollTxStatusTrigger,
			pendingTXSubject,
		]).pipe(
			mergeMap(([_, pendingTx]) => {
				txLog.debug(
					`Asking API for status of transaction with txID: ${pendingTx.txID.toString()}`,
				)
				return networkSubject.pipe(
					mergeMap(network =>
						api.getTransaction(pendingTx.txID, network).pipe(
							retryWhen(
								retryOnErrorCode({
									maxRetryAttempts: 3,
									errorCodes: [404],
								}),
							),
						),
					),
				)
			}),
			distinctUntilChanged((prev, cur) => prev.status === cur.status),
			share(),
		)

		const transactionCompletedWithStatusConfirmed$ = transactionStatus$.pipe(
			skipWhile(({ status }) => status !== TransactionStatus.CONFIRMED),
			take(1),
		)

		const transactionCompletedWithStatusFailed$ = transactionStatus$.pipe(
			skipWhile(({ status }) => status !== TransactionStatus.FAILED),
			take(1),
		)

		txSubs.add(
			transactionStatus$.subscribe({
				next: statusOfTransaction => {
					const { status, txID } = statusOfTransaction
					txLog.debug(
						`Status ${status.toString()} of transaction with txID='${txID.toString()}'`,
					)
					track({
						transactionState: statusOfTransaction,
						eventUpdateType:
							TransactionTrackingEventType.UPDATE_OF_STATUS_OF_PENDING_TX,
					})
				},
				error: (transactionStatusError: Error) => {
					// TODO hmm how to get txID here?
					txLog.error(
						`Failed to get status of transaction`,
						transactionStatusError,
					)
				},
			}),
		)

		txSubs.add(
			transactionCompletedWithStatusConfirmed$.subscribe({
				next: statusOfTransaction => {
					const { txID } = statusOfTransaction
					txLog.info(
						`Transaction with txID='${txID.toString()}' has completed succesfully.`,
					)
					track({
						transactionState: statusOfTransaction,
						eventUpdateType: TransactionTrackingEventType.COMPLETED,
					})

					completionSubject.next(txID)
					completionSubject.complete()
					txSubs.unsubscribe()
				},
			}),
		)

		txSubs.add(
			transactionCompletedWithStatusFailed$.subscribe(status => {
				const errMsg = `API status of tx with id=${status.txID.toString()} returned 'FAILED'`
				txLog.error(errMsg)
				trackError({
					error: new Error(errMsg),
					inStep:
						TransactionTrackingEventType.UPDATE_OF_STATUS_OF_PENDING_TX,
				})
				txSubs.unsubscribe()
			}),
		)

		return {
			completion: completionSubject.asObservable(),
			events: trackingSubject.asObservable(),
		}
	}

	const __makeTransactionFromBuilder = (
		transactionIntentBuilderT: TransactionIntentBuilderT,
		makeTXOptions: MakeTransactionOptions,
		builderOptions?: TransactionIntentBuilderOptions,
	): TransactionTracking => {
		radixLog.debug(`make transaction from builder`)
		const intent$ = transactionIntentBuilderT.build(
			builderOptions ?? {
				skipEncryptionOfMessageIfAny: {
					spendingSender: activeAddress.pipe(take(1)), // IMPORTANT !
				},
			},
		)
		return __makeTransactionFromIntent(intent$, makeTXOptions)
	}

	const transferTokens = (
		input: Omit<TransferTokensOptions, 'from_account'>,
	): TransactionTracking => {
		radixLog.debug(`transferTokens`)
		const builder = TransactionIntentBuilder.create().transferTokens(
			input.transferInput,
		)

		let encryptMsgIfAny = false
		if (input.message) {
			builder.message(input.message)
			encryptMsgIfAny = input.message.encrypt
		}

		return __makeTransactionFromBuilder(
			builder,
			{ ...input },
			encryptMsgIfAny
				? {
						encryptMessageIfAnyWithAccount: activeAccount.pipe(
							take(1), // Important !
						),
				  }
				: undefined,
		)
	}

	const stakeTokens = async (
		input: MakeTransactionOptions & {
			stakeInput: Omit<StakeTokensInput, 'tokenIdentifier'>
		},
	) => {
		radixLog.debug('stake')
		const nativeToken = await firstValueFrom(nativeTokenSubject)
		return __makeTransactionFromBuilder(
			TransactionIntentBuilder.create().stakeTokens({
				...input.stakeInput,
				tokenIdentifier: nativeToken.rri,
			}),
			{ ...input },
		)
	}

	const unstakeTokens = async (
		input: MakeTransactionOptions & {
			unstakeInput: Omit<UnstakeTokensInput, 'tokenIdentifier'>
		},
	) => {
		radixLog.debug('unstake')
		const nativeToken = await firstValueFrom(nativeTokenSubject)
		return __makeTransactionFromBuilder(
			TransactionIntentBuilder.create().unstakeTokens({
				...input.unstakeInput,
				tokenIdentifier: nativeToken.rri,
			}),
			{ ...input },
		)
	}

	const decryptTransaction = (
		input: SimpleExecutedTransaction,
	): Observable<string> => {
		radixLog.debug(
			`Trying to decrypt transaction with txID=${input.txID.toString()}`,
		)

		if (!input.message) {
			const noMsg = `TX contains no message, nothing to decrypt (txID=${input.txID.toString()}).`
			radixLog.info(noMsg)
			return throwError(() => new Error(noMsg))
		}

		const messageBuffer = Buffer.from(input.message, 'hex')

		const encryptedMessageResult = Message.fromBuffer(messageBuffer)

		if (!encryptedMessageResult.isOk()) {
			const errMessage = `Failed to parse message as 'EncryptedMessage' type, underlying error: '${msgFromError(
				encryptedMessageResult.error,
			)}'. Might not have been encrypted? Try decode string as UTF-8 string.`
			log.warn(errMessage)
			return throwError(new Error(errMessage))
		}

		const encryptedMessage = encryptedMessageResult.value

		if (encryptedMessage.kind !== 'ENCRYPTED')
			return of(encryptedMessage.plaintext)

		return activeAccount.pipe(
			take(1),
			mergeMap((account: AccountT) => {
				const myPublicKey = account.publicKey
				log.debug(
					`Trying to decrypt message with activeSigningKey with pubKey=${myPublicKey.toString()}`,
				)
				const publicKeyOfOtherPartyResult = singleRecipientFromActions(
					myPublicKey,
					input.actions,
				)
				if (!publicKeyOfOtherPartyResult.isOk()) {
					return throwError(
						new Error(
							msgFromError(publicKeyOfOtherPartyResult.error),
						),
					)
				}
				log.debug(
					`Trying to decrypt message with publicKeyOfOtherPartyResult=${publicKeyOfOtherPartyResult.toString()}`,
				)

				return account.decrypt({
					encryptedMessage,
					publicKeyOfOtherParty: publicKeyOfOtherPartyResult.value,
				})
			}),
			take(1),
		)
	}

	const restoreLocalHDAccountsToIndex = (
		index: number,
	): Observable<AccountsT> =>
		wallet$.pipe(
			mergeMap(wallet => wallet.restoreLocalHDAccountsToIndex(index)),
		)

	subs.add(
		deriveNextLocalHDAccountSubject
			.pipe(
				withLatestFrom(wallet$),
				mergeMap(([derivation, wallet]) =>
					wallet.deriveNextLocalHDAccount(derivation),
				),
			)
			.subscribe(),
	)

	subs.add(
		addAccountByPrivateKeySubject
			.pipe(
				withLatestFrom(wallet$),
				mergeMap(([privateKeyInput, wallet]) =>
					wallet.addAccountFromPrivateKey(privateKeyInput),
				),
			)
			.subscribe(),
	)

	subs.add(
		switchAccountSubject
			.pipe(
				withLatestFrom(wallet$),
				tap(([switchTo, wallet]) => wallet.switchAccount(switchTo)),
			)
			.subscribe(),
	)

	let headerSub: Subscription

	const methods = {
		// we forward the full `RadixAPI`, but we also provide some convenience methods based on active account/address.
		ledger: {
			...api,
		},

		__wallet: wallet$,
		__node: node$,

		__reset: () => subs.unsubscribe(),

		// Primarily useful for testing
		__withNodeConnection: (node$: Observable<NodeT>) => {
			subs.add(
				node$.subscribe(
					n => {
						radixLog.debug(`Using node ${n.url.toString()}`)
						nodeSubject.next(n)
					},
					(error: Error) => {
						errorNotificationSubject.next(nodeError(error) as any)
					},
				),
			)
			return methods
		},

		__withAPI: (radixCoreAPI$: Observable<RadixCoreAPI>) => {
			subs.add(radixCoreAPI$.subscribe(a => coreAPISubject.next(a)))
			return methods
		},

		__withWallet: (wallet: WalletT) => {
			walletSubject.next(wallet)
			return methods
		},

		__withKeychain: (signingKeychain: SigningKeychainT) => {
			firstValueFrom(networkSubject).then(network => {
				const wallet = Wallet.create({
					signingKeychain,
					network,
				})
				methods.__withWallet(wallet)
			})
			return methods
		},

		connect: async (url: string) => {
			methods.__withNodeConnection(of({ url: new URL(url) }))
			const networkId = await firstValueFrom(api.networkId())
			const nativeToken = await firstValueFrom(api.nativeToken(networkId))
			networkSubject.next(networkId)
			nativeTokenSubject.next(nativeToken)
		},

		login: (password: string, loadKeystore: () => Promise<KeystoreT>) => {
			walletSubscription?.unsubscribe()

			void SigningKeychain.byLoadingAndDecryptingKeystore({
				password,
				load: loadKeystore,
			}).then(signingKeychainResult => {
				signingKeychainResult.match(
					(signingKeychain: SigningKeychainT) => {
						walletSubscription = networkSubject.subscribe(
							network => {
								const wallet = Wallet.create({
									signingKeychain,
									network,
								})
								methods.__withWallet(wallet)
							},
						)
					},
					error => {
						errorNotificationSubject.next(walletError(error) as any)
					},
				)
			})

			return methods
		},

		errors: errorNotificationSubject.asObservable(),

		deriveNextAccount: (input?: DeriveNextInput) => {
			const derivation: DeriveNextInput = input ?? {}
			deriveNextLocalHDAccountSubject.next(derivation)
			return methods
		},

		deriveHWAccount: (
			input: DeriveHWSigningKeyInput,
		): Observable<AccountT> =>
			wallet$.pipe(mergeMap(wallet => wallet.deriveHWAccount(input))),

		displayAddressForActiveHWAccountOnHWDeviceForVerification: (): Observable<void> =>
			wallet$.pipe(
				mergeMap(wallet =>
					wallet.displayAddressForActiveHWAccountOnHWDeviceForVerification(),
				),
			),

		addAccountFromPrivateKey: (input: AddAccountByPrivateKeyInput) => {
			addAccountByPrivateKeySubject.next(input)
			return methods
		},

		switchAccount: (input: SwitchAccountInput) => {
			switchAccountSubject.next(input)
			return methods
		},

		restoreLocalHDAccountsToIndex,

		decryptTransaction: decryptTransaction,

		logLevel: (level: LogLevel) => {
			log.setLevel(level)
			return methods
		},

		transactionStatus: (
			txID: TransactionIdentifierT,
			trigger: Observable<number>,
		) =>
			trigger.pipe(
				withLatestFrom(networkSubject),
				mergeMap(([_, network]) => api.getTransaction(txID, network)),
				distinctUntilChanged((prev, cur) => prev.status === cur.status),
				filter(({ txID }) => txID.equals(txID)),
				tap(({ status }) =>
					radixLog.info(
						`Got transaction status ${status.toString()} for txID: ${txID.toString()}`,
					),
				),
			),

		withTokenBalanceFetchTrigger: (trigger: Observable<number>) => {
			subs.add(trigger.subscribe(tokenBalanceFetchSubject))
			return methods
		},

		withStakingFetchTrigger: (trigger: Observable<number>) => {
			subs.add(trigger.subscribe(stakingFetchSubject))
			return methods
		},

		// Wallet APIs
		revealMnemonic,
		activeAddress,
		activeAccount,
		accounts,

		// Active AccountAddress/Account APIs
		tokenBalances,
		stakingPositions,
		unstakingPositions,

		lookupTransaction: (
			txID: TransactionIdentifierT,
		): Observable<ExecutedTransaction> =>
			networkSubject.pipe(
				mergeMap(network =>
					api.getTransaction(txID, network).pipe(
						withLatestFrom(activeAddress),
						map(([simpleTx, aa]) =>
							decorateSimpleExecutedTransactionWithType(
								simpleTx,
								aa,
							),
						),
					),
				),
			),

		transactionHistory,
		transferTokens,
		stakeTokens,
		unstakeTokens,

		getTransaction: (txID: TransactionIdentifierT) =>
			networkSubject.pipe(
				mergeMap(network => api.getTransaction(txID, network)),
			),

		validators: () =>
			networkSubject.pipe(mergeMap(network => api.validators(network))),

		setHeaders: (headers: Record<string, string>) => {
			headerSub.unsubscribe()
			headerSub = coreAPI$.subscribe(api => api.setHeaders(headers))
		},

		targetApiVersion: apiVersion,
	}

	return methods
}

export const Radix = {
	create,
}
