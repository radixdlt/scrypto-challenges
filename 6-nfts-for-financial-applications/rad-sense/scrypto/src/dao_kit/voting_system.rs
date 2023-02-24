use scrypto::prelude::*;
use std::mem::{discriminant, Discriminant};

use super::code_execution_system::*;
use super::voting_system::VoteCount::{FungibleVoteCount, NonFungibleVoteCount};

blueprint! {

    /// The VotingSystem can be used to create votes/proposals that users can vote on.
    /// For each vote two or more options can be specified for users to choose among. An
    /// option can optionally be associated with method or function calls that are executed if the
    /// option wins the vote.
    struct VotingSystem {

        /// Internal minting authority for NFRs
        minter: Vault,

        /// A vault that holds old votes
        votes: Vault,

        /// The address and admin badge of a CodeExecutionSystem. Optional
        code_execution_system: Option<(ComponentAddress, Vault)>,

        /// Vaults for the fungible tokens that users are depositing into this component when
        /// casting their votes
        voting_power_tokens: KeyValueStore<ResourceAddress, Vault>,

        /// The resource address of the [VotingReceipt] NFR. Such a receipt is given to user when
        /// they lock their voting tokens into this component
        vote_receipt_resource: ResourceAddress,
    }

    impl VotingSystem {
        /// Instantiates a local VotingSystem component.
        ///
        /// A reference to a CodeExecutionSystem can optionally be provided. If one is provided
        /// votes can be configured to contain code executions.
        ///
        /// Arguments:
        /// - `code_execution_system`: An optional Tuple of a) the address of a CodeExecutionSystem
        /// component and b) an admin badge that can be used to call privileged methods on that
        /// component.
        ///
        /// Returns a tuple containing:
        /// - The VotingSystemComponent instance
        /// - The resource address of the vote resource (the resource that represents votes)
        /// - The resource address of the vote receipt resource (the resource that represents
        /// receipts that are given to voters if they supply fungible tokens)
        pub fn instantiate(
            code_execution_system: Option<(ComponentAddress, Bucket)>,
        ) -> (VotingSystemComponent, ResourceAddress, ResourceAddress) {
            let minter = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);

            let vote_resource = ResourceBuilder::new_non_fungible()
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter.resource_address())), LOCKED)
                .metadata("name", "Vote Token")
                .no_initial_supply();

            let vote_receipt_resource = ResourceBuilder::new_non_fungible()
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .metadata("name", "Vote Token Receipt")
                .no_initial_supply();

            let component = Self {
                minter: Vault::with_bucket(minter),
                votes: Vault::new(vote_resource),
                code_execution_system: code_execution_system
                    .map(|(address, admin_badge)| (address, Vault::with_bucket(admin_badge))),
                voting_power_tokens: KeyValueStore::new(),
                vote_receipt_resource,
            }
            .instantiate();

            (component, vote_resource, vote_receipt_resource)
        }

        /// Instantiates a global VotingSystem component.
        ///
        /// A reference to a CodeExecutionSystem can optionally be provided. If one is provided
        /// votes can be configured to contain code executions.
        ///
        /// Arguments:
        /// - `code_execution_system`: An optional Tuple of a) the address of a CodeExecutionSystem
        /// component and b) an admin badge that can be used to call privileged methods on that
        /// component.
        /// - `admin_badge`: The address of the admin badge that authorizes access to all
        /// privileged methods of this component.
        ///
        /// Returns a tuple containing:
        /// - The component address of the instantiated and globalized VotingSystemComponent
        /// - The resource address of the vote resource (the resource that represents votes)
        /// - The resource address of the vote receipt resource (the resource that represents
        /// receipts that are given to voters if they supply fungible tokens)
        pub fn instantiate_global(
            code_execution_system: Option<(ComponentAddress, Bucket)>,
            admin_badge: ResourceAddress,
        ) -> (ComponentAddress, ResourceAddress, ResourceAddress) {
            let access_rules = AccessRules::new()
                .method("create_proposal", rule!(require(admin_badge)))
                .method("create_vote", rule!(require(admin_badge)))
                .method("cast_vote", rule!(allow_all))
                .method("evaluate_vote", rule!(allow_all))
                .method("implement_vote", rule!(allow_all))
                .method("vote_resource", rule!(allow_all));

            let (mut component, vote_resource, vote_receipt_resource) = Self::instantiate(code_execution_system);
            component.add_access_check(access_rules);

            (component.globalize(), vote_resource, vote_receipt_resource)
        }

        /// Please see [VoteConfig::new_proposal]
        pub fn create_proposal(
            &mut self,
            name: String,
            description: Option<String>,
            voting_deadline: VotingDeadline,
            approve_requirement: WinRequirement,
            code_executions: Vec<CodeExecution>,
            voting_power_resource: ResourceAddress,
        ) -> Vote {
            let proposal = VoteConfig::new_proposal(
                name,
                description,
                voting_deadline,
                approve_requirement,
                code_executions,
                voting_power_resource,
            );
            self.create_vote(proposal)
        }

        /// Creates a new vote based on the given [vote_config](VoteConfig).
        /// An NFR representing the vote is stored inside the component.
        ///
        /// Arguments:
        /// - `vote_config`: The config of the vote to create
        ///
        /// Returns: the created Vote object
        pub fn create_vote(&mut self, vote_config: VoteConfig) -> Vote {
            // Make sure that if the vote contains any code executions, a code execution system is also
            // configured
            let contains_code_executions = vote_config
                .options
                .iter()
                .any(|(_, option)| !option.code_executions.is_empty());
            if contains_code_executions && self.code_execution_system.is_none() {
                panic!("The vote cannot contain code executions when no code execution system is configured");
            }

            let vote = Vote::new(vote_config);
            self.votes
                .put(self.mint_non_fungible(self.votes.resource_address(), &vote.id, vote.clone()));
            vote
        }

        /// Cast a vote, i.e. vote for a specific option.
        ///
        /// Arguments:
        /// - `vote_id`: The ID of the vote
        /// - `option_name`: The name of the option to vote for
        /// - `voting_power`: A proof of a NFR or a bucket of fungible tokens that represents
        /// the users voting power
        ///
        /// Returns a tuple containg:
        /// - The Vote object
        /// - An optional bucket, containing the vote receipt if the user voted with fungible tokens
        pub fn cast_vote(
            &mut self,
            vote_id: NonFungibleId,
            option_name: String,
            voting_power: VotingPower,
        ) -> (Vote, Option<Bucket>) {
            // Load the referenced vote
            let mut vote = self.load_vote_by_id(&vote_id);

            // Cast the vote. If the VotingPower contains fungible tokens, those tokens
            // will be stored by this component while the vote is open
            let tokens_to_store = vote.cast_vote(option_name.to_owned(), voting_power);

            // If there are fungible tokens that should be stored in this component,
            // we put them in a vault and create a receipt for the user so they have a way of
            // redeeming their tokens
            let receipt = if let Some(tokens) = tokens_to_store {
                let token_address = tokens.resource_address();
                let token_amount = tokens.amount();
                match self.voting_power_tokens.get_mut(&token_address) {
                    Some(ref mut vault) => vault.put(tokens),
                    None => self
                        .voting_power_tokens
                        .insert(token_address, Vault::with_bucket(tokens)),
                }
                let receipt = VoteReceipt {
                    vote_id,
                    option_name,
                    token_address,
                    token_amount,
                };
                let receipt: Bucket =
                    self.mint_non_fungible(self.vote_receipt_resource, &NonFungibleId::random(), receipt);
                Some(receipt)
            } else {
                None
            };

            // Before returning, save the modified vote NFR to the ledger
            self.update_non_fungible_data(self.votes.resource_address(), &vote.id, vote.clone());

            // Finally return the current vote data and the optional voting receipt (in case the
            // user supplied fungible tokens)
            (vote, receipt)
        }

        /// Evaluate the given vote
        ///
        /// Arguments:
        /// - `vote_id`: The ID of the vote to evaluate
        ///
        /// Returns: the Vote object
        pub fn evaluate_vote(&self, vote_id: NonFungibleId) -> Vote {
            let mut vote = self.load_vote_by_id(&vote_id);
            vote.evaluate();
            self.update_non_fungible_data(self.votes.resource_address(), &vote.id, vote.clone());
            vote
        }

        /// Lets a user redeem the (fungible) voting power tokens that they have locked inside the
        /// component when casting their vote.
        ///
        /// If the is in state `VoteState::Open`, the redeemed token amount will be subtracted from
        /// the vote count of the option the user originally voted for. Basically, this means that
        /// the user revokes their vote. If the vote is in any other state, the vote count will
        /// remain untouched.
        ///
        /// Arguments:
        /// - `vote_receipt`: A bucket containing the vote receipt that allows the user to redeem
        /// their tokens
        ///
        /// Returns: the fungible tokens that the user locked in the component when they cast their
        /// vote
        pub fn redeem_voting_power(&mut self, vote_receipt: Bucket) -> Bucket {
            assert_eq!(
                vote_receipt.resource_address(),
                self.vote_receipt_resource,
                "Invalid resource"
            );
            let vote_receipt_data: VoteReceipt = vote_receipt.non_fungible().data();
            let mut vote = self.load_vote_by_id(&vote_receipt_data.vote_id);

            if matches!(vote.state, VoteState::Open(_)) {
                // In case the vote is still open/ongoing, the redeemed tokens must be removed from the
                // vote count. If the vote is in any other state, the vote count is not modified.
                vote.revoke_vote(vote_receipt_data.option_name, vote_receipt.amount());
                self.update_non_fungible_data(self.vote_resource(), &vote_receipt_data.vote_id, vote);
            }

            // Burn the receipt
            self.minter.authorize(|| vote_receipt.burn());

            // Take out the voting power tokens from the internal vault and return them to the user
            self.voting_power_tokens
                .get_mut(&vote_receipt_data.token_address)
                .unwrap()
                .take(vote_receipt_data.token_amount)
        }

        /// Implements the given vote, running all code executions on the winning methods.
        /// If no code executions are configured or the winning option(s) did not contain any code
        /// executions, the vote state will simply be set to implemented without any additional side
        /// effects taking place.
        ///
        /// Arguments:
        /// - `vote_id`: The ID of the vote to implement
        /// - `voting_power_resource`: A proof of the voting power resource. Only users that were
        /// allowed cast a vote can also implement a vote.
        ///
        /// Returns:
        /// - The vote object
        /// - An optional bucket containing all the AuthorizedCodeExecution NFRs that are
        /// required to be run in order for the vote to be considered implemented. Each
        /// AuthorizedCodeExecution NFR is a non deposit-able resource that must be passed along
        /// to the proper CodeExecutionSystem so as to be executed within the same transaction.
        pub fn implement_vote(
            &mut self,
            vote_id: NonFungibleId,
            voting_power_resource: Proof,
        ) -> (Vote, Option<Bucket>) {
            let mut vote = self.load_vote_by_id(&vote_id);

            // Validate that the user has the proper authority to implement the vote
            voting_power_resource
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    vote.config.voting_power_resource,
                    dec!(1),
                ))
                .unwrap_or_else(|e| panic!("Invalid membership proof: {e:?}"));

            // Assert that the vote is in state decided
            assert!(
                matches!(vote.state, VoteState::Decided { .. }),
                "The vote must be in state decided"
            );

            // Next try to implement the vote. This will fail if the vote is not in state Decided by now
            // If any code executions are associated with the vote, these will be returned by the
            // vote.implement method.
            let code_executions: Vec<CodeExecution> = vote.implement();

            // After implementing the vote safe the vote state to the ledger
            self.update_non_fungible_data(self.votes.resource_address(), &vote_id, vote.clone());

            // If there are any code executions to implement...
            let authorized_code_execution_token = if !code_executions.is_empty() {
                // Get the code execution system (CES) address and the associated admin badge
                let (ces, ces_admin_badge) = &self.code_execution_system.as_ref().unwrap();
                // Turn the CES address into a callable component
                let ces: CodeExecutionSystemComponent = (*ces).into();
                // Call the CES to turn the Vec<CodeExecution> into a bucket containing an
                // AuthorizedCodeExecution NFR
                let authorized_code_execution_token =
                    ces_admin_badge.authorize(|| ces.authorize_code_execution(code_executions));
                Some(authorized_code_execution_token)
            } else {
                None
            };

            // Return the optional AuthorizedCodeExecution NFR
            (vote, authorized_code_execution_token)
        }

        /// Return the resource address of the tokens that represents a vote
        pub fn vote_resource(&self) -> ResourceAddress {
            self.votes.resource_address()
        }

        /// Return the resource address of the tokens that represents a vote receipt
        pub fn vote_receipt_resource(&self) -> ResourceAddress {
            self.vote_receipt_resource
        }

        /// Utility function for minting a new non fungible using the admin_badge for authorization
        fn mint_non_fungible<T: NonFungibleData>(
            &self,
            resource: ResourceAddress,
            id: &NonFungibleId,
            data: T,
        ) -> Bucket {
            let rm = borrow_resource_manager!(resource);
            self.minter.authorize(|| rm.mint_non_fungible(id, data))
        }

        /// Utility function for updating a new non fungible using the admin_badge for authorization
        fn update_non_fungible_data<T: NonFungibleData>(
            &self,
            resource: ResourceAddress,
            id: &NonFungibleId,
            new_data: T,
        ) {
            let rm = borrow_resource_manager!(resource);
            self.minter.authorize(|| rm.update_non_fungible_data(id, new_data))
        }

        /// Utility function for loading a vote by it's ID
        fn load_vote_by_id(&self, vote_id: &NonFungibleId) -> Vote {
            borrow_resource_manager!(self.votes.resource_address()).get_non_fungible_data(vote_id)
        }
    }
}

