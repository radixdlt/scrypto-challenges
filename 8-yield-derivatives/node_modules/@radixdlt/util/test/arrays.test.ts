import { isNumberArray } from '../src/arrays'

describe('arrays', () => {
	it('should be able to type check unknown against number array', () => {
		expect(isNumberArray([1, 2, 3])).toBe(true)
		expect(isNumberArray(['foo', 'bar'])).toBe(false)
		expect(isNumberArray([1, 'bar'])).toBe(false)
		expect(isNumberArray('just a string')).toBe(false)
	})
})
