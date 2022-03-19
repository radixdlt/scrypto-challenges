//use sbor::*;
use scrypto::prelude::*;

mod dex;
mod order;

//Define the Open Order book smart contrat entries.
blueprint! {

    pub struct Market {
        //manage open orders user access
        orders_badge_minter: Vault,
        orders_badge_def: ResourceDef,

        // Define market data
        pub dex: dex::Dex,
        name: String,
    }

    impl Market {
        /// create a market with specified quote and base token as initial quantity in the vault.
        /// A name identify the market.
        /// Market owner call this method to create a new market.
        /// Return the created market component address and market admin access badge.
        ///
        /// Market define 2 level of access:
        ///  * admin: badge return by this function that can withdraw fee gain during asset transfert
        ///  * trader or user badge needed to push order to the book
        ///
        pub fn instantiate_market(
            quote_token: Address,
            base_token: Address,
            name: String,
        ) -> (Component, Bucket) {

            // Define the admin badge
            let admin_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", &format!("Admin challenge market {} access Badge", name))
                .initial_supply_fungible(1);

            // Create a badge for internal use which will hold mint/burn authority for the admin badge we will soon create
            let orders_badge_minter: Bucket =
                ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", &format!("Trader challenge market:{}", name))
                .initial_supply_fungible(1);

            // Create the ResourceDef for a mutable supply admin badge
            let orders_badge_def = ResourceBuilder::new_non_fungible()
                .metadata("name", name.clone())
                .flags(MINTABLE )
                .badge(orders_badge_minter.resource_def(), MAY_MINT)
                .no_initial_supply();

            (
                Self {
                    orders_badge_minter: Vault::with_bucket(orders_badge_minter),
                    orders_badge_def,
                    name,
                    dex: dex::Dex::new(quote_token, base_token),
                }
                .instantiate(),
                admin_badge
            )
        }

        /// To trade, user must create a openorders badge to get access to the bid and ask method.
        /// create_openorders create a data structure store in the market to store transferred asset during order matching.
        /// Return the badge needed to push order and withdraw asset transferred by order execution.
        pub fn create_openorders(&mut self) -> Bucket {
            let badge = self.orders_badge_minter.authorize(|auth| {
                self.orders_badge_def.mint_non_fungible(
                    &NonFungibleKey::from(Uuid::generate()),
                    BadgeData {
                        name: self.name.clone(),
                    },
                    auth,
                )
            });

            let orders = order::UserOrders::new(self.dex.params.quote_token.clone(), self.dex.params.base_token.clone());
            self.dex.user_orders.insert(
                badge.get_non_fungible_keys().get(0).unwrap().clone(),
                orders,
            );

            badge
        }

        //use u8 for order type beacause I didn't find an example with an Option as Tx parameter.
        /// Push a bid order to the order book. Bid order are a max limit price, a amount of base to buy and a quote bucket containing all the quote
        /// needed to by the base at the limit price.
        /// Order type define how the order will be matched.
        ///  * limit: 0 when the order is push the maximum amount of quote is transferred depending on the available opposite order in the book.
        ///    If some amount can't be matched, the remaining is added to the order book.
        ///  * Immediate or Cancel: 1 same as limit but if a remaining amount can't be matched, it's cancelled.
        ///  * Post: 2 the order is not matched an immediately added to the order book. It can be useful to decrease the fee (see Fee management).
        ///
        /// Return the order id address use to cancel it and the rest of the quote not used.
        /// Provided quote is locked while the order is still pending and it's free when the order is cancelled or transferred when it's matched.
        ///
        #[auth(orders_badge_def)]
        pub fn bid_order(&mut self, price: Decimal, amount: Decimal, ordre_type: u8, quote: Bucket) -> (Bucket, Bucket) { //, auth: BucketRef
            info!("buy_order");
            let owner_keys = auth.get_non_fungible_keys();
            let data: BadgeData = auth
                .resource_def()
                .get_non_fungible_data(owner_keys.get(0).unwrap());
            assert!(data.name == self.name, "Not current market open order badge");
            self.dex.bid(
                owner_keys.get(0).unwrap().clone(),
                price,
                amount,
                quote,
                ordre_type.into(),
            )
        }

        /// Push a ask order to the order book. Ask order are a min limit price, a amount of base to sell and a base bucket containing
        /// all the base to sell.
        /// Order type define how the order will be matched.
        ///  * limit: 0 when the order is push the maximum amount of quote is transferred depending on the available opposite order in the book.
        ///    If some amount can't be matched, the remaining is added to the order book.
        ///  * Immediate or Cancel: 1 same as limit but if a remaining amount can't be matched, it's cancelled.
        ///  * Post: 2 the order is not matched an immediately added to the order book. It can be useful to decrease the fee (see Fee management).
        ///
        /// Return the order id address use to cancel it and the rest of the quote not used.
        /// Provided base is locked while the order is still pending and it's free when the order is cancelled or transferred when it's matched.
        ///
        #[auth(orders_badge_def)]
        pub fn ask_order(&mut self, price: Decimal, amount: Decimal, ordre_type: u8, base: Bucket) -> (Bucket, Bucket) {
            info!("sell order");
            let owner_keys = auth.get_non_fungible_keys();
            let data: BadgeData = auth
                .resource_def()
                .get_non_fungible_data(owner_keys.get(0).unwrap());
            assert!(data.name == self.name, "Not current market open order badge");
            self.dex.ask(
                owner_keys.get(0).unwrap().clone(),
                price,
                amount,
                base,
                ordre_type.into(),
            )
        }

        ///
        /// Withdraw all the asset (quote, base) store in the provided badge openorders and return it.
        /// Locked quote or base can't be withdrawn until associated order are pending in the order book.
        #[auth(orders_badge_def)]
        pub fn withdraw(&mut self) -> (Bucket, Bucket) {
            info!("withdraw order");
            let owner_keys = auth.get_non_fungible_keys();
            let data: BadgeData = auth
                .resource_def()
                .get_non_fungible_data(owner_keys.get(0).unwrap());
            assert!(data.name == self.name, "Not current market open order badge");

            let mut user_orders = self.dex
            .user_orders
            .get(&owner_keys.get(0).unwrap())
            .ok_or_else(|| panic!("Badge provided not declared call create_openorders to get one"))
            .unwrap();

            (user_orders.quote_vault.take_all(), user_orders.base_vault.take_all())
        }

        /// Cancel the order with the specified order_id_address. The order must owned bu the auth user badge provided.
        /// Order locked asset are transferred to the user openordersquote and base vault.
        ///
        #[auth(orders_badge_def)]
        pub fn cancel_order(&mut self, order_id_badge: Bucket)  -> Bucket {
            info!("cancel order");
            let owner_keys = auth.get_non_fungible_keys();
            let data: BadgeData = auth
                .resource_def()
                .get_non_fungible_data(owner_keys.get(0).unwrap());
            assert!(data.name == self.name, "Not current market open order badge");
            self.dex.cancel_order(order_id_badge,owner_keys.get(0).unwrap().clone())
        }
    }

}

#[derive(Debug, Clone, NonFungibleData)]
pub struct BadgeData {
    name: String,
}
