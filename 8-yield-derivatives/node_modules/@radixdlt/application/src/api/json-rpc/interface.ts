import { Result, ResultAsync } from 'neverthrow'
import {
	handleBuildTransactionResponse,
	handleLookupTXResponse,
	handleNetworkxDemandResponse,
	handleNetworkxThroughputResponse,
	handleStakesResponse,
	handleFinalizeTransactionResponse,
	handleTokenBalancesResponse,
	handleTokenInfoResponse,
	handleTransactionHistoryResponse,
	handleTransactionStatusResponse,
	handleNetworkIdResponse,
	handleUnstakesResponse,
	handleValidatorsResponse,
	handleSubmitTransactionResponse,
	handleLookupValidatorResponse,
} from './responseHandlers'
import { andThen, pipe } from 'ramda'
import {
	ApiMethod,
	BuildTransactionEndpoint,
	SubmitTransactionEndpoint,
	LookupTransactionEndpoint,
	NativeTokenEndpoint,
	NetworkIdEndpoint,
	NetworkTransactionDemandEndpoint,
	NetworkTransactionThroughputEndpoint,
	StakePositionsEndpoint,
	FinalizeTransactionEndpoint,
	TokenBalancesEndpoint,
	TokenInfoEndpoint,
	TransactionHistoryEndpoint,
	TransactionStatusEndpoint,
	UnstakePositionsEndpoint,
	ValidatorsEndpoint,
	LookupValidatorEndpoint,
} from './_types'

const callAPI = <Params extends Record<string, unknown>, DecodedResponse>(
	endpoint: ApiMethod,
) => (
	call: (
		endpoint: ApiMethod,
		params: Params,
		headers?: Record<string, string>,
	) => Promise<unknown>,
	handleResponse: (response: unknown) => Result<DecodedResponse, Error[]>,
) => (params: Params) =>
	pipe(call, andThen(handleResponse), value =>
		// @ts-ignore
		ResultAsync.fromPromise(value, (e: Error[]) => e).andThen(r => r),
	)(endpoint, params)

const setupAPICall = (
	call: (
		endpoint: ApiMethod,
		params: Record<string, unknown>,
		headers?: Record<string, string>,
	) => Promise<unknown>,
) => <I extends Record<string, unknown>, R>(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	handleResponse: (response: any) => Result<R, Error[]>,
) => (endpoint: ApiMethod) => callAPI<I, R>(endpoint)(call, handleResponse)

export const getAPI = (
	call: (
		endpoint: ApiMethod,
		params: Record<string, unknown>,
		headers?: Record<string, string>,
	) => Promise<unknown>,
) => {
	const setupAPIResponse = setupAPICall(call)

	return {
		[ApiMethod.NETWORK_ID]: setupAPIResponse<
			NetworkIdEndpoint.Input,
			NetworkIdEndpoint.DecodedResponse
		>(handleNetworkIdResponse)(ApiMethod.NETWORK_ID),

		[ApiMethod.TOKEN_BALANCES]: setupAPIResponse<
			TokenBalancesEndpoint.Input,
			TokenBalancesEndpoint.DecodedResponse
		>(handleTokenBalancesResponse)(ApiMethod.TOKEN_BALANCES),

		[ApiMethod.VALIDATORS]: setupAPIResponse<
			ValidatorsEndpoint.Input,
			ValidatorsEndpoint.DecodedResponse
		>(handleValidatorsResponse)(ApiMethod.VALIDATORS),

		[ApiMethod.LOOKUP_TX]: setupAPIResponse<
			LookupTransactionEndpoint.Input,
			LookupTransactionEndpoint.DecodedResponse
		>(handleLookupTXResponse)(ApiMethod.LOOKUP_TX),

		[ApiMethod.LOOKUP_VALIDATOR]: setupAPIResponse<
			LookupValidatorEndpoint.Input,
			LookupValidatorEndpoint.DecodedResponse
		>(handleLookupValidatorResponse)(ApiMethod.LOOKUP_VALIDATOR),

		[ApiMethod.TRANSACTION_HISTORY]: setupAPIResponse<
			TransactionHistoryEndpoint.Input,
			TransactionHistoryEndpoint.DecodedResponse
		>(handleTransactionHistoryResponse)(ApiMethod.TRANSACTION_HISTORY),

		[ApiMethod.NATIVE_TOKEN]: setupAPIResponse<
			NativeTokenEndpoint.Input,
			NativeTokenEndpoint.DecodedResponse
		>(handleTokenInfoResponse)(ApiMethod.NATIVE_TOKEN),

		[ApiMethod.TOKEN_INFO]: setupAPIResponse<
			TokenInfoEndpoint.Input,
			TokenInfoEndpoint.DecodedResponse
		>(handleTokenInfoResponse)(ApiMethod.TOKEN_INFO),

		[ApiMethod.STAKES]: setupAPIResponse<
			StakePositionsEndpoint.Input,
			StakePositionsEndpoint.DecodedResponse
		>(handleStakesResponse)(ApiMethod.STAKES),

		[ApiMethod.UNSTAKES]: setupAPIResponse<
			UnstakePositionsEndpoint.Input,
			UnstakePositionsEndpoint.DecodedResponse
		>(handleUnstakesResponse)(ApiMethod.UNSTAKES),

		[ApiMethod.TX_STATUS]: setupAPIResponse<
			TransactionStatusEndpoint.Input,
			TransactionStatusEndpoint.DecodedResponse
		>(handleTransactionStatusResponse)(ApiMethod.TX_STATUS),

		[ApiMethod.NETWORK_TX_THROUGHPUT]: setupAPIResponse<
			NetworkTransactionThroughputEndpoint.Input,
			NetworkTransactionThroughputEndpoint.DecodedResponse
		>(handleNetworkxThroughputResponse)(ApiMethod.NETWORK_TX_THROUGHPUT),

		[ApiMethod.NETWORK_TX_DEMAND]: setupAPIResponse<
			NetworkTransactionDemandEndpoint.Input,
			NetworkTransactionDemandEndpoint.DecodedResponse
		>(handleNetworkxDemandResponse)(ApiMethod.NETWORK_TX_DEMAND),

		[ApiMethod.BUILD_TX_FROM_INTENT]: setupAPIResponse<
			BuildTransactionEndpoint.Input,
			BuildTransactionEndpoint.DecodedResponse
		>(handleBuildTransactionResponse)(ApiMethod.BUILD_TX_FROM_INTENT),

		[ApiMethod.FINALIZE_TX]: setupAPIResponse<
			FinalizeTransactionEndpoint.Input,
			FinalizeTransactionEndpoint.DecodedResponse
		>(handleFinalizeTransactionResponse)(ApiMethod.FINALIZE_TX),

		[ApiMethod.SUBMIT_TX]: setupAPIResponse<
			SubmitTransactionEndpoint.Input,
			SubmitTransactionEndpoint.DecodedResponse
		>(handleSubmitTransactionResponse)(ApiMethod.SUBMIT_TX),
	}
}
