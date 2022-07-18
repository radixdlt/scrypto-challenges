# Lendi - Web Application

## Code Pattern

### Domain Driven Design (DDD)

We are utilizing a DDD style approach for the code pattern in this project.

DDD focuses on converting the business requirements of the application into models that can be used by multiple layers
of application code.

We have 3 layers: `Domain`, `Infra`, and `Service`.

#### Domain

The Domain layers holds models and use cases which represent real life concepts and actions.

A domain model aims to codify a real life object.

A domain use case aims to codify a use case involving a particular domain model.

#### Infra

The Infra layer allows us to use domain models to interact with the infrastructure layer.

A repository provides an abstraction for performing infrastrucutre operations in terms of domain models. For example, we
can store and retrieve `ContactList` models to disk using the `LocalFileContactListsRepository`.

#### Service

The Service layer contains the application code which interacts with the Infra & Domain layers. In our case, the Service
layer is where ElectronJS/VueJS lives.

### File Conventions

- filenames: kebab-case
- folder names: kebab-case
- test cases:
    - FILENAME.unit.ts
    - FILENAME.integration.ts

## Running the application
### Install the PTE browser extension
See the `Install PTE Browser Extension` at https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html

### Project setup
```
yarn install
```

#### Compiles and hot-reloads for development
```
yarn serve
```

#### Compiles and minifies for production
```
yarn build
```

#### Run your unit tests
```
yarn test:unit
```

#### Lints and fixes files
```
yarn lint
```

## Resources
- VueJS: https://vuejs.org/
- Vuetify: https://vuetifyjs.com/en/