export const isNumberArray = (test: unknown): boolean =>
	Array.isArray(test) && test.every(value => typeof value === 'number')

export const arraysEqual = <T>(a: T[], b: T[]): boolean => {
	if (a === b) return true
	if (a == null || b == null) return false
	if (a.length !== b.length) return false

	for (let i = 0; i < a.length; ++i) {
		if (a[i] !== b[i]) return false
	}
	return true
}