/// Represents the configuration of a Vote
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub struct VoteConfig {
    /// A preferably short name for the vote
    pub name: String,
    /// An optional and longer description what the vote is about
    pub description: Option<String>,
    /// The available options that can be selected
    pub options: HashMap<String, VoteOption>,
    /// Settings detailing how the vote should be evaluated
    pub evaluation_settings: EvaluationSettings,
    /// The resource that voters must show/supply in order to vote
    pub voting_power_resource: ResourceAddress,
}

impl VoteConfig {
    /// Creates a special vote config for a proposal where the only two options are to approve the
    /// proposal or to reject it. If the approve requirement is [WinRequirement::MaxVotingPower],
    /// the reject option's requirement is automatically set to [WinRequirement::MaxVotingPower]
    /// also. In all other cases, the reject option's requirement is set to Fallback. The
    /// option will thus be selected as the winning option, if the approve option does not meet its
    /// requirement.
    ///
    /// Arguments:
    /// - `name`: A preferably shot name for the proposal
    /// - `description`: An optional and longer description of the proposal
    /// - `voting_deadline`: The deadline after which votes will no longer be accepted. The vote can only be
    /// evaluated/implemented after the deadline has passed.
    /// - `approve_requirement`: The requirement for the proposal to be approve. Cannot be [WinRequirement::Fallback]
    /// - `code_executions`: The code executions that should run if the proposal is approved
    /// - `voting_power_resource`: The resource that voters must show/supply in order to vote on the proposal
    pub fn new_proposal(
        name: String,
        description: Option<String>,
        voting_deadline: VotingDeadline,
        approve_requirement: WinRequirement,
        code_executions: Vec<CodeExecution>,
        voting_power_resource: ResourceAddress,
    ) -> Self {
        // Choose the reject requirement based on the supplied accept requirement
        let reject_requirement = match approve_requirement {
            WinRequirement::MaxVotingPower => WinRequirement::MaxVotingPower,
            WinRequirement::Fallback => panic!("WinRequirement::Fallback is not allowed as the approve_requirement"),
            _ => WinRequirement::Fallback,
        };

        let mut options = HashMap::new();
        // Add an "approve" option and associate the code_executions with it
        options.insert(
            "approve".to_owned(),
            VoteOption { description: None, code_executions, requirement: approve_requirement.clone() },
        );

        // Add a "reject" option
        options.insert(
            "reject".to_owned(),
            VoteOption { description: None, code_executions: vec![], requirement: reject_requirement },
        );

        Self {
            name,
            description,
            options,
            evaluation_settings: EvaluationSettings {
                voting_deadline,
                allow_multiple_winning_options: false, // Proposal can only have one winning option
            },
            voting_power_resource,
        }
    }

