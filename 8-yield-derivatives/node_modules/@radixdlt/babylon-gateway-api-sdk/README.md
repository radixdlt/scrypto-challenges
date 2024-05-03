# Gateway API SDK

This SDK is a thin wrapper around the [Babylon Gateway API](https://docs-babylon.radixdlt.com/main/apis/api-specification.html). It enables clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission. It is designed for use by wallets and explorers. For simple use cases, you can typically use the [Core API](https://github.com/radixdlt/babylon-node/tree/main/sdk/typescript) on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.

## License

The Gateway API SDK code is released under an [Apache 2.0 license](https://github.com/radixdlt/babylon-gateway/blob/main/sdk/typescript/LICENSE). The executable components are licensed under the [Radix Software EULA](http://www.radixdlt.com/terms/genericEULA).


## Structure

There are four sub APIs available in the Gateway API:

- Status - For status and configuration details for the Gateway / Network.
- Transaction - For transaction construction, preview, submission, and monitoring the status of an individual transaction.
- Stream - For reading committed transactions.
- State - For reading the current or past ledger state of the network.

All APIs generated from OpenAPI specification are available after initialization through `innerClient` object on corresponding sub-api. See examples in [Low Level APIs section](#low-level-apis). On top of that, this package provides [high-level counterparts](#high-level-apis) for each sub api.

## Versioning

All developers looking for an RCnet V3 compatible API should install versions which have `-rc.3.X` suffix. We'll be publishing internal versions with different suffixes which might not work well with publicly available network. All availble versions are published to [NPM repository](https://www.npmjs.com/package/@radixdlt/babylon-gateway-api-sdk?activeTab=versions)

## Initialization

Calling static `intialize` method from `GatewayApiClient` class will instantiate all low-level APIs together with their high-level equivalents and return them as a single object.

`applicationName` is required purely for statictis purposes and will be added as a request header. Additional fields you can set up are `applicationVersion` and `applicationDappDefinitionAddress`

```typescript
import { GatewayApiClient, RadixNetwork } from '@radixdlt/babylon-gateway-api-sdk'

const gatewayApi = GatewayApiClient.initialize({
  networkId: RadixNetwork.Mainnet,
  applicationName: 'Your dApp Name',
  applicationVersion: '1.0.0',
  applicationDappDefinitionAddress: 'account_rdx12y4l35lh2543nfa9pyyzvsh64ssu0dv6fq20gg8suslwmjvkylejgj'
})
const { status, transaction, stream, state } = gatewayApi
```

## High Level APIs

High Level APIs will grow over time as we start encountering repeating patterns when dealing with low level APIs. They are supposed to help with most commonly performed tasks and standardize ways of working with Gateway API and Typescript.

### State

- `getEntityDetailsVaultAggregated(entities: string | string[])` - detailed information about entities
- `getAllEntityMetadata(entity: string)` - get all metadata about given entity
- `getEntityMetadata(entity: string, cursor?: string)` - get paged metadata about given entity
- `getValidators(cursor?: string)` - get paged validators
- `getAllValidators()` - get all validators
- `getNonFungibleLocation(resource: string, ids: string[])` - get list of NFT location for given resource and ids
- `getNonFungibleIds(address:string, ledgerState, cursor?: string)` - get paged non fungible ids for given address
- `getAllNonFungibleIds(address: string)` - get all non fungible ids for given address
- `getNonFungibleData(address: string, ids: string | string[])` - get non fungible data

### Status

- `getCurrent()` - Gateway API version and current ledger state
- `getNetworkConfiguration()` - network identifier, network name and well-known network addresses

### Transaction

- `getStatus(txID: string)` - transaction status for given transaction id (the intent hash)
- `getCommittedDetails(txID: string, options)` - transaction details for given transaction id (the intent hash)

### Stream

- `getTransactionsList(affectedEntities?: string[], cursor?: string)` - get transaction list for given list of entities

### Statistics

- `getValidatorsUptimeFromTo(addresses: string[], from, to)` - get uptime statistics for validators for given period 
- `getValidatorsUptime(addresses: string[])` - get uptime statistics for validators

## Low Level APIs

Low level APIs are generated automatically based on OpenAPI spec. You can get a good sense of available methods by looking at [Swagger](https://mainnet.radixdlt.com/swagger/index.html). In order to access automatically generated methods you have two options:

### Using generated APIs through `innerClient`

When using automatically generated APIs please check method parameter. They're always wrapped object with property matching method name (like in the example below) 

```typescript
async function getTransactionStatus(transactionIntentHash: string) {
  let response = await gatewayApi.transaction.innerClient.transactionStatus({
    transactionStatusRequest: {
      intent_hash: transactionIntentHash,
    },
  })
  return response.status
}
console.log(
  await getTransactionStatus(
    'txid_tdx_21_18g0pfaxkprvz3c5tee8aydhujmm74yeul7v824fvaye2n7fvlzfqvpn2kz'
  )
)
```

### Using generated APIs manually

You can always opt-out of using aggregated gateway client and instantiate sub-apis manually

```typescript
import { Configuration, StateApi } from '@radixdlt/babylon-gateway-api-sdk'
const config = new Configuration({ basePath: 'https://mainnet.radixdlt.com' })
const stateApi = new StateApi(config)
const response = await stateApi.nonFungibleData({
  stateNonFungibleDataRequest: {
    resource_address: address,
    non_fungible_ids: [id],
  },
})
console.log(response.non_fungible_ids)
```

## Fetch polyfill

Behind the scenes, this library uses the fetch API. If in an environment where `fetch` is not available, a polyfill must be used (see eg [node-fetch](https://www.npmjs.com/package/node-fetch)). Pass your own `fetch` implementation to [`initialize` parameter](https://github.com/radixdlt/babylon-gateway/blob/develop/sdk/typescript/lib/generated/runtime.ts#L20).

Starting from NodeJS 16 `fetch` is available as experimental API. You can e.g. check Gateway API status by using following snippet.

```typescript
const { GatewayApiClient, RadixNetwork } = require('@radixdlt/babylon-gateway-api-sdk')

const gateway = GatewayApiClient.initialize({
  networkId: RadixNetwork.Mainnet
})
gateway.status.getCurrent().then(console.log)
```

Save this as `index.js` and use by calling `node --experimental-fetch index.js`. This will enable experimental fetch and let code work without additional configuration. In NodeJS 18.x environment fetch API is enabled by default. Users of previous node versions will need to provide their own `fetch` implementation through configuration object.
