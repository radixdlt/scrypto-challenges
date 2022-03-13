use scrypto::prelude::*;

blueprint! {
    struct DexianPool {
        // Define what resources and data will be managed by DexianPool components
        base_vault: Vault,
        quote_vault: Vault,
        lp_minter_badge: Vault,
        fee: Decimal,
        lp_token_def: ResourceDef,
        lp_per_asset_ratio: Decimal
    }

    impl DexianPool {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(
            base_tokens: Bucket,
            quote_tokens: Bucket,
            lp_initial_supply: Decimal,
            lp_url: String,
            fee: Decimal
        ) -> (Component, Bucket) {

            assert!(
                !base_tokens.is_empty() && !quote_tokens.is_empty(),
                "You must pass in an initial supply of each token."
            );

            assert!(
                fee >= Decimal::zero() && fee <= Decimal::one(),
                "Invalid fee in thousandths"
            );

            let lp_minter_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "LP Token Mint Auth")
                .metadata("symbol", "LP")
                .initial_supply_fungible(1);
            
            let lp_token_symbol = DexianPool::get_pool_token_pair(base_tokens.resource_address(), quote_tokens.resource_address());
            let mut lp_token_def = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("symbol", lp_token_symbol)
                .metadata("url", lp_url)
                .flags(MINTABLE | BURNABLE)
                .badge(lp_minter_badge.resource_def(), MAY_MINT | MAY_BURN)
                .no_initial_supply();
            
            let lp_tokens = lp_token_def.mint(lp_initial_supply, lp_minter_badge.present());

            let lp_per_asset_ratio = lp_initial_supply / (base_tokens.amount() * quote_tokens.amount());


            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let dexian_pool = Self {
                base_vault: Vault::with_bucket(base_tokens),
                quote_vault: Vault::with_bucket(quote_tokens),
                lp_minter_badge: Vault::with_bucket(lp_minter_badge),
                fee,
                lp_token_def,
                lp_per_asset_ratio
            }
            .instantiate();

            (dexian_pool, lp_tokens)
        }

        pub fn add_liquidity(
            &mut self, 
            mut base_tokens: Bucket, 
            mut quote_tokens: Bucket,
        ) -> (Bucket, Bucket){
            let (supply_to_mint, remainder) = if self.lp_token_def.total_supply() == Decimal::zero() {
                let supply_to_mint = self.lp_per_asset_ratio * base_tokens.amount() * quote_tokens.amount();
                self.base_vault.put(base_tokens.take(base_tokens.amount()));
                self.quote_vault.put(quote_tokens);
                (supply_to_mint, base_tokens)
            }
            else{
                let base_ratio = base_tokens.amount() / self.base_vault.amount();
                let quote_ratio = quote_tokens.amount() / self.quote_vault.amount();

                let(actual_ratio, remainder) = if base_ratio <= quote_ratio {
                    self.base_vault.put(base_tokens);
                    self.quote_vault.put(quote_tokens.take(self.quote_vault.amount() * base_ratio));
                    (base_ratio, quote_tokens)
                }
                else{
                    self.quote_vault.put(quote_tokens);
                    self.base_vault.put(base_tokens.take(self.base_vault.amount() * quote_ratio));
                    (quote_ratio, base_tokens)
                };
                (
                    self.lp_token_def.total_supply() * actual_ratio, remainder
                )
            };

            let lp_tokens = self.lp_minter_badge.authorize(|auth| self.lp_token_def.mint(supply_to_mint, auth));
            (lp_tokens, remainder)
        }

        pub fn remove_liquidity(&mut self, lp_tokens: Bucket) -> (Bucket, Bucket){
            assert!(
                self.lp_token_def == lp_tokens.resource_def(),
                "wrong token type passed in"
            );

            let share = lp_tokens.amount() / self.lp_token_def.total_supply();

            let base_withdraw = self.base_vault.take(self.base_vault.amount() * share);
            let quote_withdraw = self.quote_vault.take(self.quote_vault.amount() * share);

            (base_withdraw, quote_withdraw)
        }

        pub fn swap(&mut self, input_tokens: Bucket) -> Bucket {
            let fee_amount = input_tokens.amount() * self.fee;

            let output_tokens = if input_tokens.resource_def() == self.base_vault.resource_def(){
                let quote_amount = self.quote_vault.amount() - 
                    self.base_vault.amount() * self.quote_vault.amount() / (input_tokens.amount() - fee_amount + self.base_vault.amount());
                
                self.base_vault.put(input_tokens);
                self.quote_vault.take(quote_amount)
            }
            else{
                let base_amount = self.base_vault.amount() - 
                    self.base_vault.amount() * self.quote_vault.amount() / (input_tokens.amount() - fee_amount + self.quote_vault.amount());
                
                self.quote_vault.put(input_tokens);

                self.base_vault.take(base_amount)
            };

            self.lp_per_asset_ratio = self.lp_token_def.total_supply() / (self.base_vault.amount() * self.quote_vault.amount());
            output_tokens
        }

        pub fn get_pair(&self) -> (Address, Address){
            (
                self.base_vault.resource_address(),
                self.quote_vault.resource_address(),
            )
        }

        // utility funcs
        fn get_pool_token_pair(base_address: Address, quote_address: Address) -> String {
            let base_name = DexianPool::get_token_symbol(base_address);
            let quote_name = DexianPool::get_token_symbol(quote_address);
            return format!("LP {}-{}", base_name, quote_name);
        }

        fn get_token_symbol(address: Address) -> String{
            return match ResourceDef::from(address).metadata().get("symbol"){
                Some(s) => format!("{}", s),
                None => format!("{}", address)
            }
        }
    }
}
