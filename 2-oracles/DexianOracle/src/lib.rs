use scrypto::prelude::*;

blueprint! {
    struct DeXianOracle {
         /// This is just a reular admin badge, for register/remove DataProvider
         admin_badge: ResourceAddress,
        
         // /// DataProvider badge ResourceDef
         // dataprovider_badge_def: ResourceDef,
         // minter
         callback_minter: Vault,
        
         // /// DataProvider badge ResourceDef
         // dataprovider_badge_def: ResourceDef,
         
         /// callbacks
         callback_vaults: Vault,

         /// callback that have not yet been triggered 
         unfilful_vec: Vec<NonFungibleId>,
 
         /// fee
         fee: Decimal,
 
         /// oracle (price, epoch_at) for XRD/USD
         price_map: HashMap<String, (Decimal, u64)>,
 
         /// balance(fee) vault
         vault: Vault 
    }
    
    impl DeXianOracle {
        pub fn new(
            fee: Decimal
        ) -> (ComponentAddress, Bucket) {
            assert!( fee >= Decimal::zero(), "invalid fee value.");

            let admin_badge : Bucket = ResourceBuilder::new_fungible()
                .metadata("name","DeXianOracle Admin Badge").metadata("symbol","DXADM")
                .initial_supply(Decimal::ONE);
            
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(Decimal::ONE);
            
            let callback_bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "DeXianOracle Callback").metadata("symbol", "DXCB")
                .mintable(rule!(require(admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            let component = Self {
                admin_badge: admin_badge.resource_address(),
                price_map: HashMap::new(),
                vault: Vault::new(RADIX_TOKEN),
                callback_vaults: Vault::new(callback_bucket),
                unfilful_vec: Vec::new(),
                callback_minter: Vault::with_bucket(minter_badge),
                fee
            }.instantiate();

            let access_rules = AccessRules::new()
                .method("feed_price", rule!(require(admin_badge.resource_address())))
                .method("request_price", rule!(allow_all))
                .method("get_price", rule!(allow_all));
                // .method("register_dataprovider", rule(!require(admin_badge.resource_address())))
                // .method("remove_dataprovider", rule(!require(admin_badge.resource_address())));

            (component.add_access_check(access_rules).globalize(), admin_badge)

        }

        pub fn feed_price(&mut self, pair: String, price: String) -> bool {
            self.price_map.insert(pair.clone(), (Decimal::from(price), Runtime::current_epoch()));
            self.filfull_request(&pair);
            true
        }

        pub fn get_price(&self, pair:String) -> (Decimal, u64){
            assert!(self.price_map.contains_key(&pair), "the pair not exists!");
            *self.price_map.get(&pair).unwrap()
        }

        pub fn request_price(&mut self,  fee: Bucket,  pair: String, component: ComponentAddress, 
            method: String, args: Vec<Vec<u8>>) -> NonFungibleId {
            assert!(fee.amount() <  self.fee, "Fees are lower than required!");

            let callback_id = NonFungibleId::random();
            let callback_data = CallbackData::new_instance(callback_id.clone(), component, method, pair, args);

            let callback = self.callback_minter.authorize(|| {
                let rm = borrow_resource_manager!(self.callback_vaults.resource_address());
                rm.mint_non_fungible(&callback_id, callback_data)
            });
            // Store the Callback NFR inside this component
            self.callback_vaults.put(callback);
            self.unfilful_vec.push(callback_id.clone());
            callback_id
        }

        fn filfull_request(&mut self, pair: &String) {
            let mut i = 0;
            while i < self.unfilful_vec.len() {
                if self.callback_vaults.non_fungible_ids().contains(&self.unfilful_vec[i]){
                    let callback = self.callback_vaults.take_non_fungible(&self.unfilful_vec[i]);
                    let callback_data = callback.non_fungible::<CallbackData>().data();
                    if callback_data.pair.eq(pair) {
                        callback_data.call();
                        self.unfilful_vec.remove(i);
                    }
                    i += 1;
                }                
            }
        }
    }
}

#[derive(NonFungibleData)]
pub struct CallbackData {
    /// request id
    pub id: NonFungibleId,

    pub pair: String,

    /// The target component of the callback
    pub component: ComponentAddress,

    /// The target method of the callback
    pub method: String,

    /// The args that should be passed to the target method
    pub args: Vec<Vec<u8>>,
}

impl CallbackData {
    
    pub fn new_instance(id: NonFungibleId, component: ComponentAddress,
        method: String, pair: String, args: Vec<Vec<u8>>
        ) -> Self {
        
        Self {
            args: args.to_vec(),
            pair,
            id,
            component,
            method
        }
    }

    pub fn call(&self){
        Runtime::call_method(self.component, &self.method, self.args.to_vec());
    }
}