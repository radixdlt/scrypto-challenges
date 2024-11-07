# Table of Contents
- [Overview](#overview)
- [Scrypto Package Overview](#scrypto-package-overview)
    - [YieldAMM Blueprint](#yieldamm-blueprint)
    - [State](#state)
- [Interface](#interface)
    - [set_initial_ln_implied_rate](#set_initial_ln_implied_rate)
    - [get_market_implied_Rate](#get_market_implied_rate)
    - [get_vault_reserves](#get_vault_reserves)
    - [add_liquidity](#add_liquidity)
    - [remove_liquidity](#remove_liquidity)
    - [swap_exact_pt_for_lsu](#swap_exact_pt_for_lsu)
    - [swap_exact_lsu_for_pt](#swap_exact_lsu_for_pt)
    - [swap_exact_lsu_for_yt](#swap_exact_lsu_for_yt)
    - [swap_exact_yt_for_lsu](#swap_exact_yt_for_lsu)
    - [get_exchange_rate](#get_exchange_rate)
    - [flash_loan](#flash_loan)
    - [flash_loan_repay](#flash_loan_repay)
    - [get_ln_implied_rate](#get_ln_implied_rate)
    - [time_to_expiry](#time_to_expiry)
    - [check_maturity](#check_maturity)

## Overview
The boilerplate blueprint below attempts to provide a basic implementation of [Pendle Finance](https://www.pendle.finance/) AMM to trade yield derivatives based on their [whitepaper](https://github.com/pendle-finance/pendle-v2-resources/blob/main/whitepapers/V2_AMM.pdf). The implementation includes a pricing model that simulates a logit curve for tokenized yield bearing assets which have a maturity date.

This means that the price of an asset is based on these five factors:

1. The size of the trade
2. The time to maturity
3. The scalar rate (steepness of the curve)
4. The anchor rate (where interest rate/exchange rate is anchored)
5. The fee rate

This blueprint provides a single liquidity pool for the LSU/PT-LSU pair. Pendle's V2 AMM is designed to efficiently trade these yield derivatives as it narrow's the PT's price range as the pool approach maturity. In other words, the exchange rate between LSU and PT-LSU is tied closer as the pool approaches maturity.

While the single liquidity pool supports LSU/PT-LSU pair, the YT is tradable anytime using flashswaps using the same pool.

## Scrypto Package Overview
This Scrypto package contains a single `YieldAMM` blueprint and a `liquidity_curve.rs` crate which describes the logic for the DEX and AMM to trade yield derivatives. Additionally, it also utilizes the [Native Pool Blueprint](https://docs.radixdlt.com/docs/pool-component) to manage pool logic.

### YieldAMM Blueprint
The `YieldAMM` blueprint contains the logic for the DEX that exchanges between the Principal Tokens and the LSU asset. It implements Pendle Finance's AMM V2 curve which prices the trade between the two assets. As a basic implementation, it only allows for a single liquidity pool supporting only one market of PT and LSU. 

### State

The `YieldAMM` blueprint defines 7 state in its `Struct` to allow the component to record information. These states are:

```rust
struct YieldAMM {
    pool_component: Global<TwoResourcePool>,
    flash_loan_rm: ResourceManager,
    expiry_date: UtcDateTime,
    scalar_root: Decimal,
    fee_rate: PreciseDecimal,
    reserve_fee_percent: Decimal,
    last_ln_implied_rate: PreciseDecimal,
}
```

| Field | Type  | Description |
| ----- | ----- | ----------- |
| `pool_component` | `Global<TwoResourcePool>` |  The `pt_rm` is a field that contains the `ResourceManager` for PT. It is used to mint and burn PTs and verify incoming PTs to the `YieldTokenizer` component.
| `flash_loan_rm` | `ResourceManager` | The `flash_loan_rm` is a field that contains the `ResourceManager` for the flash loan receipt. It is a receipt minted when taking out flash loans.
| `maturity_date` | `UtcDateTime` | The `requested_resource_vault` is a field that will contain the resource offered by the other party. When the other party sends the resource requested by the instantiatior, the resource will be contained in the `Vault` value.
| `scalar_root` | `Decimal` | The initial scalar value used to determine the steepness of the curve.
| `fee_rate` | `PreciseDecimal` | The fee rate charged on each trade.
| `reserve_fee_percent` | `Decimal` | The asset reserve fee charged on each trade.
| `last_ln_implied_rate` | `PreciseDecimal` | The exchange rate of the last trade.


## Interface

### set_initial_ln_implied_rate
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `set_initial_ln_implied_rate` | Method | `initial_rate_anchor` | `PreciseDecimal`| N/A | A method to set the initial peg of where exchange rates/interest rates will be trading around. This method is called after the first supply of liquidity is deposited to the pool. 

```rust
pub fn set_initial_ln_implied_rate(
    &mut self, 
    initial_rate_anchor: PreciseDecimal
) {
    // Set initial implied rate logic
}
```

### get_market_implied_rate
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `get_market_implied_rate` | Method | N/A | N/A | `PreciseDecimal` of the current market implied rate | A method to retrieve the current market implied rate.

```rust
pub fn get_market_implied_rate(&mut self) -> PreciseDecimal {
    self.last_ln_implied_rate.exp().unwrap()
}
```

### get_vault_reserves
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `get_vault_reserves` | Method | N/A | N/A | An `IndexMap<ResourceAddress, Decimal>` of the pool resource pair and its reserves. | A method to retrieve the current pool resource pair and its reserves.

```rust
pub fn get_vault_reserves(&self) -> IndexMap<ResourceAddress, Decimal> {
    self.pool_component.get_vault_amounts()
}
```


### add_liquidity
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `add_liquidity` | Method | `lsu_token`<br>`principal_token` | `FungibleBucket`<br>`FungibleBucket`| A `Bucket` of `pool_units`.<br>An `Option<Bucket>` of any unneeded assets. | A method that deposits the given PT and LSU token to the liquidity pool.

```rust
pub fn add_liquidity(
    &mut self, 
    lsu_token: FungibleBucket, 
    principal_token: FungibleBucket
) -> (Bucket, Option<Bucket>) {
    // Add liquidity logic
}
```

### remove_liquidity
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `remove_liquidity` | Method | `pool_units` | `FungibleBucket` | A `Bucket` of principal token.<br>A `Bucket` of LSU token. | A method that redeems the `pool_units` for the underlying pool resources.

```rust
pub fn remove_liquidity(
    &mut self, 
    pool_units: FungibleBucket
) -> (Bucket, Bucket) {
    // Remove liquidity logic
}
```

### swap_exact_pt_for_lsu
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `swap_exact_pt_for_lsu` | Method | `principal_token` | `FungibleBucket` | A `FungibleBucket` of LSU token. | A method that swaps the given PT for LSU tokens.

```rust
pub fn swap_exact_pt_for_lsu(
    &mut self, 
    principal_token: FungibleBucket
) -> FungibleBucket {
    // Swap PT ---> LSU logic.
}
```

### swap_exact_lsu_for_pt
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `swap_exact_lsu_for_pt` | Method | `lsu_token`<br>`desired_pt_amount` | `FungibleBucket`<br>`Decimal` | A `FungibleBucket` of principal token.<br>A `FungibleBucket` of any extra unneeded LSU token. | A method that swaps the given LSU tokens for the desired amount of PT.

```rust
pub fn swap_exact_lsu_for_pt(
    &mut self, 
    mut lsu_token: FungibleBucket, 
    desired_pt_amount: Decimal
) -> (FungibleBucket, FungibleBucket) {
    // Swap LSU ---> PT logic.
}
```

### swap_exact_lsu_for_yt
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `swap_exact_lsu_for_yt` | Method | `lsu_token` | `FungibleBucket` | A `FungibleBucket` of yield token. | A method that swaps the given LSU tokens for YT.

```rust
pub fn swap_exact_lsu_for_yt(
    &mut self, 
    mut lsu_token: FungibleBucket
) -> FungibleBucket {
    // Swap LSU ---> YT logic.
}
```

### swap_exact_yt_for_lsu
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `swap_exact_yt_for_lsu` | Method | `yield_token` | `FungibleBucket` | A `FungibleBucket` of LSU token. | A method that swaps the given yield token for LSU token.

```rust
pub fn swap_exact_yt_for_lsu(
    &mut self, 
    yield_token: FungibleBucket,
) -> (FungibleBucket, FungibleBucket) {
    // Swap YT ---> LSU logic.
}
```

### calc_trade
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `calc_trade` | Method | `net_pt_amount`<br>`time_to_expiry` | `Decimal`<br>`i64` | A `Decimal` amount of the asset given/taken from the account. | A method that calculates the trade based on the direction of the trade, size of the trade, current time to maturity, and exchange rate.

```rust
pub fn calc_trade(
    &mut self,
    net_pt_amount: Decimal,
    time_to_expiry: i64
) -> (Decimal, Decimal) {
    // Trade calculation logic.
}
```

### get_exchange_rate
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `get_exchange_rate` | Method | `net_pt_amount`<br>`time_to_expiry`<br>`optional_initial_rate_anchor` | `Decimal`<br>`i64`<br>`Option<PreciseDecimal>` | A `PreciseDecimal` of the exchange rate. amount of the asset given/taken from the account. | A method that retrieves the exchange rate based on calculation of the size of the trade, rate scalar, and rate anchor.

```rust
fn get_exchange_rate(
    &mut self, 
    net_pt_amount: Decimal, 
    time_to_expiry: i64, 
    optional_initial_rate_anchor: Option<PreciseDecimal>
) -> PreciseDecimal {
    // Get exchange rate logic
}
```

### flash_loan
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `flash_loan` | Method | `resource`<br>`amount` | `ResourceAddress`<br>`Decimal` | A `FungibleBucket` of the flash loan.<br>A `NonFungibleBucket` of the transient flash loan receipt. | A method that allows one to borrow from the pool without collateral so long as the loan is repaid within the same transaction.

```rust
pub fn flash_loan(
    &mut self, 
    resource: ResourceAddress, 
    amount: Decimal
) -> (FungibleBucket, NonFungibleBucket) {
    // Flash loan logic
}
```

### flash_loan_repay
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `flash_loan_repay` | Method | `flash_loan`<br>`flash_loan_receipt` | `FungibleBucket`<br>`NonFungibleBucket` | A `Option<FungibleBucket>` of any remainder from loan payment. | A method to repay the flash loan.

```rust
pub fn flash_loan_repay(
    &mut self, 
    mut flash_loan: FungibleBucket, 
    flash_loan_receipt: NonFungibleBucket
) -> Option<FungibleBucket> {
    // Flash loan repay logic
}
```

### get_ln_implied_rate
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `flash_loan_repay` | Method | `flash_loan`<br>`flash_loan_receipt` | `FungibleBucket`<br>`NonFungibleBucket` | A `Option<FungibleBucket>` of any remainder from loan payment. | A method to repay the flash loan.

```rust
fn get_ln_implied_rate(
    &mut self, 
    time_to_expiry: i64, 
    optional_initial_rate_anchor: Option<PreciseDecimal>
) -> PreciseDecimal {
    // Retrieve ln implied rate logic
}
```

### time_to_expiry
| Name            | Type            | Arguments       | Type | Returns | Description  
| --------------- | --------------- | ----------------- | --------------- | --------------- | --------------- |
| `time_to_expiry` | Method | N/A | N/A | A `i64` of the time left to maturity. | A method to retrieve the amount of seconds left to maturity.

```rust
pub fn time_to_expiry(&self) -> i64 {
    // Time to expiry logic
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
