//! DAO Component

use scrypto::{blueprint, external_component};

// Import the Proposal component
external_component! {
    ProposalLocalComponent {
        fn is_voting_stage(&self) -> bool;
        fn execute(&mut self) -> Option<ProposedChange>;
    }
}

// Import the Router component
external_component! {
    RouterLocalComponent {
        fn create_pool(&mut self, token: ResourceAddress, initial_rate: Decimal, min_rate: Decimal, max_rate: Decimal);
        fn claim_protocol_fees(&mut self) -> Vec<Bucket>;
    }
}

// Import the Issuer component
external_component! {
    IssuerLocalComponent {
        fn new_lender(&mut self, collateral_address: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal, oracle: ComponentAddress);
        fn change_lender_parameters(&mut self, lender_collateral: ResourceAddress, loan_to_value: Decimal, interest_rate: Decimal, liquidation_threshold: Decimal, liquidation_incentive: Decimal);
        fn change_lender_oracle(&mut self, lender_collateral: ResourceAddress, oracle: ComponentAddress);
        fn give_tokens(&mut self, tokens: Vec<Bucket>);
    }
}

#[blueprint]
mod dao {
    use crate::proposal::ProposalComponent;
    use crate::proposal_receipt::ProposalReceipt;
    use crate::proposed_change::ProposedChange;
    use crate::utils::get_current_time;
    use crate::voter_card::VoterCard;
    use stoichiometric_dex::position::Position;
    use stoichiometric_dex::router::RouterComponent;
    use stoichiometric_stablecoin::issuer::IssuerComponent;

    pub struct Dao {
        dex_router: ComponentAddress,
        stablecoin_issuer: ComponentAddress,
        stablecoin_address: ResourceAddress,
        stablecoin_minter: Vault,
        position_address: ResourceAddress,
        voter_card_address: ResourceAddress,
        voter_card_id: u64,
        proposal_receipt_address: ResourceAddress,
        resource_minter: Vault,
        protocol_admin_badge: Vault,
        proposals: HashMap<u64, ComponentAddress>,
        proposal_id: u64,
        locked_stablecoins: Vault,
        locked_positions: Vault,
        total_voting_power: Decimal,
        vote_period: i64,
        vote_validity_threshold: Decimal,
        reserves: HashMap<ResourceAddress, Vault>,
    }

