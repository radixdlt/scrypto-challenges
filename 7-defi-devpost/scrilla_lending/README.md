# Scrilla


## How to Run

#### Running on Simulator
While in the 'scrilla_lending' directory, run any of the following commands: 
- `bash ./scrypto/manifests/global1.sh` Global 1 focuses on testing liquidations, shield deposits, shield reward distributions
- `bash ./scrypto/manifests/global2.sh` Global 2 focuses on testing and showcasing the redeem_usds method that allows any user who may or may not have a loan with the platform to redeem USDS for the going market rate of XRD.  This allows USDS to maintain a 1:1 peg with USD.
- `bash ./scrypto/manifests/global3.sh` Global 3 focuses on testing scrilla staking and scrilla reward distribution of platform fees
These generic bash scripts cycle through methods for testing this dapp in various scenarios providing notes and tests about what is happening with each method.  These were created for testing purposes to make sure the math behind the methods is working, but they turned out to be a great way to showcase the functionality to whoever may be looking into this platform.  Some methods such as scrilla staking to earn platform fees require multiple accounts, many method calls, and liquidations to happen which is difficult to manage without these automated scripts.

*All values here are a bit constrained since only 1000 XRD come with the resim wallets*
I keep logs visible on the console for ease of seeing how the values for each account are being manipulated with each method call.


#### Running in Browser (Betanet dApp)
While in the 'scrilla_lending' directory, run these commands:
- `npm install` This will install necessary dependencies (You only need to do this once)
- `npm run dev` Then proceed to 'http://localhost:5173'

## About
Scrilla is our take on reverse engineering Liquity's stable coin and lending platform but without looking at any Solidity code.  

Scrilla leverages a single collateral design to minimize risk and complexity from implementing other more volatile, smaller cap coins. This platform works by giving 0% interest loans to any users that deposit XRD collateral.  Having the most stable collateral asset for the ledger (XRD) allows the protocol to get much more efficient with liquidations and collaterization rates as low as 110%.  This means you are free to leverage your XRD position by borrowing against almost all of it.  Folded leverage on XRD is also possibly if you continually borrow more USDS and buying XRD with it to then deposit as collateral.

Instead of traditional interest rates, Scrilla charges a small .5% fee for depositing and removing collateral, a small fee for liquidations (not yet implemented), and a small loan origination fee when borrowing USDS against XRD (not implemented yet).  After borrowing, you can literally hold your position open as long as you would like for no charge.  Liquidations only happen upon dropping blow the 110% collateralization rate.  Liquidators are currently incentivised to find and liquidate bad loans by a Scrilla token reward, but this will also eventually include an additional small bounty from the liquidated collateral tokens as well.

One of the coolest features of this platform when compared to something like Maker Dao is that there isnt a need for auctions for liquidations.  Auctions
can create many issues.  If a liquidation is happening because of a price drop, there can easily be price drops during an auction that can ruin the auction anyway.  Scrilla uses an incentivised instant liquidation model based on Liquity's Stability pool.  Shield depositors provide liquidity to pay back loans and also earn proportional amounts of the liquidated collateral that is worth more than the stablecoins provided at time of liquidation.

This platform will rely on at least one oracle to give accurate rates for XRD/USD.

A major goal for this submissions was also providing a scalable system of user claim based reward distributions.  This means a system where the number of users participating does not lead to an exponential increase in resources to distribute rewards while maintaining fully asset oriented design.  Using Batog's derivation of scalable reward distribution, I was able to provide a way for this dapp to successfully track running totals of rewards for all users by tracking equations of "product" and "summation" for rewards that factor out starting stake amounts.  This means every time a reward is earned, resources do not have to immediately be sent to the cooresponding vaults saving compute and gas.  A user can pull/claim their rewards at whatever time from a single vault containing all rewards for all users.  I was also able to manipulate Batog's derivation to work for both the unique deminishing deposits seen in the sheild pool as well as the static staking deposits for the Scrilla token.

Below are useful links to see the original projects site, documentation, and the mathemetical derivation used for scalable reward tracking of deminishing deposits.

https://www.liquity.org/

https://docs.liquity.org/faq/borrowing

http://batog.info/papers/scalable-reward-distribution.pdf

## Core Concepts and Methods

**1. Borrow**

**2. Shield Pool**

**3. Staking Pool**

### Borrow

Borrowing USDS requires depositing value in XRD into the collateral pool.  After depositing XRD, USDS can be borrowed while keeping at least 110% collateralization rate.

**new_user:** mints a new user NFT and passes to user in order to use the platform and track data.

**add_xrd_to_collateral:** allows user to add XRD to the platform to be used as collateral.  There is currently a 0.5% fee to deposit.

**remove_xrd_from_collateral:** allows user to remove XRD that was provided to the platform as collateral.  There is currently a 0.5% fee to withdraw.  This method will make sure withdrawing collateral does not put a user under liquidation threshold.

**borrow_usds:** allows user to borrow USDS stablecoin against XRD value up to 110% collateralization rate.

**repay_usds_loan:** allows user to pay USDS loan back and unlock a coresponding portion of XRD collateral.

### Shield Pool

The Shield Pool is the mechanism to protect the platform from liquidations.  Users can deposit  USDS into the shield that will be absorbed and burned upon liquidation as well as replaced with a larger value of XRD from the liquidated loan collateral.  

Users with or without direct loans with the platform are allowed to deposit USD-Scrilla (USDS) into Scrilla's Sheild Pool to absorb liquidations and earn a bonus chunk of collateral from the liquidated loan.  When a liquidation happens, the amount of outstanding USDS owed is removed proportionally from all shield pool depositors (if a liquidation drains 25% of the USDS Sheild pool, then every user loses 25% of their deposit and gains XRD collateral proportional to the deposit size). 

