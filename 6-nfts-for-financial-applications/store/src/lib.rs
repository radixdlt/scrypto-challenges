use scrypto::prelude::*;

blueprint! {
    struct Store {
        item: String,
    }


    impl Store {
    
        pub fn instantiate_store() -> ComponentAddress {
          

            Self {
                campaigns: campaigns,
                admin_badge: Vault::with_bucket(admin_badge),
                current_campaigns_address:current_campaigns_address,
                // current_campaigns_vault:  Vault::with_bucket(current_campaigns_bucket),
                inventory_vault: Vault::with_bucket(inventory_bucket),
                inventory_address:inventory_address,
                campaign_vaults: HashMap::new(),
                item_vaults: HashMap::new(),
            }
            .instantiate()
            .globalize()
        }

     

    }

}
