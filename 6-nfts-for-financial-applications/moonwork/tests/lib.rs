use manifests::*;
use moonwork::dispute::Dispute;
use moonwork::dispute::DisputeDocument;
use moonwork::dispute::DisputeSide;
use moonwork::moonwork::ContractorAccolades;
use moonwork::moonwork::DisputeDecision;
use moonwork::moonwork::DisputeOutcome;
use moonwork::promotion::PromotedContractor;
use moonwork::users::Client;
use moonwork::users::Contractor;
use moonwork::work::Work;
use moonwork::work::WorkStatus;
use radix_engine::ledger::*;
use radix_engine::types::*;
use scrypto_unit::*;

mod manifests;

#[test]
fn test_create_moonwork_service() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // Creating moonwork service
    create_moonwork_service(&mut test_runner, account_component, public_key);
}

#[test]
fn moonwork_service_register_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let (component, resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    // act
    let receipt = register_as_contractor(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "beemdvp",
    );
    receipt.expect_commit_success();

    // assert
    let contractor_badge_balance = get_account_balance(
        &mut test_runner,
        account_component,
        public_key,
        resources.contractor_badge,
    );

    let accolade_balance = get_account_balance(
        &mut test_runner,
        account_component,
        public_key,
        resources.contractor_accolade_resource,
    );

    assert_eq!(contractor_badge_balance, dec!(1));
    assert_eq!(accolade_balance, dec!(1));
}

#[test]
fn moonwork_service_register_as_contractor_fails_duplicate() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let (component, _resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    register_as_contractor(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "beemdvp",
    );

    // act
    let receipt = register_as_contractor(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "beemdvp",
    );

    // assert
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_register_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    // arrange
    let (public_key, _private_key, account_component) = test_runner.new_account();

    let (component, resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    // act
    let receipt = register_as_client(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "slysmik",
    );
    receipt.expect_commit_success();

    // assert
    let balance = get_account_balance(
        &mut test_runner,
        account_component,
        public_key,
        resources.client_badge,
    );

    assert_eq!(balance, dec!(1));
}

#[test]
fn moonwork_service_register_as_client_fails_duplicate() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    // arrange
    let (public_key, _private_key, account_component) = test_runner.new_account();

    let (component, _resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    register_as_client(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "slysmik",
    );

    // act
    let receipt = register_as_client(
        &mut test_runner,
        component,
        account_component,
        public_key,
        "slysmik",
    );

    // assert
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_create_new_category_client_badge_fails() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let (component, resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    let (new_client_public_key, _private_key, new_client_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        component,
        new_client_account,
        new_client_public_key,
        "foobar",
    );

    // act
    let receipt = create_new_category(
        &mut test_runner,
        resources.client_badge,
        new_client_account,
        new_client_public_key,
        component,
    );

    // assert
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_create_new_category_contractor_badge_fails() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let (component, resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);
    let (new_contractor_public_key, _private_key, new_contractor_account) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        component,
        new_contractor_account,
        new_contractor_public_key,
        "foobar",
    );

    // act
    let receipt = create_new_category(
        &mut test_runner,
        resources.contractor_badge,
        new_contractor_account,
        new_contractor_public_key,
        component,
    );

    // assert
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_create_new_work_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        account_component,
        public_key,
        "slysmik",
    );
    // act
    let receipt = create_new_work(
        &mut test_runner,
        account_component,
        public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );
    receipt.expect_commit_success();

    // assert
    let work_resource_balance = get_account_balance(
        &mut test_runner,
        account_component,
        public_key,
        service.work_components.work_resource,
    );

    assert_eq!(work_resource_balance, dec!(1));
}

#[test]
fn moonwork_service_remove_new_work_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // arrange
    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        account_component,
        public_key,
        "slysmik",
    );
    // act
    create_new_work(
        &mut test_runner,
        account_component,
        public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);
    let receipt = remove_work(
        &mut test_runner,
        &service,
        account_component,
        public_key,
        work_id,
    );

    receipt.expect_commit_success();

    // assert
    let work = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    assert!(work.work_status == WorkStatus::Delisted);
}