    /// Assert that this config is valid
    pub(crate) fn assert_valid(&self) {
        // Assert that the deadline is in the future
        use VotingDeadline::*;
        match &self.evaluation_settings.voting_deadline {
            HardEpochDeadline(epoch) | SoftEpochDeadline(epoch) => {
                assert!(epoch > &Runtime::current_epoch(), "The voting deadline must be in the future");
            }
        }

        // Check that all options are valid
        Self::assert_options_valid(&self.options);
    }

    /// Assert that the given options are valid
    fn assert_options_valid(options: &HashMap<String, VoteOption>) {
        // Check that at least two options are configured
        assert!(options.len() >= 2, "At least two options must be configured");

        // Make sure all requirements are of the same variant with the exception of one single fallback
        // requirement being allowed
        let requirement_discriminate = Self::get_win_requirement_discriminant(options).expect(
            "All options must have the same VoteRequirement variant (one single VoteRequirement::Fallback is allowed)",
        );

        // Count how often WinRequirement::Fallback was used. Only one is allowed
        let mut fallback_count = 0;
        for (option_name, option) in options.iter() {
            // Assure that each requirement is valid
            let requirement = &option.requirement;
            assert!(requirement.is_valid(), "Option {option_name} has invalid requirement: {:?}", requirement);
            if matches!(requirement, WinRequirement::Fallback) {
                fallback_count += 1
            }
        }
        assert!(fallback_count <= 1, "Only one option with WinRequirement::Fallback is allowed");

        if fallback_count > 0 && requirement_discriminate == discriminant(&WinRequirement::MaxVotingPower) {
            panic!("WinRequirement::Fallback cannot be used together with WinRequirement::MaxVotingPower");
        }
    }

