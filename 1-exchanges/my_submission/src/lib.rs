use scrypto::prelude::*;

blueprint! {
    struct MBM {
        share_vaults: HashMap<Address, Vault>,
        money_vaults: HashMap<Address, Vault>,
        share_resource_def: ResourceDef
    }

    impl MBM {
        pub fn new() -> (Component, Bucket) {

            let bucket_of_shares: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "share")
                .metadata("symbol", "SHR")
                .metadata("description", "A share of a company")
                .initial_supply_fungible(100);
 
            let component = Self {
                share_vaults: HashMap::new(),
                money_vaults: HashMap::new(),
                share_resource_def: bucket_of_shares.resource_def()
            }
            .instantiate();
            (component, bucket_of_shares)
        }

        // every share holder must deposit all his/her shares together with their value for all the shares, before redistribution
        pub fn place_trade(&mut self, holder: Address, money_for_all_shares: Bucket, share: Bucket) {
            let share_vault = self.share_vaults.entry(holder).or_insert(Vault::new(self.share_resource_def.address()));
            share_vault.put(share);
            let money_vault = self.money_vaults.entry(holder).or_insert(Vault::new(RADIX_TOKEN));
            money_vault.put(money_for_all_shares);
        }

        // high valuators become buyers, low valuators become sellets
        pub fn redistribute(&mut self, estimated_cut: usize) {
            // run only if all the shares are deposited.
            let mut num_of_shares_deposited: Decimal = Decimal::zero();
            for (_address, vault) in self.share_vaults.iter() {
                num_of_shares_deposited += vault.amount();
            }
            assert!(num_of_shares_deposited == self.share_resource_def.total_supply(), "All shares must be deposited before redistribution");

            // this needs to be randomized with TBA tools based on the amount of shares in all the vaults, to be 0 or 1.
            let marginal_agent_is_seller: usize = 1;
            // exact number of holders to cut
            let num_of_holders_to_cut = estimated_cut + marginal_agent_is_seller;
            
            // prepare data structures with share amounts as decimal
            let mut shares_array = Vec::new();
            let mut amounts: HashMap<Address, Decimal> = HashMap::new;
            for (address, vault) in self.share_vaults.iter() {
                shares_array.push(vault.amount());
                amounts.insert(address, vault.amount());
            }
            // get highest seller value (everyone above it is buyer) and price per share
            // the money deposited is tied to the bid per share because it must cover purchase of all shares,
            // and so it is divided by the total number of shares
            let price = order_stat::kth(&mut shares_array, estimated_cut) / num_of_shares_deposited;    
            let highest_seller = order_stat::kth(&mut shares_array, num_of_holders_to_cut);

            // collect all the money and get valuations
            let mut all_money = Bucket::new(self.share_resource_def.address());
            let mut valuations: HashMap<Address, Decimal> = HashMap::new;
            for (address, vault) in self.money_vaults.iter() {
                valuations.insert(address, vault.amount());
                all_money.put(valut.take_all());
            }

            // take sellers shares and pay them
            let mut total_number_of_initial_buyers_shares = Decimal::zero();
            let mut total_number_of_initial_sellers_shares = Decimal::zero();
            let mut all_sellers_shares: Bucket::new();
            // let mut buyer_array: Vec<self.share_resource_def.address()> = Vec::new();
            for (address, vault) in self.share_vaults.iter() {
                if vault.amount() <= *highest_seller {  // if seller
                    total_number_of_initial_seller_shares += vault.amout();
                    all_sellers_shares.put(vault.take_all());
                    // and then send payment to the correct holder (save him/her the trouble of calling again)
                    let account = Account::from(address);
                    account.deposit(all_money.take(valuations.get(address) + price * amounts.get(address)));
                } else {    // if buyer
                    total_number_of_initial_buyers_shares += vault.amout();
                }

            // after collecting shares to give buyers, give them proportionally to the buyers initial amounts
            for (addr, vault) in self.share_vaults.iter() {
                // if buyer
                if vault.amount() > *highest_seller {
                    let number_of_shares_for_buyer = total_number_of_initial_sellers_shares * vault.amount() / total_number_of_initial_buyers_shares;
                    vault.put(all_sellers_shares.take(number_of_shares_for_buyer));
                    // and then send everything to the correct holder (save him/her the trouble of calling again)
                    let account = Account::from(address);
                    account.deposit(vault.take_all();   // return all the shares to the holder
                    account.deposit(all_money.take(valuations.get(address) - price * amounts.get(address)));    // return remaining XRD
                } 
            }
        }
    }
}