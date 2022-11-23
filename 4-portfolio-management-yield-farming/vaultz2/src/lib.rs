use scrypto::prelude::*;

// ***This code is NOT COMPLETE but I wanted to submit my progress***


// Vaultz is a platform meant to enable you to hodl your xrd in ways you would normally invest, but with the added 
// benefit of capturing value from volatility in the market.  Althought there is a small fee to participate, the
// long term hodlers will be greatly rewarded.  Upon swapping to vXRD from XRD, you begin earning a fee on every exchange
//  of vXRD.  As market volatility creates prices differences between XRD and vXRD, arbitrage and fees associated
// are sent back to vxrd stakers, hodlers, lendors, and LP token stakers.  There is a .5% fee for minting,
// burning, and trading vXRD that would go back to users in rewards. Reward proportion can be manipulated 
// over time if, for example, enough protocol owned liquidity is achieved. 
// 


blueprint! {
    struct Vaultz {
        vxrd_address: ResourceAddress, // Resource address of vXRD
        vxrd_vault: Vault, // Vault to store the vXRD

        collected_xrd: Vault, // Vault to store the XRD collected in exchange for vXRD

        loaned_vxrd:  Vault, // Vault to store vXRD users wish to loan to a lending platform

        vxrd_xrd_lp_token: Vault, // Vault to store vXRD-XRD LP tokens for rewards

        xrd_fees_collected: Vault, // Vault to store all XRD from collected fees

        buy_fee: Decimal, // Fee associated with buying/minting vXRD
        sell_fee: Decimal, // Fee associated with selling/burning vXRD

        admin_badge: ResourceAddress, // resources address of admin badge given to instantiator of Vaultz
        vxrd_mint_badge: Vault, // minting authority badge held in component to allow component to mint/burn

        loan_receipt: ResourceAddress, // Resource address of loan_receipt which is 1:1 per vxrd loaned to platform

        lp_reward_ratio: Decimal, // the sum of the next 4 values must equal to 1 to distribute 100% of rerwards each epoch
        hodl_reward_ratio: Decimal,
        lendor_reward_ratio: Decimal,
        stake_reward_ratio: Decimal,


    }

    impl Vaultz {
        pub fn instantiate_vaultz(buy_fee: Decimal, sell_fee: Decimal) -> ComponentAddress {
            // Create an admin badge that will be returned to the caller of instantiate_vaultz
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Vaultz admin badge")
                .initial_supply(1);

            // Create a minting authority badge, that will be kept
            // inside the component to be able to mint and burn vXRD later
            let vxrd_mint_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "vXRD mint auth")
                .initial_supply(1);
            
            // Create vXRD token
            let vxrd_address: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Vault-XRD")
                .metadata("symbol", "vXRD")
                .mintable(rule!(require(vxrd_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(vxrd_mint_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let loan_receipt: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "Loan Receipt")
                .mintable(rule!(require(vxrd_mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(vxrd_mint_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Instantiate the component
            let component = Self {
                vxrd_vault: Vault::new(vxrd_address),
                collected_xrd: Vault::new(RADIX_TOKEN),
                loaned_vxrd: Vault::new(RADIX_TOKEN),
                vxrd_xrd_lp_vault: Vault::new(vxrd_xrd_lp_token),
                xrd_fees_collected: Vault::new(RADIX_TOKEN),
                buy_fee,
                sell_fee,
                admin_badge: admin_badge.resource_address(),
                vxrd_mint_badge: Vault::with_bucket(vxrd_mint_badge),
                lp_reward_ratio,
                hodl_reward_ratio,
                lendor_reward_ratio,
                stake_reward_ratio,
            }
            .instantiate();

            // Set the access rules for this component before globalizing
            // Only someone presenting the admin_badge will be able to call
            // the "distribute_rewards" and "change_distro_rules" methods.
            let access_rules = AccessRules::new()
                .method("distribute_rewards", rule!(require(admin_badge.resource_address())))
                .method("change_distro_rules", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            (component.add_access_check(access_rules).globalize(), admin_badge)
        }

        //takes xrd_deposit from user, deposits xrd into collected_xrd vault, mints vXRD and gives to user
        pub fn mint_vxrd(&mut self, xrd_deposit: Bucket) -> Bucket {
            
            let vxrd_resource_manager = borrow_resource_manager!(self.vxrd_address);
            let vxrd_to_mint = xrd_deposit.amount() * (dec!("1")-self.buy_fee);
            let fee = xrd_deposit.amount() * self.buy_fee;

            // puts xrd in vault
            self.collected_xrd.put(vxrd_to_mint);

            //puts the collected fee in vault
            self.xrd_fees_collected.put(fee);
        

            // Mint vxrd according to how much xrd was deposited
            let vxrd = self.vxrd_mint_badge
                .authorize(|| vxrd_resource_manager.mint(vxrd_to_mint));

            // Return the vxrd to user
            vxrd
        }

        // takes vXRD from user, burns the vXRD, returns XRD to user
        pub fn burn_vxrd(&mut self, vxrd_deposit: Bucket) -> Bucket {
            
            // put fee in vault
            let fee = vxrd_deposit.amount() * self.buy_fee;
            self.xrd_fees_collected.take(fee);

            // Burn vxrd that was deposited
            let vxrd_resource_manager = borrow_resource_manager!(self.vxrd_address);
            self.vxrd_mint_badge
                .authorize(|| vxrd_resource_manager.burn(vxrd_deposit));

            // Return the xrd minus fee    
            let xrd_to_return = vxrd_deposit.amount() * (dec!("1")-self.buy_fee);
            self.collected_xrd.take(xrd_to_return)
        }



        // take vxrd, deposits to loaned vxrd vault,
        // take cooresponding amount of xrd from xrd_vault and send to degenfi for collateral
        pub fn deposit_loan(&mut self, mut loan: Bucket) {
            
            let loan_amount = loan.amount();
            let lr_resource_manager = borrow_resource_manager!(self.loan_receipt);
            self.loaned_vxrd.put(loan);
            let receipt = self.vxrd_mint_badge
                .authorize(|| lr_resource_manager.mint(loan_amount));
            

            //takes user payment from vault and deposits the xrd into degenfi's loan vault
            FakeLoanPlatform.fake_loan_vault.put(self.collected_xrd.take(loan_amount)) //how do I take this payment and deposit it to degenfi?
        }
        
        pub fn collect_loan_rewards(&mut self) -> Bucket {
            FakeLoanPlatform.fake_loan_rewards_vault.take_all(self.collected_xrd.take(loan_amount)) //how do I take this payment and deposit it to degenfi?

            // figure out logic to collect pending rewards, or are they all due on withdrawal?
        }
        
        pub fn withdraw_loan(&mut self, mut loan: Bucket) -> Bucket {

            // take loan from fake loan platform
            let loan_amount = loan.amount();
            FakeLoanPlatform.fake_loan_vault.take(fake_loan_vault.take(loan_amount));

            // burn loan receipt tokens
            let lr_resource_manager = borrow_resource_manager!(self.loan_receipt);
            self.vxrd_mint_badge
                .authorize(|| lr_resource_manager.burn(loan_amount));

            // take same amount of vxrd from loaned_vxrd vault and give to user
            self.loaned_vxrd.take(loan_amount)
        }
        
        pub fn deposit_lp(&mut self, mut lp_tokens: Bucket) -> Bucket {

            return lp_receipt
        }    


        pub fn withdraw_lp(&mut self, mut lp_receipt: Bucket) -> Bucket {

            return lp_tokens
        } 


        pub fn deposit_lp(&mut self, mut stake_tokens: Bucket) -> Bucket {

            // hold lp tokens to prove liquidity

            return stake_receipt
        }    


        pub fn withdraw_lp(&mut self, mut stake_receipt: Bucket) -> Bucket {

            return stake_tokens
        } 

        // takes entire xrd_fees_collected vault and distributes xrd to vxrd holders, vxrd loans, 
        pub fn distribute_rewards(&mut self) -> (Bucket, Bucket, Bucket) {
            
            // For loop iterating over each epoch for reward distribution

            
            let vxrd_resource_manager = borrow_resource_manager!(self.vxrd_address);
            let total_vxrd_supply = vxrd_resource_manager.total_supply(); // total vxrd minted
            let total_rewards = xrd_fees_collected.amount();


            let hodler_rewards =  // total_rewards * (# of vxrd hodled / total_vxrd_supply) 


            let lending_fee_rewards = // rewardsfrom fees for using vxrd to make your loan
            //reward must be per epoch since individual contributers of loans are not tracked
            //small fee?

            let lender_rewards = // rewards generated from loaning to a lending platform



            let LP_farmers_rewards = // total rewards * (amount of vxrd in lp/ total_vxrd_supply) 


            let LP_buyback_fund = self.
                // take half of xrd, buy vxrd from radiswap
                // pair xrd-vxrd
                // put LP tokens in vxrd_xrd_lp_token vault
        }
        
        pub fn change_distro_rules(&mut self)
            //change lp_reward_ratio,
            // hodl_reward_ratio,
            // lendor_reward_ratio,
            // stake_reward_ratio,
        }

        
    }
}