#[test]
fn moonwork_service_request_work_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (client_public_key, _private_key, client_account_component) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account_component,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account_component) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account_component,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account_component,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    // act
    let receipt = request_work(
        &mut test_runner,
        contractor_account_component,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        NonFungibleId::from_u64(1),
    );

    let work = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(
        work.contractor_requests
            .get(&NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec())),
        Some(&true)
    );
    // assert
    receipt.expect_commit_success();
}

#[test]
fn moonwork_service_accept_contractor_for_work_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        NonFungibleId::from_u64(1),
    );

    let receipt = accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    let work = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(
        work.contractor_assigned,
        Some(NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()))
    );
    receipt.expect_commit_success();
}

#[test]
fn moonwork_service_finish_work_as_client_and_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        NonFungibleId::from_u64(1),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    let work_id = NonFungibleId::from_u64(1);

    let receipt = finish_work(
        &mut test_runner,
        &service.moon_work_resources,
        client_account,
        client_public_key,
        contractor_account,
        contractor_public_key,
        work_id,
        &service.work_components,
    );

    receipt.expect_commit_success();

    let work = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    // assert
    assert_eq!(work.work_status, WorkStatus::Finished);
}

#[test]
fn moonwork_service_claim_compensation_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_work_to_finish_work(
        &mut test_runner,
        client_account,
        contractor_account,
        "beemdvp",
        &service.moon_work_resources,
        &service.work_components,
        client_public_key,
        contractor_public_key,
        1,
    );

    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    receipt.expect_commit_success();

    let completed_work_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.completed_work_resource,
    );

    let contractor = get_non_fungible_data::<Contractor>(
        &store,
        service.moon_work_resources.contractor_badge,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
    );
    let client = get_non_fungible_data::<Client>(
        &store,
        service.moon_work_resources.client_badge,
        NonFungibleId::from_bytes("slysmik".as_bytes().to_vec()),
    );

    assert_eq!(client.jobs_paid_out, 1);
    assert_eq!(client.total_paid_out, dec!(1));

    assert_eq!(contractor.jobs_completed, 1);
    assert_eq!(contractor.total_worth, dec!(1));
    assert_eq!(completed_work_balance, dec!(1));
}

#[test]
fn moonwork_service_claim_compensation_as_client_fails() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_work_to_finish_work(
        &mut test_runner,
        client_account,
        contractor_account,
        "beemdvp",
        &service.moon_work_resources,
        &service.work_components,
        client_public_key,
        contractor_public_key,
        1,
    );

    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        client_account,
        client_public_key,
    );

    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_claim_compensation_with_waxing_crescent_accolade() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    // in order to test the accolade system, we're doing quite a few things to arrange:
    // 1. As a client, create new work for contractor
    // 2. As a contractor, request to take on created work
    // 3. As a client, accept to take on contractor for work
    // 4. As both client and contractor (multisig) - finish work
    for id in 1..11 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    // act
    // Finally claim contractor compensation which also mints them their very own accolade NFT
    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    receipt.expect_commit_success();

    let work_completed_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.completed_work_resource,
    );

    let accolade_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_accolade_resource,
    );

    let waxing_crescent_nft = get_non_fungible_data::<ContractorAccolades>(
        &store,
        service.moon_work_resources.contractor_accolade_resource,
        NonFungibleId::from_u64(2),
    );

    let work_ids: Vec<NonFungibleAddress> = (1..11)
        .map(|id| {
            NonFungibleAddress::new(
                service.moon_work_resources.completed_work_resource,
                NonFungibleId::from_u64(id),
            )
        })
        .collect();

    // all work ids used to achieve the accolade
    assert_eq!(waxing_crescent_nft.work_ids, work_ids);
    // 10 contracts completed!
    assert_eq!(work_completed_balance, dec!(10));
    // NewMoon and WaxingCrescent accolades achieved
    assert_eq!(accolade_balance, dec!(2));
}

