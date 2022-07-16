//! Most of my package's design based on the assumption that Proof resource type would be free to pass around like the Bucket resource type. 
//! 
//! Current Scrypto version is too restrictive on such resource type so I have to work around it a lot and it's really time-costly.

// use ground_business::ground_business_dao::{Methods, Method};
use radix_engine::{ledger::*, transaction::*, model::Receipt};
use scrypto_unit::*;
use scrypto::prelude::*;
mod neuracle_time_gateway;

const PACKAGE: &str = "ground_test";
const BLUEPRINT: &str = "GroundTestEngine";

struct GroundTestEnv<'a, L: SubstateStore> {
    env: TestEnv<'a, L>,
    // dao_share_token: ResourceAddress,
    stable_coin: ResourceAddress,
    id_sbt: ResourceAddress,
    credit_sbt: ResourceAddress,
    installment_credit_request_badge: ResourceAddress,
    installment_credit_badge: ResourceAddress,
    // cv_sbt: ResourceAddress,
    lending_account: ResourceAddress,
    dao_member_sbt: ResourceAddress,
    // dao_delegator_nft: ResourceAddress,
    // dao_proposal_badge: ResourceAddress,
    ground_id: ComponentAddress,
    ground_credit: ComponentAddress,
    ground_lending: ComponentAddress,
    // dao: ComponentAddress,
    test_component: ComponentAddress
}

impl<'a, L: SubstateStore> GroundTestEnv<'a, L> {

    /// Create new DAO MEMBER WITH A CREDIT, this account should be enough for a test.
    fn new_dao_member(&mut self, name: &str, year_commit: u8) {

        self.env.create_user(name);
        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(self.test_component, "init_dao_member", vec![scrypto_encode(&year_commit)])
        .take_from_worktop(self.dao_member_sbt, |continue_transaction, bucket_id| {
            continue_transaction.call_method(user.account, "deposit", vec![scrypto_encode(&Bucket(bucket_id))])
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);

        
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("INIT RESOURCES FOR NEW DAO MEMBER, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok());
    
    }

    /// Create new CREDIT USER WITH A DELEGATOR NFT BADGE, this account should be enough for a test and a bit different from the DAO Member account.
    /// 
    /// Default maximum monthly allowance is 30 stable coins.
    fn new_credit_user(&mut self, name: &str, maximum_monthly_allowance: Decimal) {

        self.env.create_user(name);
        self.env.acting_as(name);
        let receipt = self.env.call_method(self.test_component, "init_credit_user", vec![scrypto_encode(&maximum_monthly_allowance)]);
        println!("INIT RESOURCES FOR NEW DELEGATOR, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok());

    }

    /// ADVANCE EPOCH BY 1 AND UPDATE TIME DATA.
    /// 
    /// THIS CAN BE MANIPULATED IF INPUT SOME(TIME (String))
    fn update_neuracle_time(&mut self, manipulate_time: Option<String>) {

        let substate_store = self.env.executor.substate_store_mut();

        let current = substate_store.get_epoch();

        substate_store.set_epoch(current + 1);

        self.env.acting_as("tester");

        let time = match manipulate_time {
            None => {neuracle_time_gateway::get_time()}
            Some(time) => {
                println!("MANIPULATE TIME INTO UNIX TIME: {}", time);
                println!("NOTICE: THIS IS ONLY FOR TEST PURPOSE AND TOTALLY NOT THE PRACITAL USE OF THE NEURACLE PACKAGE!");
                time
            }
            };

        let receipt = self.env.call_method(self.test_component, "update_neuracle_data", vec![scrypto_encode(&time)]);
        println!("UPDATE NEURACLE TIME DATA, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok());

    }

    fn get_revolving_credit_amount(&mut self, name: &str) -> Receipt {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(self.ground_credit, "get_revolving_credit_amount", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&Proof(proof_id2))])
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("GET REVOLVING CREDIT AMOUNT, RECEPIT: {:?}", receipt);

        receipt

    }

    fn change_credit_type(&mut self, name: &str) -> Receipt { 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(self.ground_credit, "change_credit_type", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&Proof(proof_id2))])
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("CHANGE CREDIT TYPE, RECEPIT: {:?}", receipt);
        receipt

    }