    impl Dao {
        /// Instantiates and globalizes a new [`Dao`] and returns its address.
        ///
        /// We first need to instatiate a Lender. We also need an external oracle to start lending because the AMM is going to be empty. We chose to use Beaker.
        ///
        /// # Arguments
        /// * `vote_period` - Length of a vote.
        /// * `vote_validity_threshold` - Minimum votes for required for the proposal to be considered as valid.
        /// * `initial_collateral_token` - The initial token that can be used as collateral.
        /// * `iloan_to_value` - Loan to value of the first Lender.
        /// * `interest_rate` - Daily interest rate of a loan.
        /// * `liquidiation_threshold` - Collateralisation ratio from where you can be liquidated.
        /// * `liquidiation_penalty` - Penalty when you are liquidated.
        /// * `oracle` - Address of the oracle used at first.
        /// * `initial_rate` - Initial rate of the AMM.
        /// * `min_rate` - Minimum rate.
        /// * `max_rate` - Maximum rate.
        pub fn new(
            vote_period: i64,
            vote_validity_threshold: Decimal,
            initial_collateral_token: ResourceAddress,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
            oracle: ComponentAddress,
            initial_rate: Decimal,
            min_rate: Decimal,
            max_rate: Decimal,
        ) -> ComponentAddress {
            assert!(
                vote_validity_threshold.is_positive() && vote_validity_threshold < Decimal::ONE,
                "The validity threshold should be included in the range 0< <1"
            );

            // Creates the protocol admin badge which will control everything
            let protocol_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Stoichiometric protocol admin badge")
                .burnable(rule!(allow_all), AccessRule::DenyAll)
                .mint_initial_supply(Decimal::ONE);

            // Creates the stablecoin minter
            let mut stablecoin_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Stoichiometric stablecoin minter")
                .mintable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .recallable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .mint_initial_supply(2);

            // Creates the stablecoin resource
            let stablecoin_address = ResourceBuilder::new_fungible()
                .divisibility(18)
                .mintable(
                    rule!(require(stablecoin_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(stablecoin_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_metadata(
                    rule!(require(stablecoin_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .metadata("name", "Stoichiometric USD")
                .metadata("symbol", "SUSD")
                .metadata(
                    "icon",
                    "https://cdn-icons-png.flaticon.com/512/3215/3215346.png",
                )
                .create_with_no_initial_supply();

            // Creates the VoterCard and ProposalReceipt minter
            let resource_minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mintable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .recallable(
                    rule!(require(protocol_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .mint_initial_supply(Decimal::ONE);

            let voter_card_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric voter card")
                .mintable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .updateable_non_fungible_data(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            let proposal_receipt_address = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "Stoichiometric proposal receipt")
                .mintable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .burnable(
                    rule!(require(resource_minter.resource_address())),
                    AccessRule::DenyAll,
                )
                .create_with_no_initial_supply();

            let (dex_router, position_address) =
                RouterComponent::new(protocol_admin_badge.resource_address(), stablecoin_address);
            let stablecoin_issuer = IssuerComponent::new(
                protocol_admin_badge.resource_address(),
                stablecoin_minter.take(1),
                stablecoin_address,
            );

            let dao_rules = AccessRules::new().default(rule!(allow_all), AccessRule::DenyAll);

            let mut component = Self {
                dex_router,
                stablecoin_issuer,
                stablecoin_address: stablecoin_address.clone(),
                stablecoin_minter: Vault::with_bucket(stablecoin_minter),
                position_address: position_address.clone(),
                voter_card_address,
                voter_card_id: 0,
                proposal_receipt_address,
                resource_minter: Vault::with_bucket(resource_minter),
                protocol_admin_badge: Vault::with_bucket(protocol_admin_badge),
                proposals: HashMap::new(),
                proposal_id: 0,
                locked_stablecoins: Vault::new(stablecoin_address),
                locked_positions: Vault::new(position_address),
                total_voting_power: Decimal::ZERO,
                vote_period,
                vote_validity_threshold,
                reserves: HashMap::new(),
            };
            component.add_collateral_token(
                initial_collateral_token,
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                oracle,
                initial_rate,
                min_rate,
                max_rate,
            );
            let mut component = component.instantiate();

            component.add_access_check(dao_rules);

            component.globalize()
        }

        /// Lock stablecoins in a votercard
        ///
        /// # Arguments
        /// * `stablecoins` - Bucket of stablecoins.
        /// * `opt_voter_card_proof` - Proof of the voter card.
        pub fn lock_stablecoins(
            &mut self,
            stablecoins: Bucket,
            opt_voter_card_proof: Option<Proof>,
        ) -> Option<Bucket> {
            assert!(
                stablecoins.resource_address() == self.stablecoin_address,
                "You can only lock stablecoins as fungible resource"
            );

            let (mut voter_card, voter_card_id, opt_bucket) =
                self.get_voter_card_or_create(opt_voter_card_proof);

            self.total_voting_power += voter_card.add_stablecoins(stablecoins.amount());
            self.locked_stablecoins.put(stablecoins);
            self.update_voter_card_data(&voter_card_id, voter_card);

            opt_bucket
        }

        pub fn lock_positions(
            &mut self,
            positions: Bucket,
            opt_voter_card_proof: Option<Proof>,
        ) -> Option<Bucket> {
            assert!(
                positions.resource_address() == self.position_address,
                "You can only lock positions as non fungible resource"
            );

            let (mut voter_card, voter_card_id, opt_bucket) =
                self.get_voter_card_or_create(opt_voter_card_proof);

            for position in positions.non_fungibles::<Position>() {
                let id = position.local_id().clone();
                let position_data = self.get_position_data(&id);

                self.total_voting_power += voter_card.add_position(&position_data, id);
            }

            self.locked_positions.put(positions);
            self.update_voter_card_data(&voter_card_id, voter_card);

            opt_bucket
        }

        pub fn unlock(&mut self, voter_card: Bucket) -> (Bucket, Bucket) {
            assert!(
                voter_card.resource_address() == self.voter_card_address,
                "Please provide bucket with voter cards inside"
            );
            assert!(
                voter_card.amount() == Decimal::ONE,
                "Please provide only one voter_card"
            );

            let voter_card_nfr = voter_card.non_fungible::<VoterCard>();
            let voter_card_data = self.get_voter_card_data(voter_card_nfr.local_id());
            match self.proposals.get(&voter_card_data.last_proposal_voted_id) {
                None => {
                    // If the proposal id is 0 and no proposals have been made, we can be in this
                    // edge casE. Hence, we do nothing
                }
                Some(proposal) => {
                    let last_proposal_voted = ProposalLocalComponent::at(*proposal);
                    assert!(!last_proposal_voted.is_voting_stage(), "Cannot unlock tokens and positions from a VoterCard that is actively particpating in a vote!");
                }
            };

            self.total_voting_power -= voter_card_data.voting_power;
            let stablecoin_bucket = self
                .locked_stablecoins
                .take(voter_card_data.stablecoins_locked);
            let mut positions_bucket = Bucket::new(self.position_address);
            for position_id in voter_card_data.positions_locked_ids {
                positions_bucket.put(self.locked_positions.take_non_fungible(&position_id));
            }

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.voter_card_address).burn(voter_card);
            });

            (stablecoin_bucket, positions_bucket)
        }

        pub fn make_proposal(&mut self, proposed_change: ProposedChange) -> Bucket {
            let current_time = get_current_time();
            let vote_end = current_time + self.vote_period;
            let vote_threshold = self.total_voting_power * self.vote_validity_threshold;

            let voter_card_updater = self.protocol_admin_badge.authorize(|| {
                borrow_resource_manager!(self.resource_minter.resource_address()).mint(Decimal::ONE)
            });

            let proposal_comp = ProposalComponent::new(
                self.proposal_id,
                vote_end,
                vote_threshold,
                proposed_change,
                self.voter_card_address,
                voter_card_updater,
                self.protocol_admin_badge.resource_address(),
            );

            self.proposals.insert(self.proposal_id, proposal_comp);

            let receipt_data = ProposalReceipt {
                proposal_id: self.proposal_id,
            };
            let receipt = self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.proposal_receipt_address).mint_non_fungible(
                    &NonFungibleLocalId::Integer(self.proposal_id.into()),
                    receipt_data,
                )
            });

            self.proposal_id += 1;

            receipt
        }

        pub fn claim_dex_protocol_fees(&mut self) {
            let fees = self
                .protocol_admin_badge
                .authorize(|| RouterLocalComponent::at(self.dex_router).claim_protocol_fees());

            for bucket in fees {
                self.put_in_reserves(bucket);
            }
        }

        pub fn execute_proposal(&mut self, proposal_receipt: Bucket) -> Option<Vec<Bucket>> {
            assert!(
                proposal_receipt.resource_address() == self.proposal_receipt_address,
                "Please provide a proposal receipt"
            );
            assert!(
                proposal_receipt.amount() == Decimal::ONE,
                "Can only execute one proposal at a time"
            );

            let proposal_data: ProposalReceipt =
                borrow_resource_manager!(self.proposal_receipt_address).get_non_fungible_data(
                    proposal_receipt
                        .non_fungible::<ProposalReceipt>()
                        .local_id(),
                );
            let mut proposal = self.get_proposal(proposal_data.proposal_id);

            let changes_to_execute = self.protocol_admin_badge.authorize(|| proposal.execute());

            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.proposal_receipt_address).burn(proposal_receipt)
            });

            match changes_to_execute {
                None => None,
                Some(changes) => self.execute_proposed_change(changes),
            }
        }

        #[inline]
        pub fn put_in_reserves(&mut self, bucket: Bucket) {
            match self.reserves.get_mut(&bucket.resource_address()) {
                Some(vault) => vault.put(bucket),
                None => {
                    let new_vault = Vault::with_bucket(bucket);
                    self.reserves
                        .insert(new_vault.resource_address(), new_vault);
                }
            };
        }

        fn execute_proposed_change(
            &mut self,
            proposed_change: ProposedChange,
        ) -> Option<Vec<Bucket>> {
            match proposed_change {
                ProposedChange::ChangeVotePeriod(new_period) => {
                    self.vote_period = new_period;
                    None
                }

                ProposedChange::ChangeMinimumVoteThreshold(new_threshold) => {
                    self.vote_validity_threshold = new_threshold;
                    None
                }

                ProposedChange::GrantIssuingRight => {
                    let new_stablecoin_minter = self.protocol_admin_badge.authorize(|| {
                        borrow_resource_manager!(self.stablecoin_minter.resource_address())
                            .mint(Decimal::ONE)
                    });
                    Some(vec![new_stablecoin_minter])
                }

                ProposedChange::RemoveIssuingRight(_vault_bytes) => {
                    /* Not doable yet, but the recalling and burning the minter should be done like that:
                       self.protocol_admin_badge.authorize(|| {
                           let resource_manager = borrow_resource_manager!(self.stablecoin_minter.resource_address());
                           let minter = resource_manager.recall(vault_bytes);
                           resource_manager.burn(minter);

                       });
                       self.protocol_admin_badge.authorize(|| {
                           borrow_resource_manager!(self.stablecoin_minter.resource_address(
                       });
                    */
                    None
                }

                ProposedChange::AllowClaim(claimed_resources) => {
                    let mut vec_bucket = vec![];
                    for (token, amount) in &claimed_resources {
                        let bucket = self
                            .reserves
                            .get_mut(token)
                            .expect("There are no reserves for some of the tokens")
                            .take(*amount);
                        vec_bucket.push(bucket)
                    }
                    Some(vec_bucket)
                }

                ProposedChange::AddNewCollateralToken(
                    new_token,
                    loan_to_value,
                    interest_rate,
                    liquidation_threshold,
                    liquidation_penalty,
                    initial_rate,
                    minimum_rate,
                    maximum_rate,
                    oracle,
                ) => {
                    self.add_collateral_token(
                        new_token,
                        loan_to_value,
                        interest_rate,
                        liquidation_threshold,
                        liquidation_penalty,
                        oracle,
                        initial_rate,
                        minimum_rate,
                        maximum_rate,
                    );
                    None
                }

                ProposedChange::ChangeLenderParameters(
                    lender,
                    loan_to_value,
                    interest_rate,
                    liquidation_threshold,
                    liquidation_penalty,
                ) => {
                    let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

                    self.protocol_admin_badge.authorize(|| {
                        issuer.change_lender_parameters(
                            lender,
                            loan_to_value,
                            interest_rate,
                            liquidation_threshold,
                            liquidation_penalty,
                        );
                    });

                    None
                }

                ProposedChange::ChangeLenderOracle(lender, oracle_address) => {
                    let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

                    self.protocol_admin_badge.authorize(|| {
                        issuer.change_lender_oracle(lender, oracle_address);
                    });

                    None
                }

                ProposedChange::AddTokensToIssuerReserves(tokens_to_transfer) => {
                    let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);
                    let mut buckets_to_give = vec![];
                    for (token, amount) in &tokens_to_transfer {
                        let bucket = self
                            .reserves
                            .get_mut(token)
                            .expect("There are no reserves for some of the tokens")
                            .take(*amount);
                        buckets_to_give.push(bucket);
                    }
                    issuer.give_tokens(buckets_to_give);

                    None
                }
            }
        }

