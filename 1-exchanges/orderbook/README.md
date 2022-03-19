# Order book
This is a Serum Dex inspired Order Book.
Trade are done on a market.
A market allow the transfer of assert between a quote and a base. Quote and Base can be any token. They are declared during the market creation using the `instantiate_market` method.

## Trading using order
Trader push Bid(Buy base using quote) or Ask(sell base to quote) order to the Dex.
Pushed order depending on their type are matched for all or part of its amount depending on the opposite side order found.
For bid order if a ask price with less than the limit price is already present in the orderbook, it's matched and fund are transferred.
For ask order if a bid price is greater or equals to the limit price is found, it's matched.
Bid order define a maximum limit price and Ask order a minimum price to match the order.

## Trader Vault
To push order, a trader must create a trader order vault with the `create_openorders` method.
This method create a quote and base vault to store asset transferred between order.
To get back the asset, the trader can use the `withdraw` method.

## Push order
To push a bid order, use the method `bid_order` with the max limit price, amount to transfer, the order type, add quote asset to use for the transfer.
The amount of quote asset must be equals or more that the limit price * with the amount to be sure that there is enough quote for the transfer.
The provided quote are locked and can't be withdrew while the order is pending.
To push a ask order, use `ask_order` with the min acceptable price, amount to transfer, the order type, add base asset to use for the transfer.

With the transfer method, the badge created with `create_openorders` must be provided to identify the orders owner.

After the order is pushed, the call return the order id that can be use to cancel it.

Order type can be:
 * limit: 0 when the order is push the maximum amount of quote is transferred depending on the available opposite order in the book. If some amount can't be matched, the remaining is added to the order book.
 * Immediate or Cancel: 1 same as limit but if a remaining amount can't be matched, it's cancelled.
 * Post: 2 the order is not matched an immediately added to the order book. It can be useful to decrease the fee (see Fee management).
 
 ## Withdraw
To get back all transferred asset from badge owner vault. Locked quote for pending bid order can be retrieve without cancelling the orders.

## Cancel
The method `cancel_order`, cancel a pending order with its id. Matched order can be cancelled. Quote locked by pending bid order are unlocked. Cancelled asset are return to the vault associated to the badge provided with the method call.

## Fee
Fee are withdraw from transferred asset to the market vault. When the market is created the quote and base vault is created to store fee taken from matched orders.
Order added that match other order in the book are call taker order and order that are matched from the book are call maker order.
Maker and taker order has different fee and often maker order are less.
In this example, taker order is 10% and maker order 5%.

# Test
To test at the root of the project use the cmd: `resim publish .` and `cargo test`

Transaction manifest are in progress. 