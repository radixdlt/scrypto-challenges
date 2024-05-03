import { ok, combine } from 'neverthrow'

describe('result', () => {
	it('can use variadic generic combine', () => {
		const foo = ok('Foo')
		const bar = ok(1337)
		const buz = ok(false)
		const biz = ok(3.1415)

		const combined = combine([foo, bar, buz, biz])

		const result = combined.map(resultList => ({
			foo: resultList[0],
			bar: resultList[1],
			buz: resultList[2],
			pi: resultList[3],
		}))

		const expected = {
			foo: 'Foo',
			bar: 1337,
			buz: false,
			pi: 3.1415,
		}

		expect(result._unsafeUnwrap()).toStrictEqual(expected)
	})
})
