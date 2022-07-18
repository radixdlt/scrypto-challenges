use scrypto::prelude::*;
use crate::bond::BondToken;

blueprint! {
    struct BondMarket {
        bonds: LazyMap<ResourceAddress, ComponentAddress>, // Issuer details w Vault of Bonds, Vault of XRD to repay
        dead_vaults: Vec<Vault>, // For getting rid of dead vaults
        order_count: u64,   // For getting count of orders
        issuer_count: u64,  // For getting number of bonds issued in market
        internal_admin_badge: Vault,
    }

    impl BondMarket {

        // Instantiate BondMarket component
        pub fn instantiate_bond_market() -> ComponentAddress {
        
            let internal_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Bond Market Internal Admin Badge")
                .metadata(
                    "description",
                    "A badge used by bond market to mint and burn other badges",
                )
                .initial_supply(dec!("1"));

            // Bond Market Instantiation
            let bond_market = Self {
                bonds: LazyMap::new(),
                dead_vaults: Vec::new(),
                order_count: 0,
                issuer_count: 0,
                internal_admin_badge: Vault::with_bucket(internal_admin_badge),
            }
            .instantiate()
            .globalize();

            return bond_market
        }

        // Allow user to register themselves on the market and recieve 
        // unique user ID NFT
        // pub fn register(&mut self) -> Bucket {

            // let beneficiary_id: NonFungibleId =
            //     NonFungibleId::from_u64((self.funds.len() + self.dead_vaults.len()) as u64 + 1u64);
            // let beneficiary_badge: Bucket = self.internal_admin_badge.authorize(|| {
            //     borrow_resource_manager!(self.beneficiary_vesting_badge).mint_non_fungible(
            //         &beneficiary_id,
            //         BeneficiaryVestingSchedule::new(
            //             relative_cliff_epoch,
            //             relative_ending_epoch,
            //             funds.amount(),
            //             percentage_available_on_cliff,
            //         ),
            //     )
            // });
        // }

        // Allow user to issue a bond to sell on the market. 
        // Returns a bucket that contains the issuer badge NFT, 
        pub fn issue_bond(&mut self, face_value: Decimal, coupon_epoch: u64, 
            maturity_epoch: u64, coupon_rate: Decimal, issue_price: Decimal, supply:u32) -> Bucket {
            
            // TODO: Add coupon epoch check

            let (new_bond_component, issuer_badge) = BondToken::instantiate_bond(
                self.issuer_count, face_value, coupon_epoch, maturity_epoch,
                coupon_rate, issue_price, supply
            );

            self.bonds.insert(issuer_badge.resource_address(), new_bond_component);                
            self.issuer_count += 1;
            
            return issuer_badge;
        }



        // pub fn list_orders(&self, buy: bool) {
        //     let title = if buy { "BUY" } else { "SELL" };

        //     info!(" \\====================================================/");
        //     info!("");
        //     info!(" /'''''''''''''''''' {:>4} ORDERS '''''''''''''''''\\", title);
        //     info!(" +======================================================+");
        //     info!(" | #    | Token | {:>7} | Filled | {:>8} | Payment |", kind, store);
        //     info!(" +======================================================+");

        //     let orders = 
        //     let orders = (&self.orders).into_iter().filter(|o| o.is_buy_order() == buy).collect::<Vec<&Order>>();

        // }
    }
}