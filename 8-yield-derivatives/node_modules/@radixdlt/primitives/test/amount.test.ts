import { UInt256 } from '@radixdlt/uint256'
import { isUInt256 } from '../src'
import { Amount } from '../src/amount'

describe('Amount', () => {
	it('can check for multiple with granularity', () => {
		const granularity = Amount.fromUnsafe(1)._unsafeUnwrap()
		const amount = Amount.fromUnsafe(10)._unsafeUnwrap()

		expect(Amount.isAmountMultipleOf({ amount, granularity })).toBe(true)
		expect(
			Amount.isAmountMultipleOf({
				amount: granularity,
				granularity: amount,
			}),
		).toBe(false)
	})

	it('toString of 1 amount', () => {
		const uOne = UInt256.valueOf(1)
		expect(uOne.toString()).toBe('1')
		const amount = Amount.fromUnsafe(1)._unsafeUnwrap()
		expect(amount.toString()).toBe('1')
	})

	it('addition works as expected', () => {
		const one = Amount.fromUnsafe(1)._unsafeUnwrap()
		const two = Amount.fromUnsafe(2)._unsafeUnwrap()
		const three = Amount.fromUnsafe(3)._unsafeUnwrap()
		const res = one.add(two)
		expect(res.eq(three)).toBe(true)
		expect(res.toString()).toBe('3')
	})

	it('can typeguard UInt256', () => {
		const uOne = UInt256.valueOf(1)
		expect(isUInt256(uOne)).toBe(true)
		expect(isUInt256(1)).toBe(false)
	})
})
