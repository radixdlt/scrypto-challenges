# Basket

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Abstract
 
Basket is a decentralized automated fund with the goal of providing increased performance relative to the denominator token by using a basket of tokens. The fund is managed by stakers who earn a yield based on the performance of their selections. Investors into the fund receive tokens which are pegged to the value of assets held by the fund.
 
## Motivation
 
The average of a set of independent actor's predictions is much more accurate than the average actor's prediction. This is essentially the power of a market. Humans have limited information bandwidth and when it comes to assigning predictive percentages we tend to pick a nice round number that sounds about right. The best we can do individually is to create an algorithm, and then follow it. However, algorithms are relatively nonadaptive. If the structure of the system changes to outside the bound of what is expected, then your algorithm will likely no longer work. By creating a distributed system of humans we can do much better. While each individual prediction is crude, the statistical average is not. Basket was created with the intention of leveraging this power to allocate the capital of a fund. Investors no longer have to worry about predictions, while stakers take on the risk of a prediction for a potential reward.
 
 
## Problems solved
 
Stakers must be incentivised to pick the right tokens, while the fund must be protected from distorted price information and front running attacks. Ideally the fund also has minimal reliance on any centralized actor. Investors into the fund must have a simple user experience in which they can freely enter and exit from the fund.
 
### Price
 
What is the price of something? It might seem like a simple question, but if you dig into it, it is surprisingly difficult. Is it the best bid or offer price for it? Or is it the price you can sell it for? If you think the latter then how do you know the price without executing the trade? You could try to simulate the trade, but you have no guarantee the market has not been manipulated to present you with a fake offer. This is a problem for Basket because the central idea is to have investments weighted by the stake they have been given, and how can you determine the desired weight if you do not know the value? The solution is to not directly try. Instead, whenever a new stake is made the fund sells off a proportion of investments to account for the new stake, then buys the into the investment the stake is for using the tokens from the sales. The inverse occurs when an unstake is made. This results in investments having roughly the right weight and allows for a true price to be extracted, which can then be used for incentives for stakers.
 
### Front running attacks
 
If a staker can make the fund buy or sell tokens with a value greater than their stake then they can sandwich attack the fund and drain value out of it. There is no way around it if the staker can access the market before the fund. The solution is to make a one sided market the staker can not access nor influence. For this an auction is created with a delay to allow all market actors to participate. The staker can not manipulate the auction in any way other than giving the fund a better price for the tokens.
 
### Investors user experience
 
Auctions are great to prevent front running, but investors have a poor user experience if they have to wait for an auction. Since investors do not have the same leverage stakers do, the fund can safely utilize AMM pools. Any manipulation to the AMM pool by the investor would effectively result in them buying or selling tokens to/from themself.
 
### Limiting the selection
 
Suppose a staker wants the fund to buy a token of which they hold 100% of the supply. The fund creates an auction, but the staker holds all of the tokens, so the staker can set any price they wish. This situation shows not every token can be a part of the fund. Currently the fund creates an admin badge that is used for adding new tokens to the fund. Once all the desired tokens have been added the fund can be made immutable, permanently locking the fund meaning no more tokens can be added, and removing all centralization. For the short term I feel this is a satisfactory compromise. However, ideally instead this admin badge is given to a governance DAO that allows for a fully decentralized functioning of the fund.
 
### Incentives
 
To attract stakers and encourage stakers to put in effort when picking investments, incentives are provided. If the sell price is higher than the buy price for an investment, then the staker receives a reward that is a percentage of the profit. If the sell price is lower than the buy price for an investment, then the staker has a percentage of their stake equal to the price decrease burned. These two incentives create an equilibrium for the percentage of stakers. Above this equilibrium it would be more capital efficient for the staker to just buy the token. Below this equilibrium there is a cost free amount of exposure to the token, incentivizing them to stake.
 
 
## Interesting features
 
### Fund
 
- Fund token pegged to the value of investments.
- Decentralized capital allocation through stakers.
- Oracle independent value balancing.
- Sandwich attack protection through delayed auctions.
- Natural stake equilibrium.
- Multi-stage stakes managed by nfts.
 
### Auction
 
