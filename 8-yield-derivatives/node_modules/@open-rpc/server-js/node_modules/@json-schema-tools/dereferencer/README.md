# JSON Schema Dereferencer

<center>
  <span>
    <img alt="CircleCI branch" src="https://img.shields.io/circleci/project/github/json-schema-tools/dereferencer/master.svg">
    <img src="https://codecov.io/gh/json-schema-tools/dereferencer/branch/master/graph/badge.svg" />
    <img alt="Dependabot status" src="https://api.dependabot.com/badges/status?host=github&repo=json-schema-tools/dereferencer" />
    <img alt="npm" src="https://img.shields.io/npm/dt/@json-schema-tools/dereferencer.svg" />
    <img alt="GitHub release" src="https://img.shields.io/github/release/json-schema-tools/dereferencer.svg" />
    <img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/json-schema-tools/dereferencer/latest.svg" />
  </span>
</center>

Otherwise known as a ref parser, this tool will replace json schema using $ref with the underlying reference, returning relevant errors otherwise.

Built using @json-schema-tools/traverse

## features

- minimal dependencies
- simple & fast
- cycle detection/handling
- switchable recusive dereferencing
- works in node & in browser (isomorphic)
- handles:
  - relative pointer refs
  - http/https uris
  - local filesystem references
- complete disrespect for $id
- configurable
 - optionally de-reference internal references only, keeping it synchronous
 - ignore refs that match a set of patterns
- extensible
  - dependency injectable fetch and filesystem
  - middleware allows you to easily implement new $ref values.
  - easily add behaviors for custom reference locations

## Getting Started

`npm install @json-schema-tools/dereferencer`

```typescript
const JsonSchemaDereferencer = require("@json-schema-tools/dereferencer").default;

const mySchema = {
    type: "object",
    properties: {
      foo: { anyOf: [
        { $ref: "#/properties/bar" },
        { type: "string" }
      ]},
      bar: { $ref: "#/properties/foo" },
      baz: { $ref: "../myschemas/baz.json" },
      jsonSchemaMetaSchema: { $ref: "https://meta.json-schema.tools" }
    },
    additionalProperties: {
        type: "array",
        items: [
            { type: "array", items: { $ref: "#" } },
            { type: "boolean" }
        ]
    }
};

const dereferencer = new JsonSchemaDereferencer(mySchema);

console.log(dereferencer.resolveSync());
console.log(await dereferencer.resolve());
```


### Add custom protocol handling

```typescript
import JsonSchemaDereferencer from "@json-schema-tools/dereferencer";

const mySchema = {
    type: "object",
    properties: {
      foo: { $ref: "ipfs://39420398420384" }
    }
};

const dereferencer = new JsonSchemaDereferencer(mySchema, {
  protocolHandlerMap: {
    "ipfs": (ref) => Promise.resolve({ type: "string", title: "pretend we got this from ipfs" })
});

console.log(dereferencer.resolveSync());
console.log(await dereferencer.resolve());
```


### Contributing

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.
