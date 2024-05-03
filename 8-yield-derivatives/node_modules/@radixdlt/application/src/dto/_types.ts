import {
	AccountAddressT,
	ResourceIdentifierT,
	ValidatorAddressT,
} from '@radixdlt/account'
import {
	ActionInput,
	ActionType,
	ExecutedAction,
	IntendedAction,
	StakeTokensInput,
	TransferTokensInput,
	UnstakeTokensInput,
} from '../actions'
import {
	AmountT,
	BuiltTransactionReadyToSign,
	Network,
} from '@radixdlt/primitives'
import { PublicKeyT, SignatureT } from '@radixdlt/crypto'
import { Observable } from 'rxjs'
import { Result } from 'neverthrow'
import { AccountT, MessageInTransaction } from '../_types'

export type StakePosition = Readonly<{
	validator: ValidatorAddressT
	amount: AmountT
}>

export type UnstakePosition = Readonly<{
	validator: ValidatorAddressT
	amount: AmountT
	withdrawTxID: TransactionIdentifierT
	epochsUntil: number
}>

/**
 * A transaction identifier, 32 bytes hash of signature + hashOfTxBlob.
 * Used to lookup transactions by ID.
 */
export type TransactionIdentifierT = Readonly<{
	__witness: 'isTXId'
	__hex: string
	toString: () => string
	equals: (other: TransactionIdentifierT) => boolean
}>

export type TransactionIntentBuilderState = Readonly<{
	actionInputs: ActionInput[]
	message?: MessageInTransaction
}>

export type TransactionIntentBuilderEncryptOption = Readonly<{
	encryptMessageIfAnyWithAccount: Observable<AccountT>
	spendingSender?: Observable<AccountAddressT>
}>

export type TransactionIntentBuilderDoNotEncryptInput = Readonly<{
	spendingSender: Observable<AccountAddressT>
}>

export type TransactionIntentBuilderDoNotEncryptOption = Readonly<{
	skipEncryptionOfMessageIfAny: TransactionIntentBuilderDoNotEncryptInput
}>
export type TransactionIntentBuilderOptions =
	| TransactionIntentBuilderDoNotEncryptOption
	| TransactionIntentBuilderEncryptOption

export type TransactionIntentBuilderT = Readonly<{
	__state: TransactionIntentBuilderState

	transferTokens: (input: TransferTokensInput) => TransactionIntentBuilderT
	stakeTokens: (input: StakeTokensInput) => TransactionIntentBuilderT
	unstakeTokens: (input: UnstakeTokensInput) => TransactionIntentBuilderT
	message: (msg: MessageInTransaction) => TransactionIntentBuilderT

	// Build
	__syncBuildDoNotEncryptMessageIfAny: (
		from: AccountAddressT,
	) => Result<TransactionIntent, Error>

	build: (
		options: TransactionIntentBuilderOptions,
	) => Observable<TransactionIntent>
}>

export type TransactionIntent = Readonly<{
	actions: IntendedAction[]
	message?: Buffer
}>

export type ValidatorsRequestInput = Readonly<{
	size: number
	// AccountAddress of last seen validator in list
	cursor?: string
}>

export enum TransactionTrackingEventType {
	/* A TransactionIntent was successfully created and any message has been encrypted */
	INITIATED = 'INITIATED',
	BUILT_FROM_INTENT = 'BUILT_FROM_INTENT',
	SIGNED = 'SIGNED',
	SUBMITTED = 'SUBMITTED',
	ASKED_FOR_CONFIRMATION = 'ASKED_FOR_CONFIRMATION',
	CONFIRMED = 'CONFIRMED',
	/* API has finished "finalizing" / "confirming" the transaction, which now is pending. */
	FINALIZED = 'FINALIZED',
	UPDATE_OF_STATUS_OF_PENDING_TX = 'UPDATE_OF_STATUS_OF_PENDING_TX',
	COMPLETED = 'COMPLETED',
}

export type TransactionStateSuccess<
	T extends TransactionState = TransactionState
> = Readonly<{
	eventUpdateType: TransactionTrackingEventType
	transactionState: T
}>

export type TransactionStateError = Readonly<{
	eventUpdateType: TransactionTrackingEventType
	error: Error
}>

export type TransactionStateUpdate<
	T extends TransactionState = TransactionState
> = TransactionStateSuccess<T> | TransactionStateError

export type TransactionState =
	| TransactionIntent
	| BuiltTransaction
	| SignedTransaction
	| FinalizedTransaction
	| PendingTransaction

export type TransactionTracking = Readonly<{
	events: Observable<TransactionStateUpdate>
	completion: Observable<TransactionIdentifierT>
}>

export type TransactionHistoryOfKnownAddressRequestInput = Readonly<{
	size: number
	cursor?: string
}>

export type TransactionHistoryActiveAccountRequestInput = TransactionHistoryOfKnownAddressRequestInput

