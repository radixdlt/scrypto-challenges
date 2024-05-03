# json-pointer [![Documentation][typedoc-shield]][typedocs]

This package provides a TypeScript and JavaScript implementation of [RFC6901
JavaScript Object Notation (JSON) Pointer][rfc6901].

[typedoc-shield]: https://img.shields.io/badge/typedoc-reference-blue.svg
[typedocs]: https://json-schema-spec.github.io/json-pointer-typescript/
[rfc6901]: https://tools.ietf.org/html/rfc6901

## Usage

Construct a JSON Pointer using `Ptr.parse`, and then evaluate the pointer using
`.eval()`. You can convert back to a string using `.toString()`.

```ts
import Ptr from "@json-schema-spec/json-pointer"

const data = {
  path: {
    to: {
      thing: [
        "foo",
        "bar",
        "baz",
      ],
    },
  },
};

const pointer = Ptr.parse("/path/to/thing/2");
console.log(pointer.eval(data)); // Output: baz
console.log(pointer.toString()) // Output: /path/to/thing/2
```

If a JSON Pointer points to a non-existent property of its input, then `.eval()`
will throw `EvalError`:

```ts
import Ptr, { EvalError } from "@json-schema-spec/json-pointer"

const data = {
  path: {
    to: {
      thing: [
        "foo",
        "bar",
        "baz",
      ],
    },
  },
};

const pointer = Ptr.parse("/path/to/thing/4");
try {
  pointer.eval(data);
} catch (err) {
  if (err instanceof EvalError) {
    console.log(err.instance); // Output: ["foo", "bar", "baz"]
    console.log(err.token); // Output: 4
  }
}
```

If you're interested in parsing JSON Pointers that might be invalid, you can
catch these errors by looking for `InvalidPtrError`:

```ts
import Ptr, { InvalidPtrError } from "@json-schema-spec/json-pointer"

try {
  Ptr.parse(" invalid json pointer")
} catch (err) {
  if (err instanceof InvalidPtrError) {
    console.error(err.ptr) // Output:  invalid json pointer
  }
}
```
