# bounce

I thought of an idea for the *‘Portfolio Management and Yield Farming Challenge’* that I was quite happy with. I entertained dreams of teaching myself Scrypto, then working day and night to create a functional dApp. Unfortunately, I then discovered that an intrinsic part of my idea directly conflicts with one of the recommended design patterns in Scrypto: [‘The Withdraw Pattern’](https://docs.radixdlt.com/main/scrypto/design-patterns/withdraw-pattern.html). 

Because I have reached the peak of 'Mount Stupid' on the [Dunning-Kruger chart](https://kristianmagnus.com/dunning-kruger-effect/), I'm *still* quite happy with my idea. Therefore, instead of what I was going to submit, I’m submitting a few lines of code – a small change to the [‘account’](https://github.com/radixdlt/radixdlt-scrypto/blob/main/assets/account/src/lib.rs) component – that expedites exceptions to the pattern. This is absolutely meant to be provocative, though hopefully it’s either provocative in  a constructive way, or my hyperbole acts as  bait that prompts a detailed explanation as to why I'm wrong. In any case, it provides an excuse allowing me to describe and outline my original idea as an example of why I think the pattern *should have* exceptions. Which I’ll do shortly.

The _Withdraw Pattern_ documentation states that *“While the direct transfer of funds is possible”* in Scrypto *“it is heavily discouraged to perform deposits in this way”*. In the vast majority of cases, I wholeheartedly agree with this sentiment and with the justifications given. However, I don’t believe it should be generalised to all cases. Sometimes, direct deposits from dApps may improve the retail user experience, which should be paramount. Moreover, even if you believe that the pattern should always be adhered to, you do not have control over what is published on an permissionless platform. Therefore, it makes sense to provide a clear option to users whilst at the same time encouraging developers to behave responsibly.

I have altered the standard account component with the addition of a boolean called ‘bounce’ that defaults to ‘true’ but can be set to ‘false’ by the account owner. The intended use for this switch is to allow an account owner to signal that they are prepared to accept direct transfer of funds from a dApp or other entity on Radix at some indeterminate future point in time, in a separate transaction. Note that I can't check that the code actually works as I can't build the Account component without KeyValueStore. 
 
![image](https://github.com/marktwh/scrypto-challenges/blob/main/4-portfolio-management-yield-farming/bounce/images/2.jpg)
![image](https://github.com/marktwh/scrypto-challenges/blob/main/4-portfolio-management-yield-farming/bounce/images/3.jpg)


Anyway, you get the point. A dApp developer wishing to automate direct transfers to users could voluntarily include a requirement for an assertion that bounce = false as part of any user-submitted transaction.

‘Bouncer’ logic serves as a gatekeeper to dApps that incorporate 'non-atomic' transactions. To the extent that the modified Account component facilitates interactions with portfolio management or yield farming that incorporate 'non-atomic' transactions, it's a Scrypto package with portfolio management or yield farming features.

This was my original idea:

## The SMPL-AMM
It's offensively simple. It’s an experimental [AMM](https://www.coindesk.com/learn/2021/08/20/what-is-an-automated-market-maker/) called a Specified Maximum Permanent Loss (SMPL)-AMM that works as follows:

*	When a [liquidity provider](https://medium.com/jigstack/what-is-liquidity-provider-lp-dcc122c6327d) (LP) deposits funds into a SMPL-AMM component, they specify the maximum permanent loss they are willing to experience as a result of providing liquidity.
*	Then, every time a trader submits a swap transaction to the component, the SMPL-AMM calculates the price-change that would be caused by the proposed swap. Where this would cause [impermanent loss](https://chainbulletin.com/impermanent-loss-explained-with-examples-math) (IL) above the LP-specified amount, the component returns LPs’ stake before the swap proceeds, making the loss permanent, but limited.
*	Since LPs accrue yield from fees, their profits (or losses) depend on the volume of swaps that take place before their stake is returned. The LPs are also issued with NFTs with which they can manually remove their stake at any time.

The benefits to LPs from this arrangement are clear. While an LP can make profits from fees on a standard AMM, they are at constant risk of IL, and losses from IL can be significant. In contrast, permanent losses on a SMPL-AMM can never exceed a specified maximum.

IL-risk on a standard AMM can be mitigated by careful monitoring of positions or use of bots. However, it is a strong deterrent to many prospective LPs. Many are unwilling to spend time monitoring investments. Many are unwilling to entrust their private keys to a cloud server. Conventional portfolio management and liquidity mining dApps are another alternative, but these can be associated with risk due to third-party control of funds. Also, where rewards are paid in inflationary tokens, monitoring may still be required.

With a SMPL-AMM, the user experience is relatively predictable. An LP can ‘deposit-and-forget’ their stake. In the worst-case, their stake is returned in a short space of time with losses within acceptable parameters. In the best-case, the funds are retained in the component for longer and accrued fees accumulate so that the LP is in profit when the stake is eventually returned. Notably, the experience of ‘swap’ users need not change. Although it is their transactions that trigger return of LPs’ stakes, the SMPL-AMM component could be designed in such a way that this would not necessitate payment of additional network fees. Thus, a SMPL-AMM could attract LPs who would not otherwise invest in an AMM without reducing utilisation by retail swappers. On the other hand, the SMPL-AMM may reduce profits for arbitrage traders by facilitating more dynamic price discovery.

### Dynamic price discovery
A key advantage of a SMPL-AMM as compared to a conventional AMM can be seen when IL is considered from the perspective of the entire AMM component as well as from the perspective of individual LPs. Essentially, when IL is limited for individual LPs, IL is limited for the component also.

When liquidity is automatically removed from a SMPL-AMM as the result of a sudden price change, the $x*y=k$ (or similar) curve means that a new target price is reached on lower swap-volume. Therefore, less value can be captured by arbitrageurs capitalising on mispriced assets. To illustrate the problem with mispriced assets, the following model shows simple arbitrage between two similar AMMs:

*	AMMA and AMMB each contain a similar total value of GBP and XRD (denominated in GBP; XRD at marketwide price = 1.111GBP). However, the XRD is mispriced on AMMA at 0.9GBP, while it is correctly priced on AMMB at 1.111GBP.


![image](https://github.com/marktwh/scrypto-challenges/blob/main/4-portfolio-management-yield-farming/bounce/images/4.jpg)


*	An arbitrageur exploits the situation. In order to make use of a flash-loan denominated in GBP, and in avoid exposure to the more volatile XRD asset, they perform an atomic transaction in which cheap XRD is first brought on AMMA and then sold on AMMB for a net gain in GBP.


![image](https://github.com/marktwh/scrypto-challenges/blob/main/4-portfolio-management-yield-farming/bounce/images/5.jpg)


Importantly, the arbitrageur’s trade had the effect of removing total value from AMMA whilst it added a small amount of total value to AMMB. If the mispricing persists, AMMA will continue to be vulnerable to monodirectional arbitrage trades that reduce the value of its contents while AMMB will be subject to bidirectional trades around the market price that will gradually increase the value of its contents.

In an environment in which all AMMs have the same mechanics, price movements have similar effects on each. That is, it ‘costs’ the same – proportionately - to get from a mispriced state to a correctly-priced state. However, because a SMPL-AMM would tend to shed [TVL](https://learn.bybit.com/defi/total-value-locked-tvl/) upon the sudden price movements, it would ‘cost’ less to get to a correctly-priced state. 

An interesting possibility is that the SMPL-AMM might also get from a mispriced state to a correctly-priced state *faster*. If so, it might more frequently be on the ‘output’ side of arbitrage trades rather than the ‘input’ side. Therefore, it might benefit (acquire value) from these trades at the expense of other AMMs. Fast eats slow.

Obviously, what actually happens would depend on the individual and collective behaviour of all market participants, but it would be fun to find out, no?

### Mechanics
A prototype SMPL-AMM component would accept stable-nonstable token pairs since the calculations of threshold-prices causing particular ILs can then be calculated within individual components. Calculations for IL affecting nonstable-nonstable pairs are possible with reference prices from appropriate stable-nonstable pair-pairs provided via [TWAP oracles](https://medium.com/blockchain-development-notes/a-guide-on-uniswap-v3-twap-oracle-2aa74a4a97c5), but that’s beyond the scope of this outline.

This outline describes the instantiation function and methods for the LP deposits, swaps (incorporating automatic LP withdrawals), and manual LP withdrawals. 

I’ve used @0xOmarA’s excellent and very well annotated [RaDEX](https://github.com/radixdlt/scrypto-challenges/tree/main/1-exchanges/RaDEX) project for reference.

#### Blueprint:

The blueprint struct would require:

* Liquidity Pool vaults to contain stable and nonstable tokens
* Vault for admin badge for minting LP-NFTs
* Ascending vector for storing caller-account-address, returned LP-NFT-ID, LP-stake decimal (representing proportional pool-ownership), and upper-nonstable-price-threshold above which LP stake is returned.
* Descending vector for storing caller-account-address, returned LP-NFT-ID, LP-stake decimal, and lower-nonstable-price-threshold below which LP stake is returned.
* %-maximum-acceptable-permanent-loss decimal (or upper-nonstable-price-threshold and lower-nonstable-price-thresholds; see below).
* Fee-to-pool decimal

#### Instantiation:
The SMPL-AMM component instantiates upon asserting caller's bounce==false, receiving the caller’s account-address, a quantity of stabletokens, a quantity of nonstable tokens, a %-maximum-acceptable-permanent-loss number, and a fee-to-pool number. 

* Checks include checks bounce==false, checks that one token is a stabletoken and the other is a nonstable token (admin-controlled hashmap-list of acceptable stabletokens could be used), checks that token quantities >0, and checks that %-maximum-acceptable-permanent-loss fee-to-pool numbers are 0>100.

* Calculates $k$ in the constant market maker equation: $x * y = k$ 

* Mints the NFT minting badge. Mints an NFT with metadata corresponding to proportionate-pool-ownership (initially 100, say). Stores the proportionate-pool-ownership value as an existing-pool-size variable for use in the ‘Add-liquidity’ method.

* Calculates from the %-maximum-acceptable-permanent-loss number, the upper-nonstable-price-threshold and lower-nonstable-price-threshold corresponding to that level of IL.
 * **IL calculation:** 
 * ![image](https://github.com/marktwh/scrypto-challenges/blob/main/4-portfolio-management-yield-farming/bounce/images/6.jpg)
 * The total value ($y$) of a quantity ($m$) of nonstable tokens with price ($x$) that is held rather than deposited is linear ($y=mx+c$).
 * The total value of a deposited stable-nonstable token pair is given by $2\sqrt{mc}\sqrt{x}$ or $/2\sqrt{k}\sqrt{x}$.
 * Therefore -IL == $-l = \frac{2\sqrt{k}\sqrt{x}}{mx + c}-1$
 * From the provided %-maximum-acceptable-permanent-loss decimal/100, the prices at which the maximum allowable IL is reached are given by solving the quadratic for $x$:
 * $$x_{lower}= \frac{-4\sqrt{-ckl^2m+2cklm-ckm+k^2}-2cl^2m+4clm-2cm+4k}{2(l^2-2l+1)m^2}$$ and $$x_{upper}= \frac{4\sqrt{-ckl^2m+2cklm-ckm+k^2}-2cl^2m+4clm-2cm+4k}{2(l^2-2l+1)m^2}$$ so depending on how much calculations cost on Radix, it might be worth doing these in the frontend rather than in the component itself…
 

* Pushes the upper-nonstable-price-threshold, the caller’s account-address, the caller’s NFT-ID and the caller’s proportionate-pool-ownership to the ascending vector.

* Pushes the lower-nonstable-price-threshold, the caller’s account-address, the caller’s NFT-ID and the caller’s proportionate-pool-ownership to the descending vector.

* Returns NFT

#### Add-liquidity method:
Adds liquidity to the pool upon asserting caller's bounce==false, receiving the caller’s account-address, a quantity of stabletokens, a quantity of nonstable tokens, and a %-maximum-acceptable-permanent-loss number.

* Checks as above.

* Calculates ratio of total stable/nonstable tokens already in the pool. Takes the maximum possible number of tokens of those sent by the caller in the same proportions (all of one token, part of the other if it is in excess) – ‘tokens to be added’.

* Calculates proportionate-pool-ownership from the above. Calculates the number representing proportionate-pool-ownership from this and from the existing-pool-size variable to add as metadata to the NFT. Mints the NFT with this metadata. Updates the existing-pool-size variable.

* Calculates the upper-nonstable-price-threshold and lower-nonstable-price-threshold from the %-maximum-acceptable-permanent-loss number provided by the caller as above.

* Inserts the upper-nonstable-price-threshold, the caller’s account-address, the caller’s NFT-ID and the caller’s proportionate-pool-ownership to the ascending vector so that the vector is ordered by upper-nonstable-price-threshold and reindexed.

* Inserts the lower-nonstable-price-threshold, the caller’s account-address, the caller’s NFT-ID and the caller’s proportionate-pool-ownership to the descending vector so that the vector is ordered by lower-nonstable-price-threshold and reindexed.

* Returns any excess stabletokens or nonstable tokens. Returns NFT.

#### Swap method:
Where a caller initiates an exchange of stabletoken for nonstable token, swap first calculates the potential price-impact of the exchange and then compares this to the upper-nonstable-price-threshold at the 0 position in the ascending vector.

Where a caller initiates an exchange of nonstable for stable token, swap calculates the potential price-impact of the exchange and then compares this to the lower-nonstable-price-threshold at the 0 position in the descending vector.

In either case, potential-price-impact is calculated in a similar way.
First the potential number of output tokens is calculated according to

$$dy = \frac {dx * r * y}{x + r * dx}$$ 

where $x$ is the number of input token already in the pool, $y$ is the number of output tokens already in the pool, $dx$ is the number of input tokens provided by the caller, $dy$ is the potential amount of output tokens, and $r$ is the fee modifier; $r$ = (100 - fee) / 100 %.

Then the potential new totals of tokens in the pool are calculated (eg. $y$ - $dy$ and $dx$ + $x$). Then the nonstable token potential-price is calculated with potential-new-stable-total / potential-new-nonstable-total.

If the potential-price > upper-nonstable-price-threshold at the 0 position in the ascending vector or < lower-nonstable-price-threshold at the 0 position in the descending vector then the swap does not proceed. Instead, the corresponding proportionate-pool-ownership number in the vector is used together with the existing-pool-size variable to calculate the number of tokens to directly return funds to the corresponding LP account address. These funds are placed in earmarked buckets. The existing-pool-size variable is updated. Then the entries in both ascending and descending vectors with the corresponding NFT-ID are removed and the vectors are reindexed. Then the method loops back to recalculate a number of output tokens and potential-price change, since these will have changed due to removal of tokens from the pool, and then re-checks the new potential-price against the appropriate vector.

If the potential-price < upper-nonstable-price-threshold at the 0 position in the ascending vector or > lower-nonstable-price-threshold at the 0 position in the descending vector then the swap proceeds. The $dy$ number from the most recent iteration of the loop gives the number of tokens to return. Note that $k$ must now be recalculated for future use in the Add-liquidity method.

* Returns swapped tokens to caller. Direct-sends any removed liquidity to appropriate LP(s), if applicable.

#### Manual remove-liquidity method:
* The caller sends their NFT to the method to remove their liquidity.

* The NFT-ID is checked against the entries in one of the ascending or descending vector.

* If the NFT-ID is absent from the vector, then the NFT is burned.

* If the NFT-ID is present in the vector, then the corresponding proportionate-pool-ownership number in the vector is used together with the existing-pool-size variable to calculate the number of tokens to return. These funds are placed in earmarked buckets. The existing-pool-size variable is updated. Then the entries in both ascending and descending vectors with the corresponding NFT-ID are removed and the vectors are reindexed. Then the NFT is burned.

* Returns removed liquidity to caller.

An interesting aside is that the NFTs associated with a SMPL-AMM would not be conventionally tradable because underlying liquidity might be automatically removed at any time (rendering the NFT worthless). However, they might become tradable as a form of futures instrument if trustless timelocked containers were used as an intermediary.
