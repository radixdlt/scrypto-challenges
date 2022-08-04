use scrypto::prelude::*;
use crate::price_oracle::*;

blueprint! {
    struct FundLocker {
        fund_vaults: HashMap<ResourceAddress, Vault>,
        token_weights: HashMap<ResourceAddress, Decimal>,
        fund_admin_vault: Vault,
        fund_token_address: ResourceAddress,
        starting_share_price: Decimal,
        price_oracle_address: ComponentAddress,
    }

    impl FundLocker {
        
        pub fn new(
            fund_name: String,
            fund_ticker: String,
            starting_share_price: Decimal,
            tokens: HashMap<ResourceAddress, Decimal>,
            price_oracle_address: ComponentAddress,
        ) -> ComponentAddress 
        {

            let fund_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("{} Admin Badge", fund_name))
                .metadata("symbol", "FAB")
                .metadata("description", "Component Admin authority")
                .initial_supply(1);

            let fund_token_address = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("{} Tokens", fund_name))
                .metadata("symbol", format!("{}", fund_ticker))
                .metadata("description", "Tokens that represent ownerhip of the fund.")
                .mintable(rule!(require(fund_admin.resource_address())), LOCKED)
                .burnable(rule!(require(fund_admin.resource_address())), LOCKED)
                .no_initial_supply();
            
            let vault_amount = tokens.iter();
            let mut fund_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            let mut token_weights: HashMap<ResourceAddress, Decimal> = HashMap::new();

            for (token, weight) in vault_amount {
                fund_vaults.insert(*token, Vault::new(*token));
                token_weights.insert(*token, *weight);
            };

            return Self {
                fund_vaults: fund_vaults,
                token_weights: token_weights,
                fund_admin_vault: Vault::with_bucket(fund_admin),
                fund_token_address: fund_token_address,
                starting_share_price: starting_share_price,
                price_oracle_address: price_oracle_address,
            }
            .instantiate()
            .globalize()
        }

        // pub fn buy(
        //     &mut self,
        //     token: Bucket
        // )  
        // {

        //     let token_address = token.resource_address();

        //     // let output_token = self.fundcoins_address; 
        //     // let radex = self.radex.into();
        //     // let return_bucket = radex.swap(token, output_token);

        //     // return_bucket
        // }

        // pub fn sell(
        //     &mut self,
        //     token: Bucket
        // )
        // {
        //     let token_address = token.resource_address();

        //     let output_token = RADIX_TOKEN;
        //     // let radex = self.radex.into();
        //     // let return_bucket = radex.swap(token, output_token);

        //     // return_bucket   
        // }

        pub fn issue_tokens(
            &mut self,
            mut tokens: Vec<Bucket>,
        ) -> Bucket
        {
            // Retrieves how many bucket of tokens are being passed.
            let number_of_tokens = tokens.len();
            let mut counter = 0;
            let mut amount_to_mint: Decimal = Decimal::zero();

            info!("# of tokens: {:?}", number_of_tokens);

            while counter <= number_of_tokens {
                // Retrieves each bucket of tokens.
                let token_buckets: Option<Bucket> = tokens.pop();

                match token_buckets {
                    Some(token) => { // If a bucket exists...

                        assert_ne!(
                            borrow_resource_manager!(token.resource_address()).resource_type(), ResourceType::NonFungible,
                            "[Fund Locker]: Assets must be fungible."
                        );

                        let token_address: ResourceAddress = token.resource_address();

                        // Retrieves amount of each token.
                        let token_amount: Decimal = token.amount();

                        // * VALUES THE TOKEN RECEIVED * //.
                        let price_oracle: PriceOracle = self.price_oracle_address.into();
                        let price: Decimal = price_oracle.get_price(token_address);

                        let token_value: Decimal = token_amount * price;

                        // * MINTS FUND TOKENS * //
                        let token_weight: Decimal = *self.token_weights.get(&token_address).unwrap();
                        let fund_tokens_to_mint: Decimal = token_value * token_weight;

                        assert_eq!(self.fund_vaults.contains_key(&token_address), true,
                            "[Fund Locker]: This token does not belong to this fund."
                        );

                        let fund_vault = self.fund_vaults.get_mut(&token_address).unwrap();

                        amount_to_mint += fund_tokens_to_mint;

                        fund_vault.put(token);

                        counter += 1;
                    }
                    None => {
                        
                        info!("[Fund Locker]: All tokens deposited!");

                    }
                }
                

                info!("[Fund Locker]: Fund tokens issues: {:?}", amount_to_mint);
                info!("[Fund Locker]: Counter {:?}", counter);

                }
                info!("[Fund Locker]: Counter {:?}", counter);

            if counter == number_of_tokens {
                let fund_token = self.fund_admin_vault.authorize(|| 
                    borrow_resource_manager!(self.fund_token_address).mint(amount_to_mint)
                );
                
                info!("[Fund Locker]: Amount of Fund Tokens issued: {:?}", amount_to_mint);

                fund_token
            } else {
                let empty_bucket = self.fund_admin_vault.take(0);

                empty_bucket
            }

        }

        pub fn get_total_token_amount(
            &self,
            tokens: Vec<Bucket>
        ) -> Decimal
        {
            let mut amount = Decimal::zero();

            let buckets = tokens.iter();

            for token in buckets {
                let token_amount = token.amount();
                amount += token_amount;
            }

            amount
        }

        fn get_total_fund_tokens_issued(
            &self,
            tokens: &Vec<Bucket>
        ) -> Decimal
        {
            let mut amount = Decimal::zero();

            let buckets = tokens.iter();

            for token in buckets {
                let token_amount: Decimal = token.amount();
                let token_address: ResourceAddress = token.resource_address();
                let token_weight: Decimal = *self.token_weights.get(&token_address).unwrap();
                let price_oracle: PriceOracle = self.price_oracle_address.into();
                let token_price: Decimal = price_oracle.get_price(token_address);
                let token_value: Decimal = token_amount * token_price;
                let amount_minted: Decimal = token_value * token_weight;

                amount += amount_minted;
            }

            amount
        }

        fn withdraw(
            &mut self,
            token_address: ResourceAddress,
            amount: Decimal
        ) -> Bucket 
        {
            // Performing the checks to ensure tha the withdraw can actually go through
            // self.assert_belongs_to_pool(resource_address, String::from("Withdraw"));
            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            let fund_vault: &mut Vault = self.fund_vaults.get_mut(&token_address).unwrap();
            assert!(
                fund_vault.amount() >= amount,
                "[Withdraw]: Not enough liquidity available for the withdraw."
            );

            return fund_vault.take(amount);
        }

        pub fn swap(
            &mut self,
            token_address: ResourceAddress,
            amount: Decimal,
            output_token: ResourceAddress,
        ) -> Bucket
        {
            let bucket: Bucket = self.withdraw(token_address, amount);

            bucket
        }
    }
}