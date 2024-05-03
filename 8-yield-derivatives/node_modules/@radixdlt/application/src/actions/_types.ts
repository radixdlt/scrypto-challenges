import {
	AddressOrUnsafeInput,
	ValidatorAddressOrUnsafeInput,
	ResourceIdentifierOrUnsafeInput,
	ResourceIdentifierT,
} from '@radixdlt/account'
import { AmountOrUnsafeInput, AmountT } from '@radixdlt/primitives'

export enum ActionType {
	TOKEN_TRANSFER = 'TokenTransfer',
	STAKE_TOKENS = 'StakeTokens',
	UNSTAKE_TOKENS = 'UnstakeTokens',
	MINT_TOKENS = 'MintTokens',
	BURN_TOKENS = 'BurnTokens',
	CREATE_TOKEN_DEFINITION = 'CreateTokenDefinition',
	OTHER = 'Other',
}

export type Action<T extends ActionType = ActionType.OTHER> = Readonly<{
	type: T
}>

// ##################################
// ####                         #####
// ####     INPUTTED ACTIONS    #####
// ####                         #####
// ##################################

export type TransferTokensInput = Readonly<{
	to_account: AddressOrUnsafeInput
	amount: AmountOrUnsafeInput
	tokenIdentifier: ResourceIdentifierOrUnsafeInput
}>

// Same input for stake/unstake for now
export type StakeTokensInput = Readonly<{
	to_validator: ValidatorAddressOrUnsafeInput
	amount: AmountOrUnsafeInput
	tokenIdentifier: ResourceIdentifierOrUnsafeInput
}>

export type UnstakeTokensInput = Readonly<{
	from_validator: ValidatorAddressOrUnsafeInput
	amount?: AmountOrUnsafeInput
	unstake_percentage?: AmountOrUnsafeInput
	tokenIdentifier: ResourceIdentifierOrUnsafeInput
}>

export type ActionInput =
	| TransferTokensInput
	| StakeTokensInput
	| UnstakeTokensInput

// ##################################
// ####                         #####
// ####     INTENDED ACTIONS    #####
// ####                         #####
// ##################################
export type TransferTokensProps = Readonly<{
	to_account: string
	from_account: string
	amount: AmountT
	rri: ResourceIdentifierT
}>

export type TransferTokensAction = TransferTokensProps &
	Action<ActionType.TOKEN_TRANSFER>

export type StakeTokensProps = Readonly<{
	from_account: string
	to_validator: string
	amount: AmountT
	rri: ResourceIdentifierT
}>

export type UnstakeTokensProps = Readonly<{
	to_account: string
	from_validator: string
	amount: AmountT
	unstake_percentage: AmountT
	rri: ResourceIdentifierT
}>

export type MintTokensProps = Readonly<{
	to_account: string
	amount: AmountT
	rri: ResourceIdentifierT
}>

export type BurnTokensProps = Readonly<{
	from_account: string
	amount: AmountT
	rri: ResourceIdentifierT
}>

export type CreateTokenDefinitionProps = Readonly<{
	name: string
	description: string
	icon_url: string
	url: string
	symbol: string
	is_supply_mutable: boolean
	granularity: string
	owner?: string
	to_account?: string
	amount: AmountT
	rri: ResourceIdentifierT
}>

export type StakeTokensAction = StakeTokensProps &
	Action<ActionType.STAKE_TOKENS>
export type UnstakeTokensAction = UnstakeTokensProps &
	Action<ActionType.UNSTAKE_TOKENS>
export type MintTokensAction = MintTokensProps & Action<ActionType.MINT_TOKENS>
export type BurnTokensAction = BurnTokensProps & Action<ActionType.BURN_TOKENS>
export type CreateTokenDefinitionAction = CreateTokenDefinitionProps &
	Action<ActionType.CREATE_TOKEN_DEFINITION>
// An intended action specified by the user. Not yet accepted by
// Radix Core API.
export type IntendedActionBase<T extends ActionType> = Action<T>

export type IntendedTransferTokensAction = IntendedActionBase<ActionType.TOKEN_TRANSFER> &
	TransferTokensAction

export type IntendedStakeTokensAction = IntendedActionBase<ActionType.STAKE_TOKENS> &
	StakeTokensProps

export type IntendedUnstakeTokensAction = IntendedActionBase<ActionType.UNSTAKE_TOKENS> &
	UnstakeTokensProps

export type IntendedAction =
	| IntendedTransferTokensAction
	| IntendedStakeTokensAction
	| IntendedUnstakeTokensAction

// ##################################
// ####                         #####
// ####     EXECUTED ACTIONS    #####
// ####                         #####
// ##################################

// An executed action stored in the Radix Ledger, part
// of transaction history. Marker type.
export type ExecutedActionBase<T extends ActionType> = Action<T>

export type ExecutedTransferTokensAction = ExecutedActionBase<ActionType.TOKEN_TRANSFER> &
	TransferTokensAction

export type ExecutedStakeTokensAction = ExecutedActionBase<ActionType.STAKE_TOKENS> &
	StakeTokensAction

export type ExecutedUnstakeTokensAction = ExecutedActionBase<ActionType.UNSTAKE_TOKENS> &
	UnstakeTokensAction

export type ExecutedMintTokensAction = ExecutedActionBase<ActionType.MINT_TOKENS> &
	MintTokensAction

export type ExecutedBurnTokensAction = ExecutedActionBase<ActionType.BURN_TOKENS> &
	BurnTokensAction

export type ExecutedCreateTokenDefinitionAction = ExecutedActionBase<ActionType.CREATE_TOKEN_DEFINITION> &
	CreateTokenDefinitionAction

// OTHER (Only "Executed")
export type ExecutedOtherAction = ExecutedActionBase<ActionType.OTHER>

export type ExecutedAction =
	| ExecutedTransferTokensAction
	| ExecutedStakeTokensAction
	| ExecutedUnstakeTokensAction
	| ExecutedMintTokensAction
	| ExecutedBurnTokensAction
	| ExecutedCreateTokenDefinitionAction
	| ExecutedOtherAction