- Protected one sided market.
- Singly linked list bid chain for efficient ordering.
 
### Getting started
 
The demo uses python to provide a simplified interactive interface. Available functions are:
 
- `setup()` Resets the simulator. Creates 3 tokens A, B, and C. Creates AMM pools for these tokens. Creates a fund and adds these tokens as investments.
- `show(address)` Shows information for address. Examples: `show(account)` or `show(fund)`.
- `set_epoch` Sets epoch for the simulator. Necessary to progress stakes.
- `mint(amount)` Mints fund tokens for `amount` of XRD.
- `redeem(amount)` Redeems `amount` of fund tokens for XRD.
- `redeem_for_tokens(amount)` Redeems `amount` of fund tokens for investment tokens.
- `stake(amount, investment)` Stakes `amount` of fund tokens for `investment`. Example: `stake(30, 0)`.
- `unstake(id)` Unstakes currently staked stake receipt `id`. Example: `unstake('9e093103683a0e3254eb8c9be9bcac03')`.
- `collect_unstaked(id)` Collects fund tokens from unstaked `id`. Example: `collect_unstaked('9e093103683a0e3254eb8c9be9bcac03')`.
- `process_stakes()` Processes stakes and unstakes. More than 20 epochs must have passed to process.
- `amm_swap(amount, token, pool)` Swaps `amount` of `token` with `pool`. Examples: `amm_swap(100, token_A, pool_A)` or `amm_swap(100, xrd, pool_A)`.
- `amm_remove_liquidity(amount, lp_token, pool)` Removes `amount` of liquidity from `pool`. Example: `amm_remove_liquidity(1000, lp_token_A, pool_A)`
- `create_bid(auction, amount, token, price)` Creates a bid of `amount` for `auction` with `price`. Examples: `create_bid(buy_auction_A, 100000, token_A, 0.1)` or `create_bid(sell_auction_A, 10000, xrd, 10)`
- `close_bid(auction, id, bid_type)` Closes bid `id` for `auction`. Example: `close_bid(sell_auction_A, '64783aa9fb3d7203f041ec64d0c00d60', sell_bid_A)`
 
Available variables are:
 
- `xrd`
- `account`
- `ammdex_package`
- `basket_package`
- `token_A`
- `token_B`
- `token_C`
- `pool_A`
- `lp_token_A`
- `pool_B`
- `lp_token_B`
- `pool_C`
- `lp_token_C`
- `fund`
- `fund_admin_badge`
- `fund_token`
- `fund_stake_receipt`
- `buy_auction_A`
- `buy_bid_A`
- `sell_auction_A`
- `sell_bid_A`
- `buy_auction_B`
- `buy_bid_B`
- `sell_auction_B`
- `sell_bid_B`
- `buy_auction_C`
- `buy_bid_C`
- `sell_auction_C`
- `sell_bid_C`
 
To run the demo, first make sure you have an up to date version of python installed. Open `demo.py` and edit the main function with the desired sequence of operations. Then run:
 
```
python demo.py
```
 
Or if you wish to enter commands one at a time. Open the python interpreter by running:
 
```
python
```
 
Next import the demo:
 
```
from demo import *
```
 
Run setup to create all the necessary components:
 
```
setup()
```
 
You are now ready for testing! For example you could run the following sequence:
 
```
create_bid(buy_auction_A, 100000, token_A, 0.1)
create_bid(sell_auction_A, 10000, xrd, 10)
create_bid(buy_auction_B, 300000, token_B, 0.033)
create_bid(sell_auction_B, 10000, xrd, 30)
create_bid(buy_auction_C, 500000, token_C, 0.02)
create_bid(sell_auction_C, 10000, xrd, 50)
 
mint(1000)
stake(30, 0)
set_epoch(30)
process_stakes()
 
show(fund)
show(account)
```
 
## Conclusion

While making Basket there were several times I considered if the problem I was trying to solve might be impossible. Every problem described above was a previous failed version. At this time I can not find any fatal flaws in the current version. However, I have found this to be a very tricky problem and I would want to do rigorous testing with more minds involved before it was publicly used. That being said, I tentatively call this success.

## License

This work is licensed under Apache 2.0. The license file is provided [here](./LICENSE).