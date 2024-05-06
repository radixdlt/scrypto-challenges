# ChainDEX

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

## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.