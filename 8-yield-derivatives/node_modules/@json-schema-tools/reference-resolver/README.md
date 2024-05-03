# JSON Schema Reference Resolver

<center>
  <span>
    <img alt="CircleCI branch" src="https://img.shields.io/circleci/project/github/json-schema-tools/reference-resolver/master.svg">
    <img src="https://codecov.io/gh/json-schema-tools/reference-resolver/branch/master/graph/badge.svg" />
    <img alt="npm" src="https://img.shields.io/npm/dt/@json-schema-tools/reference-resolver.svg" />
    <img alt="GitHub release" src="https://img.shields.io/github/release/json-schema-tools/reference-resolver.svg" />
    <img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/json-schema-tools/reference-resolver/latest.svg" />
  </span>
</center>

Takes a $ref string and a root object, and returns the referenced value.

Works in browser & in node (file system refs ignored in browser).

Easily add support for your own protocols.

## Getting Started

`npm install @json-schema-tools/reference-resolver`

```typescript
import refRes from "@json-schema-tools/reference-resolver";

refRes.resolve("#/properties/foo", { properties: { foo: true } }); // returns true
refRes.resolve("https://foo.com/"); // returns what ever json foo.com returns
refRef.resolve("../my-object.json"); // you get teh idea
```

## Adding custom protocol handlers

```typescript
import referenceResolver from "@json-schema-tools/reference-resolver";
import JSONSchema from "@json-schema-tools/meta-schema";

referenceResolver.protocolHandlerMap.ipfs = (uri) => {
   const pretendItsFetchedFromIpfs = {
     title: "foo",
     type: "string",
   } as JSONSchema;
   return Promise.resolve(fetchedFromIpfs);
};

referenceResolver.protocolHandlerMap["customprotocol"] = (uri) => {
   return Promise.resolve({
     type: "string",
     title: uri.replace("customprotocol://", ""),
   });
};

referenceResolver.resolve("ipfs://80088008800880088008");
referenceResolver.resolve("customprotocol://foobar");
```


### Contributing

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.
