use dao_kit::simple_dao_system::DaoSystemAddresses;
use dao_kit::voting_system::{Vote, VoteState, VotingPower};
use radix_engine::engine::{RejectionError, RuntimeError};
use radix_engine::ledger::{ReadableSubstateStore, WriteableSubstateStore};
use radix_engine::transaction::{TransactionOutcome, TransactionReceipt, TransactionResult};
use scrypto::core::NetworkDefinition;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::signing::EcdsaSecp256k1PrivateKey;

pub struct TestEnv<'s, S: ReadableSubstateStore + WriteableSubstateStore> {
    pub test_runner: TestRunner<'s, S>,
    pub admin_account: Account,
    pub package_address: PackageAddress,
}

// Basic methods
impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    pub fn new(store: &'s mut S) -> Self {
        let mut test_runner = TestRunner::new(false, store);
        let (public_key, private_key, account_component) = test_runner.new_account();
        let admin_account = Account {
            public_key,
            private_key,
            account_component,
        };

        let package_address = test_runner.compile_and_publish(this_package!());

        Self {
            test_runner,
            admin_account,
            package_address,
        }
    }

    pub fn new_account(&mut self) -> Account {
        let (public_key, private_key, account_component) = self.test_runner.new_account();
        Account {
            public_key,
            private_key,
            account_component,
        }
    }

    #[allow(unused)]
    pub fn get_balance(&mut self, account: &Account, resource: ResourceAddress) -> Decimal {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(account.account_component, "balance", args!(resource))
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![account.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.expect_commit_success();
        receipt.output(1)
    }
}

// Voting system methods
impl<'s, S: ReadableSubstateStore + WriteableSubstateStore> TestEnv<'s, S> {
    pub fn cast_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote_id: &NonFungibleId,
        option: String,
    ) -> Result<(Vote, Option<Bucket>), TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    dao_system_addresses.voting_system_component,
                    "cast_vote",
                    args!(vote_id.to_owned(), option, VotingPower::NonFungible(Proof(proof_id))),
                )
            })
            .call_method(
                actor.account_component,
                "deposit_batch",
                args!(Expression::entire_worktop()),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(3)
    }

    pub fn evaluate_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote_id: &NonFungibleId,
    ) -> Result<Vote, TransactionError> {
        let manifest = ManifestBuilder::new(&NetworkDefinition::simulator())
            .call_method(
                dao_system_addresses.voting_system_component,
                "evaluate_vote",
                args!(*vote_id),
            )
            .call_method(
                actor.account_component,
                "deposit_batch",
                args!(Expression::entire_worktop()),
            )
            .build();
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest, vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        receipt.get_output(1)
    }

    pub fn implement_vote(
        &mut self,
        actor: &Account,
        dao_system_addresses: &DaoSystemAddresses,
        vote: &Vote,
    ) -> Result<Vote, TransactionError> {
        let winning_options = if let VoteState::Decided {
            winning_option_names, ..
        } = &vote.state
        {
            winning_option_names
        } else {
            panic!("Vote must be in state Decided")
        };
        let has_code_executions =
            vote.config.options.iter().any(|(option_name, option)| {
                winning_options.contains(option_name) && !option.code_executions.is_empty()
            });

        let mut manifest = ManifestBuilder::new(&NetworkDefinition::simulator());
        manifest
            .create_proof_from_account(dao_system_addresses.membership_resource, actor.account_component)
            .pop_from_auth_zone(|builder, proof_id| {
                builder.call_method(
                    dao_system_addresses.voting_system_component,
                    "implement_vote",
                    args!(vote.id, Proof(proof_id)),
                )
            });
        if has_code_executions {
            manifest.take_from_worktop(
                dao_system_addresses.code_execution_resource,
                |builder, code_execution_bucket_id| {
                    builder.call_method(
                        dao_system_addresses.code_execution_system_component,
                        "execute_code",
                        args!(Bucket(code_execution_bucket_id)),
                    )
                },
            );
        }
        let receipt = self
            .test_runner
            .execute_manifest_ignoring_fee(manifest.build(), vec![actor.public_key.into()]);
        println!("{:?}\n", receipt);
        let output: Result<(Vote, Option<Bucket>), TransactionError> = receipt.get_output(3);
        output.map(|(vote, _)| vote)
    }
}

pub trait GetOutput {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: Decode + TypeId;
}

impl GetOutput for TransactionReceipt {
    fn get_output<T>(self, nth: usize) -> Result<T, TransactionError>
    where
        T: Decode + TypeId,
    {
        match self.result {
            TransactionResult::Commit(c) => match c.outcome {
                TransactionOutcome::Success(x) => {
                    let encoded = &x[nth][..];
                    let decoded = scrypto_decode::<T>(encoded).expect("Wrong instruction output type!");
                    Ok(decoded)
                }
                TransactionOutcome::Failure(err) => Err(TransactionError::TransactionFailure(err)),
            },
            TransactionResult::Reject(err) => Err(TransactionError::TransactionRejected(err.error)),
        }
    }
}

#[derive(Debug)]
pub enum TransactionError {
    TransactionFailure(RuntimeError),
    TransactionRejected(RejectionError),
}

pub struct Account {
    pub public_key: EcdsaSecp256k1PublicKey,
    pub private_key: EcdsaSecp256k1PrivateKey,
    pub account_component: ComponentAddress,
}
