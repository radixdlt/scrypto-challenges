use scrypto::prelude::*;

blueprint! {
    /// This blueprint defines the state and logic involved in a dutch auction non-fungible token sale. People who
    /// instantiate components from this blueprint, signify their intent at selling their NFT(s) at a price which
    /// reduces over time.
    ///
    /// This blueprint allows multiple NFTs to be sold at once as a collection instead of requiring that these NFTs
    /// be sold separately. In addition to that, this blueprint allows XRD payments as well as non-XRD payments for
    /// sellers who opt to accept non-XRD tokens.
    struct DutchAuction {
        /// These are the vaults where the NFTs will be stored. Since this blueprint allows for multiple NFTs to be sold
        /// at once, this HashMap is used to store all of these NFTs with the hashmap key being the resource address of
        /// these NFTs if they are not all of the same _kind_.
        nft_vaults: HashMap<ResourceAddress, Vault>,

        /// This is the vault which stores the payment of the NFTs once it has been made. This vault may contain XRD or
        /// other tokens depending on the `ResourceAddress` of the accepted payment token specified by the instantiator
        payment_vault: Vault,

        /// This blueprint accepts XRD as well as non-XRD payments. This variable here is the resource address of the
        /// fungible token that will be used for payments to the component.
        accepted_payment_token: ResourceAddress,

        /// This is the starting price of the bundle of NFTs being sold. This price is in the `accepted_payment_token`
        /// which could be XRD or any other fungible token.
        starting_price: Decimal,

        /// This is the ending price of the bundle of NFTs being sold. There is a need for an ending price as this is
        /// a dutch auction. When the end_epoch is reached, the price of the NFT bundle will be the `ending_price`.
        ending_price: Decimal,

        /// This is the epoch when the sale begins.
        starting_epoch: u64,

        /// This is the ending epoch. When this epoch is reached or exceeded, the price of the NFT bundle will be the
        /// `ending_price`.
        ending_epoch: u64,
    }

    impl DutchAuction {
        /// Instantiates a new dutch auction sale for the passed NFTs.
        ///
        /// This function is used to instantiate a new dutch auction sale for the passed bucket of NFTs. The auction
        /// can be done for a single NFT or a bundle of NFTs which the seller intends to sell together. The tokens
        /// may be sold for XRD or for any other fungible token of the instantiator's choosing.
        ///
        /// This function performs a number of checks before the `DutchAuction` component is created:
        ///
        /// * **Check 1:** Checks that the passed buckets of tokens are all non-fungible tokens.
        /// * **Check 2:** Checks that the `accepted_payment_token` is a fungible token.
        /// * **Check 3:** Checks that the starting price is non-negative.
        /// * **Check 4:** Checks that the ending price is non-negative.
        /// * **Check 5:** Checks that the ending price is less than the starting price.
        /// * **Check 5:** Checks that the ending epoch has not yet passed.
        ///
        /// # Arguments:
        ///
        /// * `non_fungible_tokens` (Vec<Bucket>) - A vector of buckets of the non-fungible tokens that the instantiator
        /// wishes to sell.
        /// * `accepted_payment_token` (ResourceAddress) - Payments may be accepted in XRD or non-XRD tokens. This
        /// argument specifies the resource address of the token the instantiator wishes to accept for payment.
        /// * `starting_price` (Decimal) - The starting price of the NFT bundle sale.
        /// * `ending_price` (Decimal) - The ending price of the NFT bundle sale.
        /// * `relative_ending_epoch` (u64) - This is the relative ending epoch, meaning that this value will be added
        /// with the current epoch. This argument controls the rate at which the price of the bundle decreases. When
        /// the ending epoch is reached, the price will reach its minimum that was specified in the arguments.
        ///
        /// # Returns:
        ///
        /// This function returns a tuple which has the following format:
        /// * `ComponentAddress` - A component address of the instantiated `DutchAuction` component.
        /// * `Bucket` - A bucket containing an ownership badge which entitles the holder to the assets in this
        /// component.
        pub fn instantiate_dutch_auction(
            non_fungible_tokens: Vec<Bucket>,
            accepted_payment_token: ResourceAddress,
            starting_price: Decimal,
            ending_price: Decimal,
            relative_ending_epoch: u64,
        ) -> (ComponentAddress, Bucket) {
            // Performing checks to ensure that the creation of the component can go through
            assert!(
                !non_fungible_tokens
                    .iter()
                    .any(
                        |x| borrow_resource_manager!(x.resource_address()).resource_type()
                            != ResourceType::NonFungible
                    ),
                "[Instantiation]: Can not perform a sale for fungible tokens."
            );
            assert!(
                borrow_resource_manager!(accepted_payment_token).resource_type()
                    != ResourceType::NonFungible,
                "[Instantiation]: Only payments of fungible resources are accepted."
            );
            assert!(
                starting_price >= Decimal::zero(),
                "[Instantiation]: The starting price of the tokens can not be a negative number."
            );
            assert!(
                starting_price > ending_price,
                "[Instantiation]: The starting price must be greater than the ending price."
            );
            assert!(
                relative_ending_epoch > Runtime::current_epoch(),
                "[Instantiation]: The ending epoch has already passed."
            );

            // At this point we know that the component creation can go through.

            // Create a new HashMap of vaults and aggregate all of the tokens in the buckets into the vaults of this
            // HashMap. This means that if somebody passes multiple buckets of the same resource, then they would end
            // up in the same vault.
            let mut nft_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            for bucket in non_fungible_tokens.into_iter() {
                nft_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            // When the owner of the NFT(s) instantiates a new dutch auction sale component, their tokens are taken away
            // from them and they're given an ownership NFT which is used to authenticate them and as proof of ownership
            // of the NFTs. This ownership badge can be used to either withdraw the funds from the token sale or the
            // NFTs if the seller is no longer interested in selling their tokens.
            let ownership_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Ownership Badge")
                .metadata(
                    "description",
                    "An ownership badge used to authenticate the owner of the NFT(s).",
                )
                .metadata("symbol", "OWNER")
                .initial_supply(1);

            // Setting up the access rules for the component methods such that only the owner of the ownership badge can
            // make calls to the protected methods.
            let access_rule: AccessRule = rule!(require(ownership_badge.resource_address()));
            let access_rules: AccessRules = AccessRules::new()
                .method("cancel_sale", access_rule.clone())
                .method("withdraw_payment", access_rule.clone())
                .default(rule!(allow_all));

            // Instantiating the dutch auction sale component
            let dutch_auction_sale: ComponentAddress = Self {
                nft_vaults,
                payment_vault: Vault::new(accepted_payment_token),
                accepted_payment_token,
                starting_price,
                ending_price,
                starting_epoch: Runtime::current_epoch(),
                ending_epoch: Runtime::current_epoch() + relative_ending_epoch,
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();

            return (dutch_auction_sale, ownership_badge);
        }

        /// Used for buying the NFT(s) controlled by this component.
        ///
        /// This method takes in the payment for the non-fungible tokens being sold, verifies that the payment matches
        /// the expected resource addresses and amounts and then permits the exchange of NFTs and payment.
        ///
        /// This method performs a number of checks before the purchase goes through:
        ///
        /// * **Check 1:** Checks that the payment was provided in the required token.
        /// * **Check 1:** Checks that enough tokens were provided to cover the price of the NFT(s).
        ///
        /// # Arguments:
        ///
        /// * `payment` (Bucket) - A bucket of the tokens used for the payment
        ///
        /// # Returns:
        ///
        /// * `Vec<Bucket>` - A vector of buckets of the non-fungible tokens which were being sold.
        pub fn buy(&mut self, mut payment: Bucket) -> Vec<Bucket> {
            // Checking if the appropriate amount of the payment token was provided before approving the token sale
            assert_eq!(
                payment.resource_address(),
                self.accepted_payment_token,
                "[Buy]: Invalid tokens were provided as payment. Payment are only allowed in {}",
                self.accepted_payment_token
            );
            assert!(
                payment.amount() >= self.price().1,
                "[Buy]: Invalid quantity was provided. This sale can only go through when {} tokens are provided.",
                self.price().1
            );

            // At this point we know that the sale of the tokens can go through.

            // Taking the price of the NFT(s) and putting it in the payment vault
            self.payment_vault.put(payment.take(self.price().1));

            // Creating a vector of buckets of all of the NFTs that the component has, then adding to it the remaining
            // tokens from the payment
            let resource_addresses: Vec<ResourceAddress> =
                self.nft_vaults.keys().cloned().collect();
            let mut tokens: Vec<Bucket> = vec![payment];
            for resource_address in resource_addresses.into_iter() {
                tokens.push(
                    self.nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            return tokens;
        }

        /// Cancels the sale of the tokens and returns the tokens back to their owner.
        ///
        /// This method performs a single check before canceling the sale:
        ///
        /// * **Check 1:** Checks that the tokens have not already been sold.
        ///
        /// # Returns:
        ///
        /// * `Vec<Bucket>` - A vector of buckets of the non-fungible tokens which were being sold.
        ///
        /// # Note:
        ///
        /// * This is an authenticated method which may only be called by the holder of the `ownership_badge`.
        /// * There is no danger in not checking if the sale has occurred or not and attempting to return the tokens
        /// anyway. This is because we literally lose the tokens when they're sold so even if we attempt to give them
        /// back after they'd been sold we return a vector of empty buckets.
        pub fn cancel_sale(&mut self) -> Vec<Bucket> {
            // Checking if the tokens have been sold or not.
            assert!(
                !self.is_sold(),
                "[Cancel Sale]: Can not cancel the sale after the tokens have been sold"
            );

            // Taking out all of the tokens from the vaults and returning them back to the caller.
            let resource_addresses: Vec<ResourceAddress> =
                self.nft_vaults.keys().cloned().collect();
            let mut tokens: Vec<Bucket> = Vec::new();
            for resource_address in resource_addresses.into_iter() {
                tokens.push(
                    self.nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            return tokens;
        }

        /// Withdraws the payment owed from the sale.
        ///
        /// This method performs a single check before canceling the sale:
        ///
        /// * **Check 1:** Checks that the tokens have already been sold.
        ///
        /// # Returns:
        ///
        /// * `Bucket` - A bucket containing the payment.
        ///
        /// # Note:
        ///
        /// * This is an authenticated method which may only be called by the holder of the `ownership_badge`.
        /// * There is no danger in not checking if the sale has occurred or not and attempting to return the tokens
        /// anyway. If we do not have the payment tokens then the worst case scenario would be that an empty bucket is
        /// returned. This is bad from a UX point of view but does not pose any security risk.
        pub fn withdraw_payment(&mut self) -> Bucket {
            // Checking if the tokens have been sold or not.
            assert!(
                self.is_sold(),
                "[Withdraw Payment]: Can not withdraw the payment when no sale has happened yet."
            );
            return self.payment_vault.take_all();
        }

        // =============================================================================================================
        // The following are read-only methods which query the state of the dutch auction sale and information about it
        // without performing any state changes. These are useful when connecting a web interface with the component.
        // =============================================================================================================

        /// Returns the price of the tokens being sold.
        ///
        /// # Returns:
        ///
        /// This method returns a tuple in the following format
        ///
        /// * `ResourceAddress` - The resource address of the accepted payment token.
        /// * `Decimal` - A decimal value of the price of the NFT(s) in terms of the `accepted_payment_token`.
        pub fn price(&self) -> (ResourceAddress, Decimal) {
            let gradient: Decimal = (self.ending_price - self.starting_price)
                / (self.ending_epoch - self.starting_epoch);
            return (
                self.accepted_payment_token,
                std::cmp::max(
                    self.ending_price,
                    gradient * (Runtime::current_epoch() - self.starting_epoch)
                        + self.starting_price,
                ),
            );
        }

        /// Checks if the NFTs have been sold or not.
        ///
        /// This method checks whether the NFTs have been sold or not through the `payment_vault`. If the payment vault
        /// is empty then it means that the tokens have not been sold. On the other hand, if there are funds in the
        /// payment vault then the exchange has gone through and the tokens have been sold.
        ///
        /// # Returns:
        ///
        /// * `bool` - A boolean of whether the tokens have been sold or not. Returns `true` if the tokens have been
        /// sold and `false` if they have not been sold.
        pub fn is_sold(&self) -> bool {
            return !self.payment_vault.is_empty();
        }

        /// Returns a HashMap of the NFTs being sold through this component.
        ///
        /// This method returns a `HashMap` of the NFTs being sold through this component. The key of the HashMap is the
        /// `ResourceAddress` of the resource and the value is a vector of `NonFungibleIds` belonging to this
        /// `ResourceAddress` that are being sold.
        ///
        /// # Returns:
        ///
        /// * `bool` - A HashMap of the non-fungible-ids of the tokens being sold.
        pub fn non_fungible_ids(&self) -> HashMap<ResourceAddress, Vec<NonFungibleId>> {
            // Creating the hashmap which we will use to store the resource addresses and the non-fungible-ids.
            let mut mapping: HashMap<ResourceAddress, Vec<NonFungibleId>> = HashMap::new();

            // Adding the entires to the mapping
            let resource_addresses: Vec<ResourceAddress> =
                self.nft_vaults.keys().cloned().collect();
            for resource_address in resource_addresses.into_iter() {
                mapping.insert(
                    resource_address.clone(),
                    self.nft_vaults
                        .get(&resource_address)
                        .unwrap()
                        .non_fungible_ids()
                        .into_iter()
                        .collect::<Vec<NonFungibleId>>(),
                );
            }

            return mapping;
        }

        /// Returns a `NonFungibleAddress` vector of the NFTs being sold.
        ///
        /// # Returns:
        ///
        /// * `Vec<NonFungibleAddress>` - A Vector of `NonFungibleAddress`es of the NFTs being sold.
        pub fn non_fungible_addresses(&self) -> Vec<NonFungibleAddress> {
            // Creating the vector which will contain the NonFungibleAddresses of the tokens
            let mut vec: Vec<NonFungibleAddress> = Vec::new();

            // Iterate over the items in the hashmap of non-fungible-ids and create the `NonFungibleAddress`es through
            // them
            for (resource_address, non_fungible_ids) in self.non_fungible_ids().iter() {
                vec.append(
                    &mut non_fungible_ids
                        .iter()
                        .map(|x| NonFungibleAddress::new(resource_address.clone(), x.clone()))
                        .collect::<Vec<NonFungibleAddress>>(),
                )
            }

            return vec;
        }
    }
}
