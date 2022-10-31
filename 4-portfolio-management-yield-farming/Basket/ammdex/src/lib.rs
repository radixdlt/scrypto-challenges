use scrypto::prelude::*;

blueprint! {
    struct Pool {
        lp_token_address: ResourceAddress,
        internal_badge: Vault,
        x_vault: Vault,
        y_vault: Vault,
    }

    impl Pool {
        pub fn new(x_tokens: Bucket, y_tokens: Bucket) -> (ComponentAddress, Bucket) {
            let internal_badge: Bucket = ResourceBuilder::new_fungible()
                .initial_supply(1);

            let lp_token_address: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", "LP token")
                .metadata("symbol", "LPT")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

                
            let lp_token_manager: &ResourceManager = borrow_resource_manager!(lp_token_address);
            let mint_amount: Decimal = x_tokens.amount();
            let lp_tokens: Bucket = internal_badge.authorize(|| {
                lp_token_manager.mint(mint_amount)
            });


            let pool: ComponentAddress = Self {
                lp_token_address: lp_token_address,
                internal_badge: Vault::with_bucket(internal_badge),
                x_vault: Vault::with_bucket(x_tokens),
                y_vault: Vault::with_bucket(y_tokens),
            }
            .instantiate()
            .globalize();

            (pool, lp_tokens)
        }

        pub fn swap(&mut self, tokens: Bucket) -> Bucket {
            if tokens.resource_address() == self.x_vault.resource_address() {
                let x: Decimal = self.x_vault.amount();
                let y: Decimal = self.y_vault.amount(); 
                let dx: Decimal = tokens.amount();
                let dy: Decimal = y * dx / (x + dx);

                self.x_vault.put(tokens);
                self.y_vault.take(dy)
            } else {
                let x: Decimal = self.x_vault.amount();
                let y: Decimal = self.y_vault.amount(); 
                let dy: Decimal = tokens.amount();
                let dx: Decimal = x * dy / (y + dy);

                self.y_vault.put(tokens);
                self.x_vault.take(dx)
            }
        }

        pub fn add_liquidity(&mut self, mut x_tokens: Bucket, mut y_tokens: Bucket) -> (Bucket, Bucket) {
            let lp_token_manager: &ResourceManager = borrow_resource_manager!(self.lp_token_address);

            let (mint_amount, remainder): (Decimal, Bucket) = if lp_token_manager.total_supply() == dec!(0) {
                let mint_amount: Decimal = x_tokens.amount();
                let remainder: Bucket = Bucket::new(x_tokens.resource_address());

                self.x_vault.put(x_tokens);
                self.y_vault.put(y_tokens);

                (mint_amount, remainder)
            } else {
                let x_ratio: Decimal = x_tokens.amount() / self.x_vault.amount();
                let y_ratio: Decimal = y_tokens.amount() / self.y_vault.amount();

                if x_ratio > y_ratio {
                    self.y_vault.put(y_tokens);
                    self.x_vault.put(x_tokens.take(self.x_vault.amount() * y_ratio));
                    let mint_amount: Decimal = y_ratio * lp_token_manager.total_supply();

                    (mint_amount, x_tokens)
                } else {
                    self.x_vault.put(x_tokens);
                    self.y_vault.put(y_tokens.take(self.y_vault.amount() * x_ratio));
                    let mint_amount: Decimal = x_ratio * lp_token_manager.total_supply();

                    (mint_amount, y_tokens)
                }
            };

            let lp_tokens: Bucket = self.internal_badge.authorize(|| {
                lp_token_manager.mint(mint_amount)
            });

            (lp_tokens, remainder)
        }

        pub fn remove_liquidity(&mut self, lp_tokens: Bucket) -> (Bucket, Bucket) {
            assert!(
                lp_tokens.resource_address() == self.lp_token_address,
                "Invalid lp tokens."
            );

            let lp_token_manager: &ResourceManager = borrow_resource_manager!(self.lp_token_address);
            let ratio: Decimal = lp_tokens.amount() / lp_token_manager.total_supply();

            let x_tokens: Bucket = self.x_vault.take(self.x_vault.amount() * ratio);
            let y_tokens: Bucket = self.y_vault.take(self.y_vault.amount() * ratio);

            self.internal_badge.authorize(|| {
                lp_tokens.burn();
            });

            (x_tokens, y_tokens)
        }

        pub fn get_pair(&self) -> (ResourceAddress, ResourceAddress) {
            (self.x_vault.resource_address(), self.y_vault.resource_address())
        }
    }
}