        #[inline]
        fn add_collateral_token(
            &mut self,
            collateral_token: ResourceAddress,
            loan_to_value: Decimal,
            interest_rate: Decimal,
            liquidation_threshold: Decimal,
            liquidation_penalty: Decimal,
            oracle: ComponentAddress,
            initial_rate: Decimal,
            min_rate: Decimal,
            max_rate: Decimal,
        ) {
            let mut router = RouterLocalComponent::at(self.dex_router);
            let mut issuer = IssuerLocalComponent::at(self.stablecoin_issuer);

            self.protocol_admin_badge.authorize(|| {
                router.create_pool(collateral_token.clone(), initial_rate, min_rate, max_rate);
                issuer.new_lender(
                    collateral_token,
                    loan_to_value,
                    interest_rate,
                    liquidation_threshold,
                    liquidation_penalty,
                    oracle,
                );
            });
        }

        #[inline]
        fn get_proposal(&self, proposal_id: u64) -> ProposalLocalComponent {
            match self.proposals.get(&proposal_id) {
                None => {
                    panic!("Proposal {} does not exist", proposal_id)
                }
                Some(proposal_address) => ProposalLocalComponent::at(*proposal_address),
            }
        }

        #[inline]
        fn get_voter_card_data(&self, id: &NonFungibleLocalId) -> VoterCard {
            borrow_resource_manager!(self.voter_card_address).get_non_fungible_data(id)
        }

