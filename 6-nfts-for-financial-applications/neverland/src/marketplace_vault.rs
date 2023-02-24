use scrypto::prelude::*;

blueprint! {
    struct MarketplaceVault {
        tkn_vault: Vault,
        academy_vault: ComponentAddress,
        academy_share: Decimal,
        owner_badge: ResourceAddress
    }

    impl MarketplaceVault {
        pub fn new(
            tkn_currency: ResourceAddress, 
            academy_vault: ComponentAddress, 
            academy_share: Decimal 
        ) -> (ComponentAddress,Bucket) {
            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(Decimal::one());

            assert!(academy_share <= Decimal::from(100)," Max academy share allowed is 100% ");

            let access_rules = AccessRules::new()
                .method("tkn_withdrawal", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));

            let mut tkn_vault: MarketplaceVaultComponent = Self {
                tkn_vault: Vault::new(tkn_currency),
                academy_vault,
                academy_share,
                owner_badge: owner_badge.resource_address()
            }
            .instantiate();
            tkn_vault.add_access_check(access_rules);

            (tkn_vault.globalize(),owner_badge)
        }

            // Stock tokens in vault.
        pub fn tkn_stock(&mut self, mut tkn_bckt: Bucket) -> Decimal {
            let amount = tkn_bckt.amount();
            let academy_amount = amount*self.academy_share/Decimal::from(100);
            self.tkn_vault.put(tkn_bckt.take(amount-academy_amount));
            info!(" Amount put in vault {} ",amount-academy_amount);

            // Transfer token's academy share in Academy Vault Component  
            let method = "tkn_lock".to_string(); 
            let arg = args![tkn_bckt];
            
            amount-borrow_component!(self.academy_vault).call::<Decimal>(&method, arg)
        }

            // Withdrawal tokens from vault.
        pub fn tkn_withdrawal(&mut self, amount: Decimal) -> Bucket {
            let tkn_bckt = self.tkn_vault.take(amount);
            info!(" TKN withdrawn amount {} ",tkn_bckt.amount());

            tkn_bckt
        }

            // Check total amount in vault.
        pub fn tkn_amount(&self) -> Decimal {
            let amount = self.tkn_vault.amount();
            info!(" TKN amount in vault {} ",amount);

            amount
        }
    }
}
