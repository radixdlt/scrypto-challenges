use scrypto::prelude::*;
use crate::oracle1::*;
use crate::oracle2::*;
use crate::index_pool::*;
use crate::ociswap::*;
use crate::radex::*;

#[derive(NonFungibleData)]
pub struct LoanDue {
    pub amount_due: Decimal,
}

blueprint! {
    struct Indexer {

        //Vault where the Indexer admin badge is held
        admin_vault:Vault,

        //This hashmap maps the resource address to a index pool. The index pool is a component consisting of a single vault.
        index_pool:HashMap<ResourceAddress, ComponentAddress>,

        //The indexer token is minted 1:1 based on the amount of XRD tokens place in the XRD index pool.
        //The indexer token helps tracks the % ownership of the index fund.  
        indexer_token:ResourceAddress,

        //Index token counter is a cumulative counter of minted indexer tokens.
        //The counter is used to determine % ownership of the index fund.
        index_token_counter:Decimal,

        //This stores the Ociswap component address
        ociswap_address:Option<ComponentAddress>,

        //This stores the Ociswap component address
        radex_address:Option<ComponentAddress>,

        //This stores the Oracle1 component address
        oracle1_address:Option<ComponentAddress>,

        //This stores the Oracle2 component address
        oracle2_address:Option<ComponentAddress>,

        //transient token used for flash loans
        transient_resource_address: ResourceAddress,
    }

    impl Indexer {
        
        pub fn new() -> ComponentAddress{
            //This is the component admin badge that will be needed to mint and burn indexer tokens
            let indexer_admin:Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "Indexer Admin")
            .initial_supply(1);

            //Indexer tokens are minted and burned and used to represent ownership of indexer fund
            let indexer_token:ResourceAddress = ResourceBuilder::new_fungible()
            .metadata("name", "Indexer Token")
            .mintable(rule!(require(indexer_admin.resource_address())), LOCKED)
            .burnable(rule!(require(indexer_admin.resource_address())), LOCKED)
            .no_initial_supply();

            let transient_resource_address:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata(
                    "name",
                    "Promise token for BasicFlashLoan - must be returned to be burned!",
                )
                .mintable(rule!(require(indexer_admin.resource_address())), LOCKED)
                .burnable(rule!(require(indexer_admin.resource_address())), LOCKED)
                .restrict_deposit(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let component = Self {
                admin_vault:Vault::with_bucket(indexer_admin),
                index_pool:HashMap::new(),
                indexer_token:indexer_token,
                index_token_counter:dec!(0),
                ociswap_address:None,
                radex_address:None,
                oracle1_address:None,
                oracle2_address:None,
                transient_resource_address:transient_resource_address,
                }
                .instantiate()
                .globalize();
                return component;
        }

        //Method is used to create the individual index pool that make up the Indexer fund
        pub fn create_index_pool(&mut self, resource_address:ResourceAddress) {
            //Use the index pool blueprint to create an index pool component
            let new_index_pool:ComponentAddress = IndexPool::new(resource_address);
            //Update hashmap with resource address and index pool component
            self.index_pool.insert(resource_address, new_index_pool);
        }

        //Method used to deposit XRD which is evenly diveded up based on the number of
        //index pools making up the Indexer fund.  Each divided amount, excluding the XRD index pool
        //is swapped through Ociswap into each token that makes up the indexer fund and stores them in
        //their index pools.
        pub fn deposit(&mut self, mut deposit_amount:Bucket) -> (Bucket, Bucket){

            let bucket_amount = deposit_amount.amount();

            info!("[NOTE] You have deposited {} XRD", bucket_amount);

            //find the legnth of the hashmap, which is the number of index pool in Indexer fund
            let hash_length:usize = self.index_pool.len();
            info!("[NOTE] There are {} index pools in the index fund", hash_length);

            //find the amount to deposit in each index pool = deposit_amount / hash_length
            let amount: Decimal = deposit_amount.amount() / hash_length;
            info!("[NOTE] {} XRD will be deposited into each index pool", amount);
            info!("------------------------------------------------------------------------------------");

            //Mint indexer tokens - Amount based on individual pool sizes determined above
            let indexer_token = self.admin_vault.authorize(|| {
                borrow_resource_manager!(self.indexer_token)
                    .mint(amount)
            });

            //Increment index token counter

            self.index_token_counter += amount;

            //Iterate through the index pool hashmap and deposit funds
            for (resource_address, component_address) in &self.index_pool {

                //If the resource == XRD put it in the XRD index pool
                if resource_address.to_string() == "030000000000000000000000000000000000000000000000000004" {
                    let bucket = deposit_amount.take(amount);
                    let index_component_address:ComponentAddress  = *component_address;
                    let index_component:IndexPool = index_component_address.into();
                    info!("[FINAL] {} XRD has been deposited", amount);
                    info!("------------------------------------------------------------------------------------");
                    
                    //Deposit into index pool
                    index_component.deposit(bucket);

                //If the resource != XRD, swap XRD to the correct resource, and deposit into index pool 
                } else {
                    let bucket = deposit_amount.take(amount);

                    //swap xrd to index pool designated resource
                    let new_bucket = self.swap(bucket, *resource_address);

                    //Deposit swapped asset into index pool
                    let index_component_address:ComponentAddress  = *component_address;
                    let index_component:IndexPool = index_component_address.into();
                    index_component.deposit(new_bucket);
                }
            }
            //return any change and index tracking tokens
            return (deposit_amount, indexer_token);        
        }

        //This method takes the indexer tracker tokens and returns a percentage of each of the
        //index pools.  The % is calculated -> # of indexer_tokens/indexer_token_counter
        //This ensure all fees collected are evenly distributed back to the user
        //Each index pool except the XRD pool is swapped back to XRD and returned to the user
        pub fn withdraw(&mut self, indexer_token:Bucket) -> Bucket {

            //Assert Indexer tracker tokens are in the Bucket
            assert!(indexer_token.resource_address() == self.indexer_token, "[ERROR] Incorret resource address");

            //Create a temporary bucket for collecting XRD within the for loop
            let mut temp_bucket:Bucket = Bucket::new(RADIX_TOKEN); 
            
            //Determine how many Indexer tracker tokens are in the bucket
            let amount = indexer_token.amount();

            //Burn the Indexer tracker tokens in the bucket
            self.admin_vault.authorize(|| {
                borrow_resource_manager!(self.indexer_token)
                    .burn(indexer_token)
            });

            //Calculate index pool ownership percentage based on the number of 
            //Indexer tracker tokens and total outstanding Indexer tracker tokens

            let percent_take = amount/self.index_token_counter;
            info!("You will recieve {}% of each index pool in the Indexer Fund", percent_take*100);

            //Increment the Indexer token counter
            self.index_token_counter -= amount;

            //Iterate through the index pool hashmap and withdraw funds
            for (resource_address, component_address) in &self.index_pool {

                //If the resource == XRD take from XRD index pool
                if resource_address.to_string() == "030000000000000000000000000000000000000000000000000004" {
                    let index_component_address:ComponentAddress  = *component_address;
                    let index_component:IndexPool = index_component_address.into();

                    //Calulate the amount of XRD in the XRD pool
                    let balance:Decimal = index_component.balance();

                    //Calulate the amount to take out
                    let take_amount:Decimal = balance * percent_take;

                    //Take calculated amount from XRD index pool
                    let return_bucket: Bucket = index_component.withdraw(take_amount);
                    info!("Withdrawing {} XRD from XRD Index Pool", take_amount);

                    //Put XRD in temporary bucket
                    temp_bucket.put(return_bucket);

                } else {

                    //If the resource != XRD, swap index tokens to XRD, and deposit into temporay bucket 
                    let index_component_address:ComponentAddress  = *component_address;
                    let index_component:IndexPool = index_component_address.into();

                    //Calulate the amount of tokens in the index pool
                    let balance:Decimal = index_component.balance();

                    //Calulate the amount to take out
                    let take_amount:Decimal = balance * percent_take;

                    //Take calculated amount from index pool
                    let return_bucket: Bucket = index_component.withdraw(take_amount);
                    info!("Withdrawing {} {:?} tokens from index pool", take_amount, borrow_resource_manager!(*resource_address).metadata());

                    //swap tokens for XRD
                    let ociswap_component_address: ComponentAddress  = self.ociswap_address.unwrap();
                    let ociswap_component:Ociswap = ociswap_component_address.into();
                    let xrd_bucket = ociswap_component.swap(return_bucket,RADIX_TOKEN);

                    //Put XRD in temporary bucket
                    temp_bucket.put(xrd_bucket);
                }  
            };

            //return XRD to user
            return temp_bucket;
        }

        //Assign Ociswap component address
        pub fn oci_address(&mut self, oci_address:ComponentAddress){
            self.ociswap_address = Some(oci_address);
            info!("{}", self.ociswap_address.unwrap());
        }

        //Assign Radex component address
        pub fn radex_address(&mut self, radex_address:ComponentAddress){
            self.radex_address = Some(radex_address);
            info!("{}", self.radex_address.unwrap());
        }

        //Assign Oracle1 component address
        pub fn oracle1_address(&mut self, oracle1_address:ComponentAddress){
            self.oracle1_address = Some(oracle1_address);
            info!("{}", self.oracle1_address.unwrap());
        }

        //Assign Oracle2 component address
        pub fn oracle2_address(&mut self, oracle2_address:ComponentAddress){
            self.oracle2_address = Some(oracle2_address);
            info!("{}", self.oracle2_address.unwrap());
        }

        //This method gets the price of a resource from the oracle1
        pub fn get_price_oracle1(&self, token_address: ResourceAddress) ->Decimal {
            let oracle_component_address:ComponentAddress  = self.oracle1_address.unwrap();
            let oracle_component:Oracle1 = oracle_component_address.into();
            let token_price:Decimal = oracle_component.get_price(token_address);
            info!("The price of {} is {}", token_address, token_price);
            return token_price;
        }

        //This method gets the price of a resource from the oracle2
        pub fn get_price_oracle2(&self, token_address: ResourceAddress)->Decimal {
            let oracle_component_address:ComponentAddress  = self.oracle2_address.unwrap();
            let oracle_component:Oracle2 = oracle_component_address.into();
            let token_price = oracle_component.get_price(token_address);
            info!("The price of {} is {}", token_address, token_price);
            return token_price;
        }
        
        //This method performs a swap using the ociswap component
        pub fn swap(&self, token:Bucket, token_address:ResourceAddress) -> Bucket{
            let ociswap_component_address: ComponentAddress  = self.ociswap_address.unwrap();
            let ociswap_component:Ociswap = ociswap_component_address.into();
            return ociswap_component.swap(token, token_address);
        }

        //This method shows the balance of the index pools
        pub fn show_index_pool(&self){
            info!("INDEX POOL BALANCES");
            for (resource_address, component_address) in &self.index_pool {
                let index_component_address:ComponentAddress  = *component_address;
                let index_component:IndexPool = index_component_address.into();
                let balance = index_component.balance();
                info!("TOKEN:[{:?}] *** TOKEN BALANCE: {} ",borrow_resource_manager!(*resource_address).metadata(), balance);
            }
        }

        //This method shows the current value of the lp index token counter
        pub fn show_index_token_counter(&self) {
            info!("Index Token Counter = {}", self.index_token_counter);
        }

        //This method takes tokens from the index pool and performs an arbitrage between Ociswap and RaDEX
        pub fn arb_oci_radex(&self, token_address:ResourceAddress) {

            //Get the index pool component from the index pool hashmap
            let component_address = self.index_pool.get(&token_address);
            let index_component_address:ComponentAddress  = *component_address.unwrap();
            let index_component:IndexPool = index_component_address.into();
            let arb_bucket = index_component.take_all();
            
            info!("You have borrowed {} {:?} tokens from the index pool", arb_bucket.amount(), borrow_resource_manager!(token_address).metadata());
            info!("------------------------------------------------------------------------------------");

            let initial_investment = arb_bucket.amount();

            let ociswap_component_address: ComponentAddress  = self.ociswap_address.unwrap();
            let ociswap_component:Ociswap = ociswap_component_address.into();

            //Swap index pool tokens to XRD using Ociswap
            let oci_bucket = ociswap_component.swap(arb_bucket, RADIX_TOKEN);
            info!("You recieved {} XRD from Ociswap", oci_bucket.amount());

            //Get the Radex component
            let radex_component_address: ComponentAddress  = self.radex_address.unwrap();
            let radex_component:Radex = radex_component_address.into();

            //Swap the XRD from Ociswap for cheaper index pool token found on Radex
            let radex_bucket = radex_component.swap(oci_bucket, token_address);
            info!("You recieved {} {:?} from Radex", radex_bucket.amount(), borrow_resource_manager!(token_address).metadata());

            //Calculate arbitrage profit
            info!("You made a profit of {} {:?}", (radex_bucket.amount() - initial_investment), borrow_resource_manager!(token_address).metadata());

            //Return tokens to index pool
            info!("You have returned {} {:?} to the index pool",radex_bucket.amount(), borrow_resource_manager!(token_address).metadata());
            index_component.deposit(radex_bucket);

        }
        //FLASH LOAN utilizes the take_loan, repay_loan, and opportunity method
        pub fn take_loan(&mut self, loan_amount: Decimal, token_address:ResourceAddress) -> (Bucket, Bucket) {
            
            info!("[INFO] Enjoy your flash loan of {} {:?}!", loan_amount, borrow_resource_manager!(token_address).metadata());
            
            //check the index pool balance
            let component_address = self.index_pool.get(&token_address);
            let index_component_address:ComponentAddress  = *component_address.unwrap();
            let index_component:IndexPool = index_component_address.into();
            let loan_vault_balance = index_component.balance();

            //asset loan amount is < index pool balance
            assert!(
                loan_amount <= loan_vault_balance,
                "Not enough liquidity to supply this loan!"
            );

            //Calculate repayment amount
            let amount_due = loan_amount * dec!("1.04");
            info!("[INFO] Flash loan repayment amount = {} {:?}", amount_due, borrow_resource_manager!(token_address).metadata());

            //Mint transient token
            let loan_terms = self.admin_vault.authorize(|| {
                borrow_resource_manager!(self.transient_resource_address).mint_non_fungible(
                    &NonFungibleId::random(),
                    LoanDue {
                        amount_due: amount_due,
                    },
                )
            });

            //return loan and transient token
            return (index_component.withdraw(loan_amount), loan_terms);
        }

        pub fn repay_loan(&mut self, mut loan_repayment: Bucket, loan_terms: Bucket) -> Bucket{
            
            //asset transient token resource address
            assert!(
                loan_terms.resource_address() == self.transient_resource_address,
                "Incorrect resource passed in for loan terms"
            );

            //Get loan repayment amount
            let terms: LoanDue = loan_terms.non_fungible().data();

            //Assert repayment >= loan repayment amount
            assert!(
                loan_repayment.amount() >= terms.amount_due,
                "Insufficient repayment given for your loan!"
            );

            let token_address = loan_repayment.resource_address();
            let return_bucket = loan_repayment.take(terms.amount_due);

            let component_address = self.index_pool.get(&token_address);
            let index_component_address:ComponentAddress  = *component_address.unwrap();
            let index_component:IndexPool = index_component_address.into();
            
            info!("{} {:?} has been returned to the Index Pool",return_bucket.amount(), borrow_resource_manager!(token_address).metadata());
            //Put the repayment amount back into the index pool
            index_component.deposit(return_bucket);

            //Burn the transient token
            self.admin_vault.authorize(|| loan_terms.burn());

            return loan_repayment;
        }
        
        //This is a helper method to illustrate the functionality of the flash loan feature
        //This method take a few tokens from the OCI liquidity pools to cover loan fees
        pub fn opportunity(&self, token:Bucket) -> (Bucket,Bucket){
            
            //Get resource address
            let token_address:ResourceAddress = token.resource_address();

            //Get token amount
            let token_amount:Decimal = token.amount();

            //Get 10% of token amount
            let take_amount:Decimal = token_amount * dec!("0.1");

            //Get Ociswap component
            let ociswap_component_address: ComponentAddress  = self.ociswap_address.unwrap();
            let ociswap_component:Ociswap = ociswap_component_address.into();

            //Take 10% of token amount from specified Ociswap liquidity pool 
            let return_bucket:Bucket = ociswap_component.remove_liquidity(take_amount, token_address);

            //return orginal amount + 10% to user
            return (token, return_bucket);
        }   

    }
}
