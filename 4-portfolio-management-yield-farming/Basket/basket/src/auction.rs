use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct BidReceipt {
    price: Decimal,                 // minimum amount of offer to receive per unit of bid 
    #[scrypto(mutable)]
    amount: Decimal,                // amount of bid tokens
    #[scrypto(mutable)]
    filled: Decimal,                // amount of offer tokens claimable
    #[scrypto(mutable)]
    next: NonFungibleId,            // pointer to next bid in bid chain
}

blueprint! {
    struct Auction {
        bid_receipt_address: ResourceAddress,          // nft that represents a bid
        internal_badge: Vault,                         // badge used for all internal permission (minting, burning, etc.)
        bid_vault: Vault,                              // vault used to hold tokens from bids
        offer_vault: Vault,                            // vault used to hold tokens for filled bids
        bid_chain_head: NonFungibleId,                 // id of first bid receipt in bid chain
    }

    impl Auction {
        // instantiates auction
        // returns (auction_address, admin_badge)
        // admin_badge is used to auction tokens
        pub fn new(bid_token_address: ResourceAddress, offer_token_address: ResourceAddress) -> (ComponentAddress, Bucket) {
            // mint admin badge
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Auction Admin Badge")
                .metadata("symbol", "ADMIN")    
                .initial_supply(1);

            // mint internal badge used for minting and burning
            let internal_badge: Bucket = ResourceBuilder::new_fungible()
                .initial_supply(1);

            // create bid resource manager
            let bid_receipt_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Bid Receipt")
                .metadata("symbol", "BID")
                .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
                .burnable(rule!(require(internal_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(internal_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // create method permissions
            let auth: AccessRules = AccessRules::new()
            .method("auction", rule!(require(admin_badge.resource_address())))
            .default(rule!(allow_all));

            // instantiate and return
            let auction: ComponentAddress = Self {
                bid_receipt_address: bid_receipt_address,
                internal_badge: Vault::with_bucket(internal_badge),
                bid_vault: Vault::new(bid_token_address),
                offer_vault: Vault::new(offer_token_address),
                bid_chain_head: NonFungibleId::from_u32(0),
            }
            .instantiate()
            .add_access_check(auth)
            .globalize();

            (auction, admin_badge)
        }

        // creates a bid
        // returns bid receipt nft
        pub fn create_bid(&mut self, bid_tokens: Bucket, price: Decimal) -> Bucket {
            // check if valid price
            assert!(
                price > dec!(0),
                "Price must be a positive number."
            );

            // save amount and deposit tokens
            let amount: Decimal = bid_tokens.amount();
            assert!(
                amount > dec!(0),
                "Bucket can not be empty."
            );

            // deposit bid tokens
            self.bid_vault.put(bid_tokens);
            
            // create id
            let id: NonFungibleId = NonFungibleId::random();
            assert!(
                id != NonFungibleId::from_u32(0),
                "Id can not be zero."
            );
            
            let bid_manager: &ResourceManager = borrow_resource_manager!(self.bid_receipt_address);

            // find position and update bid chain
            let next: NonFungibleId = if self.bid_chain_head == NonFungibleId::from_u32(0) {                    // empty bid chain
                self.bid_chain_head = id.clone();

                NonFungibleId::from_u32(0)
            } else if price < bid_manager.get_non_fungible_data::<BidReceipt>(&self.bid_chain_head).price {     // place at front
                let next: NonFungibleId = self.bid_chain_head.clone();
                self.bid_chain_head = id.clone();

                next
            } else {                                                                                            // place in chain
                // find position
                let mut pointer: NonFungibleId = self.bid_chain_head.clone();
                let mut next: NonFungibleId = bid_manager.get_non_fungible_data::<BidReceipt>(&pointer).next;
                while next != NonFungibleId::from_u32(0) && price >= bid_manager.get_non_fungible_data::<BidReceipt>(&next).price {
                    pointer = next;
                    next = bid_manager.get_non_fungible_data::<BidReceipt>(&pointer).next;
                }

                // update prev bid to point to new bid
                let mut pointer_data: BidReceipt = bid_manager.get_non_fungible_data(&pointer);
                pointer_data.next = id.clone();
                self.internal_badge.authorize(|| {
                    bid_manager.update_non_fungible_data(&pointer, pointer_data);
                });

                next
            };

            // create bid receipt
            let bid_receipt: Bucket = self.internal_badge.authorize(|| {
                bid_manager.mint_non_fungible(
                    &id, 
                    BidReceipt {
                        price: price,
                        amount: amount,
                        filled: dec!(0),
                        next: next,
                    }
                )
            });
            
            // return bid receipt nft
            bid_receipt
        }

        // closes the bid for given bid receipt
        // return (remaining bid tokens, bought tokens)
        pub fn close_bid(&mut self, bid_receipt: Bucket) -> (Bucket, Bucket) {
            // check if it is a valid bid receipt
            assert!(
                bid_receipt.resource_address() == self.bid_receipt_address,
                "Invalid bid receipt."
            );
            
            let id: NonFungibleId = bid_receipt.non_fungible::<BidReceipt>().id();
            let bid_data: BidReceipt = bid_receipt.non_fungible().data();

            // if not filled
            if bid_data.amount > dec!(0) {
                // remove from bid chain
                if self.bid_chain_head == id {                  // at front of chain
                    self.bid_chain_head = bid_data.next;
                } else {                                        // in middle of chain
                    let bid_manager: &ResourceManager = borrow_resource_manager!(self.bid_receipt_address);

                    // find postion
                    let mut pointer: NonFungibleId = self.bid_chain_head.clone();
                    let mut next: NonFungibleId = bid_manager.get_non_fungible_data::<BidReceipt>(&pointer).next;
                    while next != id {
                        pointer = next;
                        next = bid_manager.get_non_fungible_data::<BidReceipt>(&pointer).next;
                    }

                    // update prev bid to point to next bid 
                    let mut pointer_data: BidReceipt = bid_manager.get_non_fungible_data(&pointer);
                    pointer_data.next = bid_data.next;
                    self.internal_badge.authorize(|| {
                        bid_manager.update_non_fungible_data(&pointer, pointer_data);
                    });
                }
            }
            
            // burn bid receipt
            self.internal_badge.authorize(|| {
                bid_receipt.burn();
            });

            // return unfilled and filled parts of bid
            (self.bid_vault.take(bid_data.amount), self.offer_vault.take(bid_data.filled))
        }

        // requires admin badge to call
        // auctions offer tokens
        // returns (bought tokens, remaining offer tokens)
        pub fn auction(&mut self, mut offer_tokens: Bucket) -> (Bucket, Bucket) {
            let mut bid_tokens: Bucket = Bucket::new(self.bid_vault.resource_address());
            let bid_manager: &ResourceManager = borrow_resource_manager!(self.bid_receipt_address);
            
            // fill bids until no more offer tokens or no more bids
            let mut pointer: NonFungibleId = self.bid_chain_head.clone();
            while pointer != NonFungibleId::from_u32(0) && offer_tokens.amount() > dec!(0) {
                let id: NonFungibleId = pointer.clone();
                let mut bid_receipt_data: BidReceipt = bid_manager.get_non_fungible_data(&id);

                let calc_buy: Decimal = bid_receipt_data.amount * bid_receipt_data.price;
                if calc_buy > offer_tokens.amount() {       // part fill bid
                    bid_receipt_data.filled += offer_tokens.amount();
                    let amount: Decimal = offer_tokens.amount() / bid_receipt_data.price;
                    self.offer_vault.put(offer_tokens.take(offer_tokens.amount()));
                    bid_receipt_data.amount -= amount;
                    bid_tokens.put(self.bid_vault.take(amount));
                } else {                                    // fully fill bid
                    bid_receipt_data.filled += calc_buy;
                    let amount: Decimal = bid_receipt_data.amount;
                    self.offer_vault.put(offer_tokens.take(calc_buy));
                    bid_receipt_data.amount = dec!(0);
                    bid_tokens.put(self.bid_vault.take(amount));

                    pointer = bid_receipt_data.next.clone();
                }

                // update bid
                self.internal_badge.authorize(|| {
                    bid_manager.update_non_fungible_data(&id, bid_receipt_data);
                });
            }

            // remove filled bids from chain
            self.bid_chain_head = pointer;

            // return bid tokens and remainder
            (bid_tokens, offer_tokens)
        }
    }
}