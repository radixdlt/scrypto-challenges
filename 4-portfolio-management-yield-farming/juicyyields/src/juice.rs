// Author: NellySayon
// Blueprint: Juice
// Purpose: creates the Juice Token and delivers methods to handle it
// Furthermore contains the lottery function as well as the reward calculation and handling

use scrypto::prelude::*;

blueprint! {

    struct Juice {
         juice_vault: Vault, // contains all created Juice Token and is used for incentives
         xrd_vault: Vault, // contains the xrd coming from the fees; is used for regular payouts
         owner_vault: Vault, // will receive it's share of the payouts
         lottery_vault: Vault, // contains the lottery pot
         lottery_tickets: Vec<ResourceAddress>, // here all bought tickets are stored
         lottery_winner: ResourceAddress,
         access_badge: ResourceAddress,
         reward_epoch: u64
    }

    impl Juice {   
        // main function to create the component 
        pub fn instantiate_juice() -> (ComponentAddress, Bucket) {
            // Create the admin badge
            let admin_badge = ResourceBuilder::new_fungible() 
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "Admin Badge")
            .initial_supply(1);

            // Define the access rules for the juice token vault
            let access_rules = AccessRules::new()
            .method("withdraw", rule!(require(admin_badge.resource_address()))) 
            .default(rule!(allow_all));

            // Create a new token called "JUICE"
            let juice_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Juice Token")
                .metadata("symbol", "JUICE")
                .initial_supply(1000000000);
   
            // Create self and return the component address and the admin badge
            (Self {
                juice_vault: Vault::with_bucket(juice_bucket),
                xrd_vault: Vault::new(RADIX_TOKEN),
                owner_vault: Vault::new(RADIX_TOKEN),
                lottery_vault: Vault::new(RADIX_TOKEN),
                lottery_tickets: Vec::new(),
                lottery_winner: admin_badge.resource_address(), // just a default
                access_badge: admin_badge.resource_address(),
                reward_epoch: Runtime::current_epoch()
            }
            .instantiate()
            .add_access_check(access_rules) 
            .globalize(),
            admin_badge)
        }            
        
        // -----------------------------------------------------------------------------
        // normal withdraw function for Juice token, protected by the admin badge
        // -----------------------------------------------------------------------------
        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
            self.juice_vault.take(amount)
        }
    
        // -----------------------------------------------------------------------------
        // gets the XRD fee and returns Juice incentive, protected by admin badge
        // -----------------------------------------------------------------------------
        pub fn get_fee (&mut self, xrd: Bucket) -> Bucket {
            let incentive = xrd.amount() / 5;
            
            self.xrd_vault.put(xrd);

            info!("You receive: {:?} $JUICE. Thank you for choosing Juicy Yields.", incentive);
            self.withdraw(incentive)
        }

        // -----------------------------------------------------------------------------
        //  stores a new lottery ticket 
        // -----------------------------------------------------------------------------
        pub fn receive_lottery_ticket (&mut self, xrd: Bucket, ticket: ResourceAddress){
            self.lottery_vault.put(xrd);
            self.lottery_tickets.push(ticket);
            info!("Lottery tickets sold so far: {:?} ", self.lottery_tickets.len());
        }

        // -----------------------------------------------------------------------------
        //  return a random number between 0 and given max value
        // -----------------------------------------------------------------------------
        fn get_random(&self, max: usize) -> usize {
            let num = Runtime::generate_uuid();
            (num % max as u128) as usize
        }

        // -----------------------------------------------------------------------------
        //  run the lottery and select a winner 
        // -----------------------------------------------------------------------------
        pub fn run_lottery (&mut self) {
            // check if there are actually lottery tickets sold
            assert!(self.lottery_tickets.len() > 0, "No lottery tickets sold so far.");

            info!("Lottery performed with {:?} tickets", self.lottery_tickets.len());
            
            // select a random winner            
            let random_winner = self.get_random(self.lottery_tickets.len());
         
            // set the winner address
            self.lottery_winner = self.lottery_tickets[random_winner];

            info!("Lottery winner: {:?}. Congratulations!", self.lottery_winner);
            
            // reset the lottery_tickets
            self.lottery_tickets = Vec::new();
        }

        // -----------------------------------------------------------------------------
        //  pay out the lottery win
        // ----------------------------------------------------------------------------- 
        pub fn withdraw_lottery (&mut self, user_id: ResourceAddress) -> Bucket {
            assert!(self.lottery_winner == user_id, "You are not the winner");

            info!("Lottery payout: {:?} XRD to {:?}", self.lottery_vault.amount(), user_id);
            self.lottery_winner = self.access_badge; // set back to default again
            self.lottery_vault.take_all()
        }         
        

        // -----------------------------------------------------------------------------
        // temporarily gets a Juice Bucket to calculate the payout
        // returns the same Juice Bucket and additionally an xrd bucket
        // -----------------------------------------------------------------------------
        pub fn payout_xrd (&mut self, juice: Bucket) -> (Bucket, Bucket) {
            let mut payout_amount = dec!("0.0");

            // payouts are just allowed every 500 epochs
            if Runtime::current_epoch() >= (self.reward_epoch + 500){
                // check how many Juice Token the requestor has
                let juice_amount = juice.amount();
                // check how many Juice Token are given out in general
                let given_juice = dec!("1000000000") - self.juice_vault.amount();

                // calculate the share of juice token compared to all given tokens
                let share = juice_amount / given_juice;

                // how many XRD do we habe in our wallet?
                let xrd_amount = self.xrd_vault.amount();

                // so what do we pay out?
                payout_amount = share * xrd_amount / 2;

                // 50 % are always given to the owner
                self.owner_vault.put(self.xrd_vault.take(payout_amount));
                
                // save the new epoch    
                self.reward_epoch = Runtime::current_epoch();

                info!("Rewards payout: {:?} XRD for {:?} JUICE", payout_amount, juice_amount);
            }
            else{
                info!("Rewards payout allowed again in {:?} epochs", (self.reward_epoch + 500) - Runtime::current_epoch());
            }   
            // ensure we always return the juice bucket back to the owner 
            (juice, self.xrd_vault.take(payout_amount))
        }        
    }
}



