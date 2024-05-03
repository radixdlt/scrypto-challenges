import { TransactionIdentifier } from '../src/dto/transactionIdentifier'

describe('TransactionIdentifier', () => {
	it('can check for equality', () => {
		const a0 = TransactionIdentifier.create(buffer0)._unsafeUnwrap()
		const b0 = TransactionIdentifier.create(buffer0)._unsafeUnwrap()
		const a1 = TransactionIdentifier.create(buffer1)._unsafeUnwrap()

		expect(a0.equals(b0)).toBe(true)
		expect(a0.equals(a1)).toBe(false)
	})

	it('can be converted to string', () => {
		const txID = TransactionIdentifier.create(buffer0)._unsafeUnwrap()
		expect(txID.toString()).toBe(deadbeefString)
	})

	it('can be created from hex string', () => {
		const txID = TransactionIdentifier.create(
			deadbeefString,
		)._unsafeUnwrap()
		expect(txID.toString()).toBe(deadbeefString)
	})

	const deadbeefString =
		'deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef'
	const buffer0 = Buffer.from(deadbeefString, 'hex')
	const buffer1 = Buffer.from(
		'FadedBeeFadedBeeFadedBeeFadedBeeFadedBeeFadedBeeFadedBeeFadedBee',
		'hex',
	)
})
