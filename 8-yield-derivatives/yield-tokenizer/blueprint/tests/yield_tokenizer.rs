use scrypto_test::{prelude::*, utils::dump_manifest_to_file_system};

// INSTANTIATE
#[test]
fn instantiate() {
   LocalTestEnvironment::instantiate();
}

// TOKENIZE
#[test]
fn tokenize_yield() {
   let mut test_environment = LocalTestEnvironment::instantiate();
   test_environment.tokenize_yield().expect_commit_success();
}

#[test]
fn tokenize_yield_with_xrd() {
   let mut test_environment = LocalTestEnvironment::instantiate();
   test_environment.tokenize_yield_with_xrd().expect_commit_success();
}

// CLAIM
#[test]
fn claim_yield_for_sxrd_no_reward() {
   let mut test_environment = LocalTestEnvironment::instantiate();

   test_environment.tokenize_yield().expect_commit_success();

   let date = UtcDateTime::new(2025, 03, 06, 0, 0, 0).ok().unwrap();

   test_environment.advance_date(date);

   test_environment.claim_yield_for_sxrd().expect_commit_failure();
}

#[test]
fn claim_yield_for_lsu_no_reward() {
   let mut test_environment = LocalTestEnvironment::instantiate();

   test_environment.tokenize_yield().expect_commit_success();

   let date = UtcDateTime::new(2025, 03, 06, 0, 0, 0).ok().unwrap();

   test_environment.advance_date(date);

   test_environment.claim_yield_for_lsu().expect_commit_failure();
}

// MINT
#[test]
fn mint_sxrd() {
   let mut test_environment = LocalTestEnvironment::instantiate();
   test_environment.mint_sxrd().expect_commit_success();
}

#[test]
fn redeem_sxrd() {
   let mut test_environment = LocalTestEnvironment::instantiate();

   test_environment.tokenize_yield().expect_commit_success();

   test_environment.redeem_sxrd(false).expect_commit_success();
}

pub struct Account {
   public_key: Secp256k1PublicKey,
   account_component: ComponentAddress,
}

pub struct LocalTestEnvironment {
   test_runner: LedgerSimulator<NoExtension, InMemorySubstateDatabase>,
   account: Account,
   tokenizer_component: ComponentAddress,
   lsu_resource_address: ResourceAddress,
   sxrd_resource: ResourceAddress,
   yt_resource: ResourceAddress,
}

