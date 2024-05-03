import {
	ActionType,
	IntendedTransferTokensAction,
	TransferTokensInput,
} from './_types'
import {
	AccountAddress,
	AccountAddressT,
	isAccountAddressOrUnsafeInput,
	ResourceIdentifierT,
	ResourceIdentifier,
	isResourceIdentifierOrUnsafeInput,
} from '@radixdlt/account'
import { Amount, AmountT, isAmountOrUnsafeInput } from '@radixdlt/primitives'
import { combine, Result } from 'neverthrow'

export const isTransferTokensInput = (
	something: unknown,
): something is TransferTokensInput => {
	const inspection = something as TransferTokensInput
	return (
		isAccountAddressOrUnsafeInput(inspection.to_account) &&
		isAmountOrUnsafeInput(inspection.amount) &&
		isResourceIdentifierOrUnsafeInput(inspection.tokenIdentifier)
	)
}

const create = (
	input: TransferTokensInput,
	from_account: AccountAddressT,
): Result<IntendedTransferTokensAction, Error> =>
	combine([
		AccountAddress.fromUnsafe(input.to_account),
		Amount.fromUnsafe(input.amount),
		ResourceIdentifier.fromUnsafe(input.tokenIdentifier),
	]).map(
		(resultList): IntendedTransferTokensAction => {
			const to_account = resultList[0] as AccountAddressT
			const amount = resultList[1] as AmountT
			const rri = resultList[2] as ResourceIdentifierT

			return {
				to_account: to_account.toString(),
				amount,
				rri,
				type: ActionType.TOKEN_TRANSFER,
				from_account: from_account.toString(),
			}
		},
	)

export const IntendedTransferTokens = {
	create,
}
