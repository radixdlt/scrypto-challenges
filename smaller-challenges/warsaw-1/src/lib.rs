use scrypto::prelude::*;

blueprint! {
    /// A struct that defines a two-party two-resource escrow blueprint.
    ///
    /// A component of this blueprint is instantiated by providing the terms of the escrow or the
    /// exchange. This is done by providing two [`ResourceSpecifier`]s which specify the amounts to
    /// be paid both parties respectively.
    ///
    /// When the component is instantiated from the blueprint, a [`Bucket`] is returned containing
    /// two NFTs which are to be given to the two respective parties. These NFTs can be freely sent
    /// around, withdrawn, deposited, and moved in any way that the person finds appropriate. This
    /// is because we would like to provide people with the ability to "transfer" the right to the
    /// funds being escrowed, if need be.
    ///
    /// When a party is ready to deposit its assets, they call the [`EscrowComponent::deposit`]
    /// function with a [`Bucket`] containing their tokens. Once both parties have deposited their
    /// funds, the escrow is considered to be fulfilled and both parties are now free to withdraw
    /// the funds of the other party through the [`EscrowComponent::withdraw`] method.
    ///
    /// There are certain features which we have chosen not to implement in this blueprint for
    /// simplicity, namely:
    ///
    /// 1. There currently is no deadline on how long the Escrow should keep accepting funds. This
    /// is an issue as one party might choose to not deposit funds when the other has already
    /// deposited. Therefore, a deadline system should be in place to allow a party to pull out of
    /// the escrow if a certain deadline is reached.
    /// 2. Fractional deposits is not currently implemented. However, it would be useful to allow
    /// people to make fractional deposits of the total amount that they need to pay and to lock it
    /// up in the escrow component.
    struct Escrow {
        /// A [`BTreeMap`] which maps the amount that should be paid to the vault that should
        /// contain that amount.
        ///
        /// # Example:
        ///
        /// If party A should pay 10 XRD and party B should pay 1 BTC, then this BTreeMap will
        /// contain two entries, one of a 10 XRD [`ResourceSpecifier`] which maps to a [`Vault`]
        /// where the 10 XRD will go to, and the other of a 1 BTC [`ResourceSpecifier`] which maps
        /// to a [`Vault`] containing 1 BTC (Once the escrow contract has been fulfilled.)
        vaults: BTreeMap<ResourceSpecifier, Vault>,

        /// Each party in the Escrow will be given an NFT which defines its obligation, or the
        /// amount of tokens that it needs to pay the Escrow to fulfill its part. This
        /// [`ResourceAddress`] is of the non-fungible token which is minted for the two parties
        /// involved in the Escrow.
        ///
        /// This resource address will be stored in the component state since we would want to
        /// verify that the NFT badges presented to the component match this [`ResourceAddress`].
        obligation_non_fungible_resource: ResourceAddress,

        /// A boolean which is used to cache whether the escrow has been fulfilled or not. When 
        /// [`true`] then this escrow has been fulfilled and all parties are allowed to withdraw
        /// the tokens owed to them. When [`false`], then this escrow has not been fulfilled yet.
        is_escrow_fulfilled: bool
    }

    impl Escrow {
        /// Instantiates a new [`Escrow`] component with the specified obligations to the two
        /// parties.
        ///
        /// This function is used to instantiate a new [`Escrow`] based on the obligations of the
        /// two parties. In other words, it creates a new component based on the amount of tokens
        /// that the two parties need to pay. The obligation that the two parties have is defined
        /// through the [`ResourceSpecifier`] enum as it allows either fungible or non-fungible
        /// resources to be defined.
        ///
        /// The logic in this function is the following:
        ///
        /// 1. Asserts that the two [`ResourceSpecifier`] passed as arguments are valid.
        /// 2. Creates the two [`EscrowObligation`] objects for the two parties of the escrow.
        /// 3. Creates the [`EscrowObligation`] resource and mints two [`EscrowObligation`] NFTs.
        /// 4. Instantiates the component based on all of the above operations.
        ///
        /// # Checks
        ///
        /// Some validation is done on the [`ResourceSpecifier`] to ensure its validity. There are
        /// two cases where this validation fails:
        ///
        /// 1. If the [`ResourceSpecifier::Fungible`] variant is used with an `amount` less than or
        /// equal to zero.
        /// 2. If the [`ResourceSpecifier::NonFungible`] variant is used with an empty set of
        /// [`NonFungibleId`]s.
        /// 3. Checks that the two resource specifiers is not the same. An exchange does not make
        /// sense between the exact same two tokens of the exact same amounts.
        ///
        /// # Arguments
        ///
        /// - `to_be_paid_by_party_1`: [`ResourceSpecifier`] - The amount of tokens that the first
        /// party needs to pay to fulfill its obligation.
        /// - `to_be_paid_by_party_2`: [`ResourceSpecifier`] - The amount of tokens that the second
        /// party needs to pay to fulfill its obligation.
        ///
        /// # Returns
        ///
        /// - [`Bucket`] - A bucket containing two non-fungible tokens which are to be given to the
        /// two parties involved in the escrow transaction. These two NFTs have the ids: 1, 2. The
        /// NFT with id 1 is to be given to the first party, and the NFT of id 2 is to be given to
        /// the second party.
        pub fn instantiate_escrow(
            to_be_paid_by_party_1: ResourceSpecifier,
            to_be_paid_by_party_2: ResourceSpecifier,
        ) -> (ComponentAddress, Bucket) {
            assert!(
                to_be_paid_by_party_1.validate().is_ok(),
                "First resource is not ok"
            );
            assert!(
                to_be_paid_by_party_1.validate().is_ok(),
                "Second resource is not ok"
            );
            assert_ne!(
                to_be_paid_by_party_1, to_be_paid_by_party_2,
                "can not equal"
            );

            let party_1_obligation: EscrowObligation = EscrowObligation {
                amount_to_pay: to_be_paid_by_party_1.clone(),
                amount_to_get: to_be_paid_by_party_2.clone()
            };

            let party_2_obligation: EscrowObligation = EscrowObligation {
                amount_to_pay: to_be_paid_by_party_2.clone(),
                amount_to_get: to_be_paid_by_party_1.clone()
            };

            let escrow_obligations: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Escrow Obligation")
                .metadata("symbol", "ESCROW")
                .metadata("description", "Escrow desc")
                .metadata("team-member-1-github-username", "marcinklet")
                .metadata("team-member-2-github-username", "mosrafal")
                .metadata("team-member-3-github-username", "DaniusKalv")
                .metadata("team-member-4-github-username", "JakubG777")
                .initial_supply([
                    (
                        NonFungibleId::from_u32(1),
                        party_1_obligation
                    ),
                    (
                        NonFungibleId::from_u32(2),
                        party_2_obligation
                    ),
                ]);

            
            let mut vaults: BTreeMap<ResourceSpecifier, Vault> = BTreeMap::new();
            vaults.insert(
                to_be_paid_by_party_1.clone(),
                Vault::new(to_be_paid_by_party_1.resource_address())
            );
            vaults.insert(
                to_be_paid_by_party_2.clone(),
                Vault::new(to_be_paid_by_party_2.resource_address())
            );

            let component_address: ComponentAddress = Self {
                vaults,
                obligation_non_fungible_resource: escrow_obligations.resource_address(),
                is_escrow_fulfilled: false
            }
            .instantiate()
            .globalize();

            (component_address, escrow_obligations)

        }

        /// Deposits funds into the escrow by one of the parties.
        ///
        /// This method is used to deposit funds into the escrow component by one of the parties of
        /// the escrow.
        ///
        /// After performing the below described checks, this method does the following:
        ///
        /// 1. Loads the [`EscrowObligation`] data in the passed NFT. This data is to be used by the
        /// function to determine if the passed `funds` are of the required [`ResourceAddress`] and
        /// `amount` or not.
        /// 2. Based on the `amount_to_pay` specified on the [`EscrowObligation`], these funds are
        /// taken from the bucket and put into the vault.
        /// 3. Any remaining funds are returned back to the caller.
        ///
        /// # Checks
        ///
        /// This method performs a number of checks before the deposit is performed into the
        /// component's vaults
        ///
        /// 1. Checks that the passed `obligation_badge`'s resource address matches the
        /// `obligation_non_fungible_resource` stored in the component state.
        /// 2. Checks that the `funds` passed in the bucket match the resource address that is
        /// required by the obligation nft. Fun fact: we can actually skip this validation and check
        /// because the Radix Engine can catch this for us. If we try to deposit funds of one
        /// resource address into a vault of another resource address, this leads to a runtime
        /// error.
        /// 3. To avoid any user errors, it checks that this obligation has not yet been fulfilled.
        /// As in, it makes sure that this party did not already pay its obligation.
        ///
        /// # Note
        ///
        /// At the current moment of time, this method does not support partial payment. This is a
        /// decision made to simplify this problem and make it easier to think and reason about.
        /// However, a production Escrow will defiantly benefit from having partial deposits.
        ///
        /// # Arguments
        ///
        /// - `obligation_proof`: [`Proof`] - A proof containing the obligation badge that defines
        /// the party's obligation to the escrow and it is owed (this is the obligation NFT).
        /// - `funds` [`Bucket`] - A bucket of the funds to deposit into the escrow. The contents of
        /// this bucket need to match specified in the NFT's `amount_to_pay`.
        ///
        /// # Returns
        ///
        /// [`Bucket`] - A bucket containing any excess tokens that were sent to this method.
        pub fn deposit(&mut self, obligation_badge: Proof, mut funds: Bucket) -> Bucket {
            // TODO: Complete this function yourself.
            let obligation_badge: ValidatedProof = obligation_badge
                .validate_proof(self.obligation_non_fungible_resource)
                .expect("Invalid badge");

            let obligation: EscrowObligation = obligation_badge.non_fungible().data();
            let vault: &mut Vault = self.vaults.get_mut(&obligation.amount_to_pay).unwrap();

            let funds_to_deposit: Bucket = match obligation.amount_to_pay {
                ResourceSpecifier::Fungible { amount, .. } => funds.take(amount),
                ResourceSpecifier::NonFungible { non_fungible_ids, .. } => funds.take_non_fungibles(&non_fungible_ids),
            };

            vault.put(funds_to_deposit);
            funds
        }

        /// Withdraws funds from the escrow after both parties have deposited their funds.
        ///
        /// This function is used to withdraw the amount owed to each party after the escrow has
        /// obtained the funds agreed on by both parties. After it does all necessary checks and 
        /// after it ensures that the escrow has been fulfilled, the following logic takes place:
        ///
        /// 1. Loads the [`EscrowObligation`] data in the passed NFT. This is needed to get the
        /// `amount_to_get` data and its corresponding [`Vault`].
        /// 2. Take all of the funds from the `amount_to_get` vault and returns it back to the
        /// caller.
        ///
        /// # Checks
        ///
        /// This method performs a number of checks before the withdraw is performed:
        ///
        /// 1. Checks that the escrow has been concluded and fulfilled (i.e. that all of the
        /// parties) have sent their required amount.
        /// 2. Checks that the passed `obligation_badge`'s resource address matches the
        /// `obligation_non_fungible_resource` stored in the component state.
        ///
        /// # Arguments
        ///
        /// - `obligation_proof`: [`Proof`] - A proof containing the obligation badge that defines
        /// the party's obligation to the escrow and it is owed (this is the obligation NFT).
        ///
        /// # Returns
        ///
        /// [`Bucket`] - A bucket containing the owed tokens.
        pub fn withdraw(&mut self, obligation_badge: Proof) -> Bucket {
            // TODO: Complete this function yourself.
            //todo!()
            assert!(
                self.is_escrow_fulfilled(),
                "error in withdraw"
            );

            let obligation_badge: ValidatedProof = obligation_badge
                .validate_proof(self.obligation_non_fungible_resource)
                .expect("invalid badge provider");

            let obligation: EscrowObligation = obligation_badge.non_fungible().data();
            let vault: &mut Vault = self.vaults.get_mut(&obligation.amount_to_get).unwrap();
            vault.take_all()
        }

        /// Checks if the escrow is fulfilled or not and returns a boolean output.
        ///
        /// This function checks the `vaults` state variable on the component to see if the escrow
        /// has been fulfilled from both sides or not. It is said to be fulfilled if each of the
        /// vaults in the `vaults` state variable contains the amount specified by the key (the
        /// [`ResourceSpecifier`]) in the mapping.
        ///
        /// # Assumptions
        ///
        /// This method makes no assumption on whether the deposit methods reject extra tokens sent
        /// or not. With this in mind, all of the operations do not check for strict equality but
        /// check that the amount required **OR MORE** is provided. For the case of NFTs and their
        /// [`NonFungibleId`]s, then this function checks that the set of [`NonFungibleId`]s in the
        /// [`Vault`] is a subset of the [`NonFungibleId`]s required by the [`ResourceSpecifier`].
        ///
        /// # Returns
        ///
        /// [`bool`] - [`true`] if the escrow is fulfilled from all sides. [`false`] if it has not
        /// been fulfilled.
        pub fn is_escrow_fulfilled(&mut self) -> bool {
            if self.is_escrow_fulfilled {
                self.is_escrow_fulfilled
            } else {
                self.is_escrow_fulfilled = self.vaults
                    .iter()
                    .map(|(resource_specifier, vault)| {
                        match resource_specifier {
                            // If this is a fungible resource specifier, then check that the resource
                            // address and the amount both match.
                            ResourceSpecifier::Fungible {
                                resource_address,
                                amount,
                            } => {
                                vault.resource_address() == *resource_address
                                    && vault.amount() >= *amount
                            }

                            // If this is a non-fungible resource specifier then check that the resource
                            // address matches and that the set of non-fungible ids in the specifier is
                            // a subset of those in the vault.
                            ResourceSpecifier::NonFungible {
                                resource_address,
                                non_fungible_ids,
                            } => {
                                vault.resource_address() == *resource_address
                                    && vault
                                        .non_fungible_ids()
                                        .iter()
                                        .all(|x| non_fungible_ids.contains(x))
                            }
                        }
                    })
                    .all(|x| x);
                self.is_escrow_fulfilled
            }
            
        }
    }
}