#[test]
fn moonwork_service_claim_compensation_with_first_quarter_accolade() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..21 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    receipt.expect_commit_success();

    let work_completed_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.completed_work_resource,
    );

    let accolade_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_accolade_resource,
    );

    let first_quarter_nft = get_non_fungible_data::<ContractorAccolades>(
        &store,
        service.moon_work_resources.contractor_accolade_resource,
        NonFungibleId::from_u64(3),
    );

    let work_ids: Vec<NonFungibleAddress> = (11..21)
        .map(|id| {
            NonFungibleAddress::new(
                service.moon_work_resources.completed_work_resource,
                NonFungibleId::from_u64(id),
            )
        })
        .collect();

    // all work ids used to achieve the accolade
    assert_eq!(first_quarter_nft.work_ids, work_ids);
    // 20 contracts completed!
    assert_eq!(work_completed_balance, dec!(20));
    // NewMoon, WaxingCrescent and FirstQuarter accolades achieved!
    assert_eq!(accolade_balance, dec!(3));
}

#[test]
fn moonwork_service_claim_compensation_with_waxing_gibbous_accolade() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..31 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    receipt.expect_commit_success();

    let work_completed_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.completed_work_resource,
    );

    let accolade_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_accolade_resource,
    );

    // 30 contracts completed!
    assert_eq!(work_completed_balance, dec!(30));
    // NewMoon, WaxingCrescent, FirstQuarter and WaxingGibbous accolades achieved!
    assert_eq!(accolade_balance, dec!(4));
}

#[test]
fn moonwork_service_claim_compensation_with_full_moon_accolade() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..41 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    let receipt = claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    receipt.expect_commit_success();

    let work_completed_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.completed_work_resource,
    );

    let accolade_balance = get_account_balance(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_accolade_resource,
    );

    // 40 contracts completed!
    assert_eq!(work_completed_balance, dec!(40));
    // NewMoon, WaxingCrescent, FirstQuarter, WaxingGibbous and FullMoon accolades achieved!
    assert_eq!(accolade_balance, dec!(5));
}

#[test]
fn moonwork_service_create_dispute_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    let receipt = create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.contractor_badge,
        contractor_account,
        contractor_public_key,
        &service.work_components,
        work_id,
    );

    receipt.expect_commit_success();

    let dispute_nft = get_non_fungible_data::<Dispute>(
        &store,
        service.work_components.dispute_resource,
        NonFungibleId::from_u64(1),
    );

    // assert
    assert_eq!(
        dispute_nft.work,
        NonFungibleAddress::new(
            service.work_components.work_resource,
            NonFungibleId::from_u64(1)
        )
    );
    assert_eq!(dispute_nft.raised_by, DisputeSide::Contractor);
    assert_eq!(
        dispute_nft.contractor,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec())
    );
    assert_eq!(
        dispute_nft.client,
        NonFungibleId::from_bytes("slysmik".as_bytes().to_vec())
    );
}

#[test]
fn moonwork_service_create_dispute_fails_as_outside_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    let (outside_contractor_public_key, _private_key, outside_contractor_account) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        outside_contractor_account,
        outside_contractor_public_key,
        "outside contractor",
    );

    // act
    let receipt = create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.contractor_badge,
        outside_contractor_account,
        outside_contractor_public_key,
        &service.work_components,
        work_id,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "unauthorized user");
}

