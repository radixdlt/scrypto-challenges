
# White_Label_Wallet

## About
The White-Label-Wallet is a general-purpose crowdfunding blueprint. The blueprint allows users to pool their resources together into a treasury. Non-fungible badges are utilized to keep track of pertinent user information such as username, contact and pro-rata share of the treasury assets. 

The objective of this project was to create a simple wallet for several purposes that allow ownership of vaults to be represented by NFTs but also be capable of fractionalizing ownership of NFTs, real estate, physical property, and traditional tokens. Bash scripts are used to automate resim commands and RTMs. 

#### How to Run
While in the 'shares' directory, run the command: `bash ./manis/global.sh`
This is the generic bash script that cycles through all methods for testing everything. The following shows three customized scenarios with a bash script for each that accomplishes the objectives of each scenario.

## Core Concepts

![wlw_high_level_bkgd drawio](https://user-images.githubusercontent.com/107159153/203169381-60a8a2a0-fa14-43e7-8b0f-e774858f4e62.png)

On a high level, there are three main parts:

**1. Shareholders (Users)**

**2. Treasury (Blueprint/Component)**

**3. Pooling, Distributing and Claiming Funds.**


### Shareholders

![wlw_shareholder_bkgd drawio](https://user-images.githubusercontent.com/107159153/203169495-832be5bd-b156-4e3f-bd90-a1859e3d8d68.png)

The shareholders are the parties that have a claim on the treasury's assets. Each shareholder receives a non-fungible owner_badge that gives them the authority to use the component's methods. In addition, a shareholder's owner_badge will contain non_fungible_data such as their username, contact, and percent_ownership_of_treasury.

### Treasury

![wlw_treasury_bkgd drawio](https://user-images.githubusercontent.com/107159153/203169785-39f9d890-7931-40c6-b285-df235b9e79d4.png)

The treasury is the shared wallet that allows users to crowdsource funds. The component is made up of a series of shared vaults, each representing a token and their corresponding resource_addresses. In order to keep track of the shareholders (users), an owner_record vector is used within the treasury to record each owner corresponding to their unique non_fungible_id. In addition, in order to keep track of everyone's pro-rata share, the funds_owed vector is used.

### Pooling, Distributing and Claiming Funds
The main function of the White-Label-Wallet is to crowdsource funds from shareholders in order to purchase assets and share in the potential future cashflows. Three primary methods are used to facilitate this process.

**1. Pool_Escrow_Funds:** Shareholder's pledge and deposit their assets to the treasury.

![wlw_pool_escrow_funds_bkgd drawio](https://user-images.githubusercontent.com/107159153/203169584-bfb2d1e4-6ce0-4b85-a64a-1cccd7831907.png)

**2. Distribute_Treasury_Funds:** The treasury logs shareholder data into the funds_owed vector.

![wlw_distribute_treasury_funds_bkgd](https://user-images.githubusercontent.com/107159153/203169649-da18f33e-93aa-49b9-b634-58a1c9a0c868.png)

**3. Claim_Treasury_Funds:** Shareholders withdraw their share of the treasury's assets.

![wlw_claim_treasury_funds_bkgd](https://user-images.githubusercontent.com/107159153/203169721-653060c7-6085-4c48-88c8-9a081bcbf3d5.png)


## Method Definitions
**pool_escrow_funds:** Collects and deposits the funds from all users based on their percent_ownership_of_treasury (Outlined in the owner_badge non_fungible_data) into the treasury.

**distribute_treasury_funds:** Allocates the pro-rata claims on treasury funds into the funds_owed vector.

**claim_treasury_funds:** Withdraws the pro-rata share of treasury funds into the owner's account.

**update_owner_user_name:** Updates the owner's username within the owner_badge's non_fungible_data.

**update_owner_contact:** Updates the owner's contact information within the owner_badge's non_fungible_data.

**merge_ownership:** Takes an owner_badge and merges its percent_ownership_of_treasury with that of a second owner_badge (burning the first in the process).

**split_ownership:** Takes an owner_badge and splits its percent_ownership_of_treasury with a newly minted owner_badge.

**deposit_to_treasury:** Allows any user with an owner_badge or depositor_badge to send resources to the treasury.

**new_depositor:** Mints a new depositor_badge and returns it to the caller.

**check_or_create_vault:** Queries the component_treasury HashMap for an existing vault, if not creates a new vault with the supplied resource_address.

**show_single_treasury_balance:** Displays the current vault balance for a supplied resource_address.

**show_all_treasury_balances:** Displays all current vault balances in the treasury.


## Use Cases

**General Case:** A person wants to fractionalize ownership of an existing NFT, a basket of tokens, property, or a mixture of assets. Logic is provided to distribute income from these assets proportionally to each owner. 

### Scenario 1
**Real Estate Finance:** A commercial building owner wants to raise funds but doesn't want to mortgage his property. Instead, the developer decides to mint an owner's NFTs and transfer ownership of his property to this NFT address. The owner can now split as much or little of his NFT into a second owner NFT to sell to leverage remodeling, updates, repairs, etc. It is also possible for him to repurchase his shares later if both owners agree on a price. It is also possible for this owner to mint multiple NFTs representing ownership of each unit of the property and sell stake on a per unit basis. 

#### How to Run
While in the 'shares' directory, run the command: `bash ./manis/Scenario1/global1.sh`
This will automate the Shares dApp by publishing the Shares package, instantiating the Shares component with one initial shareholder, and receiving the owner NFT with 100% ownership metadata. Next, the owner would deed the property to the NFT address of the owner NFT, run the split_ownership method with .05 input representing splitting this owner NFT into two representing 95% ownership on the NFT they would retain and 5% ownership to sell to raise funds.

### Scenario 2 **** UNDER DEVELOPMENT ****
**NFT Crowdfund:** Three people pool a predetermined amount of XRD, each paying equal amounts to purchase a halo cig abandoned scorpion NFT. The NFT costs 1,500 XRD, and each person will be responsible for 500 XRD. A transaction is composed via RTM to accept the required buckets from each person contributing to the pool. Each new fractional owner of the NFT can buy or sell shares of ownership of this 1 NFT to as many decimal places as Scrypto Decimal can handle.

#### How to Run
While in the 'shares' directory, run the command: `bash ./manis/Scenario2/global2.sh`
This will automate the Shares dApp by publishing the Shares package, instantiating the Shares component with three initial shareholders, and receiving the owner NFTs each with 33.3% ownership. The person setting up the Shares component would pass two owners' NFTs to the other two initial investors and then call the pool escrow funds method. These steps will allow the RTM to compose a transaction where all three owners can contribute their 500 XRD each to the component in a trustless way to purchase the scorpion NFT.

### Scenario 3 
**Developer Collaboration:** Three developers want to form a group and create a shared wallet that can accept payments from clients, pool earnings, and distribute funds when needed, similar to a multi-sig wallet. A developer can instantiate a Shares component with three initial shareholders and pass the other team members one owner NFT each. The default ownership is equal for each owner, but they can split/sell or buy/merge NFTs to increase or decrease their right to the revenues. Finally, the team mints a depositor badge for the project's client that can be used to pay developers directly via the component_treasury. 

#### How to Run
While in the 'shares' directory, run the command: `bash ./manis/Scenario3/global3.sh`
This will automate the Shares dApp by publishing the Shares package, instantiating the Shares component with three initial shareholders, and receiving the owner NFTs each with 33.3% ownership. The person setting up the Shares component would pass two owners' NFTs to the other two developers. Next, a depositor badge is minted for clients, allowing them to deposit within the shared component vault. Finally, the claim_treasury_funds method can be called via RTM after payments are made from the client to split proceeds between owners of the Shares component owner NFTs according to ownership metadata held within the owner NFT. These steps will allow the RTM to compose a transaction where all three developers have a pro-rata share of revenues that come into the treasury via clients.


## Contributors

https://github.com/aus87

https://github.com/errollgnargnar

https://github.com/krowtaergeht
