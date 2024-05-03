import {
	isUInt256,
	isUnsafeInputForUInt256,
	uint256FromUnsafe,
	UInt256InputUnsafe,
} from './uint256-extensions'
import { err, ok, Result } from 'neverthrow'
import { AmountT } from './_types'
import { UInt256 } from '@radixdlt/uint256'

export type AmountUnsafeInput = UInt256InputUnsafe
export const isAmountUnsafeInput = (
	something: unknown,
): something is AmountUnsafeInput => isUnsafeInputForUInt256(something)

export type AmountOrUnsafeInput = AmountT | AmountUnsafeInput

export const isAmount = (something: unknown): something is AmountT =>
	isUInt256(something)

export const isAmountOrUnsafeInput = (
	something: unknown,
): something is AmountOrUnsafeInput =>
	isAmount(something) || isAmountUnsafeInput(something)

const fromUnsafe = (input: AmountOrUnsafeInput): Result<AmountT, Error> =>
	isAmount(input)
		? ok(input)
		: isAmountUnsafeInput(input)
		? uint256FromUnsafe(input)
		: err(
				new Error(
					`Unable to construct 'AmountT' because of bad input: '${JSON.stringify(
						input,
						null,
						4,
					)}'`,
				),
		  )

const isAmountMultipleOf = (
	input: Readonly<{
		amount: AmountT
		granularity: AmountT
	}>,
): boolean => {
	const { amount, granularity: other } = input
	const zero = UInt256.valueOf(0)
	return amount.mod(other, false).eq(zero)
}

export const Amount = {
	fromUnsafe,
	isAmountMultipleOf,
}