#[test]
fn moonwork_service_create_dispute_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    let receipt = create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_nft = get_non_fungible_data::<Dispute>(
        &store,
        service.work_components.dispute_resource,
        NonFungibleId::from_u64(1),
    );

    // assert
    assert_eq!(
        dispute_nft.work,
        NonFungibleAddress::new(
            service.work_components.work_resource,
            NonFungibleId::from_u64(1)
        )
    );
    assert_eq!(dispute_nft.raised_by, DisputeSide::Client);
    assert_eq!(
        dispute_nft.contractor,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec())
    );
    assert_eq!(
        dispute_nft.client,
        NonFungibleId::from_bytes("slysmik".as_bytes().to_vec())
    );
    receipt.expect_commit_success();
}

#[test]
fn moonwork_service_create_dispute_fails_as_outside_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    let (outside_client_public_key, _private_key, outside_client_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        outside_client_account,
        outside_client_public_key,
        "outside client",
    );

    // act
    let receipt = create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        outside_client_account,
        outside_client_public_key,
        &service.work_components,
        work_id,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "unauthorized user");
}

#[test]
fn moonwork_service_cancel_dispute_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.contractor_badge,
        contractor_account,
        contractor_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    let receipt = cancel_dispute(
        &mut test_runner,
        &service,
        service.moon_work_resources.contractor_badge,
        contractor_account,
        contractor_public_key,
        dispute_id,
    );

    receipt.expect_commit_success();

    assert_eq!(
        does_non_fungible_exist(
            &store,
            service.work_components.dispute_resource,
            NonFungibleId::from_u64(1)
        ),
        false
    );

    let work_nft = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(work_nft.work_status, WorkStatus::InProgress);
}

#[test]
fn moonwork_service_cancel_dispute_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    let receipt = cancel_dispute(
        &mut test_runner,
        &service,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        dispute_id,
    );

    receipt.expect_commit_success();

    assert_eq!(
        does_non_fungible_exist(
            &store,
            service.work_components.dispute_resource,
            NonFungibleId::from_u64(1)
        ),
        false
    );

    let work_nft = get_non_fungible_data::<Work>(
        &store,
        service.work_components.work_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(work_nft.work_status, WorkStatus::InProgress);
}

#[test]
fn moonwork_service_submit_document_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    // act
    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);
    let dispute_document_title = "signed contract";
    let dispute_document_url = "https://example.com/doc.pdf";

    let receipt = submit_document(
        &mut test_runner,
        service.moon_work_resources.contractor_badge,
        contractor_account,
        contractor_public_key,
        &service.work_components,
        dispute_id,
        dispute_document_title,
        dispute_document_url,
    );

    // assert
    receipt.expect_commit_success();

    let dispute_document_nft = get_non_fungible_data::<DisputeDocument>(
        &store,
        service.work_components.dispute_document_resource,
        NonFungibleId::from_u64(1),
    );

    let dispute_nft = get_non_fungible_data::<Dispute>(
        &store,
        service.work_components.dispute_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(dispute_document_nft.document_title, dispute_document_title);
    assert_eq!(dispute_document_nft.document_url, dispute_document_url);
    assert_eq!(
        dispute_nft.contractor_documents,
        vec![NonFungibleId::from_u64(1)]
    );
}

#[test]
fn moonwork_service_submit_document_fails_on_outside_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);
    let dispute_document_title = "signed contract";
    let dispute_document_url = "https://example.com/doc.pdf";

    let (outside_contractor_public_key, _private_key, outside_contractor_account) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        outside_contractor_account,
        outside_contractor_public_key,
        "outside contractor",
    );

    // act
    let receipt = submit_document(
        &mut test_runner,
        service.moon_work_resources.contractor_badge,
        outside_contractor_account,
        outside_contractor_public_key,
        &service.work_components,
        dispute_id,
        dispute_document_title,
        dispute_document_url,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "unauthorized user");
}

