use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct ParticipantData {
    id: u128,
}

blueprint! {
    struct Auction {
        id: u128,
        current_bid: Decimal,
        minimal_bid: Decimal,
        bin_price: Decimal,
        ended: bool,
        product: Vault,

        gathered: Vault,
        bids: HashMap<NonFungibleId, Vault>,

        // Owner
        owner_badge_ref: ResourceAddress,

        // Participant
        participant_minter: Vault,
        participant_nft_address: ResourceAddress,
    }

    impl Auction {
        pub fn new(minimal_bid: Decimal, bin_price: Decimal, product: Bucket) -> (ComponentAddress, Bucket) {

            assert!(bin_price > minimal_bid, "BIN price has to be higher than minimal bid.");

            // Auction relative
            let id = Runtime::generate_uuid();
            let product: Vault = Vault::with_bucket(product);

            // Owner relative
            let owner_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", format!("Owner of {} auction", id.to_string()))
                .initial_supply(1);

            // Participant relative
            let participant_minter: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .initial_supply(1);

            let participant_nft_address: ResourceAddress = ResourceBuilder::new_non_fungible()
            .metadata("name", format!("Participant of {} auction", id.to_string()))
            .mintable(rule!(require(participant_minter.resource_address())), LOCKED)
            .burnable(rule!(require(participant_minter.resource_address())), LOCKED)
            .no_initial_supply();

            let component = Self {
                id,
                current_bid: dec!(0),
                minimal_bid,
                bin_price,
                ended: false,
                product,
                gathered: Vault::new(RADIX_TOKEN),
                bids: HashMap::new(),
                owner_badge_ref: owner_badge.resource_address(),
                participant_minter: Vault::with_bucket(participant_minter),
                participant_nft_address,
            }.instantiate();

            // Access control
            let access_rules = AccessRules::new()
                .method("withdraw", rule!(require(owner_badge.resource_address())))
                .method("bid", rule!(require(participant_nft_address)))
                .default(AccessRule::AllowAll);

            // Component with owner badge
            (component.add_access_check(access_rules).globalize(), owner_badge)
        }

        pub fn register(&mut self) -> Bucket {
            let badge = self.participant_minter.authorize(|| {
                let participant_nft_manager: &ResourceManager = borrow_resource_manager!(self.participant_nft_address);
                participant_nft_manager.mint_non_fungible(&NonFungibleId::random(), ParticipantData{id: Runtime::generate_uuid()})
            });

            self.bids.insert(badge.non_fungible::<ParticipantData>().id(), Vault::new(RADIX_TOKEN));

            badge
        }

        pub fn bid(&mut self, amount: Bucket, participant_badge: Proof) -> (Bucket, Bucket) {
            assert!(self.bin_price > self.current_bid, "Auction has ended, BIN price was reached.");
            assert!(!self.ended, "Auction has ended.");

            let id = participant_badge.non_fungible::<ParticipantData>().id();

            assert!(self.bids.contains_key(&id), "You need to register to participate auction first.");

            let already_bidded = self.bids[&id].amount();
            let additional_bid = amount.amount();
            let total_bid = already_bidded + additional_bid;

            assert!(total_bid >= self.minimal_bid, "Minimal bid is {}, your total bid would be {}.", self.minimal_bid, total_bid);
            assert!(total_bid > self.current_bid, "Current bid is {}, your total bid would be {}.", self.current_bid, total_bid);

            if total_bid >= self.bin_price {
                self.ended = true;
                self.current_bid = self.bin_price;
                let overflow = total_bid - self.bin_price;
                let left_overs = self.bids.get_mut(&id).unwrap().take(overflow);
                self.gathered.put(self.bids.get_mut(&id).unwrap().take_all());
                self.gathered.put(amount);

                return (left_overs, self.product.take_all())
            }

            self.current_bid = total_bid;

            self.bids.get_mut(&id).unwrap().put(amount);

            (self.bids.get_mut(&id).unwrap().take(0), self.product.take(0))
        }

        pub fn withdraw(&mut self) -> Bucket {
            self.gathered.take_all()
        }

        pub fn withdraw_outbidded(&mut self, participant_badge: Proof) -> Bucket {
            let id = participant_badge.non_fungible::<ParticipantData>().id();

            assert!(self.bids.contains_key(&id), "You are not registered to the auction, you were never outbidded.");

            let already_bidded = self.bids[&id].amount();

            assert!(already_bidded != self.current_bid, "You are holding the current bid, you were never outbidded.");

            self.bids.get_mut(&id).unwrap().take_all()
        }

        pub fn unregister(&mut self, participant_bucket: Bucket) -> Bucket {
            assert!(self.ended, "Auction has not yet ended, you can not unregister.");

            let id = participant_bucket.non_fungible::<ParticipantData>().id();

            assert!(self.bids.contains_key(&id), "You are not registered to the auction, you do not need to unregister.");

            let rest = self.bids.get_mut(&id).unwrap().take_all();

            self.participant_minter.authorize(|| {
                let participant_nft_manager: &ResourceManager = borrow_resource_manager!(self.participant_nft_address);
                participant_nft_manager.burn(participant_bucket)
            });

            rest
        }
    }
}
