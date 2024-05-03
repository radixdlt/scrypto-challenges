# open-rpc-mock-server

<center>
  <span>
    <img alt="CircleCI branch" src="https://img.shields.io/circleci/project/github/open-rpc/mock-server/master.svg">
    <img src="https://codecov.io/gh/open-rpc/mock-server/branch/master/graph/badge.svg" />
    <img alt="Dependabot status" src="https://api.dependabot.com/badges/status?host=github&repo=open-rpc/mock-server" />
    <img alt="npm" src="https://img.shields.io/npm/dt/@open-rpc/mock-server.svg" />
    <img alt="GitHub release" src="https://img.shields.io/github/release/open-rpc/mock-server.svg" />
    <img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/open-rpc/mock-server/latest.svg" />
  </span>
</center>

Provides a mock JSON-RPC API with [Service Discovery](https://github.com/open-rpc/spec#service-discovery-method) for a given [OpenRPC document](https://github.com/open-rpc/spec#openrpc-document).

Need help or have a question? Join us on [Discord](https://discord.gg/gREUKuF)!

## Install

Installing the _open-rpc-mock-server_ in your local project.

```bash
npm install --save @open-rpc/mock-server
```

or install it globally

```bash
npm install -g @open-rpc/mock-server
```

## Usage

If you installed it globally:
```bash
open-rpc-mock-server -d my-open-rpc-document.json
```

Optimize usage by adding script for _open-rpc-mock-server_ in `package.json`.

```json
"scripts": {
    "mock-server": "open-rpc-mock-server"
  },
```

The _mock-server_ will look for an `openrpc.json` document in the working directory that contains a valid OpenRPC based API. Otherwise the _mock-server_ will return an error message.

Run _mock-server_

```bash
npm run mock-server
```

The _mock-server_ will run at  http://localhost:3333/.

### Sending requests

With the _mock-server_ running at `http://localhost:3333/`, use Postman to send requests against the API.

## Example 

- [Using OpenRPC Mock Server to test against an Ethereum JSON-RPC API](https://medium.com/etclabscore/using-openrpc-mock-server-to-test-against-an-ethereum-json-rpc-api-50b86b6d02d6) - Jun 11, 2019 - ETC Labs Core

## Contributing

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.