#[test]
fn moonwork_service_submit_document_as_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);
    let dispute_document_title = "signed contract";
    let dispute_document_url = "https://example.com/doc.pdf";

    // act
    let receipt = submit_document(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        dispute_id,
        dispute_document_title,
        dispute_document_url,
    );

    receipt.expect_commit_success();

    let dispute_document_nft = get_non_fungible_data::<DisputeDocument>(
        &store,
        service.work_components.dispute_document_resource,
        NonFungibleId::from_u64(1),
    );

    let dispute_nft = get_non_fungible_data::<Dispute>(
        &store,
        service.work_components.dispute_resource,
        NonFungibleId::from_u64(1),
    );

    // assert
    assert_eq!(dispute_document_nft.document_title, dispute_document_title);
    assert_eq!(dispute_document_nft.document_url, dispute_document_url);
    assert_eq!(
        dispute_nft.client_documents,
        vec![NonFungibleId::from_u64(1)]
    );
}

#[test]
fn moonwork_service_submit_document_fails_on_outside_client() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        NonFungibleId::from_u64(1),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);
    let dispute_document_title = "signed contract";
    let dispute_document_url = "https://example.com/doc.pdf";

    let (outside_client_public_key, _private_key, outside_client_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        outside_client_account,
        outside_client_public_key,
        "outside client",
    );

    // act
    let receipt = submit_document(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        outside_client_account,
        outside_client_public_key,
        &service.work_components,
        dispute_id,
        dispute_document_title,
        dispute_document_url,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "unauthorized user");
}

#[test]
fn moonwork_service_join_and_decide_dispute() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    let client_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let contractor_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    // assert
    client_participant_receipt.expect_commit_success();
    contractor_participant_receipt.expect_commit_success();

    let dispute_nft = get_non_fungible_data::<Dispute>(
        &store,
        service.work_components.dispute_resource,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(
        dispute_nft
            .participant_clients
            .get(&NonFungibleId::from_bytes("bar".as_bytes().to_vec())),
        Some(&DisputeSide::Contractor)
    );
    assert_eq!(
        dispute_nft
            .participant_contractors
            .get(&NonFungibleId::from_bytes("foo".as_bytes().to_vec())),
        Some(&DisputeSide::Contractor)
    );
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_contractor_participates_again() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "already joined dispute");
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_client_in_own_dispute_joins_as_participant() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = join_and_decide_dispute(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id,
        DisputeSide::Client,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "cannot participate in own dispute");
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_contractor_in_own_dispute_joins_as_participant(
) {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = join_and_decide_dispute(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "cannot participate in own dispute");
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_participation_expired() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    let client_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    // assert
    client_participant_receipt.expect_commit_success();

    test_runner.set_current_epoch(241);

    let contractor_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    // assert
    contractor_participant_receipt.expect_commit_failure();
    assert_error_message(&contractor_participant_receipt, "dispute expired");
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_participation_limit_has_been_reached() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    // create an extra participant who should not be able to join a dispute
    let (
        extra_participant_contractor_public_key,
        _private_key,
        extra_participant_contractor_account,
    ) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        extra_participant_contractor_account,
        extra_participant_contractor_public_key,
        "extra",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        extra_participant_contractor_account,
        "extra",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        extra_participant_contractor_public_key,
        2,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(3);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    // try to have another contractor join a full participant
    let receipt = join_and_decide_dispute(
        &mut test_runner,
        extra_participant_contractor_account,
        extra_participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "participant limit reached");
}

#[test]
fn moonwork_service_join_and_decide_dispute_client_fails_if_participation_limit_has_been_reached() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    // create an extra participant who should not be able to join a dispute
    let (extra_participant_client_public_key, _private_key, extra_participant_client_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        extra_participant_client_account,
        extra_participant_client_public_key,
        "extra",
    );

    create_work_to_finish_work(
        &mut test_runner,
        extra_participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        extra_participant_client_public_key,
        participant_contractor_public_key,
        2,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(3);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    // try to have another contractor join a full participant
    let receipt = join_and_decide_dispute(
        &mut test_runner,
        extra_participant_client_account,
        extra_participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "participant limit reached");
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_contractor_does_not_meet_criteria() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    // act
    // participant does not meet requirements (i.e. complete 1 work successfully) so this should
    // fail
    let contractor_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    // assert
    contractor_participant_receipt.expect_commit_failure();
    assert_error_message(
        &contractor_participant_receipt,
        "contractor criteria not met",
    );
}