impl LocalTestEnvironment {
   pub fn instantiate() -> Self {
      let genesis_epoch = Epoch::of(2);
      let epoch_emissions_xrd = dec!("100");
      let validator_key = Secp256k1PrivateKey::from_u64(1u64).unwrap().public_key();

      let validators = vec![GenesisValidator::from(validator_key)];

      let accounts = validators.iter().map(|validator| validator.owner).collect::<Vec<_>>();

      let allocations = vec![(
         validator_key,
         vec![GenesisStakeAllocation {
            account_index: 0,
            xrd_amount: dec!(1000),
         }],
      )];
      let genesis_data_chunks = vec![
         GenesisDataChunk::Validators(validators),
         GenesisDataChunk::Stakes { accounts, allocations },
      ];

      let current_date = UtcDateTime::new(2024, 03, 05, 0, 0, 0).ok().unwrap();
      let current_date_ms = current_date.to_instant().seconds_since_unix_epoch * 1000;

      let custom_genesis = CustomGenesis {
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
      let mut test_runner = LedgerSimulatorBuilder::new()
         .with_custom_genesis(custom_genesis)
         .without_kernel_trace()
         .build();

      // Create an account
      let (public_key, _private_key, account_component) = test_runner.new_allocated_account();

      let account = Account {
         public_key,
         account_component,
      };

      let validator_address = test_runner.get_active_validator_with_key(&validator_key);
      let lsu_resource_address = test_runner
         .get_active_validator_info_by_key(&validator_key)
         .stake_unit_resource;

      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(account_component, XRD, dec!(1000))
         .take_all_from_worktop(XRD, "xrd")
         .call_method_with_name_lookup(validator_address, "stake", |lookup| (lookup.bucket("xrd"),))
         .deposit_batch(account_component)
         .build();

      test_runner
         .execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&public_key)])
         .expect_commit_success();

      // Publish package
      let package_address = test_runner.compile_and_publish(this_package!());

      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .call_function(
            package_address,
            "YieldTokenizer",
            "instantiate",
            manifest_args!(lsu_resource_address, 10u32, dec!(0), None::<OwnerRole>),
         )
         .build();

      let receipt = test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&public_key)]);

      let tokenizer_component = receipt.expect_commit(true).new_component_addresses()[0];
      let sxrd_resource = receipt.expect_commit(true).new_resource_addresses()[0];
      let yt_resource = receipt.expect_commit(true).new_resource_addresses()[1];

      Self {
         test_runner,
         account,
         tokenizer_component,
         lsu_resource_address,
         sxrd_resource,
         yt_resource,
      }
   }

   pub fn advance_date(&mut self, date: UtcDateTime) {
      let date_ms = date.to_instant().seconds_since_unix_epoch * 1000;
      let receipt = self.test_runner.advance_to_round_at_timestamp(Round::of(3), date_ms);
      receipt.expect_commit_success();
   }

   pub fn execute_manifest(&mut self, manifest: ManifestBuilder, name: &str) -> TransactionReceiptV1 {
      let object_manifest = manifest.object_names();
      let built_manifest = manifest.build();

      dump_manifest_to_file_system(
         object_manifest,
         &built_manifest,
         "./transaction_manifest",
         Some(name),
         &NetworkDefinition::stokenet(),
      )
      .ok();

      let receipt = self.test_runner.execute_manifest(
         built_manifest,
         vec![NonFungibleGlobalId::from_public_key(&self.account.public_key)],
      );

      return receipt;
   }

   pub fn tokenize_yield(&mut self) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, self.lsu_resource_address, dec!(1000))
         .take_all_from_worktop(self.lsu_resource_address, "LSU Bucket")
         .call_method_with_name_lookup(self.tokenizer_component, "tokenize_yield", |lookup| {
            (lookup.bucket("LSU Bucket"),)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "tokenize_yield")
   }

   pub fn tokenize_yield_with_xrd(&mut self) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, XRD, dec!(500))
         .take_all_from_worktop(XRD, "xrd")
         .call_method_with_name_lookup(self.tokenizer_component, "tokenize_yield", |lookup| {
            (lookup.bucket("xrd"),)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "tokenize_yield")
   }

   pub fn claim_yield_for_sxrd(&mut self) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, self.yt_resource, dec!(1))
         .take_all_from_worktop(self.yt_resource, "YT Bucket")
         .call_method_with_name_lookup(self.tokenizer_component, "claim_yield_for_sxrd", |lookup| {
            (lookup.bucket("YT Bucket"),)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "claim_yield_for_sxrd")
   }

   pub fn claim_yield_for_lsu(&mut self) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, self.yt_resource, dec!(1))
         .take_all_from_worktop(self.yt_resource, "YT Bucket")
         .call_method_with_name_lookup(self.tokenizer_component, "claim_yield_for_lsu", |lookup| {
            (lookup.bucket("YT Bucket"),)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "claim_yield_for_lsu")
   }

   pub fn mint_sxrd(&mut self) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, XRD, dec!(500))
         .take_all_from_worktop(XRD, "xrd")
         .call_method_with_name_lookup(self.tokenizer_component, "mint_sxrd", |lookup| (lookup.bucket("xrd"),))
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "mint_sxrd")
   }

   pub fn redeem_sxrd(&mut self, only_xrd: bool) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .withdraw_from_account(self.account.account_component, self.sxrd_resource, dec!(1000))
         .take_all_from_worktop(self.sxrd_resource, "sXRD Bucket")
         .call_method_with_name_lookup(self.tokenizer_component, "redeem_sxrd", |lookup| {
            (lookup.bucket("sXRD Bucket"), only_xrd)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "redeem_sxrd")
   }

   pub fn claim_yield_with_id(&mut self, local_id: NonFungibleLocalId) -> TransactionReceiptV1 {
      let manifest = ManifestBuilder::new()
         .lock_fee_from_faucet()
         .create_proof_from_account_of_non_fungibles(self.account.account_component, self.yt_resource, [local_id])
         .pop_from_auth_zone("YT Proof")
         .call_method_with_name_lookup(self.tokenizer_component, "claim_yield", |lookup| {
            (lookup.proof("YT Proof"),)
         })
         .deposit_batch(self.account.account_component);

      self.execute_manifest(manifest, "claim_yield")
   }
}
