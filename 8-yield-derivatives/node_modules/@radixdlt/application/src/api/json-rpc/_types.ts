import { Network } from '@radixdlt/primitives'
import { ActionType } from '../../actions'
import {
	SimpleExecutedTransaction,
	NetworkTransactionDemand,
	NetworkTransactionThroughput,
	PendingTransaction,
	RawExecutedTransaction,
	RawToken,
	FinalizedTransaction,
	StakePositions,
	StatusOfTransaction,
	Token,
	SimpleTransactionHistory,
	TransactionStatus,
	BuiltTransaction,
	UnstakePositions,
	Validators,
	SimpleTokenBalances,
	Validator,
	RawValidatorResponse,
} from '../../dto'

export enum ApiMethod {
	NETWORK_ID = 'network.get_id',
	TOKEN_BALANCES = 'account.get_balances',
	TRANSACTION_HISTORY = 'account.get_transaction_history',
	STAKES = 'account.get_stake_positions',
	UNSTAKES = 'account.get_unstake_positions',
	TX_STATUS = 'transactions.get_transaction_status',
	NETWORK_TX_THROUGHPUT = 'network.get_throughput',
	NETWORK_TX_DEMAND = 'network.get_demand',
	VALIDATORS = 'validators.get_next_epoch_set',
	LOOKUP_TX = 'transactions.lookup_transaction',
	LOOKUP_VALIDATOR = 'validators.lookup_validator',
	NATIVE_TOKEN = 'tokens.get_native_token',
	TOKEN_INFO = 'tokens.get_info',
	BUILD_TX_FROM_INTENT = 'construction.build_transaction',
	SUBMIT_TX = 'construction.submit_transaction',
	FINALIZE_TX = 'construction.finalize_transaction',
}

export namespace NetworkIdEndpoint {
	export type Input = Record<string, never>

	export type Response = {
		networkId: number
	}

	export type DecodedResponse = {
		networkId: Network
	}
}

export namespace TokenBalancesEndpoint {
	export type Input = {
		address: string
	}

	export type Response = {
		owner: string
		tokenBalances: {
			rri: string
			amount: string
		}[]
	}

	export type DecodedResponse = SimpleTokenBalances
}

export namespace TransactionHistoryEndpoint {
	export type Input = {
		address: string
		size: number // must be > 0
		cursor?: string
	}

	export type Response = Readonly<{
		cursor: string
		transactions: RawExecutedTransaction[]
	}>

	export type DecodedResponse = SimpleTransactionHistory
}

export namespace LookupTransactionEndpoint {
	export type Input = { txID: string }
	export type Response = RawExecutedTransaction
	export type DecodedResponse = SimpleExecutedTransaction
}

export namespace TokenInfoEndpoint {
	export type Input = { rri: string }
	export type Response = RawToken
	export type DecodedResponse = Token
}

export namespace NativeTokenEndpoint {
	export type Input = Record<string, never>
	export type Response = RawToken
	export type DecodedResponse = Token
}

export namespace StakePositionsEndpoint {
	export type Input = { address: string }

	export type Response = {
		validator: string
		amount: string
	}[]

	export type DecodedResponse = StakePositions
}

export namespace UnstakePositionsEndpoint {
	export type Input = { address: string }

	export type Response = {
		amount: string
		validator: string
		epochsUntil: number
		withdrawTxID: string
	}[]

	export type DecodedResponse = UnstakePositions
}

export namespace TransactionStatusEndpoint {
	export type Input = { txID: string }

	export type Response = {
		txID: string
		status: TransactionStatus
		failure?: string
	}

	export type DecodedResponse = StatusOfTransaction
}

export namespace NetworkTransactionThroughputEndpoint {
	export type Input = Record<string, never>

	export type Response = {
		tps: number
	}

	export type DecodedResponse = NetworkTransactionThroughput
}

export namespace NetworkTransactionDemandEndpoint {
	export type Input = Record<string, never>

	export type Response = {
		tps: number
	}

	export type DecodedResponse = NetworkTransactionDemand
}

export namespace ValidatorsEndpoint {
	export type Input = { size: number; cursor?: string }

	export type Response = Readonly<{
		cursor: string
		validators: RawValidatorResponse[]
	}>
	export type DecodedResponse = Validators
}

export namespace LookupValidatorEndpoint {
	export type Input = { validatorAddress: string }
	export type Response = RawValidatorResponse
	export type DecodedResponse = Validator
}

export namespace BuildTransactionEndpoint {
	export type Failure =
		| 'MALFORMED_TX'
		| 'INSUFFICIENT_FUNDS'
		| 'NOT_PERMITTED'

	export type Input = {
		actions: (
			| {
					type: ActionType.TOKEN_TRANSFER
					from: string
					to: string
					amount: string
					rri: string
			  }
			| {
					type: ActionType.STAKE_TOKENS
					from: string
					validator: string
					amount: string
			  }
			| {
					type: ActionType.UNSTAKE_TOKENS
					from: string
					validator: string
					amount: string
			  }
		)[]
		feePayer: string
		disableResourceAllocationAndDestroy?: boolean
		message?: string
	}

	export type Response = {
		transaction: Readonly<{
			blob: string
			hashOfBlobToSign: string
		}>
		fee: string
	}

	export type DecodedResponse = BuiltTransaction
}

export namespace FinalizeTransactionEndpoint {
	export type Input = {
		blob: string
		publicKeyOfSigner: string
		signatureDER: string
	}

	export type Response = {
		blob: string
		txID: string
	}

	export type DecodedResponse = FinalizedTransaction
}

export namespace SubmitTransactionEndpoint {
	export type Input = {
		blob: string
		txID: string
	}

	export type Response = {
		txID: string
	}

	export type DecodedResponse = PendingTransaction
}