#[test]
fn moonwork_service_join_and_decide_dispute_fails_if_client_does_not_meet_criteria() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "foo",
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(1);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id,
    );

    let dispute_id = NonFungibleId::from_u64(1);

    // act
    // participant does not meet requirements (i.e. complete 1 work successfully) so this should
    // fail
    let client_participant_receipt = join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );

    // assert
    client_participant_receipt.expect_commit_failure();
    assert_error_message(&client_participant_receipt, "client criteria not met");
}

#[test]
fn moonwork_service_complete_dispute() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        dispute_id,
    );

    // assert
    receipt.expect_commit_success();

    let contractor_dispute_outcome_nft = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(1),
    );
    let client_dispute_outcome_nft = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(2),
    );

    assert_eq!(
        contractor_dispute_outcome_nft.work,
        NonFungibleAddress::new(service.work_components.work_resource, work_id.clone())
    );
    assert_eq!(
        contractor_dispute_outcome_nft.decision,
        DisputeDecision::Won
    );

    assert_eq!(
        client_dispute_outcome_nft.work,
        NonFungibleAddress::new(service.work_components.work_resource, work_id)
    );
    assert_eq!(client_dispute_outcome_nft.decision, DisputeDecision::Lost);
}

#[test]
fn moonwork_service_complete_dispute_claim_work_refund_as_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Client,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Client,
    );

    let receipt = complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        dispute_id,
    );

    // assert
    receipt.expect_commit_success();

    let receipt = claim_client_work_refund(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
    );

    receipt.expect_commit_success();

    let dispute_outcome_balance = get_account_balance(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.disputed_outcome_resource,
    );

    let client_nft = get_non_fungible_data::<Client>(
        &store,
        service.moon_work_resources.client_badge,
        NonFungibleId::from_bytes("slysmik".as_bytes().to_vec()),
    );

    assert_eq!(dispute_outcome_balance, dec!(1));
    assert_eq!(client_nft.disputed, 1);
}

#[test]
fn moonwork_service_complete_dispute_fails_if_not_expired_and_no_clear_decision() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        dispute_id,
    );

    // assert
    receipt.expect_commit_failure();
    assert_error_message(&receipt, "requirements not met");
}

#[test]
fn moonwork_service_create_new_work_as_client_fails_if_dispute_outstanding_withrawal() {
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();
    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let receipt = create_work_to_complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        contractor_account,
        contractor_public_key,
        0,
    );

    receipt.expect_commit_success();

    let receipt = create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "disputes pending withdrawal");
}

#[test]
fn moonwork_service_remove_work_as_client_fails_if_dispute_outstanding_withrawal() {
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();
    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let receipt = create_work_to_complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        contractor_account,
        contractor_public_key,
        1,
    );

    receipt.expect_commit_success();

    let receipt = remove_work(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        NonFungibleId::from_u64(1),
    );

    assert_error_message(&receipt, "disputes pending withdrawal");
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_request_work_as_contractor_fails_if_outstanding_dispute_withdrawal() {
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();
    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    let receipt = create_work_to_complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        contractor_account,
        contractor_public_key,
        1,
    );

    receipt.expect_commit_success();

    let receipt = request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        NonFungibleId::from_u64(1),
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "disputes pending withdrawal");
}

#[test]
fn moonwork_service_complete_dispute_fails_because_split_decision() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Client,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = complete_dispute(
        &mut test_runner,
        &service,
        client_account,
        client_public_key,
        dispute_id,
    );

    // assert
    receipt.expect_commit_failure();
    assert_error_message(&receipt, "split decision - admin decision");
}

