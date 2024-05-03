import {
	ActionType,
	IntendedStakeTokensAction,
	StakeTokensInput,
} from './_types'
import {
	AccountAddressT,
	isValidatorAddressOrUnsafeInput,
	ValidatorAddress,
	ValidatorAddressT,
	ResourceIdentifier,
	ResourceIdentifierT,
	isResourceIdentifierOrUnsafeInput,
} from '@radixdlt/account'
import { Amount, AmountT, isAmountOrUnsafeInput } from '@radixdlt/primitives'
import { combine, Result } from 'neverthrow'

export const isStakeTokensInput = (
	something: unknown,
): something is StakeTokensInput => {
	const inspection = something as StakeTokensInput
	return (
		isValidatorAddressOrUnsafeInput(inspection.to_validator) &&
		isAmountOrUnsafeInput(inspection.amount) &&
		isResourceIdentifierOrUnsafeInput(inspection.tokenIdentifier)
	)
}

const create = (
	input: StakeTokensInput,
	from_account: AccountAddressT,
): Result<IntendedStakeTokensAction, Error> =>
	combine([
		ValidatorAddress.fromUnsafe(input.to_validator),
		Amount.fromUnsafe(input.amount),
		ResourceIdentifier.fromUnsafe(input.tokenIdentifier),
	]).map(
		(resultList): IntendedStakeTokensAction => {
			const to_validator = resultList[0] as ValidatorAddressT
			const amount = resultList[1] as AmountT
			const rri = resultList[2] as ResourceIdentifierT

			return {
				to_validator: to_validator.toString(),
				amount,
				type: ActionType.STAKE_TOKENS,
				from_account: from_account.toString(),
				rri,
			}
		},
	)

export const IntendedStakeTokens = {
	create,
}
