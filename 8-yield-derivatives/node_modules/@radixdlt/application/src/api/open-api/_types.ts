import {
	AccountAddressT,
	ResourceIdentifierT,
	ValidatorAddressT,
} from '@radixdlt/account'
import {
	LedgerState as LedgerStateRaw,
	GatewayResponse,
	TokenNativeResponse,
	TokenDeriveResponse,
	AccountBalancesResponse,
	AccountStakesResponse,
	AccountUnstakesResponse,
	AccountTransactionsResponse,
	TransactionStatusStatusEnum,
	ValidatorsResponse,
	ValidatorUptime,
	TransactionRulesResponse,
	TransactionBuildResponse,
	TransactionSubmitResponse,
	TransactionStatusResponse,
	ValidatorsRequest,
	AccountTransactionsRequest,
	ValidatorInfo,
	TokenRequest,
	TokenNativeRequest,
	TokenDeriveRequest,
	AccountBalancesRequest,
	AccountStakesRequest,
	AccountUnstakesRequest,
	TransactionRulesRequest,
	TransactionBuildRequest,
	TransactionFinalizeRequest,
	TransactionSubmitRequest,
	TransactionStatusRequest,
	ValidatorResponse,
	TransactionFinalizeResponse,
	TokenResponse,
	RecentTransactionsRequest,
	RecentTransactionsResponse,
} from '@radixdlt/networking'
import { AmountT, Network } from '@radixdlt/primitives'
import {
	TransactionIdentifierT,
	Token,
	BuiltTransaction,
	FinalizedTransaction,
	PendingTransaction,
	StakePositions,
	UnstakePosition,
	SimpleTransactionHistory,
	SimpleExecutedTransaction,
} from '../../dto'

export namespace Decoded {
	export type TokenIdentifier = {
		rri: ResourceIdentifierT
	}

	export type TokenAmount = {
		value: AmountT
		token_identifier: TokenIdentifier
	}

	export type AccountIdentifier = {
		address: AccountAddressT
	}

	export type TokenProperties = {
		name?: string
		description?: string
		icon_url?: URL
		symbol: string
		is_supply_mutable: boolean
		granularity: AmountT
		owner: AccountIdentifier
	}

	export type TokenInfo = {
		total_minted: TokenAmount
		total_burned: TokenAmount
	}

	export type Token = {
		token_identifier: TokenIdentifier
		token_supply: TokenAmount
		info: TokenInfo
		token_properties: TokenProperties
	}

	export type LedgerState = Omit<LedgerStateRaw, 'timestamp'> & {
		timestamp: Date
	}

	export type ValidatorIdentifier = {
		address: ValidatorAddressT
	}

	export type AccountTransactionStatus = {
		status: TransactionStatusStatusEnum
		confirmed_time?: Date
	}

	export type TransactionIdentifier = {
		hash: TransactionIdentifierT
	}

	export type AccountBalances = {
		liquid_balances: TokenAmount[]
		staked_and_unstaking_balance: TokenAmount
	}

	export type AccountStakeEntry = {
		validator_identifier: ValidatorIdentifier
		delegated_stake: TokenAmount
	}

	export type AccountUnstakeEntry = {
		validator_identifier: ValidatorIdentifier
		unstaking_amount: TokenAmount
		epochs_until_unlocked: number
	}

	export type ValidatorInfo = {
		owner_stake: TokenAmount
		uptime: ValidatorUptime
	}

	export type ValidatorProperties = {
		url: URL
		validator_fee: string
		name: string
		registered: boolean
		owner_account_identifier: AccountIdentifier
	}

	export type Validator = {
		validator_identifier: ValidatorIdentifier
		stake: Decoded.TokenAmount
		info: ValidatorInfo
		properties: ValidatorProperties
	}

	export enum ActionType {
		Transfer = 'TransferTokens',
		Stake = 'StakeTokens',
		Unstake = 'UnstakeTokens',
		Mint = 'MintTokens',
		Burn = 'BurnTokens',
		CreateTokenDefinition = 'CreateTokenDefinition',
	}

	type BaseAction<T extends ActionType> = {
		type: T
	}

	export type TransferTokensAction = BaseAction<ActionType.Transfer> & {
		from: AccountIdentifier
		to: AccountIdentifier
		amount: TokenAmount
	}

	export type StakeTokensAction = BaseAction<ActionType.Stake> & {
		from?: AccountIdentifier
		to?: AccountIdentifier
		amount?: TokenAmount
	}

	export type UnstakeTokensAction = BaseAction<ActionType.Unstake> & {
		from: ValidatorIdentifier
		to: AccountIdentifier
		amount: TokenAmount
	}

	export type MintTokensAction = BaseAction<ActionType.Mint> & {
		to: ValidatorIdentifier
		amount: TokenAmount
	}

	export type BurnTokensAction = BaseAction<ActionType.Burn> & {
		from: ValidatorIdentifier
		amount: TokenAmount
	}

	export type CreateTokenDefinitionAction = BaseAction<ActionType.CreateTokenDefinition> & {
		token_properties: TokenProperties
		token_supply: TokenAmount
		to?: AccountIdentifier
	}

	export type Action =
		| TransferTokensAction
		| StakeTokensAction
		| UnstakeTokensAction
		| MintTokensAction
		| BurnTokensAction
		| CreateTokenDefinitionAction