    /// Gets the [Discriminant] of the WinRequirement for all given options.
    /// This presupposes that all options have the same win requirement type. If this is not the case
    /// (invalid configuration) this method will return an error. Options with WinRequirement::Fallback
    /// will be ignored
    fn get_win_requirement_discriminant(
        options: &HashMap<String, VoteOption>,
    ) -> Result<Discriminant<WinRequirement>, &str> {
        let discriminates: HashSet<Discriminant<WinRequirement>> = options
            .iter()
            .filter(|(_, option)| matches!(option.requirement, WinRequirement::Fallback))
            .map(|(_, option)| discriminant(&option.requirement))
            .collect();

        if discriminates.len() > 1 {
            Err("The same requirement must be used for all options")
        } else {
            discriminates.into_iter().next().ok_or("No options with requirements other than fallback found")
        }
    }
}

/// Represents a vote with multiple options that users can vote on
#[derive(NonFungibleData, Encode, Decode, TypeId, Describe, Clone, Debug)]
pub struct Vote {
    /// The unique ID of the vote
    pub id: NonFungibleId,
    /// The vote configuration
    pub config: VoteConfig,
    /// The state the vote is in
    #[scrypto(mutable)]
    pub state: VoteState,
}

impl Vote {
    /// Creates a new vote based on the given config.
    /// The vote will be assigned a random ID and will start out
    /// in state [VoteState::Open] with an empty [VoteCount].
    ///
    /// Arguments:
    /// - `config`: The vote config. Will be validated before creating the vote.
    ///
    /// Returns: the create vote instance
    pub fn new(config: VoteConfig) -> Vote {
        config.assert_valid();
        let state = VoteState::Open(VoteCount::new(&config));
        Self { id: NonFungibleId::random(), config, state }
    }

    /// Record the fact that a user has cast a vote, i.e. has supplied their voting power to back
    /// one single option. The vote must be in state [VoteState:Open] and the voting deadline must
    /// not have passed.
    ///
    /// Arguments:
    /// - `option_name`: the name of the option the user has chosen
    /// - `voting_power`: The voting power that the user has chosen to put behind the given option
    pub fn cast_vote(&mut self, option_name: String, voting_power: VotingPower) -> Option<Bucket> {
        // Assert that the given option exists
        assert!(self.config.options.contains_key(&option_name), "No such option: {option_name}");

        // Get the current vote count. The vote must be in the open state
        let vote_count = match self.state {
            VoteState::Open(ref mut vote_count) => vote_count,
            _ => panic!("Vote must be in VoteState::Open"),
        };

        // Assert the voting deadline has not passed yet
        use VotingDeadline::*;
        match &self.config.evaluation_settings.voting_deadline {
            HardEpochDeadline(epoch) | SoftEpochDeadline(epoch) => {
                assert!(&Runtime::current_epoch() <= epoch, "The voting deadline has passed");
            }
        }

        // Actually change the vote count. If the voting power supplied by the user
        // is as fungible token, these tokens (bucket) will be stored in variable tokens_to_store
        let tokens_to_store: Option<Bucket> = match voting_power {
            // Handle the case that the user has voted with a fungible token
            VotingPower::Fungible(voting_power_resources) => {
                // Assert the right tokens have been supplied
                assert_eq!(
                    voting_power_resources.resource_address(),
                    self.config.voting_power_resource,
                    "Voting power must be a bucket of the membership token that matches the \
                    voting_power_resource of the vote"
                );
                // Actually record the vote
                match vote_count {
                    FungibleVoteCount(voting_powers_by_option_name) => {
                        let current_voting_power = voting_powers_by_option_name.get(&option_name).unwrap();
                        let new_voting_power = *current_voting_power + voting_power_resources.amount();
                        voting_powers_by_option_name.insert(option_name, new_voting_power);

                        // Return the voting power tokens to be stored inside the VotingSystemComponent
                        Some(voting_power_resources)
                    }
                    NonFungibleVoteCount(_) => panic!("NonFungibleVoteCount VoteCount incompatible with Fungible VotingPower. This should not happen!")
                }
            }

            // Handle the case that the user has voted by showing proof of an NFR
            VotingPower::NonFungible(membership_badge) => {
                // Validate that the proof is of the correct resource
                let proof = membership_badge
                    .validate_proof(ProofValidationMode::ValidateContainsAmount(
                        self.config.voting_power_resource,
                        dec!(1),
                    ))
                    .expect(
                        "Voting power must be a proof of exactly one membership badge \
                           that matches the voting_power_resource of the vote",
                    );
                // Actually record the vote
                match vote_count {
                    NonFungibleVoteCount(voter_ids_by_option_name) => {
                        // Iterate over all options...
                        for (option, voter_ids) in voter_ids_by_option_name.iter_mut() {
                            if option == &option_name {
                                // If the option matches, add the user's voter ID to the set of
                                // ids of this option. Because this is a set, the user cannot vote
                                // for the same option twice
                                voter_ids.insert(proof.non_fungible_id());
                            } else {
                                // If the option does not match, remove the user's voter ID from the
                                // option. If this is the users first vote, this does nothing.
                                // if the user had previously chosen another option, this removes that
                                // previous vote and thus allows easy "re-voting"
                                voter_ids.remove(&proof.non_fungible_id());
                            }
                        }

                        // No tokens need to be stored in the VotingSystemComponent
                        None
                    }
                    FungibleVoteCount(_) => {
                        panic!("Fungible VoteCount incompatible with NonFungible VotingPower. This should not happen!")
                    }
                }
            }
        };

        // Return (optionally) the voting power tokens that need to be stored inside the
        // VotingSystemComponent for the duration of the vote
        tokens_to_store
    }