        #[inline]
        fn get_position_data(&self, id: &NonFungibleLocalId) -> Position {
            borrow_resource_manager!(self.position_address).get_non_fungible_data(id)
        }

        #[inline]
        fn update_voter_card_data(&self, id: &NonFungibleLocalId, data: VoterCard) {
            self.resource_minter.authorize(|| {
                borrow_resource_manager!(self.voter_card_address)
                    .update_non_fungible_data(id, data);
            });
        }

        #[inline]
        fn get_voter_card_or_create(
            &mut self,
            opt_voter_card_proof: Option<Proof>,
        ) -> (VoterCard, NonFungibleLocalId, Option<Bucket>) {
            match opt_voter_card_proof {
                Some(voter_card_proof) => {
                    let validated_proof = voter_card_proof
                        .validate_proof(ProofValidationMode::ValidateContainsAmount(
                            self.voter_card_address,
                            Decimal::ONE,
                        ))
                        .expect("Please provide a valid voter card proof");
                    let voter_card_id = validated_proof
                        .non_fungible::<VoterCard>()
                        .local_id()
                        .clone();
                    let voter_card_data = self.get_voter_card_data(&voter_card_id);

                    (voter_card_data, voter_card_id, None)
                }

                None => {
                    let voter_card_id = NonFungibleLocalId::Integer(self.voter_card_id.into());
                    let new_voter_card = self.resource_minter.authorize(|| {
                        borrow_resource_manager!(self.voter_card_address)
                            .mint_non_fungible(&voter_card_id, VoterCard::new())
                    });

                    self.voter_card_id += 1;

                    (VoterCard::new(), voter_card_id, Some(new_voter_card))
                }
            }
        }
    }
}