#[test]
fn moonwork_service_complete_dispute_fails_if_not_involved() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    let receipt = complete_dispute(
        &mut test_runner,
        &service,
        // an account outside to complete dispute, this should fail
        participant_client_account,
        participant_contractor_public_key,
        dispute_id,
    );

    // assert
    receipt.expect_commit_failure();
}

#[test]
fn moonwork_service_complete_dispute_contractor_win_as_admin_only_when_split_decision_and_dispute_expired(
) {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Client,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    test_runner.set_current_epoch(241);

    let receipt = complete_dispute_as_admin(
        &mut test_runner,
        &service,
        dispute_id,
        DisputeSide::Contractor,
    );
    // assert
    receipt.expect_commit_success();

    let contractor_dispute_outcome = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(1),
    );

    let client_dispute_outcome = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(2),
    );

    assert_eq!(contractor_dispute_outcome.decision, DisputeDecision::Won);
    assert_eq!(client_dispute_outcome.decision, DisputeDecision::Lost);
}

#[test]
fn moonwork_service_complete_dispute_client_win_as_admin_only_when_split_decision_and_dispute_expired(
) {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let (participant_client_public_key, _private_key, participant_client_account) =
        test_runner.new_account();

    let (participant_contractor_public_key, _private_key, participant_contractor_account) =
        test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        participant_client_account,
        participant_client_public_key,
        "bar",
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
        "foo",
    );

    create_work_to_finish_work(
        &mut test_runner,
        participant_client_account,
        participant_contractor_account,
        "foo",
        &service.moon_work_resources,
        &service.work_components,
        participant_client_public_key,
        participant_contractor_public_key,
        1,
    );

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        participant_contractor_account,
        participant_contractor_public_key,
    );

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    create_new_work(
        &mut test_runner,
        client_account,
        client_public_key,
        service.moon_work_resources.client_badge,
        &service.work_components,
    );

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let work_id = NonFungibleId::from_u64(2);

    request_work(
        &mut test_runner,
        contractor_account,
        contractor_public_key,
        service.moon_work_resources.contractor_badge,
        service.work_components.work_component,
        work_id.clone(),
    );

    accept_contractor_for_work(
        &mut test_runner,
        service.work_components.work_component,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        service.work_components.work_resource,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec()),
        work_id.clone(),
    );

    create_new_dispute(
        &mut test_runner,
        service.moon_work_resources.client_badge,
        client_account,
        client_public_key,
        &service.work_components,
        work_id.clone(),
    );

    let dispute_id = NonFungibleId::from_u64(1);

    join_and_decide_dispute(
        &mut test_runner,
        participant_client_account,
        participant_client_public_key,
        service.moon_work_resources.client_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Client,
    );

    join_and_decide_dispute(
        &mut test_runner,
        participant_contractor_account,
        participant_contractor_public_key,
        service.moon_work_resources.contractor_badge,
        &service,
        dispute_id.clone(),
        DisputeSide::Contractor,
    );

    test_runner.set_current_epoch(241);

    let receipt =
        complete_dispute_as_admin(&mut test_runner, &service, dispute_id, DisputeSide::Client);
    // assert
    receipt.expect_commit_success();

    let contractor_dispute_outcome = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(1),
    );

    let client_dispute_outcome = get_non_fungible_data::<DisputeOutcome>(
        &store,
        service.moon_work_resources.disputed_outcome_resource,
        NonFungibleId::from_u64(2),
    );

    assert_eq!(contractor_dispute_outcome.decision, DisputeDecision::Lost);
    assert_eq!(client_dispute_outcome.decision, DisputeDecision::Won);
}

#[test]
fn test_create_moonwork_service_create_promotion_service() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (public_key, _private_key, account_component) = test_runner.new_account();

    // Creating moonwork service
    let (component, resources) =
        create_moonwork_service(&mut test_runner, account_component, public_key);

    create_promotion_service(
        &mut test_runner,
        &resources,
        account_component,
        component,
        public_key,
    );
}