    /// Revokes a user's vote. This is only supported if the user has voted with fungible tokens.
    /// The vote must be in state [VoteState::Open].
    ///
    /// Arguments:
    /// - `option_name`: The name for which the user's vote should be revoked
    /// - `amount`: The amount that the user has previously voted with on the given option
    pub(crate) fn revoke_vote(&mut self, option_name: String, amount: Decimal) {
        match &mut self.state {
            VoteState::Open(ref mut vote_count) => match vote_count {
                FungibleVoteCount(ref mut voting_powers) => {
                    voting_powers.entry(option_name).and_modify(|voting_power| *voting_power -= amount);
                }
                NonFungibleVoteCount(_) => panic!("Operation not possible for NonFungibleVoteCount"),
            },
            _ => panic!("Voting power can only be removed from a vote if it is in state Open"),
        }
    }

    /// Evaluates this vote, possibly advancing it's state to either [VoteState::Decided] or
    /// [VoteState::Failed].
    pub fn evaluate(&mut self) {
        // First, assert that the vote is in state Open
        let vote_count = match &self.state {
            VoteState::Open(vote_count) => vote_count,
            _ => panic!("The vote must be in state Open"),
        };

        assert!(self.may_evaluate(), "The vote cannot be evaluated yet");

        // Get the first non fallback WinRequirement from the vote's options. As all options must have the
        // same requirement (apart from one fallback option) this is representative for all options.
        // This representative requirement is needed so that we can perform the correct evaluation
        let representative_requirement = self
            .config
            .options
            .values()
            .map(|o| &o.requirement)
            .find(|r| !matches!(r, WinRequirement::Fallback))
            .unwrap(); // There will always be at least one option with a non-fallback requirement

        // Compute the voting power that has been been accumulated per option
        let voting_powers: HashMap<String, Decimal> = vote_count.get_voting_power_per_option().into_iter().collect();
        // Using the representative requirement, determine the winning options
        let winning_options = representative_requirement.determine_winning_options(
            &self.config.options,
            &voting_powers,
            self.config.voting_power_resource,
        );

        // Inspect the winning options and determine the new state
        let deadline_has_passed = Runtime::current_epoch() > self.config.evaluation_settings.voting_deadline.epoch();
        let new_state = if winning_options.is_empty() && !deadline_has_passed {
            // If there is no winning option and the deadline has not passed yet, the soft deadline
            // cannot be disregarded. Panic so that nothing changes
            panic!("The vote cannot be evaluated yet. Soft deadline cannot be disregarded")
        } else if winning_options.is_empty() {
            // If no option has won but the deadline has passed, see if a fallback option is defined
            let fallback_option = self
                .config
                .options
                .iter()
                .find(|(_option_name, option)| matches!(option.requirement, WinRequirement::Fallback))
                .map(|(option_name, _option)| option_name.to_owned());
            if let Some(fallback_option) = fallback_option {
                // If a fallback option, it wins the vote
                self.state.to_decided(vec![fallback_option])
            } else {
                // If no fallback option is defined, the vote fails
                self.state.to_failed()
            }
        } else if winning_options.len() > 1 && !self.config.evaluation_settings.allow_multiple_winning_options {
            // If there is more than one winning option but multiple winning options are not not
            // allowed, the vote fails
            self.state.to_failed()
        } else {
            // In all other cases (only a single winning option, or multiple winning options allowed)
            // the vote is decided
            self.state.to_decided(winning_options)
        };

        // Finally, set the new state
        self.state = new_state
    }

    /// Implements this vote by changing it's state to [VoteState::Decided]
    ///
    /// Returns: a vec with all [CodeExecution]s that must be run in order for the vote to be
    /// considered implemented
    fn implement(&mut self) -> Vec<CodeExecution> {
        let winning_option_names = match &self.state {
            VoteState::Decided { winning_option_names, .. } => winning_option_names,
            _ => panic!("Vote must be in state DECIDED"),
        };

        // Collect all code executions that are associated with the winning options
        let code_executions = winning_option_names
            .iter()
            .map(|option_name| self.config.options.get(option_name).unwrap())
            .flat_map(|option| option.code_executions.clone())
            .collect();

        // Set the new state
        self.state = self.state.to_implemented();

        // Return the code executions that must be run
        code_executions
    }

    /// Checks if the vote may be evaluated. This mainly depends on the voting deadline and the
    /// current epoch. Under certain configurations, the vote may be evaluated even before the
    /// deadline has passed. Under the current implementation the only such configuration is a proposal
    /// with two options, where the primary has WinRequirement::Fallback and the other has an absolut
    /// WinRequirement. Additionally the VotingDeadline must be [VotingDeadline::SoftEpochDeadline].
    /// If these conditions are met, this methods returns true, even if the epoch deadline has not
    /// been reached yet. Note that during the evaluation, if the primary option does not win, the
    /// default option must not be considered as the winning option. In this case the vote must remain
    /// in state Open until the deadline has passed or the primary option has reached is WinRequirement.
    pub fn may_evaluate(&self) -> bool {
        use VotingDeadline::*;
        use WinRequirement::*;

        match self.config.evaluation_settings.voting_deadline {
            HardEpochDeadline(epoch) => Runtime::current_epoch() > epoch,
            SoftEpochDeadline(epoch) if epoch > Runtime::current_epoch() => true,
            SoftEpochDeadline(_) => {
                // At this moment this code only handles the specific case of a proposal with two options
                // where one option is a fallback and the other one has an absolut requirement
                let has_exactly_two_options = self.config.options.len() == 2;
                let mut has_fallback_option = false;
                let mut has_option_with_an_absolut_requirement = false;

                for option in self.config.options.values() {
                    match option.requirement {
                        AbsolutAmount(_) | AbsolutRatio(_) => has_option_with_an_absolut_requirement = true,
                        Fallback => has_fallback_option = true,
                        _ => {}
                    }
                }

                has_exactly_two_options && has_fallback_option && has_option_with_an_absolut_requirement
            }
        }
    }
}

/// Represents an option users can vote for
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub struct VoteOption {
    /// A description of the option
    pub description: Option<String>,
    /// A vec with zero to multiple [CodeExecution]s that should be run, if the option wins
    pub code_executions: Vec<CodeExecution>,
    /// A requirement that must be met in order for the option to be selected as a winning option
    pub requirement: WinRequirement,
}

