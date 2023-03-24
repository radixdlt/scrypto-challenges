use radix_engine::ledger::*;
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use transaction::builder::ManifestBuilder;

use common::*;
use dao_kit::code_execution_system::CodeExecution;
use dao_kit::component_address_repo::ComponentAddressLookup;
use dao_kit::simple_dao_system::DaoSystemAddresses;
use dao_kit::voting_system::Vote;

mod common;

/// Simulates some example usage of the FlexDao
#[test]
fn simulate_example_transaction_flow() {
    // Create a testing environment
    let mut store: TypedInMemorySubstateStore = TypedInMemorySubstateStore::with_bootstrap();
    let mut env = TestEnv::new(&mut store);

    // Create an account for John
    let johns_account = env.new_account();

    let initial_members = vec![("John".to_owned(), johns_account.account_component)];
    let (flex_dao_component, dao_system_addresses, component_address_repo)
        = env.instantiate_dao(&johns_account, initial_members).unwrap();
    let flex_dao_component_address_lookup = env.create_component_address_lookup(
        &johns_account, component_address_repo, flex_dao_component).unwrap();

    let code_execution_set_usage_fee = CodeExecution::MethodCall {
        component: flex_dao_component_address_lookup,
        method: "set_usage_fee_pct".to_string(),
        args: args!(dec!("9.99")),
        required_badges: vec![dao_system_addresses.dao_system_admin_badge_resource],
    };
    let proposal = env.create_proposal(&johns_account, flex_dao_component, &dao_system_addresses,
                                       "Set usage_fee_pct to 9.99".to_owned(), vec!(code_execution_set_usage_fee)).unwrap();
    env.cast_vote(&johns_account, &dao_system_addresses, &proposal.id, "approve".to_owned()).unwrap();
    env.test_runner.set_current_epoch(200);

    let proposal = env.evaluate_vote(&johns_account, &dao_system_addresses, &proposal.id).unwrap();

    let current_fee = env.get_usage_fee_pct(&johns_account, flex_dao_component).unwrap();
    assert_eq!(current_fee, dec!("2.5"));

    env.implement_vote(&johns_account, &dao_system_addresses, &proposal).unwrap();
    let current_fee = env.get_usage_fee_pct(&johns_account, flex_dao_component).unwrap();
    assert_eq!(current_fee, dec!("9.99"));
}

trait DoGoodDaoMethods {
    fn instantiate_dao(
        &mut self,
        actor: &Account,
        initial_members: Vec<(String, ComponentAddress)>,
    ) -> Result<(ComponentAddress, DaoSystemAddresses, ComponentAddress), TransactionError>;

    fn create_proposal(
        &mut self,
        actor: &Account,
        flex_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        proposal_name: String,
        code_executions: Vec<CodeExecution>,
    ) -> Result<Vote, TransactionError>;

    fn get_usage_fee_pct(
        &mut self,
        actor: &Account,
        flex_dao_component: ComponentAddress,
    ) -> Result<Decimal, TransactionError>;

    fn create_component_address_lookup(
        &mut self,
        actor: &Account,
        component_address_repo: ComponentAddress,
        component_address: ComponentAddress,
    ) -> Result<ComponentAddressLookup, TransactionError>;
}

impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> DoGoodDaoMethods for TestEnv<'s, S> {
    fn instantiate_dao(
        &mut self,
        actor: &Account,
        initial_members: Vec<(String, ComponentAddress)>,
    ) -> Result<(ComponentAddress, DaoSystemAddresses, ComponentAddress), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_function(
                self.package_address,
                "FlexDao",
                "instantiate_global",
                args!(initial_members),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn create_proposal(
        &mut self,
        actor: &Account,
        flex_dao_component: ComponentAddress,
        dao_system_addresses: &DaoSystemAddresses,
        proposal_name: String,
        code_executions: Vec<CodeExecution>,
    ) -> Result<Vote, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .call_method(
                flex_dao_component,
                "create_proposal",
                args!(proposal_name, code_executions),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(2)
    }

    fn get_usage_fee_pct(&mut self, actor: &Account, flex_dao_component: ComponentAddress) -> Result<Decimal, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(flex_dao_component, "get_usage_fee_pct", args!())
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    fn create_component_address_lookup(
        &mut self,
        actor: &Account,
        component_address_repo: ComponentAddress,
        component_address: ComponentAddress,
    ) -> Result<ComponentAddressLookup, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(component_address_repo, "create_lookup", args!(component_address))
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }
}
