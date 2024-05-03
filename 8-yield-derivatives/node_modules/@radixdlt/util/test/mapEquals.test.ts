import { mapEquals, objectEquals } from '../src/objectEquals'

describe('Map Equals', () => {
	it('can check for equality of non empty types', () => {
		const lhs = { foo: 1 }
		const rhs = { foo: 1 }

		expect(objectEquals(lhs, rhs)).toBe(true)
	})

	it('can check for equality of empty types', () => {
		const lhs = {}
		const rhs = {}

		expect(objectEquals(lhs, rhs)).toBe(true)
	})

	it('can check for inequality of non matching values', () => {
		const lhs = { foo: 1 }
		const rhs = { foo: 2 }

		expect(objectEquals(lhs, rhs)).toBe(false)
	})

	it('can check for inequality of overlapping values', () => {
		const lhs = { foo: 1 }
		const rhs = { foo: 1, buz: 2 }

		expect(objectEquals(lhs, rhs)).toBe(false)
	})

	it('can check for equality between maps', () => {
		const a = new Map<string, number>([
			['foo', 42],
			['bar', 1337],
		])

		const b = new Map<string, number>([
			['foo', 42],
			['bar', 1337],
		])

		expect(mapEquals(a, b)).toBe(true)
	})

	it('can check for inequality between maps', () => {
		const a = new Map<string, number>([
			['foo', 42],
			['bar', 1337],
		])

		const b = new Map<string, number>([
			['foo', 42],
			['bar', 1337],
			['buz', 237],
		])

		expect(mapEquals(a, b)).toBe(false)
	})
})