    /// The function will go through the whole installment credit process 
    /// from the Ground Credit component and just let user get an installment credit badge to try on the lending protocol.
    /// 
    /// The Installment Credit data is as follow:
    /// - total_loan: tester's input.
    /// - interest_rate: 10%
    /// - interest_rate_late: 25%
    /// - period_length: 2.592.000 seconds (1 month).
    /// - period_max: tester's input.
    fn get_installment_credit(&mut self, name: &str, total_loan: Decimal, period_max: u8) { 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(self.ground_credit, "request_installment_credit", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&total_loan), scrypto_encode(&dec!(10)), scrypto_encode(&dec!(25)), scrypto_encode(&2592000u64), scrypto_encode(&period_max)])
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let mut receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("USER REQUEST INSTALLMENT CREDIT, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok()); 

        let (_, request_id): (Bucket, u64) = return_of_call_method(&mut receipt, "request_installment_credit");

        println!("USER'S REQUEST ID: {}", request_id);

        self.env.acting_as("tester");

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(self.test_component, "review_installment_credit", vec![scrypto_encode(&request_id)])
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("USER REQUEST INSTALLMENT CREDIT ACCEPTED, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok()); 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "withdraw", vec![scrypto_encode(&self.installment_credit_request_badge)])
        .take_from_worktop(self.installment_credit_request_badge, |continue_transaction, bucket_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id| {
                continue_transaction2
                .call_method(self.ground_credit, "get_installment_credit_badge", vec![scrypto_encode(&Bucket(bucket_id)), scrypto_encode(&Proof(proof_id))])
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("USER GOT THE INSTALLMENT CREDIT WITH {} STABLE COINS AMOUNT AND HAVE TO REPAID IN {} PERIODS, RECEPIT: {:?}", total_loan, period_max, receipt);
        assert!(receipt.result.is_ok()); 


    }

    fn black_list(&mut self, name: &str) {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(self.ground_id, "get_id", vec![scrypto_encode(&Proof(proof_id))])

        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let mut receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("GET IDENTITY SBT ID, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok()); 

        let id: NonFungibleId = return_of_call_method(&mut receipt, "get_id");

        self.env.acting_as("tester");

        let receipt = self.env.call_method(self.test_component, "blacklist", vec![scrypto_encode(&id)]);
        println!("BLACK LIST THE IDENTITY ID {}, RECEPIT: {:?}", id, receipt);
        assert!(receipt.result.is_ok()); 

    }

    fn new_lender(&mut self, name: &str, amount: Decimal) {

        self.new_credit_user(name, dec!("30"));
        
        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "withdraw_by_amount", vec![scrypto_encode(&amount), scrypto_encode(&self.stable_coin)])
        .take_from_worktop(self.stable_coin, |continue_transaction, bucket_id| {
            continue_transaction
            .call_method(self.ground_lending, "new_lending_account", vec![scrypto_encode(&Bucket(bucket_id))])

        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("GET NEW LENDING ACCOUNT WITH {} STABLE COIN, RECEPIT: {:?}", amount, receipt);
        assert!(receipt.result.is_ok()); 

    }

    fn withdraw(&mut self, name: &str, amount: Decimal) { 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.lending_account)])
        .pop_from_auth_zone( |continue_transaction, proof_id| {
            continue_transaction
            .call_method(self.ground_lending, "withdraw", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&amount)])

        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("WITHDRAW {} STABLE COIN FROM THE USER'S LENDING ACCOUNT, RECEPIT: {:?}", amount, receipt);
        assert!(receipt.result.is_ok()); 

    }

    fn withdraw_all(&mut self, name: &str) { 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "withdraw", vec![scrypto_encode(&self.lending_account)])
        .take_from_worktop( self.lending_account,|continue_transaction, bucket_id| {
            continue_transaction
            .call_method(self.ground_lending, "withdraw_all", vec![scrypto_encode(&Bucket(bucket_id))])

        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("WITHDRAW ALL STABLE COIN FROM THE USER'S LENDING ACCOUNT, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok()); 

    }

    fn withdraw_fail(&mut self, name: &str) { 

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "withdraw", vec![scrypto_encode(&self.lending_account)])
        .take_from_worktop( self.lending_account,|continue_transaction, bucket_id| {
            continue_transaction
            .call_method(self.ground_lending, "withdraw_all", vec![scrypto_encode(&Bucket(bucket_id))])

        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("WITHDRAW ALL STABLE COIN FROM THE USER'S LENDING ACCOUNT WHEN THE PROTOCOL'S VAULT IS NOT ENOUGH, THIS SHOULD FAIL, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_err()); 

    }

    fn revolving_credit(&mut self, name: &str, amount: Decimal) -> Receipt {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(self.ground_lending, "revolving_credit", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&Proof(proof_id2)), scrypto_encode(&amount)])
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("TAKE {} STABLE COIN LOAN FROM USER'S REVOLVING CREDIT, RECEPIT: {:?}", amount, receipt);

        receipt

    }

    fn repay_part(&mut self, name: &str, amount: Decimal) {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(user.account, "withdraw_by_amount", vec![scrypto_encode(&amount), scrypto_encode(&self.stable_coin)])
                .take_from_worktop(self.stable_coin, |continue_transaction3, bucket_id| {
                    continue_transaction3.call_method(self.ground_lending, "repay", vec![scrypto_encode(&mut Proof(proof_id)), scrypto_encode(&Proof(proof_id2)), scrypto_encode(&mut Bucket(bucket_id))])
                })
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("REPAY {} STABLE COIN TO THE PROTOCOL, RECEPIT: {:?}", amount, receipt);

        assert!(receipt.result.is_ok())

    }

    fn repay_full(&mut self, name: &str) {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(user.account, "withdraw", vec![scrypto_encode(&self.stable_coin)])
                .take_from_worktop(self.stable_coin, |continue_transaction3, bucket_id| {
                    continue_transaction3.call_method(self.ground_lending, "repay", vec![scrypto_encode(&mut Proof(proof_id)), scrypto_encode(&Proof(proof_id2)), scrypto_encode(&mut Bucket(bucket_id))])
                })
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("REPAY ALL THE USER'S LOAN FROM THE PROTOCOL, RECEPIT: {:?}", receipt);

        assert!(receipt.result.is_ok())

    }

    fn installment_credit(&mut self, name: &str) -> Receipt {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();
    
        let transaction = TransactionBuilder::new()
        .call_method(user.account, "create_proof", vec![scrypto_encode(&self.id_sbt)])
        .pop_from_auth_zone(|continue_transaction, proof_id| {
            continue_transaction
            .call_method(user.account, "create_proof", vec![scrypto_encode(&self.credit_sbt)])
            .pop_from_auth_zone(|continue_transaction2, proof_id2| {
                continue_transaction2
                .call_method(user.account, "withdraw", vec![scrypto_encode(&self.installment_credit_badge)])
                .take_from_worktop(self.installment_credit_badge, |continue_transaction3, bucket_id| {
                    continue_transaction3.call_method(self.ground_lending, "installment_credit", vec![scrypto_encode(&mut Proof(proof_id)), scrypto_encode(&mut Proof(proof_id2)), scrypto_encode(&mut Bucket(bucket_id))])
                })
            })
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("TAKE ALL THE LOAN FROM THE INSTALLMENT CREDIT, RECEPIT: {:?}", receipt);

        receipt

    }

    // /// The function will go through a simple consensus round of the Ground Business DAO.
    // fn proof_of_concept_consensus(&mut self, methods: Methods) {

    //     self.env.acting_as("DAO_member");

    //     let (user, private_key) = self.env.get_current_user();
    
    //     let transaction = TransactionBuilder::new()
    //     .call_method(user.account, "create_proof", vec![scrypto_encode(&self.dao_member_sbt)])
    //     .call_method(user.account, "create_proof", vec![scrypto_encode(&self.cv_sbt)])
    //     .pop_from_auth_zone(|continue_transaction, proof_id| {
    //         continue_transaction
    //         .call_method(self.dao, "propose_concept", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&methods), scrypto_encode(&dec!("100"))])
    //     })
    //     .call_method_with_all_resources(user.account, "deposit_batch")
    //     .build(self.env.executor.get_nonce([user.key]))
    //     .sign([private_key]);
            
    //     let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
    //     println!("LET THE DAO MEMBER PROPOSE THE CONCEPT, RECEPIT: {:?}", receipt);
    //     assert!(receipt.result.is_ok());

    //     let (user, private_key) = self.env.get_current_user();

    //     let id = NonFungibleId::from_u64(0u64);
    
    //     let transaction = TransactionBuilder::new()
    //     .call_method(user.account, "create_proof", vec![scrypto_encode(&self.dao_member_sbt)])
    //     .pop_from_auth_zone(|continue_transaction, proof_id| {
    //         continue_transaction
    //         .call_method(self.dao, "vote", vec![scrypto_encode(&Proof(proof_id)), scrypto_encode(&id), scrypto_encode(&true)])
    //     })
    //     .call_method_with_all_resources(user.account, "deposit_batch")
    //     .build(self.env.executor.get_nonce([user.key]))
    //     .sign([private_key]);
            
    //     let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
    //     println!("LET THE DAO MEMBER VOTE ON THE CONCEPT, RECEPIT: {:?}", receipt);
    //     assert!(receipt.result.is_ok());

    //     let (user, private_key) = self.env.get_current_user();
    
    //     let transaction = TransactionBuilder::new()
    //     .call_method(user.account, "withdraw", vec![scrypto_encode(&self.dao_proposal_badge)])
    //     .take_from_worktop(self.dao_proposal_badge, |continue_transaction, bucket_id| {
    //         continue_transaction
    //         .call_method(self.dao, "execute_concept", vec![scrypto_encode(&Bucket(bucket_id))])
    //     })
    //     .call_method_with_all_resources(user.account, "deposit_batch")
    //     .build(self.env.executor.get_nonce([user.key]))
    //     .sign([private_key]);
            
    //     let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
    //     println!("LET THE DAO MEMBER EXECUTE THE CONCEPT, RECEPIT: {:?}", receipt);
    //     assert!(receipt.result.is_ok())

    // }

    fn lending_use_dao(&mut self) {

        self.env.acting_as("tester");

        let receipt = self.env.call_method(self.test_component, "lending_use_dao", vec![]);

        println!("LET THE LENDING PROTOCOL USE DAO TREASURY, RECEPIT: {:?}", receipt);

        assert!(receipt.result.is_ok()); 

    }

    fn compensation(&mut self, name: &str) {

        self.env.acting_as(name);

        let (user, private_key) = self.env.get_current_user();

        let transaction = TransactionBuilder::new()
        .call_method(user.account, "withdraw", vec![scrypto_encode(&self.lending_account)])
        .take_from_worktop( self.lending_account,|continue_transaction, bucket_id| {
            continue_transaction
            .call_method(self.ground_lending, "compensate", vec![scrypto_encode(&Bucket(bucket_id))])
        })
        .call_method_with_all_resources(user.account, "deposit_batch")
        .build(self.env.executor.get_nonce([user.key]))
        .sign([private_key]);
            
        let receipt = self.env.executor.validate_and_execute(&transaction).unwrap();
        println!("COMPENSATE USER, RECEPIT: {:?}", receipt);
        assert!(receipt.result.is_ok()); 
        
    }
}

/// CREATE NEW TEST ENV.
fn new_test_env<L: SubstateStore>(mut env: TestEnv<L>) -> GroundTestEnv<L> {

    env.create_user("tester");
    env.acting_as("tester");
    env.publish_package(PACKAGE, include_package!("ground_test"));
    env.using_package(PACKAGE);
    let mut receipt = env.call_function(BLUEPRINT, "new", vec![]);
    println!("NEW TEST COMPONENT, RECEIPT: {:?}", receipt);
    assert!(receipt.result.is_ok());  

    // let dao_share_token = receipt.new_resource_addresses[1];
    let stable_coin = receipt.new_resource_addresses[2];
    let id_sbt = receipt.new_resource_addresses[10];
    let credit_sbt = receipt.new_resource_addresses[14];
    let installment_credit_request_badge = receipt.new_resource_addresses[15];
    let installment_credit_badge = receipt.new_resource_addresses[16];
    let ground_id = receipt.new_component_addresses[1];
    let ground_credit = receipt.new_component_addresses[2];
    // let cv_sbt = receipt.new_resource_addresses[18];
    let test_component: ComponentAddress = return_of_call_function(&mut receipt, BLUEPRINT);

    let mut receipt = env.call_method(test_component, "init", vec![]);
    println!("NEW LENDING PROTOCOL AND DAO COMPONENT, RECEIPT: {:?}", receipt);
    assert!(receipt.result.is_ok());  

    let lending_account = receipt.new_resource_addresses[2];
    let dao_member_sbt = receipt.new_resource_addresses[7];
    // let dao_delegator_nft = receipt.new_resource_addresses[8];
    // let dao_proposal_badge = receipt.new_resource_addresses[9];
    // let dao_unstake_badge = receipt.new_resource_addresses[10];

    let (ground_lending, _dao): (ComponentAddress, ComponentAddress) = return_of_call_method(&mut receipt, "init");

    let receipt = env.call_method(test_component, "init_stake_for_validator_node", vec![]);
    println!("INIT STAKE FOR VALIDATOR NODES, RECEIPT: {:?}", receipt);
    assert!(receipt.result.is_ok()); 

    GroundTestEnv {

        env,
        // dao_share_token,
        stable_coin,
        id_sbt,
        credit_sbt,
        installment_credit_request_badge,
        installment_credit_badge,
        // cv_sbt,
        lending_account,
        dao_member_sbt,
        // dao_delegator_nft,
        // dao_proposal_badge,
        ground_id,
        ground_credit,
        ground_lending,
        // dao,
        test_component

    }

}

/// ## Ground Credit blueprint test:
/// The test will do the following:
/// 
/// - Go through all the blueprint's functions.
/// - Test the Ground Credit blueprint's methods.
/// 
/// Testers can edit the params in the test.
#[test]
fn test_ground_credit() {

    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let env = TestEnv::new(&mut ledger);
    let mut test_env = new_test_env(env);

    println!("GET REAL CURRENT TIME BY UNIX");
    test_env.update_neuracle_time(None);

    println!("CREATE NEW REVOLVING CREDIT USER WITH MAXIMUM MONTHLY ALLOWANCE IS 30 STABLECOINS. THE USER HAS INCOME 1000 AND TRUST SCORE AT 60.");
    test_env.new_credit_user("credit_user", dec!("30"));

    println!("RETEST CREDIT USER SBT PATTERN");
    test_env.env.acting_as("credit_user");

    let (user, private_key) = test_env.env.get_current_user();
    
    let transaction = TransactionBuilder::new()
    .call_method(user.account, "create_proof", vec![scrypto_encode(&test_env.id_sbt)])
    .pop_from_auth_zone(|continue_transaction, proof_id| {
        continue_transaction.call_method(test_env.ground_credit, "get_new_credit_sbt", vec![scrypto_encode(&Proof(proof_id))])
    })
    .call_method_with_all_resources(user.account, "deposit_batch")
    .build(test_env.env.executor.get_nonce([user.key]))
    .sign([private_key]);
        
    let receipt = test_env.env.executor.validate_and_execute(&transaction).unwrap();
    println!("GET THE CREDIT SBT, THIS SHOULD FAIL, RECEPIT: {:?}", receipt);
    assert!(receipt.result.is_err()); 

    println!("GET USER'S REVOLVING CREDIT AMOUNT");
    let mut receipt = test_env.get_revolving_credit_amount("credit_user");

    assert!(receipt.result.is_ok());

    let (maximum_credit, current_allowance): (Decimal, Decimal) = return_of_call_method(&mut receipt, "get_revolving_credit_amount");

    println!("USER'S MAXIMUM CREDIT: {}, CURRENT ALLOWANCE: {}", maximum_credit, current_allowance);

    println!("TEST CHANGE CREDIT TYPE METHOD");
    let receipt = test_env.change_credit_type("credit_user");

    assert!(receipt.result.is_ok()); 

    println!("GET USER'S REVOLVING CREDIT AMOUNT");

    let mut receipt = test_env.get_revolving_credit_amount("credit_user");

    assert!(receipt.result.is_ok());

    let (maximum_credit, current_allowance): (Decimal, Decimal) = return_of_call_method(&mut receipt, "get_revolving_credit_amount");

    println!("USER'S MAXIMUM CREDIT: {}, CURRENT ALLOWANCE: {}", maximum_credit, current_allowance);

    println!("CREATE OTHER CREDIT USER WITH MAXIMUM MONTHLY ALLOWANCE IS 160 STABLECOINS. THE USER HAS INCOME 3000 AND TRUST SCORE AT 80.");
    test_env.new_dao_member("credit_user2", 8u8);

    println!("GET USER'S REVOLVING CREDIT AMOUNT");

    let mut receipt = test_env.get_revolving_credit_amount("credit_user2");

    assert!(receipt.result.is_ok());

    let (maximum_credit, current_allowance): (Decimal, Decimal) = return_of_call_method(&mut receipt, "get_revolving_credit_amount");

    println!("USER'S MAXIMUM CREDIT: {}, CURRENT ALLOWANCE: {}", maximum_credit, current_allowance);

    println!("GO THROUGH INSTALLMENT CREDIT PROCESS");
    test_env.get_installment_credit("credit_user2", dec!("1000000"), 7u8);

    println!("CHECK BLACKLIST FUNCTION");
    test_env.black_list("credit_user2");

    let receipt = test_env.get_revolving_credit_amount("credit_user2");

    assert!(receipt.result.is_err());

}

/// ## Ground Lending blueprint's lender badge pattern test:
/// The test will do the following:
/// 
/// - Go through all the blueprint's functions.
/// - Test the Ground Lending blueprint's methods for lenders. 
/// 
/// Testers can edit the params in the test.
#[test]
fn test_lender_badge_pattern() {

    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let env = TestEnv::new(&mut ledger);
    let mut test_env = new_test_env(env);

    test_env.update_neuracle_time(None);

    println!("TEST LENDING PROTOCOL'S LENDER BADGE PATTERN");
    test_env.new_lender("lender1", dec!("264"));

    test_env.withdraw("lender1", dec!("34"));

    test_env.withdraw_all("lender1");

}

/// ## Ground Lending blueprint's revolving credit and interest for lenders test:
/// The test will do the following:
/// 
/// - Go through all the blueprint's functions.
/// - Create lenders and fund some stable coin into the protocol.
/// - Test "Consumer-level" credit for borrowers: All the Ground Lending blueprint's methods for revolving credit borrowers, included late-repayment test.
/// - Test the Automatic Credit Scoring Mechanism.
/// - Test "Bank level" earning tracker for lenders: Compare the return of same lending account in amount but different in start - end time.
/// 
/// Testers can edit the params in the test.
#[test]
fn test_revolving_credit_and_interest() {

    println!("THIS TEST WILL BE REALLY COMPLEX AND TAKE A LONG TIME, PLEASE BE PATIENT.");

    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let env = TestEnv::new(&mut ledger);
    let mut test_env = new_test_env(env);
    
    test_env.update_neuracle_time(Some("10000000".to_string()));

    test_env.new_lender("lender1", dec!("9564"));

    test_env.update_neuracle_time(Some("10052635".to_string()));

    println!("NEW CREDIT USER WITH MONTHLY ALLOWANCE AT 30");
    test_env.new_credit_user("borrower1", dec!("30"));

    println!("LET BORROWER 1 TAKE LOAN");

    let receipt = test_env.revolving_credit("borrower1", dec!("21"));

    assert!(receipt.result.is_ok());

    println!("LET BORROWER 1 TAKE MORE THAN HIS CAPABILITY");

    let receipt = test_env.revolving_credit("borrower1", dec!("644"));

    assert!(receipt.result.is_err());
    
    test_env.update_neuracle_time(Some("10152356".to_string()));

    test_env.new_lender("lender2", dec!("9564"));

    test_env.update_neuracle_time(Some("10356952".to_string()));

    println!("NEW CREDIT USER WITH MONTHLY ALLOWANCE AT 112");
    test_env.new_credit_user("borrower2", dec!("112"));

    println!("LET BORROWER 2 TAKE MORE THAN HIS CAPABILITY");

    let receipt = test_env.revolving_credit("borrower2", dec!("113"));

    assert!(receipt.result.is_err());

    println!("LET BORROWER 2 TAKE LOAN");

    let receipt = test_env.revolving_credit("borrower2", dec!("112"));

    assert!(receipt.result.is_ok());

    // NEW CREDIT USER WITH MONTHLY ALLOWANCE AT 3521
    test_env.new_credit_user("borrower3", dec!("3521"));

    println!("LET BORROWER 3 TAKE LOAN");

    let receipt = test_env.revolving_credit("borrower3", dec!("3453"));

    assert!(receipt.result.is_ok());

    println!("LET BORROWER 3 TAKE MORE THAN HIS CAPABILITY");

    let receipt = test_env.revolving_credit("borrower3", dec!("2236"));

    assert!(receipt.result.is_err());

    println!("TEST CHANGE CREDIT TYPE WHEN USER HAS A DEBT. THIS SHOULD FAIL");

    let receipt = test_env.change_credit_type("borrower3");

    assert!(receipt.result.is_err()); 

    println!("LET BORROWER 3 TAKE LOAN");

    let receipt = test_env.revolving_credit("borrower3", dec!("6"));

    assert!(receipt.result.is_ok());

    println!("LET BORROWER 1 AND 2 REPAY PART OF THE LOAN");

    test_env.repay_part("borrower1", dec!("9"));

    test_env.repay_part("borrower2", dec!("65"));

    println!("LET BORROWER 3 REPAY FULL OF THE LOAN");

    test_env.repay_full("borrower3");

    test_env.update_neuracle_time(Some("11321546".to_string()));

    println!("LET BORROWER 1 REPAY FULL OF THE LOAN");

    test_env.repay_full("borrower1");

    println!("LET LENDER 2 WITHDRAW ALL HIS ACCOUNT");

    test_env.update_neuracle_time(Some("13513562".to_string()));

    test_env.withdraw_all("lender2");

    test_env.update_neuracle_time(Some("13795623".to_string()));

    println!("LET BORROWER 2 REPAY FULL OF THE LOAN");
    test_env.repay_full("borrower2");

    println!("LET BORROWER 1 TAKE LOAN");

    let receipt = test_env.revolving_credit("borrower1", dec!("30"));

    assert!(receipt.result.is_ok());

    println!("LET BORROWER 1 REPAY FULL OF THE LOAN");

    test_env.repay_full("borrower1");

    println!("LET LENDER 1 WITHDRAW ALL HIS ACCOUNT");

    test_env.withdraw_all("lender1");

}

/// ## Ground Lending blueprint's installment credit test:
/// The test will do the following:
/// 
/// - Go through all the blueprint's functions.
/// - Create lenders and fund some stable coin into the protocol.
/// - Test the Ground Lending blueprint's methods for installment credit borrowers, included late-repayment test. 
/// - Test the "Tolerance threshold" of the protocol: 
/// 
/// Borrowers cannot take a loan after the protocol's vault contain < 60% of the total return amount. (this can be edited on TestEngine blueprint)
/// 
/// Testers can edit the params in the test.
#[test]
fn test_installment_credit() {

    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let env = TestEnv::new(&mut ledger);
    let mut test_env = new_test_env(env);

    test_env.update_neuracle_time(Some("10000000".to_string()));

    test_env.new_lender("lender1", dec!("10000"));

    test_env.update_neuracle_time(Some("10056231".to_string()));

    println!("NEW CREDIT USER WITH MONTHLY ALLOWANCE AT 30");
    test_env.new_credit_user("borrower1", dec!("30"));
    
    test_env.get_installment_credit("borrower1", dec!("4100"), 3u8);

    println!("TEST THE PROTOCOL'S TOLERANCE THRESHOLD, THIS SHOULD FAIL!");
    let receipt = test_env.installment_credit("borrower1");
    assert!(receipt.result.is_err());

    test_env.new_lender("lender2", dec!("300"));

    test_env.update_neuracle_time(Some("10167622".to_string()));

    let receipt = test_env.installment_credit("borrower1");
    assert!(receipt.result.is_ok());

    test_env.update_neuracle_time(Some("10372546".to_string()));

    test_env.new_lender("lender3", dec!("10000"));

    test_env.update_neuracle_time(Some("11653624".to_string()));

    test_env.repay_part("borrower1", dec!("352"));

    test_env.update_neuracle_time(Some("12722659".to_string()));

    test_env.repay_part("borrower1", dec!("1351"));

    test_env.update_neuracle_time(Some("13565695".to_string()));

    test_env.repay_full("borrower1");

    test_env.update_neuracle_time(Some("14536859".to_string()));

    test_env.withdraw_all("lender1");

    test_env.withdraw_all("lender2");

    test_env.withdraw_all("lender3");

}

/// ## Ground Lending blueprint's delinquent loan and compensation test:
/// The test will do the following:
/// 
/// - Go through all the blueprint's functions.
/// - Go through a concept consensus round in the Ground Business DAO to let the protocol use the dao's treasury.
/// - Create lenders and fund some stable coin into the protocol.
/// - Let delinquent credit users take about 40% the fund.
/// - Lenders try to withdraw 60% left.
/// - Lenders try to get the compensation. (compensation rate is 50%, you can change this amount in the TestEngine.)
/// 
/// Testers can edit the params in the test.
#[test]
fn test_compensation() {

    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let env = TestEnv::new(&mut ledger);
    let mut test_env = new_test_env(env);

    test_env.update_neuracle_time(Some("10000000".to_string()));

    // test_env.new_dao_member("DAO_member", 8u8);

    // let methods = Methods {methods: vec![Method {

    //     component: test_env.ground_lending,

    //     method: "use_dao".to_string(),
  
    //     args: vec![scrypto_encode(&test_env.dao)],

    // }]};

    // test_env.proof_of_concept_consensus(methods);

    test_env.lending_use_dao();

    test_env.new_lender("lender1", dec!("10000"));

    test_env.new_lender("lender2", dec!("10000"));

    test_env.new_credit_user("delinquent_borrower", dec!("10000"));

    test_env.update_neuracle_time(Some("11300000".to_string()));

    let receipt = test_env.revolving_credit("delinquent_borrower", dec!("7999"));

    assert!(receipt.result.is_ok());

    test_env.withdraw_all("lender1");

    test_env.withdraw_fail("lender2");

    test_env.withdraw("lender2", dec!("2001"));

    test_env.compensation("lender2")

}