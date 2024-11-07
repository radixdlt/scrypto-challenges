use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;
use transaction::manifest::decompiler::ManifestObjectNames;

#[test]
fn instantiate() {
    TestEnvironment::instantiate();
}

#[test]
fn tokenize_yield() {
    let mut test_environment = TestEnvironment::instantiate();
    test_environment.tokenize_yield()
    .expect_commit_success();
}

#[test]
fn redeem() {
    let mut test_environment = TestEnvironment::instantiate();

    test_environment.tokenize_yield()
    .expect_commit_success();

    test_environment.redeem()
    .expect_commit_success();
}

#[test]
fn can_redeem_from_pt_after_maturity() {
    let mut test_environment = TestEnvironment::instantiate();

    test_environment.tokenize_yield()
    .expect_commit_success();

    let date = 
        UtcDateTime::new(
            2025, 
            03, 
            06, 
            0, 
            0, 
            0
        ).ok().unwrap();

    test_environment.advance_date(date);

    test_environment.redeem_from_pt()
    .expect_commit_success();
}

#[test]
fn cannot_redeem_from_pt_before_maturity() {
    let mut test_environment = TestEnvironment::instantiate();

    test_environment.tokenize_yield()
    .expect_commit_success();

    test_environment.redeem_from_pt()
    .expect_commit_failure();
}

#[test]
fn cannot_tokenize_yield_after_maturity() {
    let mut test_environment = TestEnvironment::instantiate();

    let date = 
        UtcDateTime::new(
            2025, 
            03, 
            06, 
            0, 
            0, 
            0
        ).ok().unwrap();

    test_environment.advance_date(date);

    test_environment.tokenize_yield()
    .expect_commit_failure();
}


#[derive(ScryptoSbor, ManifestSbor)]
pub enum Expiry {
    TwelveMonths,
    EighteenMonths,
    TwentyFourMonths,
}

pub struct Account {
    public_key: Secp256k1PublicKey,
    account_component: ComponentAddress,
}

pub struct TestEnvironment {
    test_runner: DefaultTestRunner,
    account: Account,
    tokenizer_component: ComponentAddress,
    lsu_resource_address: ResourceAddress,
    pt_resource: ResourceAddress,
    yt_resource: ResourceAddress,
}

impl TestEnvironment {
    pub fn instantiate() -> Self {

        let genesis_epoch = Epoch::of(2);
        let epoch_emissions_xrd = dec!("100");
        let validator_key = Secp256k1PrivateKey::from_u64(1u64).unwrap().public_key();
        
        let validators = vec![GenesisValidator::from(validator_key)];

        let accounts = validators
            .iter()
            .map(|validator| validator.owner)
            .collect::<Vec<_>>();

            let allocations = vec![
                (
                    validator_key,
                    vec![GenesisStakeAllocation {
                        account_index: 0,
                        xrd_amount: dec!(1000),
                    }],
                ),
            ];
        let genesis_data_chunks = vec![
            GenesisDataChunk::Validators(validators),
            GenesisDataChunk::Stakes {
                accounts,
                allocations,
            },
        ];

        let current_date = UtcDateTime::new(2024, 03, 05, 0, 0, 0).ok().unwrap();
        let current_date_ms = current_date.to_instant().seconds_since_unix_epoch * 1000;

        let custom_genesis = 
            CustomGenesis {
                genesis_data_chunks,
                genesis_epoch,
                initial_config: CustomGenesis::default_consensus_manager_config()
                    .with_epoch_change_condition(EpochChangeCondition {
                        min_round_count: 1,
                        max_round_count: 1, // deliberate, to go through rounds/epoch without gaps
                        target_duration_millis: 0,
                    })
                    .with_total_emission_xrd_per_epoch(epoch_emissions_xrd),
                initial_time_ms: current_date_ms,
                initial_current_leader: Some(0),
                faucet_supply: *DEFAULT_TESTING_FAUCET_SUPPLY,
            };
        // Setup the environment
        let mut test_runner = TestRunnerBuilder::new()
            .with_custom_genesis(custom_genesis)
            .without_trace()
            .build();

        // Create an account
        let (public_key, _private_key, account_component) = 
            test_runner.new_allocated_account();

            let account = Account {
                public_key,
                account_component,
            };
        
        let validator_address = 
            test_runner.get_active_validator_with_key(&validator_key);
        let lsu_resource_address = 
            test_runner.get_active_validator_info_by_key(&validator_key).stake_unit_resource;

        let manifest = ManifestBuilder::new()
            .withdraw_from_account(
                account_component, 
                XRD, 
                dec!(1000)
            )
            .take_all_from_worktop(
                XRD, 
                "xrd"
            )
            .call_method_with_name_lookup(
                validator_address,
                "stake",
                |lookup| (
                    lookup.bucket("xrd"),
                )
            )
            .deposit_batch(account_component)
            .build();

        test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        ).expect_commit_success();
    
