use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct MemberData {
    name: String
}

blueprint! {
    struct OrderBook {
	// Order book of all pairs : HashMap< PAIR->(Add1, Add2), List of all orders of this pair->Vec<([Nbr of Add1 tokens, Ratio of Add2 tokens], Address of the order maker -> Address)>>,
        orderbook: HashMap<(Address, Address), Vec<([Decimal; 2], Address)>>,
	//For example : <(Address1, Address2), [([10, 1], rdx1qsq...), ([10, 2], rdx1qstpapapa...)]>
	// That means :  rdx1qsq... want to sell 10* Address1 tokens against (1*10) Address2 tokens.
	//            :  rdx1qstpapapa... want to sell 10* Address1 tokens against (2*10) Address2 tokens.
	//if you want to sell your tokens, you place an order which consists in having your tokens redeemed, or you sell by buying the order from someone.
	
	//Chests that contains all order tokens waiting to be purchased
        vault: HashMap<Address, Vault>,
	//ResourceDef of the admin badge, to perform admin method like : init a pair or withdraw from chests.
        admin_auth_badge: ResourceDef,

	//Member badge for each member of this orderbook
        member_badge_bag: Vault,
        member_auth_badge: ResourceDef,
    }

    impl OrderBook {
        pub fn init() -> (Bucket, Component) {
            let admin_auth = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Attests the holder to be an admin of the DUCKM exchange book")
                .initial_supply_fungible(1);
            
            let member_badge_bag: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .initial_supply_fungible(1);

            let member_auth_badge: ResourceDef = ResourceBuilder::new_non_fungible()
                .flags(MINTABLE)
                .badge(member_badge_bag.resource_def(), MAY_MINT)
                .no_initial_supply();

            let component = Self {
                orderbook: HashMap::new(),
                vault: HashMap::new(),
                admin_auth_badge: admin_auth.resource_def(),


                member_badge_bag: Vault::with_bucket(member_badge_bag),
                member_auth_badge: member_auth_badge,
            }
            .instantiate();

            (admin_auth, component)
        }

        /* --- Badges --- */
        //Returns the resource address of the admin badge
        pub fn get_admin_badge_address(&self) -> Address {
            return self.admin_auth_badge.address()
        }
        // Returns the resource address of the member badge
        pub fn get_member_badge_address(&self) -> Address {
            return self.member_auth_badge.address()
        }
        //Returns your user badge allowing you to interact with the order book
        pub fn become_member(&mut self, name: String) -> Bucket {
            return self.member_badge_bag.authorize(|auth| {
                self.member_auth_badge.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), MemberData{ name: name }, auth)
            });
        }


        /* --- Utils --- */
        //Returns the best order for the pair: (input,output)
        pub fn get_best_price_orderbook(&mut self, input: Address, ouput: Address) {
            let orders = self.orderbook.get_mut(&(input, ouput)).unwrap();
            let mut best_price = orders[0].0;
            for (i, _v) in (0..(orders.len())).enumerate() {
                if best_price[1]>orders[i].0[1] {
                    best_price = orders[i].0;
                }
            }
            info!("Current best price : {} per unit.", best_price[1]);
            info!("Corresponding order : {:?} ", best_price);   
        }

	    //Returns the order book for the pair: (input,output)
        pub fn look_orderbook(&self, input: Address, ouput: Address) {
            info!("{:?}",self.orderbook.get(&(input, ouput)));
        }

	    /* --- Administrate the orderbook --- */
        //Destroy all orders present for the pair (input, output)
        #[auth(admin_auth_badge)]
        pub fn reset_pair_orderbook(&mut self, input: Address, ouput: Address) {
            self.orderbook.insert((input, ouput), vec![([Decimal::from(0),Decimal::from(0)], input)]);
        }

        //Initialize the pair (input, output)
        #[auth(admin_auth_badge)]
        pub fn init_pair_orderbook(&mut self, input: Address, ouput: Address) {
            self.orderbook.insert((input, ouput), vec![]);
            self.vault.insert(input, Vault::new(ResourceDef::from(input)));
            info!("Order book correctly initialized!");
        }
        
        //Withdraw amount token from the tokenAddress Component Vault
 	    #[auth(admin_auth_badge)]
        pub fn withdraw(&mut self, sale_tokens: Address, amount: Decimal) -> Bucket {
            return self.vault.get_mut(&sale_tokens).unwrap().take(amount)
        }
	

	   /* --- Basic utilisation of the orderbook --- */
       //Place an order like this: user_address sell amount tokens of input for output_ratio*amount tokens of output.
        #[auth(member_auth_badge)]
        pub fn add_order_orderbook(&mut self, input: Address ,ouput: Address, amount: Decimal, xrd_price: Decimal, mut payment: Bucket, user_address: Address) -> Bucket {
            self.vault.get_mut(&input).unwrap().put(payment.take(amount));
            info!("Funds deposited on the order!");
            self.update_register_orderbook(input, ouput, amount, xrd_price, user_address, auth);
            return payment;
        }

        //Accept an order from this pair : (input,output) if anyone wants to sell amount tokens of input for amount*output_ratio tokens of output. The payment bucket need to be filled with enough tokens output, you need at least: amount*output_ratio tokens of output
        #[auth(member_auth_badge)]
        pub fn accept_an_order(&mut self, input: Address ,ouput: Address, order_amount: Decimal, order_price: Decimal, mut payment: Bucket) -> (Bucket, Bucket) {
            let orders = self.orderbook.get_mut(&(input, ouput)).unwrap();
            if payment.resource_address() != ouput {
                panic!("Bad bucket !");
            }
            if orders.len()==0 {
                panic!("No orders on this pair.");
            } else {
                for (i, _v) in (0..(orders.len())).enumerate() {
                    if vec![order_amount, order_price]==orders[i].0 {
                        info!("The order exists! ");
                        if payment.amount()<(order_amount*order_price){
                            panic!("Pas assez pour combler l'ordre ! ");
                        } else {
                            let purchased_tokens = self.vault.get_mut(&input).unwrap().take(order_amount);
                            info!("Ordre complet ! ");
                            let args = vec![scrypto_encode(&payment.take(order_amount*order_price))];
                            let _rtn = call_method(orders[i].1, "deposit", args);
                            self.update_unsubscribe_orderbook(input, ouput, order_amount, order_price, auth);
                            return (payment, purchased_tokens);
                        }
                    }
                }
                info!("No order like this! ");
            }
            return (payment, Bucket::new(ResourceDef::from(input)));
        }

        //Allows you to add a given order in the book of this pair
        #[auth(member_auth_badge)]
        pub fn update_register_orderbook(&mut self, input: Address, ouput: Address, amount: Decimal, xrd_price: Decimal, user_address: Address) {
            let orders = self.orderbook.get_mut(&(input, ouput)).unwrap();
            orders.push(([amount, xrd_price], user_address));
            info!("New order in the book, All orders : {:?} ", orders);
        }

        //Allows you to remove a given order in the book of a specific pair
        #[auth(member_auth_badge)]
        pub fn update_unsubscribe_orderbook(&mut self, input: Address, ouput: Address, amount: Decimal, xrd_price: Decimal) {
            let orders = self.orderbook.get_mut(&(input, ouput)).unwrap();
            for (i, _v) in (0..(orders.len())).enumerate() {
                if vec![amount, xrd_price]==orders[i].0 {
                    orders.remove(i);
                    break;
                }
            }
            info!("One less order in the book, All orders : {:?} ", orders);
        }       
    }
}