#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub enum WinRequirement {
    /// An option is considered the winning option if it gets the most voting power among all options.
    MaxVotingPower,

    /// An option will be considered a winning option if it gets the specified amount of total
    /// voting power. This can be used to e.g. configure that at least two members have to approve
    /// a proposal.
    AbsolutAmount(Decimal),

    /// An option will be considered a winning option if it gets the specified ratio of voting power
    /// in respect to the total supply of the voting power resource. For example, if the ratio is
    /// set to 0.50000000001 and the total supply of the voting power token is 100 (non fungible),
    /// at least 51 votes are needed for the option to win.
    AbsolutRatio(Decimal),

    /// An option will be considered a winning option if it gets the specified ratio of voting power
    /// in respect to the sum of the voting power that has been spent on all options. For example, if the ratio is
    /// set to 0.50000000001 and the total supply of the voting power token is 100 (non fungible),
    /// an option is considered a winning option if it gets just over 50% of all votes. If only
    /// 10 out of 100 members cast their vote, an option will still win if gets at least 6 votes.
    RelativeRatio(Decimal),

    /// An option will be considered a winning option if no other option has been selected as the
    /// winning option. Only one option per vote can have this requirement. This requirement variant
    /// is useful for cases where one does not want the vote to fail if none of the primary options amasses the required
    /// voting power. A special case of this is e.g. the proposal where the options are "approve" and
    /// "reject". Here one is only really interested in the approve option. If it does not get its
    /// required voting power, the proposal should be considered rejected, even if (in the extreme case)
    /// it did not get a single "reject" vote.
    ///
    /// Cannot be combined with [WinRequirement::MaxVotingPower]!
    ///
    /// Warning: A fallback option will not be selected as the winning option if others options meet
    /// their requirements and are therefore selected as the winning options. This is true even if
    /// the fallback option amasses more voting power then all other options!
    Fallback,
}

impl WinRequirement {
    /// Checks whether the requirement is well defined
    ///
    /// Returns true if the requirement is valid and false otherwise
    pub fn is_valid(&self) -> bool {
        match self {
            WinRequirement::MaxVotingPower | WinRequirement::Fallback => true,
            WinRequirement::AbsolutAmount(amount) => *amount > dec!(0),
            WinRequirement::AbsolutRatio(ratio) | WinRequirement::RelativeRatio(ratio) => {
                *ratio > dec!(0) && *ratio <= dec!(1)
            }
        }
    }

    /// Determine the winning options, i.e. options that meet their resp. [WinRequirement].
    ///
    /// An option with [WinRequirement::Fallback] will never be considered a winning option by this
    /// method.
    ///
    /// Arguments:
    /// - `options`: The set of all options
    /// - `voting_power`: The voting power that has been accumulated by each option
    /// - `voting_power_resource`: The resource that users have used to cast their vote
    ///
    /// Returns: The names of all options that have been determined winning options. This vec will
    /// never contain options with WinRequirement::Fallback.
    pub(crate) fn determine_winning_options(
        &self,
        options: &HashMap<String, VoteOption>,
        voting_powers: &HashMap<String, Decimal>,
        voting_power_resource: ResourceAddress,
    ) -> Vec<String> {
        match self {
            WinRequirement::MaxVotingPower => Self::determine_winning_options_by_max_voting_power(voting_powers),
            WinRequirement::AbsolutAmount(..) => {
                Self::determine_winning_options_by_absolut_amount(options, voting_powers)
            }
            WinRequirement::AbsolutRatio(_) => {
                // For requirement AbsolutRatio the actual ratio of each option is calculated w.r.t
                // the total supply of the voting power token
                let total_voting_power = borrow_resource_manager!(voting_power_resource).total_supply();
                Self::determine_winning_options_by_ratio(options, voting_powers, total_voting_power)
            }
            WinRequirement::RelativeRatio(_) => {
                // For requirement RelativeRatio the actual ratio of each option is calculated w.r.t
                // the sum of all voting power tokens that have been spent over all options
                let total_voting_power = voting_powers.values().map(|vp| vp.to_owned()).sum();
                Self::determine_winning_options_by_ratio(options, voting_powers, total_voting_power)
            }
            WinRequirement::Fallback => panic!("Not allowed/possible"),
        }
    }

    /// Returns every option that has accumulated at least the required amount of voting power
    fn determine_winning_options_by_absolut_amount(
        options: &HashMap<String, VoteOption>,
        voting_powers: &HashMap<String, Decimal>,
    ) -> Vec<String> {
        options
            .iter()
            // Only retain options that have at leased the required absolut amount
            .filter_map(|(option_name, option)| match &option.requirement {
                WinRequirement::AbsolutAmount(required_amount) => {
                    let actual_amount = voting_powers.get(option_name).unwrap();
                    if actual_amount >= required_amount {
                        Some(option_name.to_owned())
                    } else {
                        None
                    }
                }
                WinRequirement::Fallback => None, // Skip fallback options
                invalid_req => panic!("Invalid requirement {:?}. This is a bug", invalid_req),
            })
            .collect()
    }

    /// Returns the option that has accumulated the most voting power. If multiple options
    /// If multiple options share the same maximum voting power they are all returned
    fn determine_winning_options_by_max_voting_power(voting_powers: &HashMap<String, Decimal>) -> Vec<String> {
        let mut max_vp = dec!(0);
        let mut options_by_voting_power: HashMap<Decimal, Vec<String>> = HashMap::new();
        for (option_name, voting_power) in voting_powers {
            if *voting_power >= max_vp {
                max_vp = *voting_power;
                options_by_voting_power
                    .entry(*voting_power)
                    .and_modify(|option_names| option_names.push(option_name.to_owned()))
                    .or_insert(vec![option_name.to_owned()]);
            }
        }
        options_by_voting_power.get(&max_vp).unwrap().to_owned()
    }

