mod common;

use common::*;

use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;

use dao_kit::simple_dao_system::DaoSystemAddresses;
use dao_kit::demo_do_good_dao::{BoardMember, Charity, CharityChange};
use dao_kit::voting_system::{Vote, VoteCount, VoteState};

/// Simulates an example transaction flow for the DoGoodDao component
#[test]
fn simulate_example_transaction_flow() {
    // Create a testing environment
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();
    let mut env = TestEnv::new(&mut store);

    // Create an account for John
    let johns_account = env.new_account();

    // John instantiates the DoGooDao with himself as the only member
    let john = BoardMember::new("John".to_owned(), "Doe".to_owned());
    let initial_members = vec![(john, johns_account.account_component)];
    let (do_good_dao_component, dao_system_addresses, donor_resource) = env.instantiate_dao(initial_members).unwrap();
    assert_eq!(
        env.get_balance(&johns_account, dao_system_addresses.membership_resource),
        dec!(1)
    );

    // Create an account for jane
    let janes_account = env.new_account();

    // John proposes to add Jane as a board member
    let jane = BoardMember::new("Jane".to_owned(), "Doe".to_owned());
    let vote: Vote = env
        .propose_add_board_member(
            &johns_account,
            do_good_dao_component,
            &dao_system_addresses,
            jane,
            janes_account.account_component,
        )
        .unwrap();
    assert!(matches!(vote.state, VoteState::Open(..)));
    // Jane is only proposed as a member at this point. The change has not been executed yet
    assert_eq!(
        env.get_balance(&janes_account, dao_system_addresses.membership_resource),
        dec!(0)
    );

    // John that votes to approve Jane as a new member
    let (vote, _receipt) = env
        .cast_vote(&johns_account, &dao_system_addresses, &vote.id, "approve".to_owned())
        .unwrap();
    assert!(matches!(
        vote.state,
        VoteState::Open(VoteCount::NonFungibleVoteCount(..))
    ));

    // He evaluates the vote which succeeds even though the deadline is at epoch 350.
    // This is because the deadline is defined as a soft deadline and it can already be safely
    // concluded that the proposal's approve option has already met it's absolut requirement
    let vote = env.evaluate_vote(&johns_account, &dao_system_addresses, &vote.id).unwrap();
    match &vote.state {
        VoteState::Decided { winning_option_names, .. } => {
            assert_eq!(winning_option_names, &vec!["approve"]);
        }
        unexpected_state => panic!("Unexpected vote state: {:?}", unexpected_state)
    }

    // Jane is only proposed as a member at this point. The change has been approved but not executed yet
    assert_eq!(
        env.get_balance(&janes_account, dao_system_addresses.membership_resource),
        dec!(0)
    );

    // John implements the vote
    env.implement_vote(&johns_account, &dao_system_addresses, &vote)
        .unwrap();
    // Jane has now become a member
    assert_eq!(
        env.get_balance(&janes_account, dao_system_addresses.membership_resource),
        dec!(1)
    );

    // Simulate that John proposes to add a fake charity which points it to his own account
    let change = CharityChange::CreateNewCharity {
        name: "Some fake charity".to_owned(),
        account_address: johns_account.account_component,
    };
    let vote = env
        .propose_implement_charity_change(&johns_account, do_good_dao_component, &dao_system_addresses, change)
        .unwrap();

    // John votes to approve his own proposal
    env.cast_vote(&johns_account, &dao_system_addresses, &vote.id, "approve".to_owned())
        .unwrap();
    // He even votes twice... (this does nothing ;-))
    env.cast_vote(&johns_account, &dao_system_addresses, &vote.id, "approve".to_owned())
        .unwrap();

    // Jane does not even catch John's attempt at fraud and thus does not reject the proposal

    // John tries to evaluate his proposal right on the spot, not waiting for the deadline.
    // This should fail, as the proposal has not gotten >50% approve votes and thus the soft deadline
    // is enforced
    env.evaluate_vote(&johns_account, &dao_system_addresses, &vote.id)
        .expect_err("proposal should not evaluable yet");

    // John waits until the deadline has passed...
    env.test_runner.set_current_epoch(702);
    //  ...and evaluates the vote again
    let vote = env
        .evaluate_vote(&johns_account, &dao_system_addresses, &vote.id)
        .unwrap();

    // This time the evaluation has succeeded but the proposal was of course rejected
    match &vote.state {
        VoteState::Decided {
            winning_option_names, ..
        } => {
            assert_eq!(winning_option_names, &vec!["reject"]);
        }
        unexpected_state => panic!("Expected vote to be in state Decided but was {:?}", unexpected_state),
    }

    // The vote can also be implemented but, because no code executions are associated with the reject option,
    // nothing changes (except for the votes state)
    let vote = env
        .implement_vote(&johns_account, &dao_system_addresses, &vote)
        .unwrap();
    match vote.state {
        VoteState::Implemented {
            implemented_option_names,
            ..
        } => {
            assert_eq!(implemented_option_names, vec!["reject"]);
        }
        unexpected_state => panic!(
            "Expected vote to be in state Implemented but was {:?}",
            unexpected_state
        ),
    }

    // Indeed the list of charities is still empty
    let charities = env.get_charities(&johns_account, do_good_dao_component).unwrap();
    assert!(charities.is_empty(), "No charity should exist yet");

    // Now Jane proposes to add a proper charity which both she and John vote to approve
    let charity_account = env.new_account();
    let change = CharityChange::CreateNewCharity {
        name: "A proper charity".to_owned(),
        account_address: charity_account.account_component,
    };
    let vote = env
        .propose_implement_charity_change(&janes_account, do_good_dao_component, &dao_system_addresses, change)
        .unwrap();
    env.cast_vote(&johns_account, &dao_system_addresses, &vote.id, "approve".to_owned())
        .unwrap();
    env.cast_vote(&janes_account, &dao_system_addresses, &vote.id, "approve".to_owned())
        .unwrap();

    env.test_runner.set_current_epoch(702);
    let vote = env
        .evaluate_vote(&johns_account, &dao_system_addresses, &vote.id)
        .unwrap();
    match &vote.state {
        VoteState::Decided {
            winning_option_names, ..
        } => {
            assert_eq!(winning_option_names, &vec!["approve"]);
        }
        unexpected_state => panic!("Expected vote to be in state Decided but was {:?}", unexpected_state),
    }
    let _vote = env
        .implement_vote(&janes_account, &dao_system_addresses, &vote)
        .unwrap();

    // After implementing Jane's proposal there is finally one charity
    let mut charities = env.get_charities(&johns_account, do_good_dao_component).unwrap();
    match charities.as_slice() {
        [charity] => {
            assert_eq!(charity.name, "A proper charity");
            assert_eq!(charity.account_address, charity_account.account_component);
            assert_eq!(charity.donations_received, dec!(0));
        }
        _ => panic!("There should be exactly one charity now"),
    }
    let charity = charities.remove(0);

    // Now a donor registers
    let donor_account = env.new_account();
    env.register_as_donor(&donor_account, do_good_dao_component).unwrap();

    // And makes two donations
    env.make_donation(&donor_account, do_good_dao_component, charity.id.to_owned(), donor_resource, dec!(100)).unwrap();
    env.make_donation(&donor_account, do_good_dao_component, charity.id, donor_resource, dec!(50)).unwrap();

    // now Jane checks the charities and discovers that a donation has been made
    let charities = env.get_charities(&janes_account, do_good_dao_component).unwrap();
    match charities.as_slice() {
        [charity] => {
            assert_eq!(charity.donations_received, dec!(150))
        }
        _ => panic!("There should be exactly one charity now"),
    }
    println!("{}", vote.id)
}

