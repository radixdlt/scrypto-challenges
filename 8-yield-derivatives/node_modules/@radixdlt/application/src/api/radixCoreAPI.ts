import { NodeAPI, NodeT } from './_types'
import { ResultAsync } from 'neverthrow'
import { defer, Observable } from 'rxjs'
import {
	AccountAddressT,
	ResourceIdentifierT,
	ValidatorAddressT,
} from '../../../account'
import { map } from 'rxjs/operators'
import {
	FinalizedTransaction,
	SignedTransaction,
	TransactionHistoryRequestInput,
	RecentTransactionsRequestInput,
	TransactionIntent,
	TransactionIdentifierT,
} from '../dto'
import { ActionType } from '../actions'
import { toObservable } from '../../../util'
import {
	AccountTransactionsEndpoint,
	BuildTransactionEndpoint,
	NativeTokenInfoEndpoint,
	StakePositionsEndpoint,
	SubmitTransactionEndpoint,
	TokenInfoEndpoint,
	UnstakePositionsEndpoint,
	ValidatorEndpoint,
	ValidatorsEndpoint,
	FinalizeTransactionEndpoint,
	TransactionEndpoint,
	RecentTransactionEndpoint,
} from './open-api/_types'

export const radixCoreAPI = (node: NodeT, api: NodeAPI) => {
	let headers: Record<string, string>

	const toObs = <I, E, O>(
		pickFn: (api: NodeAPI) => (input: I) => ResultAsync<O, E | E[]>,
		input: I,
	): Observable<O> =>
		// @ts-ignore
		defer(() => {
			const fn = pickFn(api)
			// @ts-ignore
			return toObservable(fn(input, headers))
		})

	const toObsMap = <I extends Record<string, unknown>, E, O, P>(
		pickFn: (api: NodeAPI) => (input: I) => ResultAsync<O, E | E[]>,
		mapOutput: (output: O) => P,
		input: I,
	): Observable<P> => toObs(pickFn, input).pipe(map(o => mapOutput(o)))

	return {
		setHeaders: (newHeaders: typeof headers) => (headers = newHeaders),

		node,

		validators: (
			input: string,
		): Observable<ValidatorsEndpoint.DecodedResponse> =>
			toObs(a => a['validators'], {
				network_identifier: { network: input },
			}),

		lookupValidator: (
			input: ValidatorAddressT,
		): Observable<ValidatorEndpoint.DecodedResponse> =>
			toObs(a => a['validator'], {
				network_identifier: { network: input.network },
				validator_identifier: {
					address: input.toString(),
				},
			}),

		networkId: () =>
			toObsMap(
				a => a['gateway'],
				m => m.network,
				{
					body: {},
				},
			),

		tokenBalancesForAddress: (address: AccountAddressT) =>
			toObs(a => a['accountBalances'], {
				network_identifier: { network: address.network },
				account_identifier: {
					address: address.toString(),
				},
			}),

		transactionHistory: (
			input: TransactionHistoryRequestInput,
		): Observable<AccountTransactionsEndpoint.DecodedResponse> =>
			toObs(a => a['accountTransactions'], {
				account_identifier: {
					address: input.address.toString(),
				},
				network_identifier: { network: input.address.network },
				limit: input.size,
				cursor: input.cursor?.toString(),
			}),

		recentTransactions: (
			input: RecentTransactionsRequestInput,
		): Observable<RecentTransactionEndpoint.DecodedResponse> =>
			toObs(a => a['recentTransactions'], {
				cursor: input.cursor?.toString(),
				network_identifier: { network: input.network },
			}),

		nativeToken: (
			network: string,
		): Observable<NativeTokenInfoEndpoint.DecodedResponse> =>
			toObs(a => a['nativeTokenInfo'], {
				network_identifier: { network },
			}),

		tokenInfo: (
			rri: ResourceIdentifierT,
		): Observable<TokenInfoEndpoint.DecodedResponse> =>
			toObs(a => a['tokenInfo'], {
				network_identifier: { network: rri.network },
				token_identifier: {
					rri: rri.toString(),
				},
			}),

		stakesForAddress: (
			address: AccountAddressT,
		): Observable<StakePositionsEndpoint.DecodedResponse> =>
			toObs(a => a['stakePositions'], {
				network_identifier: { network: address.network },
				account_identifier: {
					address: address.toString(),
				},
			}),

		unstakesForAddress: (
			address: AccountAddressT,
		): Observable<UnstakePositionsEndpoint.DecodedResponse> =>
			toObs(a => a['unstakePositions'], {
				network_identifier: { network: address.network },
				account_identifier: {
					address: address.toString(),
				},
			}),

		transactionStatus: (
			txID: TransactionIdentifierT,
			network: string,
		): Observable<TransactionEndpoint.DecodedResponse> =>
			toObs(a => a['getTransaction'], {
				network_identifier: { network },
				transaction_identifier: {
					hash: txID.toString(),
				},
			}),

		buildTransaction: (
			transactionIntent: TransactionIntent,
			from: AccountAddressT,
		): Observable<BuildTransactionEndpoint.DecodedResponse> =>
			toObs(a => a['buildTransaction'], {
				network_identifier: { network: from.network },
				actions: transactionIntent.actions.map(action =>
					action.type === ActionType.TOKEN_TRANSFER
						? {
								type: 'TransferTokens',
								from_account: {
									address: action.from_account.toString(),
								},
								to_account: {
									address: action.to_account.toString(),
								},
								amount: {
									value: action.amount.toString(),
									token_identifier: {
										rri: action.rri.toString(),
									},
								},
						  }
						: action.type === ActionType.STAKE_TOKENS
						? {
								type: 'StakeTokens',
								from_account: {
									address: action.from_account.toString(),
								},
								to_validator: {
									address: action.to_validator.toString(),
								},
								amount: {
									value: action.amount.toString(),
									token_identifier: {
										rri: action.rri.toString(),
									},
								},
						  }
						: Object.assign(
								{
									type: 'UnstakeTokens',
									from_validator: {
										address: action.from_validator.toString(),
									},
									to_account: {
										address: action.to_account.toString(),
									},
								},
								action.amount.valueOf() != 0
									? {
											amount: {
												value: action.amount.toString(),
												token_identifier: {
													rri: action.rri.toString(),
												},
											},
									  }
									: {
											unstake_percentage: action.unstake_percentage.valueOf(),
									  },
						  ),
				),
				fee_payer: {
					address: from.toString(),
				},
				message: transactionIntent.message
					? transactionIntent.message.toString('hex')
					: undefined,
				disable_token_mint_and_burn: true,
			}),

		finalizeTransaction: (
			network: string,
			signedTransaction: SignedTransaction,
		): Observable<FinalizeTransactionEndpoint.DecodedResponse> =>
			toObs(a => a['finalizeTransaction'], {
				network_identifier: { network },
				unsigned_transaction: signedTransaction.transaction.blob,
				signature: {
					bytes: signedTransaction.signature.toDER(),
					public_key: {
						hex: signedTransaction.publicKeyOfSigner.toString(),
					},
				},
			}),

		submitSignedTransaction: (
			network: string,
			finalizedTx: FinalizedTransaction,
		): Observable<SubmitTransactionEndpoint.DecodedResponse> =>
			toObs(a => a['submitTransaction'], {
				network_identifier: { network },
				signed_transaction: finalizedTx.blob,
			}),
	}
}