    /// Returns every option that has accumulated at least the required ratio of voting power
    ///
    /// The `total_voting_power` argument must be the total supply of the voting power token in the
    /// absolut case and the sum of the voting power accumulated by all options in the relative case
    fn determine_winning_options_by_ratio(
        options: &HashMap<String, VoteOption>,
        voting_powers: &HashMap<String, Decimal>,
        total_voting_power: Decimal,
    ) -> Vec<String> {
        options
            .iter()
            // Only retain options that have at least the required ratio
            .filter_map(|(option_name, option)| match &option.requirement {
                WinRequirement::AbsolutRatio(required_ratio) | WinRequirement::RelativeRatio(required_ratio) => {
                    let actual_ratio = *voting_powers.get(option_name).unwrap() / total_voting_power;
                    if actual_ratio >= *required_ratio {
                        Some(option_name.to_owned())
                    } else {
                        None
                    }
                }
                WinRequirement::Fallback => None, // Skip fallback options
                invalid_req => panic!("Invalid requirement {:?}. This is a bug", invalid_req),
            })
            .collect()
    }
}

/// Define certain evaluation settings that will be applied when evaluating a vote
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub struct EvaluationSettings {
    /// The deadline until which voting should be possible
    pub voting_deadline: VotingDeadline,

    /// Whether to allow multiple winning options.
    /// - If the options' requirements are one of AbsolutAmount/AbsolutRatio/RelativeRatio and multiple
    /// options reach their resp. requirement, then all those options will be selected as the winning options if
    /// `allow_multiple_winning_options` is true, else the vote will fail.
    /// - If the options' requirements are MaxVotingPower the option with the highest voting power will always win.
    /// Should multiple options have the same highest voting power, if `allow_multiple_winning_options` is
    /// true, all these options will be selected as the winning options, otherwise the vote will fail.
    pub allow_multiple_winning_options: bool,
}

/// The deadline after which voting should be impossible
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub enum VotingDeadline {
    /// A hard deadline. Voting is possible up to this epoch (inclusive). This deadline will be
    /// enforced no matter what. Evaluation of the vote is only possible in the next epoch.
    HardEpochDeadline(u64),

    /// A soft deadline. Voting is possible up to this epoch (inclusive). This deadline might be
    /// disregarded if it is safe to do so. Imagine for example the following case:
    /// - A proposal has two options: "approve", requiring [WinRequirement::AbsolutAmount] of 5 votes
    /// and "reject", which is configured as the fallback option.
    /// - Now, if at least 5 people have voted to "approve" the proposal and they do not change
    /// their mind, there is nothing other votes can do to reject the proposal.
    ///
    /// If the deadline cannot be safely disregarded, it will be enforced and the vote will only be
    /// evaluable and implementable after it has passed.
    SoftEpochDeadline(u64),
}

impl VotingDeadline {
    /// Returns the epoch configured in the deadline variant
    pub(crate) fn epoch(&self) -> u64 {
        use VotingDeadline::*;
        match &self {
            HardEpochDeadline(epoch) | SoftEpochDeadline(epoch) => *epoch,
        }
    }
}

/// The state the vote is currently in
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub enum VoteState {
    /// The vote is ongoing and users can still vote.
    /// The vote count reflects the voting power that has currently been spent on the vote's
    /// options
    Open(VoteCount),

    /// The vote has been decided and users cannot vote anymore
    Decided {
        /// The winning options. Depending on the EvaluationSettings this might be either only one
        /// ore multiple options
        winning_option_names: Vec<String>,

        /// The vote count at the point where the vote was evaluated and determined to be decided
        vote_count: VoteCount,
    },

    /// The vote has been implemented
    /// Only truly meaningful if the implemented options where associated with at least one
    /// [CodeExecution].
    Implemented {
        /// The implemented options. If [CodeExecution]s are associated with these options they have
        /// been run by now.  Depending on the EvaluationSettings this might be either only one
        /// ore multiple options
        implemented_option_names: Vec<String>,

        /// The vote count at the point where the vote was evaluated and determined to be decided
        vote_count: VoteCount,
    },

    /// The vote has failed, either because no option was chosen and no fallback was configured or
    /// because multiple options won but the EvaluationSettings did not allow this
    Failed(VoteCount),
}

impl VoteState {
    /// Returns [VoteState::Decided] with `winning_options` set and the same vote count as this
    /// state
    fn to_decided(&self, winning_options: Vec<String>) -> Self {
        match self {
            VoteState::Open(vote_count) => {
                VoteState::Decided { winning_option_names: winning_options, vote_count: vote_count.clone() }
            }
            _ => panic!("Can only transition to Decided from Open"),
        }
    }

    /// Returns [VoteState::Failed] with the same vote count as this state
    fn to_failed(&self) -> Self {
        match self {
            VoteState::Open(vote_count) => VoteState::Failed(vote_count.clone()),
            _ => panic!("Can only transition to Failed from Open"),
        }
    }

    /// Returns [VoteState::Implemented] with the same vote count as this state and the same
    /// winning/implemented options as this vote
    pub(crate) fn to_implemented(&self) -> VoteState {
        match self {
            VoteState::Decided { winning_option_names, vote_count } => VoteState::Implemented {
                implemented_option_names: winning_option_names.clone(),
                vote_count: vote_count.clone(),
            },
            _ => panic!("Can only transition into Implemented from Decided"),
        }
    }
}

/// Represents the current vote count of an option
#[derive(Encode, Decode, TypeId, Describe, Clone, Debug)]
pub enum VoteCount {
    /// If the voting power resource is fungible, this is the amount of tokens that has been "spent"
    /// on each option
    FungibleVoteCount(HashMap<String, Decimal>),

    /// If the voting power resource is non fungible, this is a set of all voter's IDs that have
    /// vote for each option
    NonFungibleVoteCount(HashMap<String, HashSet<NonFungibleId>>),
}

