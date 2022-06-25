use scrypto::prelude::*;
use sha2::{Digest, Sha256};

// Here, we define the data that will be present in
// each of the ticket NFTs. Note that only the "used" data will
// be updateable
#[derive(NonFungibleData)]
struct LendingTicket {
    #[scrypto(mutable)]
    number_of_lendings: u16,
    #[scrypto(mutable)]   
    l1: bool,
    #[scrypto(mutable)]   
    l2: bool,
    #[scrypto(mutable)]   
    in_progress: bool          
}

// Here, we define the data that will be present in
// each of the ticket NFTs. Note that only the "used" data will
// be updateable
#[derive(NonFungibleData)]
struct BorrowingTicket {
    #[scrypto(mutable)]
    number_of_borrowings: u16,
    #[scrypto(mutable)]   
    l1: bool,
    #[scrypto(mutable)]   
    l2: bool,
    #[scrypto(mutable)]   
    in_progress: bool          
}

blueprint! {
    struct LendingApp {
        /// The resource definition of LOAN token.
        loan_resource_def: ResourceAddress,
        /// The resource definition of LENDING_NFT token.
        lending_nft_resource_def: ResourceAddress,
        /// The resource definition of BORROWING_NFT token.
        borrowing_nft_resource_def: ResourceAddress,                
        /// LOAN tokens mint badge.
        loan_admin_badge: Vault,       
        /// LOAN tokens Vault.
        loan_pool: Vault,          

        /// The reserve for main pool
        main_pool: Vault,

        ///loans along time
        //let mut loan_allocated = Vec::new();
        loan_allocated: Vec<Decimal>,
        /// The starting amount of tokec accepted
        start_amount: Decimal,
        /// The fee to apply for every loan
        fee: Decimal,
        /// The reward to apply for every loan
        reward: Decimal
    }


    ///resim call-function $radiswap_package Radiswap instantiate_pool 100,$btc 3,$gumball 100 0.01)
    impl LendingApp {
        /// Creates a LendingApp component for token pair A/B and returns the component address
        /// along with the initial LP tokens.
        pub fn instantiate_pool(
            starting_tokens: Bucket,
            start_amount: Decimal,
            fee: Decimal,
            reward: Decimal,
        ) -> ComponentAddress {
            info!("My start amount is: {}", start_amount);
            info!("My fee for borrower is: {}", fee);
            info!("My reward for lenders is: {}", reward);
            // Check arguments 
            assert!(
                fee > Decimal::one(),
                "Invalid fee "
            );
            assert!(
                start_amount > Decimal::zero(),
                "Start with at least one!"
            );   

            // Create the loan admin badge. This will be store on the component's vault 
            // and will allow it to do some actions on the user NFTs
            let loan_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Loan Token Auth")
                .initial_supply(1);    
                
            // Create the non fungible resource that will represent the lendings
            let lending_nft: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Lending NFTs")
                .mintable(rule!(require(loan_admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(loan_admin_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(loan_admin_badge.resource_address()))))
                .no_initial_supply();                

            // Create the non fungible resource that will represent the borrowings
            let borrowing_nft: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Borrowing NFTs")
                .mintable(rule!(require(loan_admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(loan_admin_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), MUTABLE(rule!(require(loan_admin_badge.resource_address()))))
                .no_initial_supply();                 

            let loan_tokens = ResourceBuilder::new_fungible()
            .metadata("symbol", "LND")
            .metadata("name", "Loan token")
            .metadata("url", "https://lendingapp.com")
            .mintable(rule!(require(loan_admin_badge.resource_address())), LOCKED)
            .burnable(rule!(require(loan_admin_badge.resource_address())), LOCKED)
            .initial_supply(start_amount);

            let loan_allocated = Vec::new();

            info!("Loan pool size is: {}", start_amount);
            info!("Main pool size is: {}", starting_tokens.amount());

            // Instantiate our LendingApp component
            let lendingapp = Self {
                loan_resource_def: loan_tokens.resource_address(),
                lending_nft_resource_def: lending_nft,
                borrowing_nft_resource_def: borrowing_nft,
                loan_admin_badge: Vault::with_bucket(loan_admin_badge),
                loan_pool: Vault::with_bucket(loan_tokens),
                main_pool: Vault::with_bucket(starting_tokens),
                loan_allocated,
                start_amount,
                fee,
                reward,
            }
            .instantiate();
            //order of resources build is that of the order created
            //Admin Badge, Lend NFT, Borrow NFT, LND token

            // Return the new LendingApp component, as well as the initial supply of LP tokens
            lendingapp.globalize()
        }

        // Allow someone to register its account
        pub fn register(&self) -> Bucket {
            let epoch = Runtime::current_epoch();    
            let mut hasher = Sha256::new();
            hasher.update(epoch.to_string());
            let epoch_hash = hasher.finalize();
            let lend_id = NonFungibleId::from_bytes(epoch_hash.to_vec());                    

            // Create a lending NFT. Note that this contains the number of lending and the level arwarded
            let lending_nft = self.loan_admin_badge.authorize(|| {
                borrow_resource_manager!(self.lending_nft_resource_def)
                    .mint_non_fungible(&lend_id, LendingTicket {number_of_lendings: 0, l1: false, l2: false, in_progress: false  })
                }); 

            // Return the NFT
            lending_nft
        }
        

        /// Lend XRD token to then pool and get back Loan tokens plus reward
        pub fn lend_money(&mut self, xrd_tokens: Bucket, ticket: Proof) -> Bucket {
            // The ratio of added liquidity.
            let percent: Decimal = dec!("100");
            let ratio = xrd_tokens.amount() * percent / self.main_pool.amount();
            info!("Actual ratio is: {}", ratio.floor());
            
            //check if lend is acceptable
            let min_ratio: Decimal = dec!("5");
            let max_ratio: Decimal = dec!("20");
            let min_level: Decimal = min_ratio * self.main_pool.amount() / percent;
            let max_level: Decimal = max_ratio * self.main_pool.amount() / percent;
            assert!(
                ratio > min_ratio,
                "Lend is below the minimum level, actual minimum is: {} Min tokens you can lend is {}", ratio.floor(), min_level.floor()
            );  
            assert!(
                ratio < max_ratio,
                "Lend is above the minimum level, actual maximum is: {} Max tokens you can lend is {}", ratio.floor(), max_level.floor()
            );               
            //put xrd token in main pool
            let num_xrds = xrd_tokens.amount();
            self.main_pool.put(xrd_tokens);
            //give back lnd token plus reward %
            let value_backed = self.loan_pool.take(num_xrds + (num_xrds*self.reward/100));

            // Get the data associated with the Lending NFT and update the variable values
            let non_fungible: NonFungible<LendingTicket> = ticket.non_fungible();
            let mut lending_nft_data = non_fungible.data();
            //check if no operation is already in place            
            assert!(!lending_nft_data.in_progress, "You already have a lend open!");
            info!("NFT size is: {} L1 : {} L2 : {}", lending_nft_data.number_of_lendings, lending_nft_data.l1, lending_nft_data.l2);
            let number_of_lendings = 1 + lending_nft_data.number_of_lendings;
            lending_nft_data.number_of_lendings = number_of_lendings;
            info!("New NFT size is: {}", lending_nft_data.number_of_lendings);
            if number_of_lendings > 10 {
                lending_nft_data.l1 = true;
                println!("L1 reached !");
            } else if number_of_lendings > 20 {
                lending_nft_data.l2 = true;
                println!("L2 reached !");
            }
            let mut hasher = Sha256::new();
            hasher.update(ratio.to_string());            

            // Update the data on that NFT globally
            lending_nft_data.in_progress = true;
            self.loan_admin_badge.authorize(|| {
                borrow_resource_manager!(self.lending_nft_resource_def).update_non_fungible_data(&non_fungible.id(), lending_nft_data);
                info!("Updates Lender NFT !");
            });

            // Return the tokens along with NFT
            value_backed
        }

        /// Gives money back to the lenders adding their reward
        pub fn take_money_back(&mut self, lnd_tokens: Bucket, ticket: Proof) -> Bucket {
            // Get the data associated with the Lending NFT and update the variable values (in_progress=false)
            let non_fungible: NonFungible<LendingTicket> = ticket.non_fungible();
            let mut lending_nft_data = non_fungible.data();
            //check if no operation is already in place            
            assert!(lending_nft_data.in_progress, "You have not a lend open!");

            // The amount of $xrd token to be repaid back (reward included)
            let how_many_to_give_back = lnd_tokens.amount();
            info!("Gettin from main pool xrd tokens size: {}", how_many_to_give_back);
            //take $xrd from main pool
            let xrds_to_give_back = self.main_pool.take(how_many_to_give_back);

            let percent: Decimal = dec!("100");
            let amount = how_many_to_give_back*percent/(percent+self.reward);
            let lnd_to_be_burned = how_many_to_give_back - amount;
            //lnd token to put back in the pool
            info!("Putting back into loan pool lnd tokens size: {} then burning the reward because not needed anymore {} ", amount, lnd_to_be_burned);
            self.loan_pool.put(lnd_tokens);
            //burn the reward
            self.loan_admin_badge.authorize(|| {
                self.loan_pool.take(lnd_to_be_burned).burn();
            }); 

            info!("Loan pool size is: {}", self.main_pool.amount());
            info!("Current pool size is: {}", self.main_pool.amount());
   
            lending_nft_data.in_progress = false;
            // Update the data on that NFT globally         
            self.loan_admin_badge.authorize(|| {
                borrow_resource_manager!(self.lending_nft_resource_def).update_non_fungible_data(&non_fungible.id(), lending_nft_data)
            });            
            info!("Process complete !");

            xrds_to_give_back
        }        
     
    }
}