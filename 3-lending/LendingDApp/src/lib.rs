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
    l2: bool    
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
    l2: bool    
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

            // Instantiate the loan badge 
            // let loan_admin_badge = ResourceBuilder::new_fungible()
            //     .divisibility(DIVISIBILITY_NONE)
            //     .metadata("name", "Loan Token Auth")
            //     .initial_supply(1);
            // Create the loan admin badge. This will be store on the component's vault 
            // and will allow it to do some actions on the user NFTs
            let loan_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
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

            // Return the new LendingApp component, as well as the initial supply of LP tokens
            lendingapp.globalize()
        }

        // Allow someone to register 
        pub fn register(&self) -> Bucket {
            let epoch = Runtime::current_epoch();    
            let mut hasher = Sha256::new();
            hasher.update(epoch.to_string());
            let epoch_hash = hasher.finalize();
            let lend_id = NonFungibleId::from_bytes(epoch_hash.to_vec());                    

            // Create a lending NFT. Note that this contains the number of lending and the level arwarded
            let lending_nft = self.loan_admin_badge.authorize(|| {
                borrow_resource_manager!(self.lending_nft_resource_def)
                    .mint_non_fungible(&lend_id, LendingTicket {number_of_lendings: 0, l1: false, l2: false })
                }); 

            // Return the NFT
            lending_nft
        }
        

        /// Adds liquidity to this pool and return the Loan tokens representing pool shares
        /// along with any remainder.
        pub fn lend_money(&mut self, xrd_tokens: Bucket, ticket: Proof) -> (Bucket, Bucket) {
            // The ratio of added liquidity in existing liquidty.
            let ratio = xrd_tokens.amount() / self.main_pool.amount();
            info!("Actual ratio is: {}", ratio);
            let how = xrd_tokens.amount();
            self.main_pool.put(xrd_tokens);

            let value_backed = self.loan_pool.take(how + self.reward);

            // Get the data associated with the ticket NFT and update the "used" state
            let non_fungible: NonFungible<LendingTicket> = ticket.non_fungible();
            let mut ticket_data = non_fungible.data();
            //assert!(!ticket_data.used, "You already used this ticket!");
            info!("NFT size is: {}", ticket_data.number_of_lendings);
            info!("NFT l1 : {}", ticket_data.l1);
            info!("NFT l2 : {}", ticket_data.l2);
            let number_of_lendings = 1 + ticket_data.number_of_lendings;
            let l1 = ticket_data.l1;
            let l2 = ticket_data.l2;
            let mut hasher = Sha256::new();
            hasher.update(ratio.to_string());
            let ratio_hash = hasher.finalize();    
            let lend_id = NonFungibleId::from_bytes(ratio_hash.to_vec());                    

            // Create a lending ticket NFT. Note that this contains the number of lending and the level arwarded
            let lending_nft = self.loan_admin_badge.authorize(|| {
                borrow_resource_manager!(self.lending_nft_resource_def)
                    .mint_non_fungible(&lend_id, LendingTicket {number_of_lendings, l1, l2 })
            });            

            // Return the tokens along with NFT
            (value_backed, lending_nft)
        }

        /// Adds liquidity to this pool and return the Loan tokens representing pool shares
        /// along with any remainder.
        pub fn take_money_back(&mut self, lnd_tokens: Bucket) -> Bucket {
            // The amount of token to be repaid back
            let how = lnd_tokens.amount();
            // plus reward
            let value_backed = self.main_pool.take(how);
            //lnd token to put back in the pool
            self.loan_pool.put(lnd_tokens);

            let ratio = how / self.main_pool.amount();
            info!("Actual ratio is: {}", ratio);

            value_backed
        }        
     
    }
}