impl VoteCount {
    /// Creates a new, initial vote count.
    /// For each option the vote count is either initialized to dec!(0) (fungible voting power
    /// resource) or to an empty HashSet (non-fungible voting power resource).
    ///
    /// Arguments:
    /// - `vote_config`: The config of the vote for which the initial VoteCount is created
    ///
    /// Returns: the new and initialized vote count
    pub fn new(vote_config: &VoteConfig) -> Self {
        let rm = borrow_resource_manager!(vote_config.voting_power_resource);
        match rm.resource_type() {
            ResourceType::Fungible { .. } => {
                let mut voting_powers_by_option = HashMap::new();
                for option in vote_config.options.keys() {
                    voting_powers_by_option.insert(option.to_owned(), dec!(0));
                }
                FungibleVoteCount(voting_powers_by_option)
            }
            ResourceType::NonFungible => {
                let mut member_ids_by_option_name = HashMap::new();
                for option in vote_config.options.keys() {
                    member_ids_by_option_name.insert(option.to_owned(), HashSet::new());
                }
                NonFungibleVoteCount(member_ids_by_option_name)
            }
        }
    }

    /// Returns the voting power that has been accumulated by each option.
    /// If the voting power resource is fungible, this is the amount of tokens that have been spent
    /// on each option.
    /// If the voting power resource is non-fungible, this is the count of voter IDs that have been
    /// assigned to each option.
    pub fn get_voting_power_per_option(&self) -> HashMap<String, Decimal> {
        match self {
            FungibleVoteCount(voting_powers) => voting_powers.clone(),
            NonFungibleVoteCount(member_ids) => {
                member_ids.iter().map(|(option, ids)| (option.to_owned(), Decimal::from(ids.len()))).collect()
            }
        }
    }
}

/// Represents the voting power with which a user can vote
#[derive(Encode, Decode, TypeId, Describe, Debug)]
pub enum VotingPower {
    /// A bucket with fungible tokens that the user locks into the VotingSystem component in order
    /// to cast their vote
    Fungible(Bucket),

    /// A proof of a user's non-fungible badge (usually the user's DAO membership badge)
    NonFungible(Proof),
}

/// Represents a receipt that a user receives if they cast their vote and thereby locked fungible
/// tokens in the VotingSystem component. This receipt can be used to redeem those tokens.
#[derive(NonFungibleData)]
struct VoteReceipt {
    /// The ID of the vote for which the receipt is valid.
    vote_id: NonFungibleId,

    /// The name of the option the user has voted for. This is required in case the user revokes
    /// their vote before the vote has been evaluated (i.e. is closed).
    option_name: String,

    /// The address of the tokens that the user locked in the VotingSystem component
    token_address: ResourceAddress,

    /// The amount of tokens the user locked in the voting system component
    token_amount: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_win_requirement_valid_works_as_expected() {
        let requirements = vec![
            (WinRequirement::Fallback, true),
            (WinRequirement::MaxVotingPower, true),
            (WinRequirement::AbsolutAmount(dec!("0.1")), true),
            (WinRequirement::AbsolutAmount(dec!("1000")), true),
            (WinRequirement::AbsolutAmount(dec!("0")), false),
            (WinRequirement::AbsolutAmount(dec!("-1")), false),
            (WinRequirement::AbsolutRatio(dec!("0.1")), true),
            (WinRequirement::AbsolutRatio(dec!("1")), true),
            (WinRequirement::AbsolutRatio(dec!("0")), false),
            (WinRequirement::AbsolutRatio(dec!("-1")), false),
            (WinRequirement::AbsolutRatio(dec!("1.1")), false),
            (WinRequirement::RelativeRatio(dec!("0.1")), true),
            (WinRequirement::RelativeRatio(dec!("1")), true),
            (WinRequirement::RelativeRatio(dec!("0")), false),
            (WinRequirement::RelativeRatio(dec!("-1")), false),
            (WinRequirement::RelativeRatio(dec!("1.1")), false),
        ];

        for (req, should_be_valid) in requirements {
            assert_eq!(
                req.is_valid(),
                should_be_valid,
                "Requirement {:?}: should_be_valid={}, is_valid={}",
                req,
                should_be_valid,
                req.is_valid()
            );
        }
    }

    #[test]
    fn assert_options_valid_is_true_for_proper_proposal() {
        let valid_reqs = vec![
            WinRequirement::MaxVotingPower,
            WinRequirement::AbsolutAmount(dec!(1)),
            WinRequirement::AbsolutRatio(dec!(1)),
            WinRequirement::RelativeRatio(dec!(1)),
        ];
        for req in valid_reqs {
            let mut options = HashMap::new();
            options.insert("approve".to_owned(), VoteOption::new(req));
            options.insert("reject".to_owned(), VoteOption::new(WinRequirement::Fallback));
            VoteConfig::assert_options_valid(&options);
        }
    }

    #[test]
    #[should_panic(expected = "Only one option with WinRequirement::Fallback is allowed")]
    fn test_assert_options_valid_panics_for_more_than_one_fallback_option() {
        let mut options = HashMap::new();
        options.insert("a".to_owned(), VoteOption::new(WinRequirement::MaxVotingPower));
        options.insert("b".to_owned(), VoteOption::new(WinRequirement::Fallback));
        options.insert("c".to_owned(), VoteOption::new(WinRequirement::Fallback));
        VoteConfig::assert_options_valid(&options);
    }

    #[test]
    #[should_panic(expected = "At least two options must be configured")]
    fn test_assert_options_valid_panics_for_vote_with_only_one_option() {
        let mut options = HashMap::new();
        options.insert("a".to_owned(), VoteOption::new(WinRequirement::MaxVotingPower));
        VoteConfig::assert_options_valid(&options);
    }

    #[test]
    #[should_panic(expected = "Option a has invalid requirement: AbsolutRatio(-1)")]
    fn test_assert_options_valid_panics_for_option_with_invalid_requirement() {
        let mut options = HashMap::new();
        options.insert("a".to_owned(), VoteOption::new(WinRequirement::AbsolutRatio(dec!(-1))));
        options.insert("b".to_owned(), VoteOption::new(WinRequirement::Fallback));
        VoteConfig::assert_options_valid(&options);
    }

    impl VoteOption {
        fn new(requirement: WinRequirement) -> Self {
            Self { description: None, code_executions: vec![], requirement }
        }
    }
}
