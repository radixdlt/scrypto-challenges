# JSON Schema Meta Schema

This repo contains the json schema meta schema and code to package it on npm, generate typings, etc.

## Installing

### Typescript

`npm install --save @json-schema-tools/meta-schema`

### Golang

`go get github.com/json-schema-tools/meta-schema`


### Rust

`cargo install json_schema`

## Using

### Typescript
```typescript
import JSONSchema, { JSONSchemaObject, Properties, Items } from "@json-schema-tools/meta-schema"
```

### Rust

#### From a string
```rust
let foo = r#"{
    "title": "helloworld",
    "type": "string"
}"#;

let as_json_schema: JSONSchemaObject = serde_json::from_str(foo).unwrap();
```

#### Using builder pattern
```rust
let schema = JSONSchemaObjectBuilder::default()
    .title("foobar".to_string())
    ._type(Type::SimpleTypes(SimpleTypes::String))
    .build()
    .unwrap();

let as_str = serde_json::to_string(&schema).unwrap();
```

### Contributing

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.
