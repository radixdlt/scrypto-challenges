import { Decoded } from './api/open-api/_types'

export type APIError = Decoded.TransactionBuildError

export type WalletError = ErrorT<'wallet'>

export type NodeError = ErrorT<'node'>

export type ErrorT<T extends 'api' | 'node' | 'wallet'> = {
	cause: T extends 'api'
		? APIErrorCause
		: T extends 'node'
		? NodeErrorCause
		: T extends 'wallet'
		? WalletErrorCause
		: unknown
	category: T extends 'api'
		? ErrorCategory.API
		: T extends 'node'
		? ErrorCategory.NODE
		: T extends 'wallet'
		? ErrorCategory.WALLET
		: unknown
	message: string | undefined
}

export enum ErrorCategory {
	NODE = 'node',
	WALLET = 'wallet',
	API = 'api',
}

export enum WalletErrorCause {
	LOAD_KEYSTORE_FAILED = 'LOAD_KEYSTORE_FAILED',
}

export enum NodeErrorCause {
	GET_NODE_FAILED = 'GET_NODE_FAILED',
}

export type APIErrorObject = { code: number; message: string }

export enum APIErrorCause {
	TOKEN_BALANCES_FAILED = 'TOKEN_BALANCES_FAILED',
	TRANSACTION_HISTORY_FAILED = 'TRANSACTION_HISTORY_FAILED',
	RECENT_TRANSACTIONS_FAILED = 'RECENT_TRANSACTIONS_FAILED',
	NATIVE_TOKEN_FAILED = 'NATIVE_TOKEN_FAILED',
	TOKEN_INFO_FAILED = 'TOKEN_INFO_FAILED',
	STAKES_FOR_ADDRESS_FAILED = 'STAKES_FOR_ADDRESS_FAILED',
	UNSTAKES_FOR_ADDRESS_FAILED = 'UNSTAKES_FOR_ADDRESS_FAILED',
	TX_STATUS_FAILED = 'TX_STATUS_FAILED',
	NETWORK_TX_THROUGHPUT_FAILED = 'NETWORK_TX_THROUGHPUT_FAILED',
	NETWORK_TX_DEMAND_FAILED = 'NETWORK_TX_DEMAND_FAILED',
	LOOKUP_TX_FAILED = 'LOOKUP_TX_FAILED',
	LOOKUP_VALIDATOR_FAILED = 'LOOKUP_VALIDATOR_FAILED',
	VALIDATORS_FAILED = 'VALIDATORS_FAILED',
	BUILD_TRANSACTION_FAILED = 'BUILD_TRANSACTION_FAILED',
	SUBMIT_SIGNED_TX_FAILED = 'SUBMIT_SIGNED_TX_FAILED',
	FINALIZE_TX_FAILED = 'FINALIZE_TX_FAILED',
	NETWORK_ID_FAILED = 'NETWORK_ID_FAILED',
}

const APIError = (cause: APIErrorCause) => (error: any): APIError => ({
	cause,
	...error,
})

export const nodeError = (error: Error): ErrorT<'node'> => ({
	cause: NodeErrorCause.GET_NODE_FAILED,
	category: ErrorCategory.NODE,
	message: error.message,
})

export const walletError = (error: Error): ErrorT<'wallet'> => ({
	cause: WalletErrorCause.LOAD_KEYSTORE_FAILED,
	category: ErrorCategory.WALLET,
	message: error.message,
})

export const tokenBalancesErr = APIError(APIErrorCause.TOKEN_BALANCES_FAILED)
export const transactionHistoryErr = APIError(
	APIErrorCause.TRANSACTION_HISTORY_FAILED,
)
export const recentTransactionsErr = APIError(
	APIErrorCause.RECENT_TRANSACTIONS_FAILED,
)
export const nativeTokenErr = APIError(APIErrorCause.NATIVE_TOKEN_FAILED)
export const tokenInfoErr = APIError(APIErrorCause.TOKEN_INFO_FAILED)
export const stakesForAddressErr = APIError(
	APIErrorCause.STAKES_FOR_ADDRESS_FAILED,
)
export const unstakesForAddressErr = APIError(
	APIErrorCause.UNSTAKES_FOR_ADDRESS_FAILED,
)
export const txStatusErr = APIError(APIErrorCause.TX_STATUS_FAILED)
export const NetworkTxThroughputErr = APIError(
	APIErrorCause.NETWORK_TX_THROUGHPUT_FAILED,
)
export const NetworkTxDemandErr = APIError(
	APIErrorCause.NETWORK_TX_DEMAND_FAILED,
)
export const buildTxFromIntentErr = (error: APIErrorObject): APIError =>
	APIError(APIErrorCause.BUILD_TRANSACTION_FAILED)(error)

export const submitSignedTxErr = APIError(APIErrorCause.SUBMIT_SIGNED_TX_FAILED)
export const finalizeTxErr = APIError(APIErrorCause.FINALIZE_TX_FAILED)

export const networkIdErr = APIError(APIErrorCause.NETWORK_ID_FAILED)

export const lookupTxErr = APIError(APIErrorCause.LOOKUP_TX_FAILED)

export const lookupValidatorErr = APIError(
	APIErrorCause.LOOKUP_VALIDATOR_FAILED,
)

export const validatorsErr = APIError(APIErrorCause.VALIDATORS_FAILED)