        // Publish package
        let package_address = test_runner.compile_and_publish(this_package!());
        
        let expiry = Expiry::TwelveMonths;
        
        let manifest = ManifestBuilder::new()
            .call_function(
                package_address,
                "YieldTokenizer",
                "instantiate_yield_tokenizer",
                manifest_args!(
                    expiry,
                    lsu_resource_address
                ),
            )
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        );

        let tokenizer_component = receipt.expect_commit(true).new_component_addresses()[0];
        let pt_resource = receipt.expect_commit(true).new_resource_addresses()[0];
        let yt_resource = receipt.expect_commit(true).new_resource_addresses()[1];

        Self {
            test_runner,
            account,
            tokenizer_component,
            lsu_resource_address,
            pt_resource,
            yt_resource
        }
    }

    pub fn advance_date(
        &mut self,
        date: UtcDateTime,
    ) {
        let date_ms = date.to_instant().seconds_since_unix_epoch * 1000;
        let receipt = self.test_runner.advance_to_round_at_timestamp(
            Round::of(3), 
            date_ms
        );
        receipt.expect_commit_success();
    }

    pub fn execute_manifest(
        &mut self, 
        object_manifest: ManifestObjectNames, 
        built_manifest: TransactionManifestV1,
        name: &str
    ) -> TransactionReceiptV1 {
        dump_manifest_to_file_system(
            object_manifest, 
            &built_manifest, 
            "./transaction_manifest", 
            Some(name), 
            &NetworkDefinition::stokenet()
        ).ok();

        let receipt = self.test_runner.execute_manifest_ignoring_fee(
            built_manifest,
            vec![NonFungibleGlobalId::from_public_key(&self.account.public_key)],
        );

        return receipt
    }

    pub fn tokenize_yield(&mut self) -> TransactionReceiptV1 {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(
                self.account.account_component,
                self.lsu_resource_address,
                dec!(1000)
            )
            .take_all_from_worktop(
                self.lsu_resource_address,
                "LSU Bucket"
            )
            .call_method_with_name_lookup(
                self.tokenizer_component,
                "tokenize_yield",
                |lookup| (
                    lookup.bucket("LSU Bucket"),
                )
            )
            .deposit_batch(self.account.account_component);

        self.execute_manifest(
            manifest.object_names(),
            manifest.build(),
            "tokenize_yield"
        )
    }

    pub fn redeem(&mut self) -> TransactionReceiptV1 {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(
                self.account.account_component,
                self.pt_resource,
                dec!(1000)
            )
            .withdraw_from_account(
                self.account.account_component,
                self.yt_resource,
                dec!(1)
            )
            .take_all_from_worktop(
                self.pt_resource,
                "PT Bucket"
            )
            .take_all_from_worktop(
                self.yt_resource,
                "YT Bucket"
            )
            .call_method_with_name_lookup(
                self.tokenizer_component,
                "redeem",
                |lookup| (
                    lookup.bucket("PT Bucket"),
                    lookup.bucket("YT Bucket"),
                    dec!(1000)
                )
            )
            .deposit_batch(self.account.account_component);

        self.execute_manifest(
            manifest.object_names(),
            manifest.build(),
            "redeem"
        )
    }

    pub fn redeem_from_pt(&mut self) -> TransactionReceiptV1 {
        let manifest = ManifestBuilder::new()
            .withdraw_from_account(
                self.account.account_component,
                self.pt_resource,
                dec!(1000)
            )
            .take_all_from_worktop(
                self.pt_resource,
                "PT Bucket"
            )
            .call_method_with_name_lookup(
                self.tokenizer_component,
                "redeem_from_pt",
                |lookup| (
                    lookup.bucket("PT Bucket"),
                )
            )
            .deposit_batch(self.account.account_component);

        self.execute_manifest(
            manifest.object_names(),
            manifest.build(),
            "redeem_from_pt"
        )
    }

    pub fn claim_yield(
        &mut self,
        local_id: NonFungibleLocalId
    ) -> TransactionReceiptV1 {
        let manifest = ManifestBuilder::new()
            .create_proof_from_account_of_non_fungibles(
                self.account.account_component,
                self.yt_resource,
                [local_id],
            )
            .pop_from_auth_zone(
                "YT Proof"
            )
            .call_method_with_name_lookup(
                self.tokenizer_component,
                "claim_yield",
                |lookup| (
                    lookup.proof("YT Proof"),
                )
            )
            .deposit_batch(self.account.account_component);

        self.execute_manifest(
            manifest.object_names(),
            manifest.build(),
            "claim_yield"
        )
    }
}