#[test]
fn test_create_moonwork_service_promotion_service_promote_contractor() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let promotion_service = create_promotion_service(
        &mut test_runner,
        &service.moon_work_resources,
        service.service_admin_component_address,
        service.moon_work_component,
        service.service_admin_public_key,
    );

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..21 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    let receipt = promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        contractor_account,
        contractor_public_key,
        20,
        3,
    );

    receipt.expect_commit_success();

    let promoted_contractor = get_non_fungible_data::<PromotedContractor>(
        &store,
        promotion_service.contractor_promotion,
        NonFungibleId::from_u64(1),
    );

    assert_eq!(
        promoted_contractor.contractor_id,
        NonFungibleId::from_bytes("beemdvp".as_bytes().to_vec())
    );
}

#[test]
fn test_create_moonwork_service_promotion_service_promote_contractor_fails_if_already_running_promotion(
) {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let promotion_service = create_promotion_service(
        &mut test_runner,
        &service.moon_work_resources,
        service.service_admin_component_address,
        service.moon_work_component,
        service.service_admin_public_key,
    );

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..21 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        contractor_account,
        contractor_public_key,
        20,
        3,
    );

    let receipt = promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        contractor_account,
        contractor_public_key,
        20,
        3,
    );

    receipt.expect_commit_failure();
    assert_error_message(&receipt, "already promoted");
}

#[test]
fn test_create_moonwork_service_promotion_service_promote_contractor_remove_expired_promotions() {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let promotion_service = create_promotion_service(
        &mut test_runner,
        &service.moon_work_resources,
        service.service_admin_component_address,
        service.moon_work_component,
        service.service_admin_public_key,
    );

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    let (second_contractor_public_key, _private_key, second_contractor_account) =
        test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        second_contractor_account,
        second_contractor_public_key,
        "second",
    );

    for id in 1..21 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    for id in 20..31 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "second",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            second_contractor_public_key,
            id,
        );
    }

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        contractor_account,
        contractor_public_key,
        20,
        3,
    );

    promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        second_contractor_account,
        second_contractor_public_key,
        20,
        3,
    );

    test_runner.set_current_epoch(481);

    let receipt = remove_expired_promotions(&mut test_runner, &service, &promotion_service);

    receipt.expect_commit_success();

    assert_eq!(
        get_total_supply(&store, promotion_service.contractor_promotion),
        Decimal::zero()
    );
}

#[test]
fn test_create_moonwork_service_promotion_service_promote_contractor_fails_if_not_enough_work_completed(
) {
    // Setup the environment
    let mut store = TypedInMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(false, &mut store);

    let (client_public_key, _private_key, client_account) = test_runner.new_account();

    let service = create_moon_work_service_with_work_category(&mut test_runner);

    let promotion_service = create_promotion_service(
        &mut test_runner,
        &service.moon_work_resources,
        service.service_admin_component_address,
        service.moon_work_component,
        service.service_admin_public_key,
    );

    register_as_client(
        &mut test_runner,
        service.moon_work_component,
        client_account,
        client_public_key,
        "slysmik",
    );

    let (contractor_public_key, _private_key, contractor_account) = test_runner.new_account();

    register_as_contractor(
        &mut test_runner,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
        "beemdvp",
    );

    for id in 1..11 {
        create_work_to_finish_work(
            &mut test_runner,
            client_account,
            contractor_account,
            "beemdvp",
            &service.moon_work_resources,
            &service.work_components,
            client_public_key,
            contractor_public_key,
            id,
        );
    }

    claim_contractor_compensation(
        &mut test_runner,
        &service.moon_work_resources,
        service.moon_work_component,
        contractor_account,
        contractor_public_key,
    );

    let receipt = promote_contractor(
        &mut test_runner,
        &service,
        &promotion_service,
        contractor_account,
        contractor_public_key,
        10,
        2,
    );

    assert_error_message(&receipt, "not enough work");

    receipt.expect_commit_failure();
}
