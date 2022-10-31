use scrypto::prelude::*;
use crate::oracle1::*;
use crate::oci_pools::*;

blueprint! {
    struct Ociswap {

        //Hashmap to store resource address to token vaults
        oci_pool:HashMap<ResourceAddress, OciPool>,

        //This stores the ocracle component address
        oracle1_address:Option<ComponentAddress>,
    }

    impl Ociswap {
        
        pub fn new() -> ComponentAddress{
            let component = Self {
                oci_pool:HashMap::new(),
                oracle1_address:None,
                }
                .instantiate()
                .globalize();
                return component;
        }
        
        //Create a pool of tokens which will be used to trade with on Ociswap
        pub fn create_pool(&mut self, token_address:ResourceAddress) {
            //Uses the oci pool blueprint to create an ociswap pool component
            let new_oci_pool:ComponentAddress = OciPool::new(token_address);

            //Update hashmap with resource address and oci pool component
            self.oci_pool.insert(token_address, new_oci_pool.into());
        }

        //Add tokens to the created pools
        pub fn add_liquidity(&mut self, token:Bucket) {

            //Get the tokens resource address
            let token_address = token.resource_address();

            //Get the component from the oci pool hashmap using the tokens resource address
            let token_address:&OciPool = self.oci_pool.get(&token_address).unwrap();

            //Deposit tokens into component's vault 
            token_address.deposit(token);
        }

        //Remove tokens from liquidity pools
        pub fn remove_liquidity(&mut self, token_amount:Decimal, token_address:ResourceAddress)->Bucket {

            //Get the component from the oci pool hashmap using the tokens resource address
            let token_component:&OciPool = self.oci_pool.get(&token_address).unwrap();

            //Withdraw tokens from component's vault 
            let return_bucket:Bucket = token_component.withdraw(token_amount);

            return return_bucket;
        }



        //Swap tokens by providing a bucket of tokens and the resource address wanted token 
        pub fn swap(&mut self, token1:Bucket, token2_address:ResourceAddress ) -> Bucket{

            //Determine how many tokens are in the bucket
            let token1_amount:Decimal = token1.amount();

            //Determing the resource address
            let token1_address:ResourceAddress = token1.resource_address();

            //Get the componet from the oci pool hashmap using the resource address of the deposited token
            let oci_pool_address1:&OciPool = self.oci_pool.get(&token1_address).unwrap();

            //Deposit the token into the oci pool component
            oci_pool_address1.deposit(token1);

            //Get the component from the oci pool hashmap using the resource address of the wanted token
            let oci_pool_address2:&OciPool = self.oci_pool.get(&token2_address).unwrap();

            //Get the Oracle component
            let oracle1_component_address:ComponentAddress  = self.oracle1_address.unwrap();
            let oracle1_compoent:Oracle1 = oracle1_component_address.into();

            //Get the price of deposited token
            let oracle1_price1:Decimal = oracle1_compoent.get_price(token1_address).into();
            info!("[NOTE] The price of {:?} is {} USD",borrow_resource_manager!(token1_address).metadata(), oracle1_price1);

            //Get the price of the wanted token
            let oracle_price2:Decimal = oracle1_compoent.get_price(token2_address.into());
            info!("[NOTE] The price of {:?} is {} USD",borrow_resource_manager!(token2_address).metadata(), oracle_price2);

            //Calculate the amount of the wanted token based on the price of the tokens
            let return_amount:Decimal = (token1_amount * oracle1_price1) / oracle_price2;
            info!("[CALC] {} tokens X ${} per token = ${}",token1_amount, oracle1_price1, (token1_amount * oracle1_price1));
            info!("[CALC] ${} / ${} per token = {} tokens",(token1_amount * oracle1_price1), oracle_price2, return_amount);
            info!("[FINAL] {} {:?} has been deposited",return_amount, borrow_resource_manager!(token2_address).metadata());
            info!("------------------------------------------------------------------------------------");

            //Withdraw wanted token from the Ociswap pool component
            let return_bucket:Bucket = oci_pool_address2.withdraw(return_amount);

            //Returned swapped token to user
            return return_bucket;
        }

        //Assign Oracle component address
        pub fn oracle1_address(&mut self, oracle1_address:ComponentAddress){
            self.oracle1_address = Some(oracle1_address);
            info!("{}", self.oracle1_address.unwrap());
        }

        //This method shows the balance of the liquidity pools
        pub fn show_liquidity_pool(&self){
            info!("LIQUIDITY POOL BALANCES");
            for (resource_address, component_address) in &self.oci_pool {
                let oci_pool_component:&OciPool = component_address;
                let balance = oci_pool_component.balance();
                info!("TOKEN:[{:?}] *** TOKEN BALANCE: {} ",borrow_resource_manager!(*resource_address).metadata(), balance);
            }
        }
    }
}

        

