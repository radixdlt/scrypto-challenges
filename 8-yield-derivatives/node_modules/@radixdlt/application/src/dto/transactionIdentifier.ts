import { err, ok, Result } from 'neverthrow'
import { TransactionIdentifierT } from './_types'

const isTransactionIdentifier = (
	something: unknown,
): something is TransactionIdentifierT => {
	const inspection = something as TransactionIdentifierT
	return inspection.__hex !== undefined && inspection.__witness === 'isTXId'
}

const create = (
	bytes: Buffer | string,
): Result<TransactionIdentifierT, Error> => {
	const buffer = typeof bytes === 'string' ? Buffer.from(bytes, 'hex') : bytes
	const length = 32
	if (buffer.length !== length) {
		return err(
			new Error(`Expected #${length} bytes, but got #${buffer.length}`),
		)
	}

	const asString = buffer.toString('hex')

	return ok({
		__witness: 'isTXId',
		__hex: asString,
		toString: () => asString,
		equals: (other: TransactionIdentifierT) =>
			other.toString() === asString,
	})
}

export const TransactionIdentifier = {
	is: isTransactionIdentifier,
	create,
}
