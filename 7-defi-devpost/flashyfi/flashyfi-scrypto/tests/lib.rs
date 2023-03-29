use radix_engine::engine::{RejectionError, RuntimeError};
use radix_engine::transaction::{AbortReason, TransactionOutcome, TransactionReceipt, TransactionResult};
use radix_engine::types::node::NetworkDefinition;
use radix_engine::types::GlobalAddress;
use radix_engine_interface::model::FromPublicKey;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::manifest::decompile;
use transaction::model::{BasicInstruction, TransactionManifest};
use transaction::signing::EcdsaSecp256k1PrivateKey;

use flashyfi_scrypto::flashyfi::{Fee, FlashyfiAddresses, FungibleAmount, ResourceAmount};

#[test]
fn happy_path() {
    // Setup the environment
    let test_runner = TestRunner::builder().without_trace().build();
    let mut env = TestEnv::new(test_runner);

    let admin = env.new_account();

    let (flashyfi_component, flashyfi_addresses) = env.instantiate_component(&admin, false).unwrap();

    let lender = env.new_account();
    let mut fungible_fee_configs = HashMap::new();
    fungible_fee_configs.insert(RADIX_TOKEN, (true, Fee::Percentage(dec!("1"))));
    env.flashyfi_account(&lender, flashyfi_component, &flashyfi_addresses, fungible_fee_configs, HashMap::new())
        .unwrap();

    let borrower = env.new_account();
    env.borrow_tokens(&borrower, flashyfi_component, &flashyfi_addresses, lender.account_component).unwrap();
}

impl TestEnv {
    fn instantiate_component(
        &mut self,
        actor: &Account,
        allow_regular_components: bool,
    ) -> Result<(ComponentAddress, FlashyfiAddresses), TransactionError> {
        let manifest = ManifestBuilder::new()
            .call_function(self.package_address, "Flashyfi", "instantiate_global", args!(allow_regular_components))
            .build();
        println!("{}", decompile(&manifest.instructions, &NetworkDefinition::nebunet()).unwrap());
        self.submit_transaction_and_return_nth_output(manifest, actor, 1)
    }

    fn flashyfi_account(
        &mut self,
        actor: &Account,
        flashyfi_component: ComponentAddress,
        flashyfi_addresses: &FlashyfiAddresses,
        fungible_fee_configs: HashMap<ResourceAddress, (bool, Fee)>,
        non_fungible_fee_configs: HashMap<ResourceAddress, (bool, FungibleAmount)>,
    ) -> Result<(), TransactionError> {
        let create_set_method_access_rule_instruction = |method_name: &str| BasicInstruction::SetMethodAccessRule {
            entity_address: GlobalAddress::Component(actor.account_component),
            index: 1,
            key: AccessRuleKey::ScryptoMethod(method_name.to_string()),
            rule: AccessRule::Protected(AccessRuleNode::AnyOf(vec![
                AccessRuleNode::ProofRule(ProofRule::Require(actor.public_key_global_id().into())),
                AccessRuleNode::ProofRule(ProofRule::Require(flashyfi_addresses.flashyfi_badge_resource.into())),
            ])),
        };

        let mut manifest = ManifestBuilder::new();
        manifest.add_instruction(create_set_method_access_rule_instruction("withdraw_by_amount"));
        manifest.add_instruction(create_set_method_access_rule_instruction("withdraw_by_ids"));
        manifest.call_method(flashyfi_component, "flashyfi_account", args!(actor.account_component));
        manifest.create_proof_from_account(actor.account_component, flashyfi_addresses.account_config_badge_resource);
        manifest.pop_from_auth_zone(|manifest, account_config_badge| {
            manifest.call_method(
                flashyfi_component,
                "update_account_config",
                args!(account_config_badge, fungible_fee_configs, non_fungible_fee_configs),
            )
        });

        let ms = decompile(&manifest.build().instructions, &NetworkDefinition::nebunet()).unwrap();
        println!("{ms}");

        self.submit_transaction_and_return_nth_output::<()>(manifest.build(), actor, 0)
    }

