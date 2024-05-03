import {
	ActionType,
	IntendedUnstakeTokensAction,
	UnstakeTokensInput,
} from './_types'
import {
	AccountAddressT,
	isResourceIdentifierOrUnsafeInput,
	isValidatorAddressOrUnsafeInput,
	ResourceIdentifier,
	ResourceIdentifierT,
	ValidatorAddress,
	ValidatorAddressT,
} from '@radixdlt/account'
import { combine, Result } from 'neverthrow'
import { Amount, AmountT } from '@radixdlt/primitives'

const create = (
	input: UnstakeTokensInput,
	to_account: AccountAddressT,
): Result<IntendedUnstakeTokensAction, Error> =>
	combine([
		ValidatorAddress.fromUnsafe(input.from_validator),
		Amount.fromUnsafe(input.amount ?? 0),
		Amount.fromUnsafe(input.unstake_percentage ?? 0),
		ResourceIdentifier.fromUnsafe(input.tokenIdentifier),
	]).map(
		(resultList): IntendedUnstakeTokensAction => {
			const from_validator = resultList[0] as ValidatorAddressT
			const amount = resultList[1] as AmountT
			const unstake_percentage = resultList[2] as AmountT
			const rri = resultList[3] as ResourceIdentifierT

			return {
				from_validator: from_validator.toString(),
				amount,
				unstake_percentage,
				type: ActionType.UNSTAKE_TOKENS,
				to_account: to_account.toString(),
				rri,
			}
		},
	)

export const IntendedUnstakeTokens = {
	create,
}
