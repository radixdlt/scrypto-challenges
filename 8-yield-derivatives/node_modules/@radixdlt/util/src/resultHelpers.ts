import { err, Result } from 'neverthrow'
import { isObject, isResult } from './typeGuards'

const unwrap = (maybeResult: unknown, errors: (Error | Error[])[]): unknown => {
	if (isResult(maybeResult)) {
		if (maybeResult.isOk()) {
			const value = maybeResult.value
			if (isResult(value)) {
				return unwrap(value, errors)
			}
			return value
		} else {
			errors.push(maybeResult.error)
			return maybeResult
		}
	}
	return maybeResult
}

export const flattenResultsObject = (
	json: Result<unknown, Error | Error[]>,
): Result<unknown, Error[]> => {
	const errors: (Error | Error[])[] = []

	const flattened = json
		.map(value => {
			if (!isObject(value)) return value
			for (const item in value) {
				const objValue = value[item]

				if (objValue && isResult(objValue)) {
					const res = flattenResultsObject(objValue)
					value[item] = unwrap(res, errors)
				}
			}
			return value
		})
		.mapErr(err => {
			errors.push(err)
			return errors.flat()
		})

	return errors.length > 0 ? err(errors.flat()) : flattened
}
