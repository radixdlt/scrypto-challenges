# Order book
This is a Serum Dex inspired Order Book.
Trade are done on a market.
A market allow tp transfer assert between a quote and a base. Quote and Base can be any token. They are declared during the market creation using the `instantiate_market` method.

## Trading using order
Trader push Bid(Buy base using quote) or Ask(sell base to quote) order to the Dex.
Pushed order depending on their type are matched for all or part of its amount depending on the opposite found side order.
For bid order if a ask price with less than the limite price is already present in the orderbook, it's matched and fund are tranfered.
For ask order if a bid price is greater or equals to the limite price is found, it's matched.
Bid order define a maximum limite price and Ask order a minimum price to match the order.

## Trader Vault
I push order, a trader must create a trader order vault with the `create_openorders` method.
This methode create a quote and base vault to store asset tranfered between order.
To get back the asset, the trader can use the `withdraw` method

## Resources and Data
```rust
struct GumballMachine {
  gumballs: Vault,
  collected_xrd: Vault,
  price: Decimal
}
```
Our gumball machine will hold two kinds of resources in vaults: the gumballs to be dispensed, and any XRD which has been collected as payment.

We'll also need to maintain the price, which we're using `Decimal` for.  `Decimal` is a bounded type appropriate for use for resource quantities.  In Scrypto, it has a fixed precision of 10<sup>-18</sup>, and a maximum value of 2<sup>96</sup>.  Unless we're selling spectacularly expensive gumballs, this should be fine.  If we wanted an unbounded type to use for quantity, we could use `BigDecimal` instead.

## Getting Ready for Instantiation
In order to instantiate a new gumball machine, the only input we need from the caller is to set the price of each gumball.  After creation, we'll be returning the address of our new component, so we'll set our function signature up appropriately:

```rust
pub fn instantiate_gumball_machine(price: Decimal) -> Component {
```

Within the `instantiate_gumball_machine` function, the first thing we need to do is create a new supply of gumballs which we intend to populate our new component with:

```rust
let bucket_of_gumballs = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
  .metadata("name", "Gumball")
  .metadata("symbol", "GUM")
  .metadata("description", "A delicious gumball")
  .initial_supply_fungible(100);
```

All that's left is to populate our `GumballMachine` struct with our supply of gumballs, the user-specified price, and an empty Vault which we will force to contain XRD.  Then we'll instantiate it, which returns the address, and we'll return that to the caller.

```rust
Self {
  gumballs: Vault::with_bucket(bucket_of_gumballs),
  collected_xrd: Vault::new(RADIX_TOKEN),
  price: price
}
.instantiate()
```

## Allowing Callers to Buy Gumballs
In order to sell a gumball, we just need the caller to pass us in enough XRD to cover the price.  We'll return the purchased gumball, as well as giving back their change if they overpaid, so we actually need to return _two_ buckets.  This is easily accomplished by simply returning a tuple, giving us a method signature like this:

```rust
pub fn buy_gumball(&mut self, payment: Bucket) -> (Bucket, Bucket) {
```

Note that we used `&mut self` because our reference to ourself must be mutable; we will be changing the contents of our vaults, if all goes well.

Accomplishing the actual mechanics of putting the XRD in our vault, taking a gumball out, and then returning the gumball as well as whatever is left in the caller's input bucket are trivial:

```rust
let our_share = payment.take(self.price);
self.collected_xrd.put(our_share);
(self.gumballs.take(1), payment)
```

Note that we didn't have to check that the input bucket contained XRD...when we attempt to put tokens into our `collected_xrd` Vault (which was initialized to contain XRD), we'll get a runtime error if we try to put in anything else.

Similarly, we'll get a runtime error if we try to take out a quantity matching the price, and find that there is insufficient quantity present.

Finally, if the user provided exactly the correct amount of XRD as input, when we return their `payment` bucket, it will simply contain quantity 0.

## Closing Thoughts
In order to keep this as straightforward as possible in teaching a few basic concepts, we haven't made the best gumball machine that we could.

First, we have provided no way to actually retrieve the collected XRD!  This would most appropriately be done by adding a withdrawal method, protected by an admin badge which ensures that only appropriate parties are able to perform the withdrawal.

Second, there's no reason to force payment in XRD only...we could easily have changed the `instantiate_gumball_machine` function to accept the `ResourceDef` of the currency that we intend to collect in exchange for gumballs.

Third, we could allow for greater flexibility in the gumballs themselves.  We could let the instantiator define the symbol, name, and quantity...or even directly pass in the gumballs to be sold rather than creating a new resource at instantiation time.