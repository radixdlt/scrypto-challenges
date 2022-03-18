ChainDEX is a decentralized order book exchange. It uses two one-way linked lists of Order NFTs to represent the order book. One for buys and one for sells. When someone places a order their tokens are deposited into the ChainBook vault and a Order NFT is returned to them. This NFT can be returned to the ChainBook to claim tokens if it is filled. It can also be returned to ChainBook to cancel the order and refund tokens if the order is not filled. If there is a better price than the order asks at the time it is placed then it completes instantly and returns tokens without creating a Order NFT. This can happen in part or in full.

```math
pub fn instantiate_chain_book(name: String, a_token_address: Address, b_token_address: Address) -> Component
```
Create ChainBook trading pair for tokens a,b. Returns ChainBook.

```math
pub fn create_order(&mut self, mut tokens: Bucket, price: Decimal) -> (Bucket, Bucket)
```
Fills orders with a better price than asking then creates a limit order with the remaining tokens. Returns (tokens, ?order).

```math
pub fn claim_tokens(&mut self, mut order: Bucket) -> (Bucket, Bucket)
```
Gives tokens for filled part of order, burns order if completed else returns updated order. Returns (tokens, ?order).

```math
pub fn cancel_order(&mut self, order: Bucket) -> (Bucket, Bucket)
```
Claims filled part of order then refunds remaining part of order. Return (remaining_tokens, filled_tokens).
