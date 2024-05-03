// @ts-nocheck
/* eslint-disable */
export const objectEquals = <K extends symbol, V>(
	lhs: Readonly<{ [key in K]: V }>,
	rhs: Readonly<{ [key in K]: V }>,
): boolean => {
	if (Object.keys(lhs).length !== Object.keys(rhs).length) return false
	return (
		Object.keys(lhs).filter(
			key =>
				rhs[key] !== lhs[key] ||
				(rhs[key] === undefined && !(key in rhs)),
		).length === 0
	)
}

export const autoConvertMapToObject = map => {
	const obj = {}
	for (const item of [...map]) {
		const [key, value] = item
		obj[key] = value
	}
	return obj
}

export const mapEquals = <K, V>(lhs: Map<K, V>, rhs: Map<K, V>): boolean =>
	objectEquals(autoConvertMapToObject(lhs), autoConvertMapToObject(rhs))

/* eslint-enable */