/// A struct that defines the [`NonFungibleData`] of the NFTs that are given to the two parties of
/// the escrow.
///
/// This struct defines the obligation of this party and the obligation of the other party. In other
/// terms, this struct defines the amount of tokens that this party needs to pay and the amount of
/// tokens that the other party needs to pay.
#[derive(Debug, NonFungibleData)]
pub struct EscrowObligation {
    /// The amount of tokens which this party needs to pay to the other party.
    amount_to_pay: ResourceSpecifier,
    /// The amount of tokens paid by the other party to this party.
    amount_to_get: ResourceSpecifier,
}

/// An enum used to specify a specific amount of a given resource or specific [`NonFungibleId`]s of
/// a resource based on the type of the resource.
///
/// The main use of this enum is in specifying the amount of tokens that each party owes to the
/// other.
#[derive(Debug, TypeId, Encode, Decode, Describe, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum ResourceSpecifier {
    /// A variant used to specify the amount of a fungible resource through the [`ResourceAddress`]
    /// of the resource the amount of that resource as a [`Decimal`].
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal,
    },
    /// A variant used to specify non-fungible of that resource based on the [`ResourceAddress`] of
    /// the resource and a set of the [`NonFungibleId`]s being specified by the enum.
    NonFungible {
        resource_address: ResourceAddress,
        non_fungible_ids: BTreeSet<NonFungibleId>,
    },
}

