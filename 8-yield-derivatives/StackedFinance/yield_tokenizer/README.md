# Table of Contents
- [Overview](#overview)
- [Scrypto Package Overview](#scrypto-package-overview)
    - [YieldTokenizer Blueprint](#yieldtokenizer-blueprint)
    - [State](#state)
- [Interface](#interface)
    - [instantiate_yield_tokenizer](#instantiate_yield_tokenizer)
    - [retrieve_validator_component](#retrieve_validator_component)
    - [tokenize_yield](#tokenize_yield)
    - [redeem](#redeem)
    - [reedem_from_pt](#redeem_from_pt)
    - [claim_yield](#claim_yield)
    - [calc_yield_owed](#calc_yield_owed)
    - [calc_required_lsu_for_yield_owed](#calc_required_lsu_for_yield_owed)
    - [pt_address](#pt_address)
    - [yt_address](#yt_address)
    - [underlying_resource](#underlying_resource)
    - [maturity_date](#maturity_date)
    - [check_maturity](#maturity_date)

## Overview
The boilerplate blueprint below is a basic implementation of what "yield tokenization" could look like. Yield tokenization is the act of taking a yield bearing asset, such as a Liquid Staking Unit (LSU), to split it into two parts:

1. Its Principal Token (PT) - The rights to the principal of the asset.
2. Its Yield Token (YT) - The rights to the yield of the asset.

For example, if a 100 LSU which has an 8% APY from staking rewards were to be tokenized it would be split into:

1. 100 PT-LSU - The right to the 100 LSU.
2. 100 YT-LSU - The right to 8% APY from 100 LSU.

This essentially creates a derivative of the yield bearing asset. Splitting an asset into its two parts locks the underlying asset until a maturity date has lapsed. This maturity date is important for the yield derivative trading DEX as it creates a window of time where yield can be speculated and aligns incentives with market participants.

Redeeming the underlying asset can be done under these conidtions:
* Posessing both the PT-Asset and YT-Asset of equal quantity. 
* Posessing the PT-Asset at maturity date.

While PT & YT assets alone can't be redeemed for the underlying asset before the maturity date, they can be traded in the market for its underlying asset.

## Scrypto Package Overview
This Scrypto package contains a single `YieldTokenizer` blueprint which describes the logic for tokenizing a yield bearing assets into its compartmentalized parts. 

### YieldTokenizer Blueprint
The `YieldTokenizer` The blueprint instantiates a component which expects the expiry date and underlying asset to be passed. As a basic implementation, only one LSU of its kind can be accepted as the component will validate the `ResourceAddress` to be an LSU.

Instantiating the `YieldTokenizer` blueprint will also create 2 resources:

* `pt_rm` - The PT `ResourceManager` which is responsible for minting/burning fungible PTs.
* `yt_rm` - The YT `ResourceManager` which is responsible for minting non fungible YTs.

### State

The `YieldTokenizer` blueprint defines 6 state in its `Struct` to allow the component to record information. These states are:

```rust
struct YieldTokenizer {
    pt_rm: ResourceManager,
    yt_rm: ResourceManager,
    maturity_date: UtcDateTime,
    lsu_validator_component: Global<Validator>,
    lsu_address: ResourceAddress,
    lsu_vault: FungibleVault,
}
```

| Field | Type  | Description |
| ----- | ----- | ----------- |
| `pt_rm` | `ResourceManager` |  The `pt_rm` is a field that contains the `ResourceManager` for PT. It is used to mint and burn PTs and verify incoming PTs to the `YieldTokenizer` component.
| `yt_rm` | `ResourceManager` | The `yt_rm` is a field that contains the `ResourceManager` for PT. It is used to mint YT and verify incoming YTs to the `YieldTokenizer` component.
| `maturity_date` | `UtcDateTime` | The `requested_resource_vault` is a field that will contain the resource offered by the other party. When the other party sends the resource requested by the instantiatior, the resource will be contained in the `Vault` value.
| `lsu_validator_component` | `Global<Validator>` | The `lsu_validator_component` is a field that will allow the component to call on the Native Validator component of the LSU to calculate redemption value. 
| `lsu_address` | `ResourceAddress` | The `lsu_address` is a field that will allow the component to verify that any LSU's the component receives is the correct LSU. Also, it allows to broadcast to any component using the `YieldTokenizer` the supported LSU.
| `lsu_vault` | `FungibleVault` | The `lsu_vault` is a field where incoming LSUs to be tokenized will be deposited to and where LSU for redemption are taken out of.


## Interface

### instantiate_yield_tokenizer
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `instantiate_yield_tokenizer` | Function | `expiry`<br>`accepted_lsu` | `Expiry`<br>`ResourceAddress`| A `Global<YieldTokenizer>` component type. | An instantiation function which instantiates the `YieldTokenizer` component. 

```rust
pub fn instantiate_yield_tokenizer(
    expiry: Expiry,
    accepted_lsu: ResourceAddress,
) -> Global<YieldTokenizer> {
    // Instantiation logic
}
```

### retrieve_validator_component
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `retrieve_validator_component` | Function | `lsu_address` | `ResourceAddress` | Returns a `Global<Validator` component type | A function for utility used to retrieve the `Global<Validator>` component of a given LSU.

```rust
fn retrieve_validator_component(
    lsu_address: ResourceAddress
) -> Global<Validator> {
    // Retrieve validator logic
}
```

### validate_lsu
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `validate_lsu` | Function | `inout_lsu_address` | `ResourceAddress` | A `bool` of whether the given LSU is in fact the native LSU. | A function for utility used to validate whether the `ResourceAddress` is in fact from a native LSU.

```rust
fn validate_lsu(
    input_lsu_address: ResourceAddress
    ) -> bool {
    // Validate LSU logic
}
```


### tokenize_yield
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `tokenize_yield` | Method | `lsu_token` | `FungibleBucket` | A `FungibleBucket` of PT.<br>A `NonFungibleBucket` of YT. | A method that tokenizes a yield bearing asset to its PT and YT.

```rust
pub fn tokenize_yield(
    &mut self, 
    lsu_token: FungibleBucket
) -> (FungibleBucket, NonFungibleBucket) {
    // Tokenize yield logic
}
```

### redeem
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `redeem` | Method | `pt_bucket`<br>`yt_bucket` | `FungibleBucket`<br>`NonFungibleBucket` | A `FungibleBucket` of the underlying LSU token. | A method that redeems the PT and YT for the underlying LSU.

```rust
pub fn redeem(
    &mut self, 
    pt_bucket: FungibleBucket, 
    yt_bucket: NonFungibleBucket, 
) -> FungibleBucket {
    // Redeem logic
}
```

### redeem_from_pt
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `redeem_from_pt` | Method | `pt_bucket` | `FungibleBucket` | A `FungibleBucket` of the underlying LSU token. | A method that redeems the PT for the underlying LSU if maturity date has passed.

```rust
pub fn redeem_from_pt(
    &mut self,
    pt_bucket: FungibleBucket,
) -> FungibleBucket {
    // Redeem from PT logic
}
```

### claim_yield
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `claim_yield` | Method | `yt_proof` | `NonFungibleProof` | A `Bucket` of Unstake NFT. | A method to claim any yield earned from the underlying LSU.

```rust
pub fn claim_yield(
    &mut self, 
    yt_proof: NonFungibleProof,
) -> Bucket {
    // Claim yield logic
}
```

### calc_yield_owed
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `calc_yield_owed` | Method | `data` | `&YieldTokenData` | A `Decimal` of yield token. | A method to calculate any yield earned from the `NonFungibleData` of YT.

```rust
fn calc_yield_owed(
    &self,
    data: &YieldTokenData,
) -> Decimal {
    // Calculate yield owed logic
}
```

### calc_required_lsu_for_yield_owed
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `calc_required_lsu_for_yield_owed` | Method | `yield_owed` | `Decimal` | A `Decimal` of LSU token. | A method that swaps the given yield token for LSU token.

```rust
fn calc_required_lsu_for_yield_owed(
    &self, 
    yield_owed: Decimal
) -> Decimal {
    // Calc required LSU for yield owed logic
}
```

### pt_address
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `pt_address` | Method | N/A | N/A | The `ResourceAddress` of PT. | A method to retrieve the PT `ResourceAddress`.

```rust
pub fn pt_address(&self) -> ResourceAddress {
    self.pt_rm.address()
}
```

### yt_address
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `yt_address` | Method | N/A | N/A | The `ResourceAddress` of YT. | A method to retrieve the YT `ResourceAddress`.

```rust
pub fn yt_address(&self) -> ResourceAddress {
    self.yt_rm.address()
}
```

### underlying_resource
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `underlying_resource` | Method | N/A | N/A | The `ResourceAddress` of the underlying LSU. | A method to retrieve the LSU `ResourceAddress`.

```rust
pub fn underlying_resource(&self) -> ResourceAddress {
    self.lsu_address
}
```

### maturity_date
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `maturity_date` | Method | N/A | N/A | A `UtcDateTime` of the maturity date. | A method to retrieve the maturity date.

```rust
pub fn maturity_date(&self) -> UtcDateTime {
    self.maturity_date
}
```

### check_maturity
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `check_maturity` | Method | N/A | N/A | A `bool` of whether the maturity has lapsed. | A method to check whether maturity has lapsed or not.

```rust
pub fn check_maturity(&self) -> bool {
    // Check maturity logic
}
```
