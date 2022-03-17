use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct HelperNFT {}

blueprint! {
    struct Helper{}

    impl Helper {
        /// Simple helper function to mint a new NFT with specified key (note the
        /// Resource will be new every time)
        pub fn new_nft(symbol: String, keys: BTreeSet<NonFungibleKey>) -> Bucket {
            info!("at new_nft");
            info!("symbol: {}", symbol);
            info!("keys: {:?}", keys);
            let keys = keys.into_iter().map(|key| (key, HelperNFT{}));
            info!("keys: {:?}", keys);
            let b = ResourceBuilder::new_non_fungible()
                .metadata("symbol", symbol)
                .initial_supply_non_fungible(keys);
            info!("b: {:?}", b);
            b
        }
    }
}