	export type AccountTransaction = {
		transaction_status: AccountTransactionStatus
		transaction_identifier: TransactionIdentifier
		actions: Action[]
		fee_paid: TokenAmount
		metadata: {
			hex: string
			message?: string
		}
	}

	export type TransactionRules = {
		maximum_message_length: number
		minimum_stake: Decoded.TokenAmount
	}

	export type TransactionBuild = {
		fee: Decoded.TokenAmount
		unsigned_transaction: string
		payload_to_sign: string
	}

	export type NotEnoughResourcesError = {
		type: 'NotEnoughResourcesError'
		requested_amount: AmountT
		available_amount: AmountT
	}

	export type BelowMinimumStakeError = {
		type: 'BelowMinimumStakeError'
		requested_amount: AmountT
		minimum_amount: AmountT
	}

	export type NotValidatorOwnerError = {
		type: 'NotValidatorOwnerError'
		owner: Decoded.AccountIdentifier
		user: Decoded.AccountIdentifier
	}

	export type MessageTooLongError = {
		type: 'MessageTooLongError'
		length_limit: number
		attempted_length: number
	}

	export type CouldNotConstructFeesError = {
		type: 'CouldNotConstructFeesError'
		attempts: number
	}

	export type TransactionBuildError =
		| NotEnoughResourcesError
		| BelowMinimumStakeError
		| NotValidatorOwnerError
		| MessageTooLongError
		| CouldNotConstructFeesError
}

export namespace GatewayEndpoint {
	export type Input = {}

	export type Response = GatewayResponse

	export type DecodedResponse = {
		network: Network
	}
}

export namespace TokenInfoEndpoint {
	export type Input = TokenRequest

	export type Response = TokenResponse

	export type DecodedResponse = Token
}

export namespace NativeTokenInfoEndpoint {
	export type Input = TokenNativeRequest

	export type Response = TokenNativeResponse

	export type DecodedResponse = Token
}

export namespace DeriveTokenIdentifierEndpoint {
	export type Input = TokenDeriveRequest

	export type Response = TokenDeriveResponse

	export type DecodedResponse = {
		ledger_state: Decoded.LedgerState
		creator_account_identifier: Decoded.AccountIdentifier
		symbol: string
	}
}

export namespace AccountBalancesEndpoint {
	export type Input = AccountBalancesRequest

	export type Response = AccountBalancesResponse

	export type DecodedResponse = {
		ledger_state: Decoded.LedgerState
		account_balances: Decoded.AccountBalances
	}
}

export namespace StakePositionsEndpoint {
	export type Input = AccountStakesRequest
	export type Response = AccountStakesResponse
	export type DecodedResponse = {
		stakes: StakePositions
		pendingStakes: StakePositions
	}
}

export namespace UnstakePositionsEndpoint {
	export type Input = AccountUnstakesRequest
	export type Response = AccountUnstakesResponse
	export type DecodedResponse = {
		unstakes: Omit<UnstakePosition, 'withdrawTxID'>[]
		pendingUnstakes: Omit<UnstakePosition, 'withdrawTxID'>[]
	}
}

export namespace AccountTransactionsEndpoint {
	export type Input = AccountTransactionsRequest
	export type Response = AccountTransactionsResponse
	export type DecodedResponse = SimpleTransactionHistory
}

type Validator = {
	address: ValidatorAddressT
	ownerAddress: AccountAddressT
	name: string
	infoURL?: URL
	totalDelegatedStake: AmountT
	ownerDelegation: AmountT
	validatorFee: number
	registered: boolean
	isExternalStakeAccepted: boolean
	uptimePercentage: number
	proposalsMissed: number
	proposalsCompleted: number
}
export namespace ValidatorEndpoint {
	export type Input = ValidatorInfo
	export type Response = ValidatorResponse
	export type DecodedResponse = Validator
}

export namespace ValidatorsEndpoint {
	export type Input = ValidatorsRequest
	export type Response = ValidatorsResponse
	export type DecodedResponse = { validators: Validator[] }
}

export namespace TransactionRulesEndpoint {
	export type Input = TransactionRulesRequest
	export type Response = TransactionRulesResponse
	export type DecodedResponse = {
		ledger_state: Decoded.LedgerState
		transaction_rules: Decoded.TransactionRules
	}
}
export namespace BuildTransactionEndpoint {
	export type Input = TransactionBuildRequest
	export type Response = TransactionBuildResponse
	export type DecodedResponse = BuiltTransaction
}

export namespace FinalizeTransactionEndpoint {
	export type Input = TransactionFinalizeRequest
	export type Response = TransactionFinalizeResponse
	export type DecodedResponse = FinalizedTransaction
}

export namespace SubmitTransactionEndpoint {
	export type Input = TransactionSubmitRequest
	export type Response = TransactionSubmitResponse
	export type DecodedResponse = PendingTransaction
}

export namespace TransactionEndpoint {
	export type Input = TransactionStatusRequest
	export type Response = TransactionStatusResponse
	export type DecodedResponse = SimpleExecutedTransaction
}

export namespace RecentTransactionEndpoint {
	export type Input = RecentTransactionsRequest
	export type Response = RecentTransactionsResponse
	export type DecodedResponse = SimpleTransactionHistory
}