**deposit_to_shield_pool:** allows a user to deposit USDS to shield pool.

**withdraw_shield_deposit_and_rewards:** allows a user to deposit USDS to shield pool.

**call_liquidation:** allows anyone to call a liquidation on a bad loan.  When a user gets liquidated at 110%, that extra 10% goes towards incentivizing shield pool participants proportionally.  While the shield pool is designed to actually LOSE deposits over time and liquidations, the users are paid more back in XRD tokens.  Users triggering liquidations on loans will be rewarded with Scrilla platform tokens.

**show_liquidation_book:** method that returns the liquidation_book hashmap with key of user's NonfungibleLocalId and value of the xrd price at which the loan will be liquidated.  This value is stored and updated when appropriate within the component to track when a loan needs to be liquidated and provide this data to those doing the liquidating.

**sort_liquidation_book:** this is a method that sorts the liquidation_book hashmap into a BTreeMap while inverting the key and value . Liquidation price is now the key and since this is a BTreeMap, the list is sorted by key value.  This allows easier access to those loans closest to being liquidated which means less compute used when searching for loans to liquidate upon price changes.

**redeem_usds:** The main function of the dapp that keeps the peg of our USDS token to $1 USD is the ability to redeem USDS for 1:1 for the value of XRD at any time.  By using the scrilla protocol, each borrower assumes a little risk of ensuring fellow borrowers in this way.  If a person is paid in USDS and wishes to swap these tokens back for XRD, the tokens can be passed to the Scrilla component which will redeem these tokens against loans on the platform with the lowest collateralization rate.  The people closest to liquidation will have a small portion of their loans redeemed: this portion of USDS will no longer be owed but they will lose a cooresponding value of XRD collateral as well.  This process will improve the collateralization rate of these loans that are closer to liquidation.

### Stake Pool

**stake_scrilla:** allows user to stake Scrilla tokens and earn a portion of platform fees based upon ownership of the pool.

**unstake_scrilla_and_claim_rewards:** allows user to unstake all Scrilla tokens and claim all accrued platform fees based upon ownership of the pool when rewards occured.

**update_collateralization_and_liquidation_value:** this method is frequently used within the component and also in the bash script for resim.  While this is a "fake" method becuase everything actually autoupdates the NFT data before it is needed to be updated via the liquidation book if it is not updated by the user calling other methods.

**get_id_and_data_from_proof:** While the Scrilla component doesn't have any access rules to restrict calling methods, each method in scrilla that a user will interact with requires a proof of their individual NFT in order to know who is using the platform.  Only the person holding each NFT can interact on that accounts behalf.

**get_nft_data_from_id:** helper function to retrieve user NFT data.

**set_price:** "fake" method for the price oracle.  Allows tester to change the current price of XRD in order to trigger liquidations.

**find_platform_collateralization_ratio:** this method is not in full use yet as "Recovery Mode" is not fully implemented.  Recovery mode allows 

**take_platform_fee:** helper function to allow the component to put the appropriate fee in the fee vault as well as track the summation total needed in order to provide scalable rewards.

**show_info:** this method is used for testing purposes only.  It shows relevant information for a particular user as well as important data stored within the component.

**get_xrd_price:** this method calls the "fake" price oracle component to get the current price of XRD.

## Future Considerations and Improvements

Liquid staking unit implementation - This will be interesting but decided to not tackle this until all details are clear about how a standardized LSU can work

More robust transaction history tracking through NFT metadata

Recovery Mode - Liquity offers a recovery mode that allows for the platform to recover some collateral in case of catastrophic loss.  Recovery mode allows for anyone under the platform's collateralization rate as a whole to be liquidated.  If the platform is at 150% and a user is at 145%, the user can then be liquidated even though they are above 110%.  This is only for drastic measures to make sure the platform can survive.

Draining the shield pool - Liquity liquidations are successfully completed on Ethereum with a much higher estimated gas/transaction costs which should give even more value to work with while implmenting this design on the Radix ledger.  From this 10% difference in collateral and loan value, Scrilla can incentivize USDS shield depositers with a "less risky" steady stream income when liquidations occur.  It can also have small portions going toward buying protocol owned liquidity for XRD/USDS or possibly even USDC/USDA/USDS stable pool that could be used a secondary mechanism to absorb liquidations if for some reason the shield pools were drained.

The platform currently treats this as "The lowest 50% of loans will absorb all redemption" but this is just for ease of testing.  I would eventually build this platform out with the ability to calculate how much value is in a certain portion of loans before judgeing whether or not to redeem against it.  For example:

If amount to be redeemed is not less than 5% of bottom 10% of loans, then select a bigger group.
If amount to be redeemed is not less than 5% of bottom 20% of loans, then select a bigger group and so on...

## Betanet

While Scrilla is up and running on the Radix Babylon Betanet Network, a few important functions are not currently operational due to betanet development.  The front end SDK does appear to still use nonfungible types such as NonFungibleId, NonFungibleAddress even though scrypto was updated to v0.8 and now uses NonFungibleLocalId and NonFungibleGlobalId instead.  Since our liquidation method currently relies upon liquidating a loan based on a specific NonFungibleLocalId, the liquidation process is unable to execute on betanet.  This unfortunately also means that scrilla staking and fee earning is unavailable since liquidations are the path to obtaining Scrilla tokens.  These are all simple fixes as soon as the front end SDK matches the Scrypto types for NFTs.

## Contributors

https://github.com/aus87

https://github.com/errollgnargnar
