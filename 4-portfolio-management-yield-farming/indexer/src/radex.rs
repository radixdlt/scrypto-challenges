use scrypto::prelude::*;
use crate::oracle2::*;
use crate::radex_pools::*;

blueprint! {
    struct Radex {

        //Hashmap to store resource address to token vaults
        radex_pool:HashMap<ResourceAddress, RadexPool>,

        //This stores the ocracle component address
        oracle2_address:Option<ComponentAddress>,
    }

    impl Radex {
        
        pub fn new() -> ComponentAddress{
            let component = Self {
                radex_pool:HashMap::new(),
                oracle2_address:None,
                }
                .instantiate()
                .globalize();
                return component;
        }
        
        //Create a pool of tokens which will be used to trade with on Radex
        pub fn create_pool(&mut self, token_address:ResourceAddress) {
            //Uses the Radex pool blueprint to create an ociswap pool component
            let new_radex_pool:ComponentAddress = RadexPool::new(token_address);

            //Update hashmap with resource address and oci pool component
            self.radex_pool.insert(token_address, new_radex_pool.into());
        }

        //Add tokens to the created pools
        pub fn add_liquidity(&mut self, token:Bucket) {

            //Get the tokens resource address
            let token_address = token.resource_address();

            //Get the component from the Radex pool hashmap using the tokens resource address
            let token_address:&RadexPool = self.radex_pool.get(&token_address).unwrap();

            //Deposit tokens into component's vault 
            token_address.deposit(token);
        }

        //Swap tokens by providing a bucket of tokens and the resource address wanted token 
        pub fn swap(&mut self, token1:Bucket, token2_address:ResourceAddress ) -> Bucket{

            //Determine how many tokens are in the bucket
            let token1_amount:Decimal = token1.amount();

            //Determing the resource address
            let token1_address:ResourceAddress = token1.resource_address();

            //Get the componet from the oci pool hashmap using the resource address of the deposited token
            let radex_pool_address1:&RadexPool = self.radex_pool.get(&token1_address).unwrap();

            //Deposit the token into the oci pool component
            radex_pool_address1.deposit(token1);

            //Get the component from the oci pool hashmap using the resource address of the wanted token
            let radex_pool_address2:&RadexPool = self.radex_pool.get(&token2_address).unwrap();

            //Get the Oracle component
            let oracle2_component_address:ComponentAddress  = self.oracle2_address.unwrap();
            let oracle2_component:Oracle2 = oracle2_component_address.into();

            //Get the price of deposited token
            let oracle_price1:Decimal = oracle2_component.get_price(token1_address).into();
            info!("[NOTE] The price of {:?} is {} USD",borrow_resource_manager!(token1_address).metadata(), oracle_price1);

            //Get the price of the wanted token
            let oracle2_price2:Decimal = oracle2_component.get_price(token2_address.into());
            info!("[NOTE] The price of {:?} is {} USD",borrow_resource_manager!(token2_address).metadata(), oracle2_price2);

            //Calculate the amount of the wanted token based on the price of the tokens
            let return_amount:Decimal = (token1_amount * oracle_price1) / oracle2_price2;
            info!("[CALC] {} tokens X ${} per token = ${}",token1_amount, oracle_price1, (token1_amount * oracle_price1));
            info!("[CALC] ${} / ${} per token = {} tokens",(token1_amount * oracle_price1), oracle2_price2, return_amount);
            info!("[FINAL] {} {:?} has been deposited",return_amount, borrow_resource_manager!(token2_address).metadata());
            info!("------------------------------------------------------------------------------------");

            //Withdraw wanted token from the Ociswap pool component
            let return_bucket:Bucket = radex_pool_address2.withdraw(return_amount);

            //Returned swapped token to user
            return return_bucket;
        }

        //Assign Oracle component address
        pub fn oracle2_address(&mut self, oracle2_address:ComponentAddress){
            self.oracle2_address = Some(oracle2_address);
            info!("{}", self.oracle2_address.unwrap());
        }

        //This method shows the balance of the liquidity pools
        pub fn show_liquidity_pool(&self){
            info!("LIQUIDITY POOL BALANCES");
            for (resource_address, component_address) in &self.radex_pool {
                let oci_pool_component:&RadexPool = component_address;
                let balance = oci_pool_component.balance();
                info!("TOKEN:[{:?}] *** TOKEN BALANCE: {} ",borrow_resource_manager!(*resource_address).metadata(), balance);
            }
        }
    }
}

        