trait DoGoodDaoMethods {
    fn instantiate_dao(
        &mut self,
        initial_members: Vec<(BoardMember, ComponentAddress)>,
    ) -> Result<(ComponentAddress, DaoSystemAddresses, ResourceAddress), TransactionError>;

    fn propose_add_board_member(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        new_member: BoardMember,
        new_member_account: ComponentAddress,
    ) -> Result<Vote, TransactionError>;

    fn propose_implement_charity_change(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        change: CharityChange,
    ) -> Result<Vote, TransactionError>;

    fn get_charities(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
    ) -> Result<Vec<Charity>, TransactionError>;

    fn register_as_donor(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
    ) -> Result<(), TransactionError>;

    fn make_donation(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
        charity_id: NonFungibleId,
        donor_resource: ResourceAddress,
        donation_amount: Decimal,
    ) -> Result<(), TransactionError>;
}

impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> DoGoodDaoMethods for TestEnv<'s, S> {
    fn instantiate_dao(
        &mut self,
        initial_members: Vec<(BoardMember, ComponentAddress)>,
    ) -> Result<(ComponentAddress, DaoSystemAddresses, ResourceAddress), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_function(
                self.package_address,
                "DoGoodDao",
                "instantiate_global",
                args!(initial_members),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![self.admin_account.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn propose_add_board_member(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        new_member: BoardMember,
        new_member_account: ComponentAddress,
    ) -> Result<Vote, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .call_method(
                do_good_dao_component,
                "propose_add_board_member",
                args!(new_member, new_member_account),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }

    fn propose_implement_charity_change(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        change: CharityChange,
    ) -> Result<Vote, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .call_method(do_good_dao_component, "propose_implement_charity_change", args!(change))
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }

    fn get_charities(
        &mut self,
        actor: &Account,
        do_good_dao_component: ComponentAddress,
    ) -> Result<Vec<Charity>, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(do_good_dao_component, "get_charities", args!())
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn register_as_donor(&mut self, actor: &Account, do_good_dao_component: ComponentAddress) -> Result<(), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(do_good_dao_component, "register_as_donor", args!())
            .call_method(actor.account_component, "deposit_batch", args!(Expression::entire_worktop()))
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(0)
    }

    fn make_donation(&mut self, actor: &Account, do_good_dao_component: ComponentAddress,
                     charity_id: NonFungibleId, donor_resource: ResourceAddress,
                     donation_amount: Decimal) -> Result<(), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .withdraw_from_account_by_amount(donation_amount, RADIX_TOKEN, actor.account_component)
            .take_from_worktop_by_amount(donation_amount, RADIX_TOKEN, |builder, bucket_id|
                builder.create_proof_from_account(donor_resource, actor.account_component)
                    .pop_from_auth_zone(|builder, proof_id|
                        builder.call_method(do_good_dao_component, "make_donation",
                                            args!(charity_id,Bucket(bucket_id), Proof(proof_id) ))),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(0)
    }
}