export type TransactionHistoryRequestInput = TransactionHistoryOfKnownAddressRequestInput &
	Readonly<{
		address: AccountAddressT
	}>

export type RecentTransactionsRequestInput = Readonly<{
	network: Network
	cursor?: string
}>

export type SimpleExecutedTransaction = Readonly<{
	txID: TransactionIdentifierT
	sentAt: Date
	status: TransactionStatus
	fee: AmountT
	message?: string
	actions: ExecutedAction[]
}>

export enum TransactionType {
	FROM_ME_TO_ME = 'FROM_ME_TO_ME',
	INCOMING = 'INCOMING',
	OUTGOING = 'OUTGOING',
	UNRELATED = 'UNRELATED',
}

export type ExecutedTransaction = SimpleExecutedTransaction &
	Readonly<{
		transactionType: TransactionType
	}>

export type TokenAmount = Readonly<{
	tokenIdentifier: ResourceIdentifierT
	amount: AmountT
}>

export type SimpleTokenBalance = TokenAmount

export type TokenBalance = Readonly<{
	token: Token
	amount: AmountT
}>

export type Token = Readonly<{
	name: string
	rri: ResourceIdentifierT
	symbol: string
	description?: string
	granularity: AmountT
	isSupplyMutable: boolean
	currentSupply: AmountT
	tokenInfoURL?: URL
	iconURL?: URL
}>

export type StatusOfTransaction = Readonly<{
	txID: TransactionIdentifierT
	status: TransactionStatus
}>

export type BuiltTransaction = Readonly<{
	transaction: BuiltTransactionReadyToSign
	fee: AmountT
}>

export type SignedTransaction = Readonly<{
	transaction: BuiltTransactionReadyToSign
	publicKeyOfSigner: PublicKeyT
	signature: SignatureT
}>

export type FinalizedTransaction = Readonly<{
	blob: string
	txID: TransactionIdentifierT
}>

export type PendingTransaction = Readonly<{
	txID: TransactionIdentifierT
}>

export type RawToken = Readonly<{
	name: string
	rri: string
	symbol: string
	description?: string
	granularity: string
	isSupplyMutable: boolean
	currentSupply: string
	tokenInfoURL: string
	iconURL: string
}>

export type RawExecutedActionBase<T extends ActionType> = Readonly<{
	type: T
}>

export type RawOtherExecutedAction = RawExecutedActionBase<ActionType.OTHER>

export type RawTransferAction = RawExecutedActionBase<ActionType.TOKEN_TRANSFER> &
	Readonly<{
		from: string
		to: string
		amount: string
		rri: string
	}>

export type RawStakesAction = RawExecutedActionBase<ActionType.STAKE_TOKENS> &
	Readonly<{
		from: string
		validator: string
		amount: string
	}>

export type RawUnstakesAction = RawExecutedActionBase<ActionType.UNSTAKE_TOKENS> &
	Readonly<{
		from: string
		validator: string
		amount: string
	}>

export type NetworkTransactionThroughput = Readonly<{
	tps: number
}>
export type NetworkTransactionDemand = NetworkTransactionThroughput

export enum TransactionStatus {
	PENDING = 'PENDING',
	CONFIRMED = 'CONFIRMED',
	FAILED = 'FAILED',
}

export type RawExecutedAction =
	| RawTransferAction
	| RawStakesAction
	| RawUnstakesAction
	| RawOtherExecutedAction

export type SimpleTokenBalances = Readonly<{
	owner: AccountAddressT
	tokenBalances: SimpleTokenBalance[]
}>

export type TokenBalances = Readonly<{
	owner: AccountAddressT
	tokenBalances: TokenBalance[]
}>

export type SimpleTransactionHistory = Readonly<{
	cursor: string
	transactions: SimpleExecutedTransaction[]
}>

export type TransactionHistory = SimpleTransactionHistory &
	Readonly<{
		transactions: ExecutedTransaction[]
	}>

export type Validator = Readonly<{
	address: ValidatorAddressT
	ownerAddress: AccountAddressT
	name: string
	infoURL: URL
	totalDelegatedStake: AmountT
	ownerDelegation: AmountT
	validatorFee: number
	registered: boolean
	isExternalStakeAccepted: boolean
	uptimePercentage: number
	proposalsMissed: number
	proposalsCompleted: number
}>

export type Validators = Readonly<{
	validators: Validator[]
}>

export type RawExecutedTransaction = Readonly<{
	txID: string
	sentAt: string
	fee: string
	message?: string
	actions: RawExecutedAction[]
}>

export type RawValidatorResponse = Readonly<{
	address: string
	ownerAddress: string
	name: string
	infoURL: string
	totalDelegatedStake: string
	ownerDelegation: string
	validatorFee: string
	registered: boolean
	isExternalStakeAccepted: boolean
	uptimePercentage: string
	proposalsMissed: number
	proposalsCompleted: number
}>

export type StakePositions = StakePosition[]

export type UnstakePositions = UnstakePosition[]
