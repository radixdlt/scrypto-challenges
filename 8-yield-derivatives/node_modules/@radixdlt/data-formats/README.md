# `@radixdlt/data-formats`

## Usage
### JSON Decoding


#### Examples

Without dependencies, using provided taggedStringDecoder:

```typescript
import { JSONDecoding, taggedStringDecoder } from '@radixdlt/data-formats'

const strTagDecoder = taggedStringDecoder(':str:')((value) => ok(value))

const { fromJSON } = JSONDecoding.withDecoders(strTagDecoder).create()

fromJSON(':str:xyz') // Ok('xyz')
```

An object with dependencies:

```typescript
import { JSONDecoding, taggedStringDecoder } from '@radixdlt/data-formats'
import { ok } from 'neverthrow'

const strTagDecoder = taggedStringDecoder(':str:')((value) => ok(value))

const Object1 = {
    ...JSONDecoding.withDecoders(strTagDecoder).create()
}

const tstTagDecoder = taggedStringDecoder(':tst:')((value) => ok(value))

const { fromJSON } = JSONDecoding
	.withDependencies(Object1)
	.withDecoders(testTagDecoder)
	.create()

fromJSON({
    a: ':str:foo',
    b: ':tst:bar'
}) // ok({ a: 'foo', b: 'bar' })
```

JSON decoding takes an object and applies `decoder`s to each key-value pair. `taggedObjectDecoder` and `taggedStringDecoder` are provided, but you can easily define a new decoder. Here is how `taggedStringDecoder` is defined:

```typescript
import { decoder } from '@radixdlt/data-formats'

export const taggedStringDecoder = (tag: string) => <T>(
	algorithm: (value: string) => Result<T, Error>,
): Decoder =>
	decoder<T>((value) =>
		isString(value) && `:${value.split(':')[1]}:` === tag
			? algorithm(value.slice(tag.length))
			: undefined,
	)
```

A `decoder` should supply a function that defines how the decoding should be applied. First it should do some validation logic (does this decoder apply to this value?), in this case checking if the value is a string and if has a matching tag. Then, apply some `algorithm` function, which is the actual decoding (create an instance of some object). If the validation fails, the decoder has to return `undefined`.