    pub(crate) fn borrow_tokens(
        &mut self,
        actor: &Account,
        flashyfi_component: ComponentAddress,
        flashyfi_addresses: &FlashyfiAddresses,
        lender_account: ComponentAddress,
    ) -> Result<(), TransactionError> {
        let manifest = ManifestBuilder::new()
            .call_method(
                flashyfi_component,
                "borrow",
                args!(ResourceAmount::FungibleAmount(FungibleAmount::new(RADIX_TOKEN, dec!("100"))), lender_account),
            )
            .withdraw_from_account_by_amount(actor.account_component, dec!(1), RADIX_TOKEN)
            .take_from_worktop_by_amount(dec!("1"), RADIX_TOKEN, |builder, fee_bucket| {
                builder.take_from_worktop_by_amount(dec!("100"), RADIX_TOKEN, |builder, borrowed_tokens_bucket| {
                    builder.take_from_worktop(flashyfi_addresses.loan_receipt_resource, |builder, receipt_bucket| {
                        builder.call_method(
                            flashyfi_component,
                            "repay_loan",
                            args!(receipt_bucket, borrowed_tokens_bucket, fee_bucket, lender_account),
                        )
                    })
                })
            })
            .take_from_worktop(RADIX_TOKEN, |builder, xrd| {
                builder.call_method(actor.account_component, "deposit", args!(xrd))
            })
            .build();
        println!("{}", decompile(&manifest.instructions, &NetworkDefinition::nebunet()).unwrap());

        self.submit_transaction_and_return_nth_output(manifest, actor, 0)
    }
}

pub struct TestEnv {
    pub test_runner: TestRunner,
    pub admin_account: Account,
    pub package_address: PackageAddress,
}

impl TestEnv {
    pub fn new(mut test_runner: TestRunner) -> Self {
        let (public_key, private_key, account_component) = test_runner.new_account(true);
        let admin_account = Account { public_key, private_key, account_component };

        let package_address = test_runner.compile_and_publish(this_package!());

        Self { test_runner, admin_account, package_address }
    }

    pub fn new_account(&mut self) -> Account {
        let (public_key, private_key, account_component) = self.test_runner.new_allocated_account();
        Account { public_key, private_key, account_component }
    }

    #[allow(unused)]
    pub fn get_balance(&mut self, account: &Account, resource: ResourceAddress) -> Decimal {
        let manifest =
            ManifestBuilder::new().call_method(account.account_component, "balance", args!(resource)).build();
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![account.public_key_global_id()]);
        println!("{receipt:?}\n");
        receipt.expect_commit_success();
        receipt.output(1)
    }

    #[allow(unused)]
    pub fn submit_transaction_and_return_nth_output<T>(
        &mut self,
        manifest: TransactionManifest,
        actor: &Account,
        nth: usize,
    ) -> Result<T, TransactionError>
    where
        T: ScryptoDecode,
    {
        let receipt = self.test_runner.execute_manifest_ignoring_fee(manifest, vec![actor.public_key_global_id()]);
        println!("{receipt:?}\n");
        receipt.get_output(nth)
    }
}

#[derive(Debug)]
pub enum TransactionError {
    TransactionFailure(RuntimeError),
    TransactionRejected(RejectionError),
    TransactionAborted(AbortReason),
}

pub struct Account {
    pub public_key: EcdsaSecp256k1PublicKey,
    pub private_key: EcdsaSecp256k1PrivateKey,
    pub account_component: ComponentAddress,
}

impl Account {
    fn public_key_global_id(&self) -> NonFungibleGlobalId {
        NonFungibleGlobalId::from_public_key(&self.public_key)
    }
}

pub trait GetOutput {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: ScryptoDecode;
}

impl GetOutput for TransactionReceipt {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: ScryptoDecode,
    {
        match self.result {
            TransactionResult::Commit(c) => match c.outcome {
                TransactionOutcome::Success(x) => {
                    let encoded = &x[nth].as_vec();
                    let decoded = scrypto_decode::<T>(encoded).expect("Wrong instruction output type!");
                    Ok(decoded)
                }
                TransactionOutcome::Failure(err) => Err(TransactionError::TransactionFailure(err)),
            },
            TransactionResult::Reject(reject_result) => Err(TransactionError::TransactionRejected(reject_result.error)),
            TransactionResult::Abort(abort_result) => Err(TransactionError::TransactionAborted(abort_result.reason)),
        }
    }
}