impl ResourceSpecifier {
    /// Performs validation on a resource specifier to ensure that it makes sense.
    ///
    /// This method performs validation on [`ResourceSpecifier`]s to validate that they specify
    /// amounts that make sense. The two main validations performed by this method are:
    ///
    /// 1. Validating that the `amount` field on [`ResourceSpecifier::Fungible`] is greater than or
    /// equal to zero.
    /// 2. Validating that the `non_fungible_ids` field on [`ResourceSpecifier::NonFungible`] is not
    /// empty.
    ///
    /// There are other validations which can be added to this to ensure that no [`EscrowComponent`]
    /// component can be created with invalid [`ResourceSpecifier`]s. Such as [`ResourceAddress`]
    /// validations, divisibility validations for fungible tokens, and existence validations for
    /// non-fungible tokens. However, those are not implemented in this method to keep it simple.
    ///
    /// # Returns:
    ///
    /// - [`Result<(), ()>`] - A result type that returns `Unit` in both the [`Result::Ok`] and
    /// [`Result::Err`] cases. When [`Result::Ok`] is returned, then the validation has succeeded,
    /// if [`Result::Err`] is returned then the validation has failed.
    pub fn validate(&self) -> Result<(), ()> {
        match self {
            Self::Fungible { amount, .. } => {
                if *amount <= Decimal::zero() {
                    Err(())
                } else {
                    Ok(())
                }
            }
            Self::NonFungible {
                non_fungible_ids, ..
            } => {
                if non_fungible_ids.is_empty() {
                    Err(())
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Gets the resource address of the specified resource.
    ///
    /// # Returns
    ///
    /// [`ResourceAddress`] - The resource address of the specified resource.
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible {
                resource_address, ..
            }
            | Self::NonFungible {
                resource_address, ..
            } => *resource_address,
        }
    }
}
