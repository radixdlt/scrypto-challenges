use radix_engine_interface::model::FromPublicKey;
use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use kaupa::AskingType as AskingType;
use kaupa::ProposalType as ProposalType;
use kaupa::Fees as Fees;
use radix_engine::transaction::TransactionReceipt;
use transaction::model::BasicInstruction;
use kaupa::Uuid as Uuid;


/// Empty data structure for our non-fungibles. None of our tests care
/// what is in the data (nor does the Kaupa component).
#[derive(NonFungibleData)]
struct NFData {
}


/// When the Kaupa component does awkward divisions we sometimes end
/// up with inconvenient fractions that we don't want to have to
/// accurately predict in these tests. This function asserts that a
/// value is within a small delta of the expected value, for these
/// situations.
fn assert_dec_approx(expected: Decimal,
                     actual: Decimal,
                     delta: Decimal,
                     msg: &str) {
    assert!(expected - delta <= actual, "low delta: expected={} actual={} delta={} - {}",
            expected, actual, delta, msg);
    assert!(expected + delta >= actual, "high delta: expected={} actual={} delta={} - {}", 
            expected, actual, delta, msg);
}

/// Creates an NFT resource with integer-based local ids. Local ids
/// will start on `base` and count upwards until there are `amount`
/// NFTs in the resource. All NFTs will be given to `owner_account`.
fn create_nft_resource(test_runner: &mut TestRunner,
                       owner_nfgid: &NonFungibleGlobalId,
                       owner_account: &ComponentAddress,
                       base: u64,
                       amount: u64) -> ResourceAddress {
    // Create the side1 NFT resource
    let manifest = ManifestBuilder::new()
        .create_non_fungible_resource_with_owner(
            NonFungibleIdType::Integer,
            BTreeMap::new(),
            owner_nfgid.clone(),
            Some((1..amount+1).map(
                |n| (NonFungibleLocalId::Integer((base+n).into()), NFData{}))
                 .collect::<HashMap<NonFungibleLocalId, NFData>>()))
        .call_method(*owner_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let resaddr =
        receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[0];

    assert_eq!(&Decimal::from(amount),
               test_runner.get_component_resources(*owner_account).get(&resaddr).unwrap(),
               "NFT balance should start as expected");

    resaddr
}

/// Gives a number of tokens from one user account to another. Works
/// for fungibles and non-fungibles alike although you cannot name
/// specific local ids to transfer.
fn give_tokens(test_runner: &mut TestRunner,
               giver_account: &ComponentAddress,
               giver_nfgid: &NonFungibleGlobalId,
               recip_account: &ComponentAddress,
               gift_token: &ResourceAddress,
               amount: u64) {
   // Create the side1 NFT resource
    let manifest = ManifestBuilder::new()
        .withdraw_from_account_by_amount(
            *giver_account,
            amount.into(),
            *gift_token)
        .call_method(*recip_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![giver_nfgid.clone()],
    );

    receipt.expect_commit_success();
}

/// Returns `true` if `component` has a vault with resource
/// `nf_resource` containing all the non-fungibles with local ids
/// given in `nflids`. Otherwise returns false.
///
/// This is useful for asserting on when e.g. a user is supposed to
/// have some set of NFTs in their account.
fn component_has_nflids(test_runner: &mut TestRunner,
                        component: ComponentAddress,
                        nf_resource: ResourceAddress,
                        nflids: Vec<u64>) -> bool {
    let vaults = test_runner.get_component_vaults(component, nf_resource);
    let vault_nflids = test_runner.inspect_nft_vault(vaults[0]).unwrap();
    return nflids.into_iter().all(
        |nflid| vault_nflids.contains(&NonFungibleLocalId::Integer(nflid.into())));
}

/// Returns a vector holding those ids in `nflids` (or resource
/// `nf_resource`) that are not currently held by `component`.
fn filter_by_owned_nflids(test_runner: &mut TestRunner,
                          component: ComponentAddress,
                          nf_resource: ResourceAddress,
                          nflids: Vec<u64>) -> Vec<u64>{
    let vaults = test_runner.get_component_vaults(component, nf_resource);
    let vault_nflids = test_runner.inspect_nft_vault(vaults[0]).unwrap();
    let mut retval: Vec<u64> = Vec::new();
    for nflid_int in nflids {
        let nflid = NonFungibleLocalId::Integer(nflid_int.into());
        if vault_nflids.contains(&nflid) { retval.push(nflid_int); }
    }
    retval
}

/// Helper function to print the nonfungible local ids owned by a
/// component.
fn _print_nflids_owned(test_runner: &mut TestRunner,
                      component: ComponentAddress,
                      nf_resource: ResourceAddress) {
    let vaults = test_runner.get_component_vaults(component, nf_resource);
    let vault_nflids = test_runner.inspect_nft_vault(vaults[0]).unwrap();
    println!("{:?}", vault_nflids);
}

/// Creates a proposal on a fungible-to-fungible trading pair.
fn make_trading_pair_proposal_f2f(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    offer_resaddr: ResourceAddress,
    offer_amount: Decimal,
    ask_resaddr: ResourceAddress,
    ask_amount: Decimal,
    fee_resaddr: ResourceAddress,
    fee_amount: Decimal) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(*account, *owning_nft_resaddr)
        .withdraw_from_account_by_amount(*account, offer_amount, offer_resaddr)
        .withdraw_from_account_by_amount(*account, fee_amount, fee_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.take_from_worktop_by_amount(
                    offer_amount,
                    offer_resaddr,
                    |builder, paying_bucket_id| {
                        builder.take_from_worktop_by_amount(
                            fee_amount,
                            fee_resaddr,
                            |builder, fee_bucket_id| {
                                builder.call_method(
                                    *kaupa, "make_proposal",
                                    args!(proof_id,
                                          None::<NonFungibleGlobalId>,
                                          ProposalType::Barter,
                                          Vec::<ManifestBucket>::from([paying_bucket_id]),
                                          HashMap::<ResourceAddress, AskingType>::from([
                                              (ask_resaddr,
                                               AskingType::Fungible(ask_amount)),
                                          ]),
                                          true,
                                          Vec::<ManifestBucket>::from([fee_bucket_id])
                                    ))
                            })
                    })
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Creates a proposal for a non-fungible to non-fungible trading
/// pair. Note that option to add more than one resource for fees.
fn make_trading_pair_proposal_nf2nf(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    offer_resaddr: ResourceAddress,
    offer_nflids: Vec<u64>,
    ask_resaddr: ResourceAddress,
    ask_amount: Option<u64>,
    ask_nflids: Option<HashSet<u64>>,
    fee1_resaddr: ResourceAddress,
    fee1_amount: u64,
    fee2_resaddr: ResourceAddress,
    fee2_amount: u64) -> TransactionReceipt
{
    let ask_nflids = match ask_nflids {
        None => None,
        Some(s) => Some(s.into_iter().map(|v| NonFungibleLocalId::Integer(v.into())).collect()),
    };
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(*account, *owning_nft_resaddr)
        .withdraw_from_account_by_amount(*account, fee1_amount.into(), fee1_resaddr)
        .withdraw_from_account_by_amount(*account, fee2_amount.into(), fee2_resaddr)
        .withdraw_from_account_by_ids(*account,
                                      &offer_nflids.into_iter().map(
                                          |nflid| NonFungibleLocalId::Integer(nflid.into())).collect(),
                                      offer_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.take_from_worktop(
                    offer_resaddr,
                    |builder, offer_bucket_id| {
                        builder.take_from_worktop(
                            fee1_resaddr,
                            |builder, fee1_bucket_id| {
                                builder.take_from_worktop(
                                    fee2_resaddr,
                                    |builder, fee2_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "make_proposal",
                                            args!(proof_id,
                                                  None::<NonFungibleGlobalId>,
                                                  ProposalType::Barter,
                                                  Vec::<ManifestBucket>::from([offer_bucket_id]),
                                                  HashMap::<ResourceAddress, AskingType>::from([
                                                      (ask_resaddr,
                                                       AskingType::NonFungible(ask_nflids, ask_amount))
                                                  ]),
                                                  true,
                                                  Vec::<ManifestBucket>::from([
                                                      fee1_bucket_id,fee2_bucket_id])
                                            ))
                                    })
                            })
                    })
            })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}


/// Creates a proposal for a non-fungible to fungible trading
/// pair. Note that option to add more than one resource for fees.
///
/// Note that "nf2f" means that you are buying fungibles from a
/// non-fungible/fungible pair. If you are instead offering fungibles
/// into such a pair that particular offering tx flips the tables and
/// is considered an f2nf transaction.
fn make_trading_pair_proposal_nf2f(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    offer_resaddr: ResourceAddress,
    offer_nflids: Vec<u64>,
    ask_resaddr: ResourceAddress,
    ask_amount: Decimal,
    fee1_resaddr: ResourceAddress,
    fee1_amount: u64,
    fee2_resaddr: ResourceAddress,
    fee2_amount: u64) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(*account, *owning_nft_resaddr)
        .withdraw_from_account_by_amount(*account, fee1_amount.into(), fee1_resaddr)
        .withdraw_from_account_by_amount(*account, fee2_amount.into(), fee2_resaddr)
        .withdraw_from_account_by_ids(*account,
                                      &offer_nflids.into_iter().map(
                                          |nflid| NonFungibleLocalId::Integer(nflid.into())).collect(),
                                      offer_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.take_from_worktop(
                    offer_resaddr,
                    |builder, offer_bucket_id| {
                        builder.take_from_worktop(
                            fee1_resaddr,
                            |builder, fee1_bucket_id| {
                                builder.take_from_worktop(
                                    fee2_resaddr,
                                    |builder, fee2_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "make_proposal",
                                            args!(proof_id,
                                                  None::<NonFungibleGlobalId>,
                                                  ProposalType::Barter,
                                                  Vec::<ManifestBucket>::from([offer_bucket_id]),
                                                  HashMap::<ResourceAddress, AskingType>::from([
                                                      (ask_resaddr,
                                                       AskingType::Fungible(ask_amount))
                                                  ]),
                                                  true,
                                                  Vec::<ManifestBucket>::from([
                                                      fee1_bucket_id,fee2_bucket_id])
                                            ))
                                    })
                            })
                    })
            })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Creates a proposal for a fungible to non-fungible trading
/// pair. Note that option to add more than one resource for fees.
///
/// Note that "f2nf" means that you are buying non-fungibles from a
/// fungible/non-fungible pair. If you are instead offering fungibles
/// into such a pair that particular offering tx flips the tables and
/// is considered an nf2f transaction.
fn make_trading_pair_proposal_f2nf(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    offer_resaddr: ResourceAddress,
    offer_amount: Decimal,
    ask_resaddr: ResourceAddress,
    ask_nflids: Vec<u64>,
    ask_amount: u64,
    fee1_resaddr: ResourceAddress,
    fee1_amount: u64,
    fee2_resaddr: ResourceAddress,
    fee2_amount: u64) -> TransactionReceipt
{
    let nflids =
        if ask_nflids.len() > 0 {
            Some(ask_nflids.into_iter().map(
                |nflid| NonFungibleLocalId::Integer(nflid.into())).collect())
        } else {
            None
        };

    let ask_amount = if ask_amount > 0 { Some(ask_amount) } else { None };
    
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(*account, *owning_nft_resaddr)
        .withdraw_from_account_by_amount(*account, fee1_amount.into(), fee1_resaddr)
        .withdraw_from_account_by_amount(*account, fee2_amount.into(), fee2_resaddr)
        .withdraw_from_account_by_amount(*account, offer_amount, offer_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.take_from_worktop(
                    offer_resaddr,
                    |builder, offer_bucket_id| {
                        builder.take_from_worktop(
                            fee1_resaddr,
                            |builder, fee1_bucket_id| {
                                builder.take_from_worktop(
                                    fee2_resaddr,
                                    |builder, fee2_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "make_proposal",
                                            args!(proof_id,
                                                  None::<NonFungibleGlobalId>,
                                                  ProposalType::Barter,
                                                  Vec::<ManifestBucket>::from([offer_bucket_id]),
                                                  HashMap::<ResourceAddress, AskingType>::from([
                                                      (ask_resaddr,
                                                       AskingType::NonFungible(nflids, ask_amount))
                                                  ]),
                                                  true,
                                                  Vec::<ManifestBucket>::from([
                                                      fee1_bucket_id,fee2_bucket_id])
                                            ))
                                    })
                            })
                    })
            })
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Determines how many fungibles an `AskingType` is asking for.
fn at_to_fung(at: &AskingType) -> Decimal {
    match at {
        AskingType::Fungible(d) => d.clone(),
        AskingType::NonFungible(_, _) => Decimal::ZERO,
    }
}


/// Determines how many non-fungibles an `AskingType` is asking for.
fn at_to_nonfung(at: &AskingType) -> BTreeSet<NonFungibleLocalId> {
    match at {
        AskingType::Fungible(_) => BTreeSet::new(),
        AskingType::NonFungible(nflids, d) => {
            assert!(d.is_none(), "random nonfungibles not supported");
            if nflids.is_none() { BTreeSet::new() } else {
                nflids.as_ref().unwrap().clone().into_iter().collect()
            }
        }
    }
}


/// Determines whether an `AskingType` is for a fungible type.
fn is_at_fung(at: &AskingType) -> bool {
    match at {
        AskingType::Fungible(_) => true,
        AskingType::NonFungible(_, _) => false,
    }
}


/// Converts a vector of currency descriptions (basically resource +
/// amount) into a series of "withdraw from account" + "take from
/// worktop" type manifest instructions, putting them into the input
/// manifest. Returns a vector of all the buckets created.
fn currency_vec_to_manifest_grab<'a>(mut manifest: &'a mut ManifestBuilder,
                                 currencies: Vec<(ResourceAddress, AskingType)>,
                                 account: &ComponentAddress)
                                 -> (&'a mut ManifestBuilder, Vec<ManifestBucket>) {
    let mut manifest_buckets = Vec::new();
    
    for currency in &currencies {
        if is_at_fung(&currency.1) {
            // Fungible resource
            manifest = manifest
                .withdraw_from_account_by_amount(*account, at_to_fung(&currency.1), currency.0);

            let bucket_id;
            (manifest, bucket_id, _) =
                manifest.add_instruction(BasicInstruction::TakeFromWorktopByAmount{
                    amount: at_to_fung(&currency.1),
                    resource_address: currency.0.clone()
                });
            manifest_buckets.push(bucket_id.expect("couldn't make manifest f-bucket"));
        } else {
            // Non-fungible resource
            manifest = manifest
                .withdraw_from_account_by_ids(*account, &at_to_nonfung(&currency.1), currency.0);

            let bucket_id;
            (manifest, bucket_id, _) =
                manifest.add_instruction(BasicInstruction::TakeFromWorktopByIds{
                    ids: at_to_nonfung(&currency.1),
                    resource_address: currency.0.clone()
                });
            manifest_buckets.push(bucket_id.expect("couldn't make manifest nf-bucket"));
        }
    }
    (manifest, manifest_buckets)
}

/// Converts a vector of currency descriptions (basically resource +
/// amount) into a series of "withdraw from account" + "take from
/// worktop" type manifest instructions, putting them into the input
/// manifest. Returns a vector of all the buckets created.
fn currency_vec_to_manifest_grab_fm_worktop<'a>(mut manifest: &'a mut ManifestBuilder,
                                 currencies: Vec<(ResourceAddress, AskingType)>,)
                                 -> (&'a mut ManifestBuilder, Vec<ManifestBucket>) {
    let mut manifest_buckets = Vec::new();
    
    for currency in &currencies {
        if is_at_fung(&currency.1) {
            // Fungible resource
            let bucket_id;
            (manifest, bucket_id, _) =
                manifest.add_instruction(BasicInstruction::TakeFromWorktopByAmount{
                    amount: at_to_fung(&currency.1),
                    resource_address: currency.0.clone()
                });
            manifest_buckets.push(bucket_id.expect("couldn't make manifest f-bucket"));
        } else {
            // Non-fungible resource
            let bucket_id;
            (manifest, bucket_id, _) =
                manifest.add_instruction(BasicInstruction::TakeFromWorktopByIds{
                    ids: at_to_nonfung(&currency.1),
                    resource_address: currency.0.clone()
                });
            manifest_buckets.push(bucket_id.expect("couldn't make manifest nf-bucket"));
        }
    }
    (manifest, manifest_buckets)
}


/// Converts a vector of u64 into a vector of NonFungibleLocalId
fn to_nflids(ints: Vec<u64>) -> HashSet<NonFungibleLocalId> {
    ints.into_iter().map(|n| NonFungibleLocalId::Integer(n.into())).collect()
}

/// Creates a proposal on any Kaupa.
fn make_generic_proposal(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    counterparty: Option<NonFungibleGlobalId>,
    ptype: ProposalType,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    allow_partial: bool,
    offering: Vec<(ResourceAddress, AskingType)>,
    asking: Vec<(ResourceAddress, AskingType)>,
    fees: Vec<(ResourceAddress, AskingType)>) -> ( TransactionReceipt, Uuid )
{
    let mut askingmap = HashMap::new();
    for (resaddr, at) in asking {
        askingmap.insert(resaddr, at);
    }

    // This is here to help us count towards the instruction that
    // returns our proposal's uuid (i.e. the actual make_proposal
    // call)
    let mut instruction_counter = 0;

    let mut builder = ManifestBuilder::new();
    instruction_counter += 1;
    let mut manifest = builder
        .create_proof_from_account_by_ids(*account,
                                          &BTreeSet::from([owning_nft_lid.into()]),
                                          *owning_nft_resaddr);
    instruction_counter += 1;

    let offering_bucket_ids;
    (manifest, offering_bucket_ids) = 
        currency_vec_to_manifest_grab(manifest, offering, account);
    instruction_counter += 2 * offering_bucket_ids.len();

    let fee_bucket_ids;
    (manifest, fee_bucket_ids) = 
        currency_vec_to_manifest_grab(manifest, fees, account);
    instruction_counter += 2 * fee_bucket_ids.len();

    let manifest = manifest
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.call_method(
                    *kaupa, "make_proposal",
                    args!(proof_id,
                          counterparty,
                          ptype,
                          Vec::<ManifestBucket>::from(offering_bucket_ids),
                          askingmap,
                          allow_partial,
                          Vec::<ManifestBucket>::from(fee_bucket_ids)
                    ))
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    instruction_counter += 1;

    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    let uuid = receipt.output::<(u128, Vec<Bucket>)>(instruction_counter).0;
    ( receipt,  uuid )
}

/// Accepts a proposal that takes fungible payment and fungible
/// fees. Note the option to pay fees both in the currency of the
/// trade payment and an additional currency.
fn accept_proposal_f(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    allow_partial: bool,
    payment_resaddr: ResourceAddress,
    payment_amount: Decimal,
    fee_amount: Decimal,
    other_fee_resaddr: ResourceAddress,
    other_fee_amount: Decimal) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .withdraw_from_account_by_amount(*account,
                                         other_fee_amount,
                                         other_fee_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         payment_amount + fee_amount,
                                         payment_resaddr)
        .take_from_worktop_by_amount(
            payment_amount,
            payment_resaddr,
            |builder, payment_bucket_id| {
                builder.take_from_worktop_by_amount(
                    fee_amount,
                    payment_resaddr,
                    |builder, fee_bucket_id| {
                        builder.take_from_worktop_by_amount(
                            other_fee_amount,
                            other_fee_resaddr,
                            |builder, other_fee_bucket_id| {
                                builder.call_method(
                                    *kaupa, "accept_proposal",
                                    args!(None::<NonFungibleGlobalId>,
                                          proposal_uuid,
                                          allow_partial,
                                          Vec::<ManifestBucket>::from([payment_bucket_id]),
                                          Vec::<ManifestBucket>::from([fee_bucket_id,
                                                                       other_fee_bucket_id])))
                            })
                    })
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Accepts a proposal for the nf2f tests.
fn accept_proposal_nf2f(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    allow_partial: bool,
    payment_resaddr: ResourceAddress,
    payment_amount: Decimal,
    payment_fee: Decimal,
    fee1_resaddr: ResourceAddress,
    fee1_amount: Decimal,
    fee2_resaddr: ResourceAddress,
    fee2_amount: Decimal) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .withdraw_from_account_by_amount(*account,
                                         fee1_amount,
                                         fee1_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         fee2_amount,
                                         fee2_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         payment_amount + payment_fee,
                                         payment_resaddr)
        .take_from_worktop_by_amount(
            payment_amount,
            payment_resaddr,
            |builder, payment_bucket_id| {
                builder.take_from_worktop_by_amount(
                    payment_fee,
                    payment_resaddr,
                    |builder, payment_fee_bucket_id| {
                        builder.take_from_worktop_by_amount(
                            fee1_amount,
                            fee1_resaddr,
                            |builder, fee1_bucket_id| {
                                builder.take_from_worktop_by_amount(
                                    fee2_amount,
                                    fee2_resaddr,
                                    |builder, fee2_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "accept_proposal",
                                            args!(None::<NonFungibleGlobalId>,
                                                  proposal_uuid,
                                                  allow_partial,
                                                  Vec::<ManifestBucket>::from([payment_bucket_id]),
                                                  Vec::<ManifestBucket>::from([payment_fee_bucket_id,
                                                                               fee1_bucket_id,
                                                                               fee2_bucket_id])))
                                    })
                            })
                    })
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Accepts a proposal for the f2nf tests.
fn accept_proposal_f2nf(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    allow_partial: bool,
    payment_resaddr: ResourceAddress,
    payment_nflids: Vec<u64>,
    fee1_resaddr: ResourceAddress,
    fee1_amount: Decimal,
    fee2_resaddr: ResourceAddress,
    fee2_amount: Decimal) -> TransactionReceipt
{
    let payment_nflids = payment_nflids.into_iter().map(
        |nflid| NonFungibleLocalId::Integer(nflid.into())).collect();
    let manifest = ManifestBuilder::new()
        .withdraw_from_account_by_amount(*account,
                                         fee1_amount,
                                         fee1_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         fee2_amount,
                                         fee2_resaddr)
        .withdraw_from_account_by_ids(*account,
                                         &payment_nflids,
                                         payment_resaddr)
                .take_from_worktop_by_ids(
                    &payment_nflids,
                    payment_resaddr,
                    |builder, payment_bucket_id| {
                        builder.take_from_worktop_by_amount(
                            fee1_amount,
                            fee1_resaddr,
                            |builder, fee1_bucket_id| {
                                builder.take_from_worktop_by_amount(
                                    fee2_amount,
                                    fee2_resaddr,
                                    |builder, fee2_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "accept_proposal",
                                            args!(None::<NonFungibleGlobalId>,
                                                  proposal_uuid,
                                                  allow_partial,
                                                  Vec::<ManifestBucket>::from([payment_bucket_id]),
                                                  Vec::<ManifestBucket>::from([fee1_bucket_id,
                                                                               fee2_bucket_id])))
                                    })
                            })
                    })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Accepts a proposal that takes payment in non-fungibles. Note that
/// while the different fees *can* also be non-fungibles you cannot
/// name which local ids exactly to pay in fees.
fn accept_proposal_nf(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    allow_partial: bool,
    payment_resaddr: ResourceAddress,
    payment_nflids: Vec<u64>,
    fee1_resaddr: ResourceAddress,
    fee1_amount: Decimal,
    fee2_resaddr: ResourceAddress,
    fee2_amount: Decimal,
    fee3_resaddr: ResourceAddress,
    fee3_amount: Decimal) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .withdraw_from_account_by_amount(*account,
                                         fee1_amount,
                                         fee1_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         fee2_amount,
                                         fee2_resaddr)
        .withdraw_from_account_by_amount(*account,
                                         fee3_amount,
                                         fee3_resaddr)
        .withdraw_from_account_by_ids(*account,
                                      &payment_nflids.into_iter().map(
                                          |nflid| NonFungibleLocalId::Integer(nflid.into())).collect(),
                                      payment_resaddr)
        .take_from_worktop(
            payment_resaddr,
            |builder, payment_bucket_id| {
                builder.take_from_worktop_by_amount(
                    fee1_amount,
                    fee1_resaddr,
                    |builder, fee1_bucket_id| {
                        builder.take_from_worktop_by_amount(
                            fee2_amount,
                            fee2_resaddr,
                            |builder, fee2_bucket_id| {
                                builder.take_from_worktop_by_amount(
                                    fee3_amount,
                                    fee3_resaddr,
                                    |builder, fee3_bucket_id| {
                                        builder.call_method(
                                            *kaupa, "accept_proposal",
                                            args!(None::<NonFungibleGlobalId>,
                                                  proposal_uuid,
                                                  allow_partial,
                                                  Vec::<ManifestBucket>::from([payment_bucket_id]),
                                                  Vec::<ManifestBucket>::from([
                                                      fee1_bucket_id,
                                                      fee2_bucket_id,
                                                      fee3_bucket_id])))
                                    })
                            })
                    })
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}



/// Accepts a proposal taking any configuration of currencies.
fn accept_otc_proposal(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    use_proof: Option<(ResourceAddress, u64)>,
    allow_partial: bool,
    paying: Vec<(ResourceAddress, AskingType)>,
    fees: Vec<(ResourceAddress, AskingType)>) -> TransactionReceipt
{
    let mut manifest = ManifestBuilder::new();

    let (manifest, paying_bucket_ids) = 
        currency_vec_to_manifest_grab(&mut manifest, paying, account);

    let (mut manifest, fee_bucket_ids) = 
        currency_vec_to_manifest_grab(manifest, fees, account);

    let trader_proof = match use_proof {
        None => None,
        Some((resaddr, nflid)) => {
            manifest = 
                manifest.create_proof_from_account_by_ids(
                    *account,
                    &BTreeSet::from([nflid.into()]),
                    resaddr);
            let proof;
            (manifest, _, proof) =
                manifest.add_instruction(BasicInstruction::CreateProofFromAuthZoneByIds{
                    ids: BTreeSet::from([nflid.into()]),
                    resource_address: resaddr
                });
            proof
        }
    };
    
    let manifest = manifest
        .call_method(
            *kaupa, "accept_proposal",
            args!(trader_proof,
                  proposal_uuid,
                  allow_partial,
                  Vec::<ManifestBucket>::from(paying_bucket_ids),
                  Vec::<ManifestBucket>::from(fee_bucket_ids)))
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}


/// Accepts a flash loan proposal and asserts that we received the
/// funds before paying it back.
fn accept_flash_loan(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    proposal_uuid: Uuid,
    use_proof: Option<(ResourceAddress, u64)>,
    paying: Vec<(ResourceAddress, AskingType)>,
    fees: Vec<(ResourceAddress, AskingType)>,
    repay: Vec<(ResourceAddress, AskingType)>,
    transient_resource: ResourceAddress,
) -> TransactionReceipt
{
    let mut manifest = ManifestBuilder::new();

    let (manifest, paying_bucket_ids) = 
        currency_vec_to_manifest_grab(&mut manifest, paying, account);

    let (mut manifest, fee_bucket_ids) = 
        currency_vec_to_manifest_grab(manifest, fees, account);

    let trader_proof = match use_proof {
        None => None,
        Some((resaddr, nflid)) => {
            manifest = 
                manifest.create_proof_from_account_by_ids(
                    *account,
                    &BTreeSet::from([nflid.into()]),
                    resaddr);
            let proof;
            (manifest, _, proof) =
                manifest.add_instruction(BasicInstruction::CreateProofFromAuthZoneByIds{
                    ids: BTreeSet::from([nflid.into()]),
                    resource_address: resaddr
                });
            proof
        }
    };
    
    let mut manifest = manifest
        .call_method(
            *kaupa, "accept_proposal",
            args!(trader_proof,
                  proposal_uuid,
                  false,
                  Vec::<ManifestBucket>::from(paying_bucket_ids),
                  Vec::<ManifestBucket>::from(fee_bucket_ids)));

    // This tests that we now have the loan funds on the worktop
    for (resaddr, ask) in &repay {
        match ask {
            AskingType::Fungible(amount) => {
                manifest = manifest
                    .assert_worktop_contains_by_amount(*amount, *resaddr);
            },
            AskingType::NonFungible(nflids, _) => {
                if let Some(nflids) = nflids {
                    manifest = manifest
                        .assert_worktop_contains_by_ids(&nflids.iter().map(|v| v.clone()).collect(),
                                                        *resaddr);
                }
            }
        }
    }

    
    let transient_bucket_id;
    (manifest, transient_bucket_id, _) =
        manifest
        // The following line is for debugging the transient_resource
        // problem farther down.
        .assert_worktop_contains_by_amount(1.into(), transient_resource)
        .add_instruction(BasicInstruction::TakeFromWorktop{
            resource_address: transient_resource
        });
    
    let (manifest, repay_bucket_ids) = 
        currency_vec_to_manifest_grab_fm_worktop(&mut manifest, repay.clone());

    let mut manifest = manifest
        .call_method(
            *kaupa, "repay_flash_loan",
            args!(
                transient_bucket_id.unwrap(),
                Vec::<ManifestBucket>::from(repay_bucket_ids))
        )
        // If we uncomment the following line the assertion fails
        // because that resource got burnt.
    //        .assert_worktop_contains_by_amount(1.into(), transient_resource)
        ;

    for (resaddr, ask) in &repay {
        match ask {
            AskingType::Fungible(_) => {
                manifest = manifest
                    .assert_worktop_contains_by_amount(Decimal::ZERO, *resaddr);
            },
            AskingType::NonFungible(_, _) => {
                manifest = manifest
                    .assert_worktop_contains_by_amount(Decimal::ZERO, *resaddr);
            }
        }
    }
    
    let manifest = manifest
        // TODO find out why this fails even though we now know the
        // transient_resource is burnt.
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}


/// Sweeps zero or one or more proposals
fn sweep_proposals(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    use_proof: Option<(ResourceAddress, u64)>,
    max_price: Option<Decimal>,
    paying: (ResourceAddress, AskingType),
    fees: Vec<(ResourceAddress, AskingType)>) -> TransactionReceipt
{
    let mut manifest = ManifestBuilder::new();

    let (manifest, paying_bucket_ids) = 
        currency_vec_to_manifest_grab(&mut manifest, [paying].into(), account);

    let (mut manifest, fee_bucket_ids) = 
        currency_vec_to_manifest_grab(manifest, fees, account);

    let trader_proof = match use_proof {
        None => None,
        Some((resaddr, nflid)) => {
            manifest = 
                manifest.create_proof_from_account_by_ids(
                    *account,
                    &BTreeSet::from([nflid.into()]),
                    resaddr);
            let proof;
            (manifest, _, proof) =
                manifest.add_instruction(BasicInstruction::CreateProofFromAuthZoneByIds{
                    ids: BTreeSet::from([nflid.into()]),
                    resource_address: resaddr
                });
            proof
        }
    };
    
    let manifest = manifest
        .call_method(
            *kaupa, "sweep_proposals",
            args!(trader_proof,
                  max_price,
                  &paying_bucket_ids[0],
                  Vec::<ManifestBucket>::from(fee_bucket_ids)))
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}


/// Collects payments and optionally fees owed a user.
fn collect_funds(
    test_runner: &mut TestRunner,
    user_nfgid: &NonFungibleGlobalId,
    kaupa: &ComponentAddress,
    account: &ComponentAddress,
    collect_fees: bool,
    owning_nft_resaddr: &ResourceAddress,
    owning_nft_lid: u64,
    collect_token: Option<ResourceAddress>) -> TransactionReceipt
{
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(*account, *owning_nft_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([owning_nft_lid.into()]),
            *owning_nft_resaddr,
            |builder, proof_id| {
                builder.call_method(*kaupa, "collect_funds",
                                    args!(proof_id,
                                          true,
                                          collect_fees,
                                          collect_token))
            })
        .call_method(*account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );

    receipt
}

/// Sets up a fungible/fungible trading pair and runs it through its
/// paces.
#[test]
//#[ignore]
fn test_f2f_trading_pair() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let nfgid_user1 = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    let manifest = ManifestBuilder::new()
        .create_fungible_resource_with_owner(
            18,
            BTreeMap::new(),
            user2_nfgid.clone(),
            Some(dec!("100000")))
        .call_method(user2_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![nfgid_user1.clone()],
    );

    receipt.expect_commit_success();
    let side1_resaddr = receipt
        .expect_commit()
        .entity_changes
        .new_resource_addresses[0];
    assert_eq!(&dec!("100000"),
               test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "TOK balance should start as expected");

    let nft_resaddr =
        test_runner.create_non_fungible_resource(user1_account);

    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());
    
    // Call the `instantiate_kaupa` function with a fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([(
                                     RADIX_TOKEN, AskingType::Fungible("0.5".into()))])),
                                 per_tx_taker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([(
                                     RADIX_TOKEN, AskingType::Fungible("0.75".into()))])),
                                 per_nft_flat_fee: None,
                                 per_payment_bps_fee: Some("10".into()),
                             }),
                             Some(HashSet::from([side1_resaddr])),
                             Some(HashSet::from([RADIX_TOKEN])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    let mut sum_of_all_fees = HashMap::from([(RADIX_TOKEN, Decimal::ZERO),
                                             (side1_resaddr, Decimal::ZERO)]);
    
    // Add a trade proposal owned by NFT 1
    // Offering 10 XRD in exhcange for 1 TOK

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             RADIX_TOKEN,
                                             "10".into(),
                                             side1_resaddr,
                                             "1".into(),
                                             RADIX_TOKEN,
                                             "0.5".into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.5");

    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(7).0;

    assert_eq!(pre_xrd_balance - dec!("10.5"),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 10.5");


    // Try to rescind the proposal using the wrong id, should fail
    let manifest = ManifestBuilder::new()
        .create_proof_from_account(user1_account, nft_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([2.into()]),
            nft_resaddr,
            |builder, proof_id| {
                builder.call_method(component, "rescind_proposal",
                                    args!(proof_id,
                                          uuid1
                                    ))
            })
        .call_method(user1_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![nfgid_user1.clone()],
    );
    receipt.expect_commit_failure();


    // Rescind the trade proposal
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();

    let manifest = ManifestBuilder::new()
        .create_proof_from_account(user1_account, nft_resaddr)
        .create_proof_from_auth_zone_by_ids(
            &BTreeSet::from([1.into()]),
            nft_resaddr,
            |builder, proof_id| {
                builder.call_method(component, "rescind_proposal",
                                    args!(proof_id,
                                          uuid1
                                    ))
            })
        .call_method(user1_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![nfgid_user1.clone()],
    );

    receipt.expect_commit_success();

    assert_eq!(pre_xrd_balance + 10,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be back up again (sans the maker fee)");


    // user1 tries to add a trade proposal but doesn't send enough
    // maker fee
    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             RADIX_TOKEN,
                                             "5".into(),
                                             side1_resaddr,
                                             "25".into(),
                                             RADIX_TOKEN,
                                             "0.1".into()); // fee is 0.5
    receipt.expect_commit_failure();


    // user1 adds a trade proposal owned by their NFT 1
    // Offering 5 XRD in exhcange for 25 TOK
    // This tests that we can trade in the reverse direction.
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             RADIX_TOKEN,
                                             "5".into(),
                                             side1_resaddr,
                                             "25".into(),
                                             RADIX_TOKEN,
                                             "0.5".into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.5");

    let uuid = receipt.output::<(u128, Vec<Bucket>)>(7).0;

    assert_eq!(pre_xrd_balance - dec!("5.5"),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 5.5");


    // user2 tries to accept a proposal but sends insufficient fees
    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  false,
                                  side1_resaddr,
                                  "25".into(),
                                  "0.025".into(),
                                  RADIX_TOKEN,
                                  "0.2".into()); // fee is 0.75
    receipt.expect_commit_failure();


    // user2 tries to accept a proposal but sends the wrong token for
    // fees
    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  false,
                                  side1_resaddr,
                                  "25".into(),
                                  "0.025".into(),
                                  side1_resaddr,
                                  "1".into()); // fee is 0.75 XRD
    receipt.expect_commit_failure();

    
    // user2 accepts the proposal
    let pre_tok_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  false,
                                  side1_resaddr,
                                  "25".into(),
                                  "0.025".into(),
                                  RADIX_TOKEN,
                                  "0.75".into());
    receipt.expect_commit_success();
    
    assert_eq!(pre_tok_balance - 25 - dec!("0.025"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be down by 25 and fee");
    assert_eq!(pre_xrd_balance + 5 - dec!("0.75"),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be up by 4.25");
    *sum_of_all_fees.get_mut(&side1_resaddr).unwrap() += dec!("0.025");
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.75");

    
    // user1 collects the payment
    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &nft_resaddr,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(&dec!("25"),
               test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be up by 25");

    
    // user1 tries to add a trade proposal but sends the wrong token
    // type as fee
    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             side1_resaddr,
                                             "10".into(),
                                             RADIX_TOKEN,
                                             "3".into(),
                                             side1_resaddr,
                                             "1".into()); // fee is 0.5 XRD
    receipt.expect_commit_failure();

    

    // user1 adds a trade proposal owned by their NFT 1
    // Offering 10 TOK in exchange for 3 XRD
    // This tests that we can trade in the normal direction.
    // We send extra fees to test that the excess is returned.
    let pre_tok_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             side1_resaddr,
                                             "10".into(),
                                             RADIX_TOKEN,
                                             "3".into(),
                                             RADIX_TOKEN,
                                             "5.5".into()); // only 0.5 actually needed
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.5");

    // The generated UUID for our trade proposal
    let uuid = receipt.output::<(u128, Vec<Bucket>)>(7).0;

    assert_eq!(pre_tok_balance - 10,
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be down by 10");
    assert_eq!(pre_xrd_balance - dec!("0.5"),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 0.5");



    // user2 accepts the proposal
    let pre_tok_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  false,
                                  RADIX_TOKEN,
                                  "3".into(),
                                  "0.003".into(),
                                  RADIX_TOKEN,
                                  "0.75".into());
    receipt.expect_commit_success();
    
    assert_eq!(pre_tok_balance + 10,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be up by 10");
    assert_eq!(pre_xrd_balance - 3 - dec!("0.003")- dec!("0.75"),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 3 and fee");
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.003") + dec!("0.75");


    
    // user1 collects the payment
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &nft_resaddr,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + 3,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be up by 3");



    // Repeat the last to test that we can re-use a maker's payment
    // vaults (they are created for a resource on the maker's first
    // collect and then reused later)
    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             side1_resaddr,
                                             "10".into(),
                                             RADIX_TOKEN,
                                             "3".into(),
                                             RADIX_TOKEN,
                                             "0.5".into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.5");

    // The generated UUID for our trade proposal
    let uuid = receipt.output::<(u128, Vec<Bucket>)>(7).0;

    assert_eq!(&dec!(5),
               test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be down by 10");



    // user2 accepts the proposal
    //
    // Here we also test that giving too much fees to the contract
    // returns the excess to us
    let pre_tok_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  false,
                                  RADIX_TOKEN,
                                  "3".into(),
                                  "1".into(), // only 0.003 actually needed
                                  RADIX_TOKEN,
                                  "0.75".into()); 
    receipt.expect_commit_success();
    
    assert_eq!(pre_tok_balance + 10,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be up by 10");
    assert_eq!(pre_xrd_balance - 3 - dec!("0.003") - dec!("0.75"),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 3 and fee");
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.003") + dec!("0.75");


    
    // user1 collects the payment
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account)[&RADIX_TOKEN].clone();

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &nft_resaddr,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + 3,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be up by 3");


    
    // Kaupa owner collects their fees
    assert!(test_runner.get_component_resources(owner_account).get(&side1_resaddr).is_none(),
            "Kaupa owner should not yet have any TOK");
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert_eq!(*sum_of_all_fees.get(&side1_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&side1_resaddr).unwrap(),
               "Kaupa owner should have their TOK fees");



    let mut sum_of_all_fees = HashMap::from([(RADIX_TOKEN, Decimal::ZERO),
                                             (side1_resaddr, Decimal::ZERO)]);


    // user1 adds a trade proposal owned by their NFT 1
    // Offering 10 XRD in exchange for 3 TOK
    // This will be used to test partial satisfaction
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = make_trading_pair_proposal_f2f(&mut test_runner,
                                             &nfgid_user1,
                                             &component,
                                             &user1_account,
                                             &nft_resaddr,
                                             1,
                                             RADIX_TOKEN,
                                             "10".into(),
                                             side1_resaddr,
                                             "3".into(),
                                             RADIX_TOKEN,
                                             "0.5".into()); // only 0.5 actually needed
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.5");

    // The generated UUID for our trade proposal
    let uuid = receipt.output::<(u128, Vec<Bucket>)>(7).0;

    assert_eq!(pre_xrd_balance - dec!("10") - dec!("0.5"),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 10 and fee");



    // user2 partially accepts the proposal

    // We deliberately provoke a divide by three just to make things
    // interesting
    let pre_tok_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = accept_proposal_f(&mut test_runner,
                                  &user2_nfgid,
                                  &component,
                                  &user2_account,
                                  uuid,
                                  true,
                                  side1_resaddr,
                                  "1".into(),
                                  "0.001".into(),
                                  RADIX_TOKEN,
                                  "0.75".into());
    receipt.expect_commit_success();

    assert_dec_approx(pre_xrd_balance + dec!("10") / 3
                      - dec!("0.75"),
                      *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
                      dec!("0.000000001"),
                      "XRD balance should be up by 3.333... and down by fee");
    assert_dec_approx(pre_tok_balance - dec!("1") - dec!("0.001"),
                      *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "TOK balance should be down by 1 and fee");
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.75");
    *sum_of_all_fees.get_mut(&side1_resaddr).unwrap() += dec!("0.001");


    // Create user3
    let (pubk_user3, _privk_user3, account_user3) = test_runner.new_allocated_account();
    let nfgid_user3 = NonFungibleGlobalId::from_public_key(&pubk_user3);

    // Give user3 some TOK
    give_tokens(
        &mut test_runner,
        &user2_account,
        &user2_nfgid,
        &account_user3,
        &side1_resaddr,
        40);



    // user3 accepts the remainder of the proposal
    let pre_tok_balance =
        test_runner.get_component_resources(account_user3).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(account_user3).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = accept_proposal_f(&mut test_runner,
                                  &nfgid_user3,
                                  &component,
                                  &account_user3,
                                  uuid,
                                  false,
                                  side1_resaddr,
                                  "2.1".into(),
                                  "0.0021".into(),
                                  RADIX_TOKEN,
                                  "0.75".into());
    receipt.expect_commit_success();

    assert_dec_approx(pre_xrd_balance + dec!("10") * 2 / 3
                      - dec!("0.75"),
                      *test_runner.get_component_resources(account_user3).get(&RADIX_TOKEN).unwrap(),
                      dec!("0.000000001"),
                      "XRD balance should be up by 6.666... and down by fee");
    assert_dec_approx(pre_tok_balance - dec!("2") - dec!("0.002"),
                      *test_runner.get_component_resources(account_user3).get(&side1_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "TOK balance should be down by 2 and fee");
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("0.75");
    *sum_of_all_fees.get_mut(&side1_resaddr).unwrap() += dec!("0.002");



    // user1 collects the payment
    let pre_tok_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &nft_resaddr,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_tok_balance + dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "TOK balance should be up by 3");



    // Kaupa owner collects their fees
    let pre_tok_balance =
        test_runner.get_component_resources(owner_account).get(&side1_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert_dec_approx(pre_tok_balance + *sum_of_all_fees.get(&side1_resaddr).unwrap(),
                      *test_runner.get_component_resources(owner_account).get(&side1_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "Kaupa owner should have their TOK fees");

    
}


/// Sets up a non-fungible/non-fungible trading pair and puts it
/// through its paces.
#[test]
//#[ignore]
fn test_nf2nf_trading_pair() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let nfgid_user1 = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Create the side1 NFT resource
    let side1_resaddr = create_nft_resource(
        &mut test_runner,
        &nfgid_user1,
        &user1_account,
        0,
        100);
    
    // Create the side2 NFT resource
    let side2_resaddr = create_nft_resource(
        &mut test_runner,
        &user2_nfgid,
        &user2_account,
        100,
        100);

    // Create the fee NFT resource
    let feenft_resaddr = create_nft_resource(
        &mut test_runner,
        &owner_nfgid,
        &owner_account,
        1000,
        100);

    give_tokens(
        &mut test_runner,
        &owner_account,
        &owner_nfgid,
        &user1_account,
        &feenft_resaddr,
        40);

    assert_eq!(dec!("40"),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "user1 fee NFT balance should start as expected");

    give_tokens(
        &mut test_runner,
        &owner_account,
        &owner_nfgid,
        &user2_account,
        &feenft_resaddr,
        40);
    
    assert_eq!(dec!("40"),
               *test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap(),
               "user2 fee NFT balance should start as expected");

    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);

    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());
    

    // Call the `instantiate_kaupa` function with a non-fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([
                                         (RADIX_TOKEN, AskingType::Fungible("7".into())),
                                         (feenft_resaddr,
                                          AskingType::NonFungible(None, Some(1))),
                                     ])),
                                 per_tx_taker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([
                                         (RADIX_TOKEN, AskingType::Fungible("19".into())),
                                         (feenft_resaddr,
                                          AskingType::NonFungible(None, Some(2))),
                                     ])),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (side1_resaddr.clone(),
                                          (RADIX_TOKEN.clone(), dec!("10"))),
                                         (side2_resaddr.clone(),
                                          (RADIX_TOKEN.clone(), dec!("4"))),
                                     ])),
                                 per_payment_bps_fee: None,
                             }),
                             Some(HashSet::from([side1_resaddr])),
                             Some(HashSet::from([side2_resaddr])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    let mut sum_of_all_fees = HashMap::<ResourceAddress, Decimal>::from([
        (RADIX_TOKEN, Decimal::ZERO),
        (feenft_resaddr, Decimal::ZERO)
    ]);
    
    // Add a trade proposal owned by NFT 1
    // Offering 2 side1 (nflid 1 and 2) in exchange for 1 side2
    let pre_xrd_balance = test_runner.get_component_resources(user1_account).
        get(&RADIX_TOKEN).unwrap().clone();
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();
    let pre_feenft_balance = test_runner.get_component_resources(user1_account).
        get(&feenft_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &nfgid_user1,
                                                &component,
                                                &user1_account,
                                                &user1_nftres,
                                                1,
                                                side1_resaddr,
                                                [ 1, 2 ].into(),
                                                side2_resaddr,
                                                Some(1),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_xrd_balance - dec!("7"),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "xrd balance should be down by 7");
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_feenft_balance - dec!("1"),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "side1 fee balance should be down by 1");


    // user2 accepts the proposal
    assert!(test_runner.get_component_resources(user2_account).get(&side1_resaddr).is_none(),
            "user2 should not start with side1 tokens");
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &user2_nfgid,
                                     &component,
                                     &user2_account,
                                     uuid,
                                     false,
                                     side2_resaddr,
                                     [101].into(),
                                     RADIX_TOKEN,
                                     "24".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("24") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(dec!("2"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 2");
    assert_eq!(pre_side2_balance - dec!("1"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 1");
    assert_eq!(pre_xrd_balance - dec!("24") - dec!(19),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 24");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");

    
    // user1 collects the payment
    assert!(test_runner.get_component_resources(user1_account).get(&side2_resaddr).is_none(),
            "user1 should start with no side2 tokens");

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &user1_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(dec!("1"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 1");



    // Kaupa owner collects their fees
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert_eq!(pre_feenft_balance + *sum_of_all_fees.get(&feenft_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap(),
               "Kaupa owner should have their fee-NFT fees");

    // We just collected the fees, so start from zero again
    let mut sum_of_all_fees = HashMap::<ResourceAddress, Decimal>::from([
        (RADIX_TOKEN, Decimal::ZERO),
        (feenft_resaddr, Decimal::ZERO)
    ]);



    // Let's make a few proposals of various types


    // user1 adds a trade proposal owned by NFT 1
    // Offering 4 side1 (nflid 3 4 5 6) in exchange for 3 side2
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &nfgid_user1,
                                                &component,
                                                &user1_account,
                                                &user1_nftres,
                                                1,
                                                side1_resaddr,
                                                [ 3, 4, 5, 6 ].into(),
                                                side2_resaddr,
                                                Some(3),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side1_balance - dec!("4"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 4");
    


    // user1 adds a trade proposal owned by their NFT 2
    // Offering 3 side1 (nflid 7 8 9) in exchange for 2 side2
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &nfgid_user1,
                                                &component,
                                                &user1_account,
                                                &user1_nftres,
                                                2,
                                                side1_resaddr,
                                                [ 7, 8, 9 ].into(),
                                                side2_resaddr,
                                                Some(2),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side1_balance - dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 3");


    // user1 adds an outrageous trade proposal owned by their NFT 2
    // Offering 1 side1 (nflid 10) in exchange for 20 side2
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &nfgid_user1,
                                                &component,
                                                &user1_account,
                                                &user1_nftres,
                                                2,
                                                side1_resaddr,
                                                [ 10 ].into(),
                                                side2_resaddr,
                                                Some(20),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid3 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side1_balance - dec!("1"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 1");


    
    // user2 adds a trade proposal owned by their NFT 1
    // Offering 3 side2 (nflid 102 103 104) in exchange for 1 side1
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 102, 103, 104 ].into(),
                                                side1_resaddr,
                                                Some(1),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid4 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("3"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 3");


    
    // user2 adds a trade proposal owned by their NFT 1

    // Offering 3 side2 (nflid 105 106 107) in exchange for 2 random
    // side1 PLUS NFT 11 from side1
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 105, 106, 107 ].into(),
                                                side1_resaddr,
                                                Some(2),
                                                Some([11].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid5 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("3"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 3");


    // user2 tries to accept the unreasonable trade in uuid3
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &user2_nfgid,
                                     &component,
                                     &user2_account,
                                     uuid3,
                                     false,
                                     side2_resaddr,
                                     [113].into(),
                                     RADIX_TOKEN,
                                     "200".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    // which should fail
    receipt.expect_commit_failure();



    // user2 accepts the uuid2 proposal, selling NFTs 115 114
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &user2_nfgid,
                                     &component,
                                     &user2_account,
                                     uuid2,
                                     false,
                                     side2_resaddr,
                                     [115, 114].into(),
                                     RADIX_TOKEN,
                                     "38".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("38") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance + dec!("3"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 3");
    assert_eq!(pre_side2_balance - dec!("2"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 2");
    assert_eq!(pre_xrd_balance - dec!("38") - dec!(19),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 38 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");



    // user2 tries to accept the uuid2 proposal again, this time
    // selling NFTs 113 112 but it fails because the proposal is gone
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &user2_nfgid,
                                     &component,
                                     &user2_account,
                                     uuid2,
                                     false,
                                     side2_resaddr,
                                     [113, 112].into(),
                                     RADIX_TOKEN,
                                     "38".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_failure();

    

    // user2 accepts the uuid1 proposal, selling NFTs 113 112 111
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &user2_nfgid,
                                     &component,
                                     &user2_account,
                                     uuid1,
                                     false,
                                     side2_resaddr,
                                     [112, 111, 113].into(),
                                     RADIX_TOKEN,
                                     "52".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("52") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance + dec!("4"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 4");
    assert_eq!(pre_side2_balance - dec!("3"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 3");
    assert_eq!(pre_xrd_balance - dec!("52") - dec!(19),
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 52 and fees");
    assert_eq!(pre_feenft_balance - dec!("2"),
               *test_runner.get_component_resources(user2_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");

    
    // user1 accepts the uuid4 proposal, selling NFT 12
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid4,
                                     false,
                                     side1_resaddr,
                                     [12].into(),
                                     RADIX_TOKEN,
                                     "22".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("22") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("1"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 1");
    assert_eq!(pre_side2_balance + dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 3");
    assert_eq!(pre_xrd_balance - dec!("22") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 22 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    // user1 tries to accept the uuid5 proposal, selling NFTs 10 13 14
    // but the maker specifically asked for NFT 11 so this fails
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid5,
                                     false,
                                     side1_resaddr,
                                     [10, 13, 14].into(),
                                     RADIX_TOKEN,
                                     "42".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_failure();

    
    // user1 accepts the uuid5 proposal, selling NFT 11 and the two
    // randos 13 14
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid5,
                                     false,
                                     side1_resaddr,
                                     [11, 13, 14].into(),
                                     RADIX_TOKEN,
                                     "42".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("42") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 3");
    assert_eq!(pre_side2_balance + dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 3");
    assert_eq!(pre_xrd_balance - dec!("42") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 42 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    
    // user1 collects payments from their ownership NFTs (1 2) separately
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &user1_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_side2_balance + dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 3");



    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &nfgid_user1,
                                &component,
                                &user1_account,
                                false,
                                &user1_nftres,
                                2,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_side2_balance + dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 2");


    
    // user2 collects payments from their NFT 1
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &user2_nfgid,
                                &component,
                                &user2_account,
                                false,
                                &user2_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_side1_balance + dec!("4"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 4");



    // Kaupa owner collects their fees
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert_eq!(pre_feenft_balance + *sum_of_all_fees.get(&feenft_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap(),
               "Kaupa owner should have their fee-NFT fees");


    // We just collected the fees, so start from zero again
    let mut sum_of_all_fees = HashMap::<ResourceAddress, Decimal>::from([
        (RADIX_TOKEN, Decimal::ZERO),
        (feenft_resaddr, Decimal::ZERO)
    ]);


    // We will now test partial satisfaction


    // user2 adds a trade proposal owned by their NFT 1

    // Offering 5 side2 (nflid 120 121 122 123 124) in exchange for 3
    // random side1 (this cannot result in any tradable ratio less
    // than 1.0)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 120, 121, 122, 123, 124 ].into(),
                                                side1_resaddr,
                                                Some(3),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("5"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 5");


    // user1 tries to partially accept the uuid1 proposal, but this is
    // never going to work
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid1,
                                     true,
                                     side1_resaddr,
                                     [11, 13].into(),
                                     RADIX_TOKEN,
                                     "50".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_failure();
    


    // user2 adds a trade proposal owned by their NFT 1

    // Offering 10 side2 (nflid 130 131 132 133 134 135 136 137 138
    // 139) in exchange for 4 random side1 (this has the potential for
    // partial trade)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 130, 131, 132, 133, 134,
                                                  135, 136, 137, 138, 139].into(),
                                                side1_resaddr,
                                                Some(4),
                                                None,
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("10"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 10");
    



    // user1 partially accepts the uuid5 proposal, trying to sell NFTs
    // 20 21 22 - but only two of them will actually be taken.
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     [20, 21, 22].into(),
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("40") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    

    // user1 fully accepts the remainder of the uuid2 proposal, 
    // selling NFTs 23 24
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     [23, 24].into(),
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("40") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    
    // Now test with only named NFTs

    
    // user2 adds a trade proposal owned by their NFT 1

    // Offering 5 side2 (nflid 140 141 142 143 144) in exchange for 3
    // named side1 (this cannot result in any tradable ratio less
    // than 1.0)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 140, 141, 142, 143, 144 ].into(),
                                                side1_resaddr,
                                                None,
                                                Some([30, 31, 32].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("5"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 5");


    // user1 tries to partially accept the uuid1 proposal, but this is
    // never going to work
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid1,
                                     true,
                                     side1_resaddr,
                                     [31, 32].into(),
                                     RADIX_TOKEN,
                                     "50".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_failure();
    

    
    // A full buyout by user1 should however work
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid1,
                                     true,
                                     side1_resaddr,
                                     [ 30, 31, 32 ].into(),
                                     RADIX_TOKEN,
                                     "50".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 50 + 19;
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += 2;

    assert_eq!(pre_side1_balance - dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 3");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("50") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");

    
    // user2 adds a trade proposal owned by their NFT 1

    // Offering 10 side2 (nflid 150 151 152 153 154 155 156 157 158
    // 159) in exchange for 4 named side1 33 34 35 36 (this has the
    // potential for partial trade)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 150, 151, 152, 153, 154,
                                                  155, 156, 157, 158, 159].into(),
                                                side1_resaddr,
                                                None,
                                                Some([33, 34, 35, 36].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("10"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 10");
    



    // user1 partially accepts the uuid2 proposal, trying to sell NFTs
    // 33 34 35 - but only two of them will actually be taken.
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     [33, 34, 35].into(),
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 40 + 19;
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += 2;
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    

    // user1 fully accepts the remainder of the uuid2 proposal, 
    // selling two of NFTs 33 34 35 36
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    // We need to find out which nflids are still left to use for
    // payment
    let payment: Vec<u64> = filter_by_owned_nflids(&mut test_runner,
                                                   user1_account,
                                                   side1_resaddr,
                                                   [33, 34, 35, 36].into());
    
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     payment,
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());

    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("40") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    
    // Now Test partial buy with mixed named/random NFTs

    
    // user2 adds a trade proposal owned by their NFT 1

    // Offering 5 side2 (nflid 160 161 162 163 164) in exchange for 1
    // unnamed and 2 named side1 (40 41) (this cannot result in any tradable
    // ratio less than 1.0)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 160, 161, 162, 163, 164 ].into(),
                                                side1_resaddr,
                                                Some(1),
                                                Some([40, 41].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("5"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 5");


    // user1 tries to partially accept the uuid1 proposal, but this is
    // never going to work
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid1,
                                     true,
                                     side1_resaddr,
                                     [40, 41].into(),
                                     RADIX_TOKEN,
                                     "50".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_failure();
    

    
    // A full buyout by user1 should however work
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();
    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid1,
                                     true,
                                     side1_resaddr,
                                     [ 40, 41, 42 ].into(),
                                     RADIX_TOKEN,
                                     "50".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 50 + 19;
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += 2;

    assert_eq!(pre_side1_balance - dec!("3"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 3");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("50") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");



    // user2 adds a trade proposal owned by their NFT 1

    // Offering 10 side2 (nflid 170 171 172 173 174 175 176 177 178
    // 179) in exchange for 3 named side1 45 46 47 and 1 unnamed (this
    // has the potential for partial trade)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 170, 171, 172, 173, 174,
                                                  175, 176, 177, 178, 179].into(),
                                                side1_resaddr,
                                                Some(1),
                                                Some([45, 46, 47].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("10"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 10");
    



    // user1 partially accepts the uuid2 proposal, trying to sell NFTs
    // 47 48 49 - but only two of them will actually be taken.
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     [47, 48, 49].into(),
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("40") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");


    

    // user1 fully accepts the remainder of the uuid2 proposal, 
    // selling NFTs 45 46
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid2,
                                     true,
                                     side1_resaddr,
                                     [45, 46].into(),
                                     RADIX_TOKEN,
                                     "40".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());

    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("40") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_side2_balance + dec!("5"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 5");
    assert_eq!(pre_xrd_balance - dec!("40") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 40 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");




    // user2 adds a bigger trade proposal owned by their NFT 1

    // Offering 5 side2 (nflid 180 181 182 183 184 179) in exchange
    // for 13 named side1 50 51 52 53 54 55 56 57 58 59 60 61 62 and 7
    // unnamed (this has the potential for partial trade)
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2nf(&mut test_runner,
                                                &user2_nfgid,
                                                &component,
                                                &user2_account,
                                                &user2_nftres,
                                                1,
                                                side2_resaddr,
                                                [ 180, 181, 182, 183, 184 ].into(),
                                                side1_resaddr,
                                                Some(7),
                                                Some([50, 51, 52, 53, 54,
                                                      55, 56, 57, 58, 59,
                                                      60, 61, 62].into()),
                                                RADIX_TOKEN,
                                                7,
                                                feenft_resaddr,
                                                1);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(1);

    // The generated UUID for our trade proposal
    let uuid3 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_side2_balance - dec!("5"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 5");



    // user1 partially accepts the uuid3 proposal, trying to sell NFTs
    // 71 53 54 57 61 - but only four of them will actually be taken
    // and the rando should not be taken.
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid3,
                                     true,
                                     side1_resaddr,
                                     [71, 53, 54, 57, 61].into(),
                                     RADIX_TOKEN,
                                     "44".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("44") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("4"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 4");
    assert_eq!(pre_side2_balance + dec!("1"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 1");
    assert_eq!(pre_xrd_balance - dec!("44") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 44 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");
    assert!(component_has_nflids(&mut test_runner,
                                 user1_account,
                                 side1_resaddr,
                                 [ 71 ].into()),
            "user1 should still have nflid 71");



    // user1 partially accepts more of the uuid2 proposal, trying to
    // sell NFTs 71 72 73 74 75 76 77 51 - 7 will be taken as
    // randos and 51 as named leaving only named to take.
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf(&mut test_runner,
                                     &nfgid_user1,
                                     &component,
                                     &user1_account,
                                     uuid3,
                                     true,
                                     side1_resaddr,
                                     [71, 72, 73, 74, 75, 76, 77, 51].into(),
                                     RADIX_TOKEN,
                                     "88".into(),
                                     RADIX_TOKEN,
                                     19.into(),
                                     feenft_resaddr,
                                     2.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!("88") + dec!(19);
    *sum_of_all_fees.get_mut(&feenft_resaddr).unwrap() += dec!(2);
    
    assert_eq!(pre_side1_balance - dec!("8"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 8");
    assert_eq!(pre_side2_balance + dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 2");
    assert_eq!(pre_xrd_balance - dec!("88") - dec!(19),
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 88 and fees");
    assert_eq!(pre_feenft_balance - dec!(2),
               *test_runner.get_component_resources(user1_account).get(&feenft_resaddr).unwrap(),
               "fee NFT balance should be down by 2");

    



    // Kaupa owner collects their fees
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_feenft_balance =
        test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert_eq!(pre_feenft_balance + *sum_of_all_fees.get(&feenft_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&feenft_resaddr).unwrap(),
               "Kaupa owner should have their fee-NFT fees");
}




/// Sets up a non-fungible/fungible trading pair and puts it through
/// its paces. Note that "buying" in this pair is, in our terminology,
/// using an nf2f pair, whereas selling into it is using an f2nf pair.
#[test]
fn test_nf2f_trading_pair() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Create the side1 NFT resource, with nflids 1001-1999
    let side1_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        1000,
        999);

    // Create the side2 fungible resource
    let side2_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user1_account);

    // Give some side2 to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &side2_resaddr,
                100000);
    

    // Create the fee resource
    let fee_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Give some fee tokens to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee_resaddr,
                100000);


    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());
    

    // Call the `instantiate_kaupa` function with a
    // non-fungible/fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([
                                         (RADIX_TOKEN, AskingType::Fungible(7.into())),
                                         (fee_resaddr, AskingType::Fungible(2.into())),
                                     ])),
                                 per_tx_taker_fixed_fee: Some(
                                     HashMap::<ResourceAddress, AskingType>::from([
                                         (RADIX_TOKEN, AskingType::Fungible(19.into())),
                                         (fee_resaddr, AskingType::Fungible(3.into())),
                                     ])),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (side1_resaddr.clone(),
                                          (RADIX_TOKEN.clone(), 11.into())),
                                     ])),
                                 per_payment_bps_fee: Some(3.into()),
                             }),
                             Some::<HashSet<ResourceAddress>>([side1_resaddr].into()),
                             Some::<HashSet<ResourceAddress>>([side2_resaddr].into()),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    let mut sum_of_all_fees = HashMap::<ResourceAddress, Decimal>::from([
        (RADIX_TOKEN, Decimal::ZERO),
        (side2_resaddr, Decimal::ZERO),
        (fee_resaddr, Decimal::ZERO)
    ]);



    // user1 adds a trade proposal owned by NFT 1
    // Offering 2 side1 (nflid 1000 and 1001) in exchange for 24.5 side2
    let pre_xrd_balance = test_runner.get_component_resources(user1_account).
        get(&RADIX_TOKEN).unwrap().clone();
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();
    let pre_fee_balance = test_runner.get_component_resources(user1_account).
        get(&fee_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2f(&mut test_runner,
                                                   &user1_nfgid,
                                                   &component,
                                                   &user1_account,
                                                   &user1_nftres,
                                                   1,
                                                   side1_resaddr,
                                                   [ 1001, 1002 ].into(),
                                                   side2_resaddr,
                                                   25.into(),
                                                   RADIX_TOKEN,
                                                   7,
                                                   fee_resaddr,
                                                   2);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += dec!(2);

    // The generated UUID for our trade proposal
    let uuid1 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_xrd_balance - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "xrd balance should be down by 7");
    assert_eq!(pre_side1_balance - dec!("2"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 2");
    assert_eq!(pre_fee_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap(),
               "side1 fee balance should be down by 2");



    // user2 tries to accept the proposal but underpays so it fails
    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid1,
                                       false,
                                       side2_resaddr,
                                       24.into(), // price is actually 25
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       41.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();


    // user2 tries to accept the proposal but attaches insufficient
    // fees (in various ways) so it fails

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid1,
                                       false,
                                       side2_resaddr,
                                       25.into(),
                                       dec!("0.0074"), // fee is actually 0.0075
                                       RADIX_TOKEN,
                                       41.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid1,
                                       false,
                                       side2_resaddr,
                                       25.into(),
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       40.into(), // fee is actually 41
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid1,
                                       false,
                                       side2_resaddr,
                                       25.into(),
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       41.into(),
                                       fee_resaddr,
                                       2.into()); // fee is actually 3
    receipt.expect_commit_failure();
    
    
    // user2 successfully accepts the full proposal
    assert!(test_runner.get_component_resources(user2_account).get(&side1_resaddr).is_none(),
            "user2 should not start with side1 tokens");
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid1,
                                       false,
                                       side2_resaddr,
                                       25.into(),
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       41.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 41;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    *sum_of_all_fees.get_mut(&side2_resaddr).unwrap() += dec!("0.0075");
    
    assert_eq!(dec!("2"),
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 2");
    assert_eq!(pre_side2_balance - 25 - dec!("0.0075"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be down by 25 and fee");
    assert_eq!(pre_xrd_balance - 41,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 41");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");




    // user1 adds a trade proposal owned by NFT 1
    // Offering 13 side1 (nflid 1011 - 1023) in exchange for 130 side2
    let pre_xrd_balance = test_runner.get_component_resources(user1_account).
        get(&RADIX_TOKEN).unwrap().clone();
    let pre_side1_balance = test_runner.get_component_resources(user1_account).
        get(&side1_resaddr).unwrap().clone();
    let pre_fee_balance = test_runner.get_component_resources(user1_account).
        get(&fee_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_nf2f(&mut test_runner,
                                                   &user1_nfgid,
                                                   &component,
                                                   &user1_account,
                                                   &user1_nftres,
                                                   1,
                                                   side1_resaddr,
                                                   ( 1011..1024 ).collect(),
                                                   side2_resaddr,
                                                   130.into(),
                                                   RADIX_TOKEN,
                                                   7,
                                                   fee_resaddr,
                                                   2);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += dec!(2);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_xrd_balance - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "xrd balance should be down by 7");
    assert_eq!(pre_side1_balance - dec!("13"),
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 13");
    assert_eq!(pre_fee_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap(),
               "side1 fee balance should be down by 2");



    // user2 tries to partial buy some NFTs but sends not enough funds
    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid2,
                                       true,
                                       side2_resaddr,
                                       9.into(), // minimum 10 to get an NFT
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();


    // user2 partially accepts the proposal
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid2,
                                       true,
                                       side2_resaddr,
                                       10.into(),
                                       dec!("0.003"),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 30;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    *sum_of_all_fees.get_mut(&side2_resaddr).unwrap() += dec!("0.003");
    
    assert_eq!(pre_side1_balance + 1,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 1");
    assert_dec_approx(pre_side2_balance - 10 - dec!("0.003"),
                      *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "side2 balance should be down by 10 and fee");
    assert_eq!(pre_xrd_balance - 30,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 30");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");

    

    // user2 further partially accepts the proposal, slightly
    // overpaying because they think they will get 2.5 NFTs but of
    // course can only get 2.
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid2,
                                       true,
                                       side2_resaddr,
                                       25.into(), // only 20 gets taken
                                       dec!("0.0075"), // so only 0.006 is needed
                                       RADIX_TOKEN,
                                       "46.5".into(),  // and only 41 is needed here
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 41;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    *sum_of_all_fees.get_mut(&side2_resaddr).unwrap() += dec!("0.006");
    
    assert_eq!(pre_side1_balance + 2,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 2");
    assert_dec_approx(pre_side2_balance - 20 - dec!("0.006"),
                      *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "side2 balance should be down by 20 and fee");
    assert_eq!(pre_xrd_balance - 41,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 41");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");

    
    // user2 finally fully accepts the remainder of the proposal
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid2,
                                       true,
                                       side2_resaddr,
                                       100.into(),
                                       dec!("0.03"),
                                       RADIX_TOKEN,
                                       129.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 129;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    *sum_of_all_fees.get_mut(&side2_resaddr).unwrap() += dec!("0.03");
    
    assert_eq!(pre_side1_balance + 10,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be up by 10");
    assert_dec_approx(pre_side2_balance - 100 - dec!("0.03"),
                      *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
                      dec!("0.000000001"),
                      "side2 balance should be down by 100 and fee");
    assert_eq!(pre_xrd_balance - 129,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 129");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");


    // user2 tries one last buy but the proposal is no more
    let receipt = accept_proposal_nf2f(&mut test_runner,
                                       &user2_nfgid,
                                       &component,
                                       &user2_account,
                                       uuid2,
                                       true,
                                       side2_resaddr,
                                       10.into(),
                                       dec!("0.0075"),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();


    
    // user2 adds a trade proposal owned by NFT 2
    // Offering 25.5 side2 in return for 2 unnamed side1 NFTs
    let pre_xrd_balance = test_runner.get_component_resources(user2_account).
        get(&RADIX_TOKEN).unwrap().clone();
    let pre_side2_balance = test_runner.get_component_resources(user2_account).
        get(&side2_resaddr).unwrap().clone();
    let pre_fee_balance = test_runner.get_component_resources(user2_account).
        get(&fee_resaddr).unwrap().clone();

    let receipt = make_trading_pair_proposal_f2nf(&mut test_runner,
                                                  &user2_nfgid,
                                                  &component,
                                                  &user2_account,
                                                  &user2_nftres,
                                                  2,
                                                  side2_resaddr,
                                                  dec!("25.5"),
                                                  side1_resaddr,
                                                  [].into(),
                                                  2,
                                                  RADIX_TOKEN,
                                                  7,
                                                  fee_resaddr,
                                                  2);
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += dec!(7);
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += dec!(2);

    // The generated UUID for our trade proposal
    let uuid2 = receipt.output::<(u128, Vec<Bucket>)>(9).0;

    assert_eq!(pre_xrd_balance - 7,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "xrd balance should be down by 7");
    assert_eq!(pre_side2_balance - dec!("25.5"),
               *test_runner.get_component_resources(user2_account).get(&side2_resaddr).unwrap(),
               "side1 balance should be down by 25.5");
    assert_eq!(pre_fee_balance - 2,
               *test_runner.get_component_resources(user2_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 2");




    // user1 buys half the proposal
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_f2nf(&mut test_runner,
                                       &user1_nfgid,
                                       &component,
                                       &user1_account,
                                       uuid2,
                                       true,
                                       side1_resaddr,
                                       [ 1030 ].into(),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 30;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    
    assert_eq!(pre_side1_balance - 1,
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 1");
    assert_eq!(pre_side2_balance + dec!("12.75"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 12.75");
    assert_eq!(pre_xrd_balance - 30,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 20");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");


    // user1 buys the rest of the proposal
    let pre_side1_balance =
        test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap().clone();
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee_balance =
        test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap().clone();

    let receipt = accept_proposal_f2nf(&mut test_runner,
                                       &user1_nfgid,
                                       &component,
                                       &user1_account,
                                       uuid2,
                                       true,
                                       side1_resaddr,
                                       [ 1031 ].into(),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 30;
    *sum_of_all_fees.get_mut(&fee_resaddr).unwrap() += 3;
    
    assert_eq!(pre_side1_balance - 1,
               *test_runner.get_component_resources(user1_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be down by 1");
    assert_eq!(pre_side2_balance + dec!("12.75"),
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 12.75");
    assert_eq!(pre_xrd_balance - 30,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 20");
    assert_eq!(pre_fee_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&fee_resaddr).unwrap(),
               "fee balance should be down by 3");


    // The proposal is now gone so this fails for user1
    let receipt = accept_proposal_f2nf(&mut test_runner,
                                       &user1_nfgid,
                                       &component,
                                       &user1_account,
                                       uuid2,
                                       true,
                                       side1_resaddr,
                                       [ 1032 ].into(),
                                       RADIX_TOKEN,
                                       30.into(),
                                       fee_resaddr,
                                       3.into());
    receipt.expect_commit_failure();

    

    // user1 collects payments from their NFT 1
    let pre_side2_balance =
        test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &user1_nfgid,
                                &component,
                                &user1_account,
                                false,
                                &user1_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_side2_balance + 25 + 130,
               *test_runner.get_component_resources(user1_account).get(&side2_resaddr).unwrap(),
               "side2 balance should be up by 155");


    // user2 tries to collect payments from their NFT 1, but that's
    // the wrong NFT
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();
    let receipt = collect_funds(&mut test_runner,
                                &user2_nfgid,
                                &component,
                                &user2_account,
                                false,
                                &user2_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();
    assert_eq!(pre_side1_balance,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side1 balance should be unchanged");


    // user2 collects payments from their NFT 2
    let pre_side1_balance =
        test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap().clone();

    let receipt = collect_funds(&mut test_runner,
                                &user2_nfgid,
                                &component,
                                &user2_account,
                                false,
                                &user2_nftres,
                                2,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_side1_balance + 2,
               *test_runner.get_component_resources(user2_account).get(&side1_resaddr).unwrap(),
               "side2 balance should be up by 2");
    assert!(component_has_nflids(&mut test_runner,
                                 user2_account, side1_resaddr, [1030, 1031].into()),
            "user2 should now have nflids 1030, 1031");



    // Kaupa owner collects their XRD fees
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(owner_account).get(&side2_resaddr).is_none(),
            "owner should not start with side2 tokens");
    assert!(test_runner.get_component_resources(owner_account).get(&fee_resaddr).is_none(),
            "owner should not start with fee tokens");

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                Some(RADIX_TOKEN),);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance + *sum_of_all_fees.get(&RADIX_TOKEN).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should have their XRD fees");
    assert!(test_runner.get_component_resources(owner_account).get(&side2_resaddr).is_none(),
            "owner should still not have side2 tokens");
    assert!(test_runner.get_component_resources(owner_account).get(&fee_resaddr).is_none(),
            "owner should still not have fee tokens");


    // Kaupa owner collects their remaining fees
    let pre_xrd_balance =
        test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(owner_account).get(&side2_resaddr).is_none(),
            "owner should not start with side2 tokens");
    assert!(test_runner.get_component_resources(owner_account).get(&fee_resaddr).is_none(),
            "owner should not start with fee tokens");

    let receipt = collect_funds(&mut test_runner,
                                &owner_nfgid,
                                &component,
                                &owner_account,
                                true,
                                &kaupa_admin_badge.resource_address(),
                                1,
                                None,);
    receipt.expect_commit_success();
    
    assert_eq!(pre_xrd_balance,
               *test_runner.get_component_resources(owner_account).get(&RADIX_TOKEN).unwrap(),
               "Kaupa owner should be unchanged");
    assert_eq!(*sum_of_all_fees.get(&side2_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&side2_resaddr).unwrap(),
               "Kaupa owner should have their side2 fees");
    assert_eq!(*sum_of_all_fees.get(&fee_resaddr).unwrap(),
               *test_runner.get_component_resources(owner_account).get(&fee_resaddr).unwrap(),
               "Kaupa owner should have their fee-resource fees");
}


/// Sets up a Kaupa that allows any bag of tokens to be trade for any
/// bag of tokens, and runs make/accept tests on it.
#[test]
fn test_otc_any2any() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Create user3
    let (user3_pubk, _user3_privk, user3_account) = test_runner.new_allocated_account();
    let user3_nfgid = NonFungibleGlobalId::from_public_key(&user3_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Create the n1 NFT resource, with nflids 1000-1999
    let n1_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        999,
        1000);

    // Create the n2 NFT resource, with nflids 2000-2999
    let n2_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        1999,
        1000);

    // Create the n3 NFT resource, with nflids 3000-3999
    let n3_resaddr = create_nft_resource(
        &mut test_runner,
        &user2_nfgid,
        &user2_account,
        2999,
        1000);

    // Create the n4 NFT resource, with nflids 4000-4999
    let n4_resaddr = create_nft_resource(
        &mut test_runner,
        &user2_nfgid,
        &user2_account,
        3999,
        1000);

    // Create the f1 fungible resource
    let f1_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user1_account);

    // Create the f2 fungible resource
    let f2_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user1_account);

    // Create the f3 fungible resource
    let f3_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user2_account);

    // Create the f4 fungible resource
    let f4_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user2_account);

    // user3 is going to need some f4
    give_tokens(&mut test_runner,
                &user2_account,
                &user2_nfgid,
                &user3_account,
                &f4_resaddr,
                100000);

    // Create the fee1 resource
    let fee1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Give some fee1 tokens to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee1_resaddr,
                100000);


    // Create the fee2 resource
    let fee2_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Give some fee2 tokens to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee2_resaddr,
                100000);

    // Give some fee2 tokens to user3
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user3_account,
                &fee2_resaddr,
                100000);

    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let user3_nftres =
        test_runner.create_non_fungible_resource(user3_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());


    let maker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(7.into())),
            (fee1_resaddr, AskingType::Fungible(2.into())),
        ]);
    let taker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(19.into())),
            (fee2_resaddr, AskingType::Fungible(3.into())),
        ]);

    // Call the `instantiate_kaupa` function with a
    // non-fungible/fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(maker_fixed_fee.clone()),
                                 per_tx_taker_fixed_fee: Some(taker_fixed_fee.clone()),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (n1_resaddr.clone(),
                                          (RADIX_TOKEN.clone(), 11.into())),
                                         (n2_resaddr.clone(),
                                          (f3_resaddr.clone(), 13.into())),
                                         (n3_resaddr.clone(),
                                          (f1_resaddr.clone(), 17.into())),
                                     ])),
                                 per_payment_bps_fee: Some(2.into()),
                             }),
                             None::<String>,
                             None::<String>,
                             false,
                             false,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    let mut sum_of_all_fees = HashMap::<ResourceAddress, Decimal>::from([
        (RADIX_TOKEN, Decimal::ZERO),
        (f1_resaddr, Decimal::ZERO),
        (f2_resaddr, Decimal::ZERO),
        (f3_resaddr, Decimal::ZERO),
        (f4_resaddr, Decimal::ZERO),
        (fee1_resaddr, Decimal::ZERO),
        (fee2_resaddr, Decimal::ZERO)
    ]);


    // user1 creates a simple proposal owned by their NFT 1

    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let ( receipt, uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            false,
            [(RADIX_TOKEN, AskingType::Fungible(5.into()))].into(),
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 7;
    *sum_of_all_fees.get_mut(&fee1_resaddr).unwrap() += 2;
    
    assert_eq!(pre_xrd_balance - 5 - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user1 should be down 5 XRD and fee");
    assert_eq!(pre_fee1_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user1 should be down 2 fee1");




    // user2 accepts the proposal

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f4_balance =
        test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();


    let mut fees = taker_fixed_fee.clone();
    fees.insert(f4_resaddr, AskingType::Fungible(dec!("0.0042")));
    
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid1,
            None,
            false,
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            fees.into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 19;
    *sum_of_all_fees.get_mut(&fee2_resaddr).unwrap() += 3;

    // (You can wonder user2's wisdom in buying 5 XRD and paying 19
    // XRD in fees for the privilege. Maybe they're just feeling
    // nostalgic for the good old Ethereum days ...)
    assert_eq!(pre_xrd_balance + 5 - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 14 XRD");
    assert_eq!(pre_f4_balance - 21 - dec!("0.0042"),
               *test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap(),
               "user2 should be down 21 f4 and fee");
    assert_eq!(pre_fee2_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down 3 fee2");



    // user1 creates a many-to-many proposal owned by their NFT 1

    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_n2_balance =
        test_runner.get_component_resources(user1_account).get(&n2_resaddr).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let ( receipt, uuid3 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            false,
            [(RADIX_TOKEN, AskingType::Fungible(69.into())),
             (n1_resaddr, AskingType::NonFungible(Some(to_nflids([1003, 1004, 1005, 1006].into())),
                                                  None)),
             (n2_resaddr, AskingType::NonFungible(Some(to_nflids([2002, 2003, 2004, 2005, 2006].into())),
                                                  None)),
             (f1_resaddr, AskingType::Fungible(2500.into())),             
             (f2_resaddr, AskingType::Fungible(53.into())),
            ].into(),
            [(n3_resaddr, AskingType::NonFungible(Some(to_nflids([3000, 3001, 3002, 3003].into())),
                                                  Some(5))),
             (n4_resaddr, AskingType::NonFungible(Some(to_nflids([4010, 4011, 4012].into())),
                                                  Some(10))),
             (f3_resaddr, AskingType::Fungible(256.into())),             
             (f4_resaddr, AskingType::Fungible(39.into())),
            ].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 7;
    *sum_of_all_fees.get_mut(&fee1_resaddr).unwrap() += 2;
    
    assert_eq!(pre_xrd_balance - 69 - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user1 should be down 69 XRD and fee");
    assert_eq!(pre_n1_balance - 4,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "user1 should be down 4 n1");
    assert_eq!(pre_n2_balance - 5,
               *test_runner.get_component_resources(user1_account).get(&n2_resaddr).unwrap(),
               "user1 should be down 5 n2");
    assert_eq!(pre_f1_balance - 2500,
               *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
               "user1 should be down 2500 f1");
    assert_eq!(pre_f2_balance - 53,
               *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
               "user1 should be down 53 f2");
    assert_eq!(pre_fee1_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user1 should be down 2 fee1");




    // user1 creates a many-to-one proposal owned by their NFT 1

    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_n2_balance =
        test_runner.get_component_resources(user1_account).get(&n2_resaddr).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let ( receipt, uuid2 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            false,
            [(RADIX_TOKEN, AskingType::Fungible(69.into())),
             (n1_resaddr, AskingType::NonFungible(Some(to_nflids([1000, 1001, 1002].into())),
                                                  None)),
             (n2_resaddr, AskingType::NonFungible(Some(to_nflids([2000, 2001].into())),
                                                  None)),
             (f1_resaddr, AskingType::Fungible(2500.into())),             
             (f2_resaddr, AskingType::Fungible(53.into())),
            ].into(),
            [(f3_resaddr, AskingType::Fungible(12345.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 7;
    *sum_of_all_fees.get_mut(&fee1_resaddr).unwrap() += 2;
    
    assert_eq!(pre_xrd_balance - 69 - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user1 should be down 69 XRD and fee");
    assert_eq!(pre_n1_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "user1 should be down 3 n1");
    assert_eq!(pre_n2_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&n2_resaddr).unwrap(),
               "user1 should be down 2 n2");
    assert_eq!(pre_f1_balance - 2500,
               *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
               "user1 should be down 2500 f1");
    assert_eq!(pre_f2_balance - 53,
               *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
               "user1 should be down 53 f2");
    assert_eq!(pre_fee1_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user1 should be down 2 fee1");


    // User2 tries a partial accept but this is not allowed

    let mut fees = taker_fixed_fee.clone();
    fees.insert(RADIX_TOKEN, AskingType::Fungible((19 + 33).into()));
    fees.insert(f3_resaddr, AskingType::Fungible(dec!("2.469") + 26));

    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid2,
            None,
            true,
            [(f3_resaddr, AskingType::Fungible(12340.into()))].into(),
            fees.into_iter().collect());
    receipt.expect_commit_failure();
    
    
    // User2 tries to accept with the wrong currency

    let mut fees = taker_fixed_fee.clone();
    fees.insert(RADIX_TOKEN, AskingType::Fungible((19 + 33).into()));
    fees.insert(f3_resaddr, AskingType::Fungible(dec!("2.469") + 26));
    fees.insert(f4_resaddr, AskingType::Fungible(dec!("2.469") + 26));

    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid2,
            None,
            true,
            [(f4_resaddr, AskingType::Fungible(12345.into()))].into(),
            fees.into_iter().collect());
    receipt.expect_commit_failure();


    
    // user2 finally accepts the proposal

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(user2_account).get(&n1_resaddr).is_none(),
            "user2 should start with no n1");
    assert!(test_runner.get_component_resources(user2_account).get(&n2_resaddr).is_none(),
            "user2 should start with no n2");
    assert!(test_runner.get_component_resources(user2_account).get(&f1_resaddr).is_none(),
            "user2 should start with no f1");
    assert!(test_runner.get_component_resources(user2_account).get(&f2_resaddr).is_none(),
            "user2 should start with no f2");
    let pre_f3_balance =
        test_runner.get_component_resources(user2_account).get(&f3_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();


    let mut fees = taker_fixed_fee.clone();
    fees.insert(RADIX_TOKEN, AskingType::Fungible((19 + 33).into()));
    fees.insert(f3_resaddr, AskingType::Fungible(dec!("2.469") + 26));
    
    
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid2,
            None,
            false,
            [(f3_resaddr, AskingType::Fungible(12345.into()))].into(),
            fees.into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 19 + 33;
    *sum_of_all_fees.get_mut(&f3_resaddr).unwrap() += dec!("2.469") + 26;
    *sum_of_all_fees.get_mut(&fee2_resaddr).unwrap() += 3;

    assert_eq!(pre_xrd_balance + 69 - 19 - 33,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be up 17 XRD");
    assert_eq!(Decimal::from(3),
               *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
               "user2 should be up 3 n1");
    assert_eq!(Decimal::from(2),
               *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
               "user2 should be up 2 n2");
    assert_eq!(Decimal::from(2500),
               *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
               "user2 should be up 2500 f1");
    assert_eq!(Decimal::from(53),
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should be up 53 f2");
    assert_eq!(pre_f3_balance - 12345 - 26 - dec!("2.469"),
               *test_runner.get_component_resources(user2_account).get(&f3_resaddr).unwrap(),
               "user2 should be down 12345 f3 and fee");
    assert_eq!(pre_fee2_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down 3 fee2");


    // user2 accepts the uuid3 proposal

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap().clone();
    let pre_n2_balance =
        test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_f3_balance =
        test_runner.get_component_resources(user2_account).get(&f3_resaddr).unwrap().clone();
    let pre_f4_balance =
        test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();


    // Fees incurred here
    //
    // Taker per tx fixed fee:                         19 XRD + 3 FEE2
    // Per NFT transacted fees: (for n1) 4x 11 XRD =   44 XRD
    //                          (for n2) 5x 13 F3 =    65 F3
    //                          (for n3) 9x 17 F1 =   153 F1
    //                          (for n4) 13x 0
    // Payment fungible bps fee: (for f3) 256x 0.0002 = 0.0512 F3
    //                           (for f4) 39x  0.0002 = 0.0078 F4
    //
    // Total: 19 + 44 XRD = 63 XRD
    //        3 FEE2
    //        65 + 0.0512 F3
    //        153 F1
    //        0.0078 F4
    let fees = 
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(63.into())),
            (f1_resaddr, AskingType::Fungible(153.into())),
            (f4_resaddr, AskingType::Fungible(dec!("0.0078"))),
            (f3_resaddr, AskingType::Fungible(dec!("0.0512") + 65)),
            (fee2_resaddr, AskingType::Fungible(3.into())),
        ]);
    
    
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid3,
            None,
            false,
            [(n3_resaddr, AskingType::NonFungible(Some(to_nflids([3004, 3005, 3006,
                                                                  3000, 3001, 3002,
                                                                  3007, 3003, 3008].into())), None)),
             (n4_resaddr, AskingType::NonFungible(Some(to_nflids([4010, 4001, 4011,
                                                                  4000, 4012, 4002,
                                                                  4003, 4005, 4006,
                                                                  4007, 4008, 4004,
                                                                  4009].into())), None)),
             (f3_resaddr, AskingType::Fungible(256.into())),
             (f4_resaddr, AskingType::Fungible(39.into())),
            ].into(),
            fees.into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 51;
    *sum_of_all_fees.get_mut(&f1_resaddr).unwrap() += dec!("0.0512") + 153;
    *sum_of_all_fees.get_mut(&f2_resaddr).unwrap() += dec!("0.0078");
    *sum_of_all_fees.get_mut(&f3_resaddr).unwrap() += 65;
    *sum_of_all_fees.get_mut(&fee2_resaddr).unwrap() += 3;

    assert_eq!(pre_xrd_balance + 69 - 63,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be up 6 XRD");
    assert_eq!(pre_n1_balance + 4,
               *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
               "user2 should be up 4 n1");
    assert_eq!(pre_n2_balance + 5,
               *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
               "user2 should be up 5 n2");
    assert_eq!(pre_f1_balance + 2500 - 153,
               *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
               "user2 should be up 2500 f1 and down fees");
    assert_eq!(pre_f2_balance + 53,
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should be up 53 f2");
    assert_eq!(pre_f3_balance - 256 - 65 - dec!("0.0512"),
               *test_runner.get_component_resources(user2_account).get(&f3_resaddr).unwrap(),
               "user2 should be down 12345 f3 and fee");
    assert_eq!(pre_f4_balance - 39 - dec!("0.0078"),
               *test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap(),
               "user2 should be down 39 f3 and fee");
    assert_eq!(pre_fee2_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down 3 fee2");


    
    // user1 creates a proposal targeted at user2
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let ( receipt, uuid4 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user2_nftres, 2.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            false,
            [(RADIX_TOKEN, AskingType::Fungible(500.into()))].into(),
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 7;
    *sum_of_all_fees.get_mut(&fee1_resaddr).unwrap() += 2;
    
    assert_eq!(pre_xrd_balance - 500 - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user1 should be down 500 XRD and fee");
    assert_eq!(pre_fee1_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user1 should be down 2 fee1");



    // user3 tries to accept the proposal but doesn't have user2's
    // NFT so doesn't work

    let mut fees = taker_fixed_fee.clone();
    fees.insert(f4_resaddr, AskingType::Fungible(dec!("0.0042")));

    // no proof NFT, no sale
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user3_nfgid,
            &component,
            &user3_account,
            uuid4,
            None, // a proof is needed here
            false,
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            fees.clone().into_iter().collect());
    receipt.expect_commit_failure();

    // user3's own proof NFT, is the wrong one
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user3_nfgid,
            &component,
            &user3_account,
            uuid4,
            Some((user3_nftres, 2)), // this is the wrong proof
            false,
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            fees.clone().into_iter().collect());
    receipt.expect_commit_failure();

    // user2's proof NFT, while the right one, isn't available to
    // user3
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user3_nfgid,
            &component,
            &user3_account,
            uuid4,
            Some((user2_nftres, 2)), // user3 doesn't own this proof
            false,
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            fees.clone().into_iter().collect());
    receipt.expect_commit_failure();
    
    
    // user2 accepts the proposal

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f4_balance =
        test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();
    
    let receipt = 
        accept_otc_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid4,
            Some((user2_nftres, 2)), // correct user with correct proof
            false,
            [(f4_resaddr, AskingType::Fungible(21.into()))].into(),
            fees.into_iter().collect());
    receipt.expect_commit_success();
    *sum_of_all_fees.get_mut(&RADIX_TOKEN).unwrap() += 19;
    *sum_of_all_fees.get_mut(&fee2_resaddr).unwrap() += 3;

    assert_eq!(pre_xrd_balance + 500 - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 14 XRD");
    assert_eq!(pre_f4_balance - 21 - dec!("0.0042"),
               *test_runner.get_component_resources(user2_account).get(&f4_resaddr).unwrap(),
               "user2 should be down 21 f4 and fee");
    assert_eq!(pre_fee2_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down 3 fee2");
}


/// Sets of a fungible-to-fungible trading pair and tests the sweep
/// operation on it.
#[test]
fn test_sweep_f2f() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Create user3
    let (_user3_pubk, _user3_privk, user3_account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());


    // Create the f1 fungible resource
    let f1_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user1_account);

    // Create the f2 fungible resource
    let f2_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user2_account);

    // user3 is going to need some f2
    give_tokens(&mut test_runner,
                &user2_account,
                &user2_nfgid,
                &user3_account,
                &f2_resaddr,
                100000);

    // Create the fee1 resource
    let fee1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Give some fee1 tokens to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee1_resaddr,
                100000);


    // Give some fee1 tokens to user3
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user3_account,
                &fee1_resaddr,
                100000);
    

    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let user3_nftres =
        test_runner.create_non_fungible_resource(user3_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());


    let maker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(7.into())),
            (fee1_resaddr, AskingType::Fungible(2.into())),
        ]);
    let taker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(19.into())),
            (fee1_resaddr, AskingType::Fungible(3.into())),
        ]);

    // Call the `instantiate_kaupa` function with a
    // fungible/fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(maker_fixed_fee.clone()),
                                 per_tx_taker_fixed_fee: Some(taker_fixed_fee.clone()),
                                 per_nft_flat_fee: None,
                                 per_payment_bps_fee: Some(1.into()),
                             }),
                             Some(HashSet::from([f1_resaddr])),
                             Some(HashSet::from([f2_resaddr])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    // user2 adds a number of limit buy orders

    let ( receipt, _uuid11 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(21.into()))].into(),   // price per = 0.0429
            [(f1_resaddr, AskingType::Fungible(490.into()))].into(),  // price per = 23.333...
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    
    let ( receipt, _uuid12 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(10.into()))].into(),   // price per = 0.0435
            [(f1_resaddr, AskingType::Fungible(230.into()))].into(),  // price per = 23
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid13 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(20.into()))].into(),   // price per = 0.04166...
            [(f1_resaddr, AskingType::Fungible(480.into()))].into(),  // price per = 24
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid14 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some(NonFungibleGlobalId::new(user1_nftres, 1.into())),
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(18.into()))].into(),   // price per = 0.0367
            [(f1_resaddr, AskingType::Fungible(490.into()))].into(),  // price per = 27.222...
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid15 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(20.into()))].into(),   // price per = 0.0308
            [(f1_resaddr, AskingType::Fungible(650.into()))].into(),  // price per = 32.5
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();


    let ( receipt, _uuid16 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some(NonFungibleGlobalId::new(user3_nftres, 1.into())),
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(20.into()))].into(),   // price per = 0.0404
            [(f1_resaddr, AskingType::Fungible(495.into()))].into(),  // price per = 24.75
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();


    // user1 adds a number of limit sell orders
    
    let ( receipt, _uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(500.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(21.into()))].into(),  // price per = 0.042
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid2 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(500.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(20.into()))].into(),  // price per = 0.04
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid3 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(501.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(21.into()))].into(),  // price per =~ 0.041916
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    
    let ( receipt, _uuid4 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user2_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(250.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(10.into()))].into(),  // price per = 0.04
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid5 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(240.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(10.into()))].into(),  // price per = 0.04
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    let ( receipt, _uuid6 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user3_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(f1_resaddr, AskingType::Fungible(235.into()))].into(),
            [(f2_resaddr, AskingType::Fungible(10.into()))].into(),  // price per = 0.0426
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    

    // This is the F1 order book sell depth right now:
    //
    // uuid2 sells 500 F1 for 20 F2: price per F1 = 0.04
    // uuid4 sells 250 F1 for 10 F2: price per F1 = 0.04   (only for user2)
    // uuid5 sells 240 F1 for 10 F2: price per F1 = 0.04167
    // uuid3 sells 501 F1 for 21 F2: price per F1 = 0.0419
    // uuid6 sells 235 F1 for 10 F2: price per F1 = 0.0426 (but only for user3)
    // uuid1 sells 500 F1 for 21 F2: price per F1 = 0.0427


    // user2 makes a market buy spending 40 F2
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(user2_account).get(&f1_resaddr).is_none(),
            "user2 should start with no f1");
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.0045")));

    // should nab uuid2 uuid4 uuid5 and ~23.81% of uuid3
    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            None,
            (f2_resaddr, AskingType::Fungible(45.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_dec_approx(dec!("1109.28571429"),
                      *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be up 1109.29-ish f1");
    assert_eq!(pre_f2_balance - 45 - dec!("0.0045"),
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should be down 45 + 0.0045 f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");



    // user2 makes another market buy, now spending 10 F2
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.001")));

    // should nab another ~238.57 of uuid3
    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            None,
            (f2_resaddr, AskingType::Fungible(10.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_dec_approx(pre_f1_balance + dec!("238.5714285710"),
                      *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be up 238.57-ish f1");
    assert_eq!(pre_f2_balance - 10 - dec!("0.001"),
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should be down 10 + 0.001 f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");




    // user2 buys out what F1 is left, except uuid6 which is earmarked
    // for user3
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.0027")));

    // should nab the remaining ~643.143
    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            None,
            (f2_resaddr, AskingType::Fungible(100.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_dec_approx(pre_f1_balance + dec!("643.1428571400"),
                      *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be up 643.143-ish f1");
    assert_eq!(pre_f2_balance - 27 - dec!("0.0027"),
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should be down 27 + 0.0027 f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");



    // buying from an empty book should have no effect beyond charging
    // some fees
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("1")));

    // should nab nothing (but still pays the fixed taker fees)
    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            None,
            (f2_resaddr, AskingType::Fungible(100.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_f1_balance,
               *test_runner.get_component_resources(user2_account).get(&f1_resaddr).unwrap(),
               "user2 should have unchanged f1");
    assert_eq!(pre_f2_balance,
               *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
               "user2 should have unchanged f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");


    // This is the F1 order book buy depth right now:

    // uuid12 offers to buy 230 F1 for 10 F2, price: 0.04348 per F1
    // uuid11 offers to buy 490 F1 for 21 F2, price: 0.04286 per F1
    // uuid13 offers to buy 480 F1 for 20 F2, price: 0.04167 per F1
    // uuid16 offers to buy 495 F1 for 20 F2, price: 0.04040 per F1 (only for user3)
    // uuid14 offers to buy 490 F1 for 18 F2, price: 0.03673 per F1 (only for user1)
    // uuid15 offers to buy 650 F1 for 20 F2, price: 0.03077 per F1

    
    // user1 does a market sell, spending 200 F1 to get ~8.6957 F2
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    assert!(test_runner.get_component_resources(user1_account).get(&f2_resaddr).is_none(),
            "user1 should start with no f2");
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f1_resaddr.clone(), AskingType::Fungible(dec!("0.02")));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            None,
            (f1_resaddr, AskingType::Fungible(200.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_f1_balance - 200 - dec!("0.02"),
               *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
               "user2 should be down 200 f1 and fee");
    assert_dec_approx(dec!("8.6956521739"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.0000000001"),
                      "user2 should be up 8.6957-ish f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");



    // user1 does a market sell, spending another 200 F1 to get the
    // remaining ~1.3043 F2 from uuid12 and ~7.2857 F2 from uuid11.
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f1_resaddr.clone(), AskingType::Fungible(dec!("0.02")));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            None,
            (f1_resaddr, AskingType::Fungible(200.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_f1_balance - 200 - dec!("0.02"),
               *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
               "user2 should be down 200 f1 and fee");
    assert_dec_approx(pre_f2_balance + dec!("8.5900621118"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.0000000001"),
                      "user2 should be up 8.59-ish f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");


    // user1 sells into the rest of the order book except for the
    // lowball uuid15 since we set a price limit at 0.035, and of
    // course won't get uuid16 which is earmarked for user3
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f1_resaddr.clone(), AskingType::Fungible(dec!("0.2")));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some((user1_nftres, 1)),
            Some(Decimal::ONE / dec!("0.035")), // invert price limit since we're selling
            (f1_resaddr, AskingType::Fungible(2000.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_f1_balance - 1290 - dec!("0.129"),
               *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
               "user2 should be down 1290 f1 and fee");
    assert_dec_approx(pre_f2_balance + dec!("51.7142857143"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.0000000001"),
                      "user2 should be up 51.71-ish f2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");
}


/// Sets up a non-fungible-to-fungible trading pair and tests the
/// sweep operation on it.
#[test]
fn test_sweep_nf2f() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Create user3
    let (_user3_pubk, _user3_privk, user3_account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());


    // Create the n1 NFT resource, with nflids 1000-5999
    let n1_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        999,
        5000);

    // Create the f2 fungible resource
    let f2_resaddr = test_runner.create_fungible_resource(
        1000000.into(), 18, user2_account);

    // user3 is going to need some f2
    give_tokens(&mut test_runner,
                &user2_account,
                &user2_nfgid,
                &user3_account,
                &f2_resaddr,
                100000);

    // Create the fee1 resource
    let fee1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Give some fee1 tokens to user2
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee1_resaddr,
                100000);


    // Give some fee1 tokens to user3
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user3_account,
                &fee1_resaddr,
                100000);
    

    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let user3_nftres =
        test_runner.create_non_fungible_resource(user3_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());


    let maker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(7.into())),
            (fee1_resaddr, AskingType::Fungible(2.into())),
        ]);
    let taker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(19.into())),
            (fee1_resaddr, AskingType::Fungible(3.into())),
        ]);

    // Call the `instantiate_kaupa` function with a
    // non-fungible/fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(maker_fixed_fee.clone()),
                                 per_tx_taker_fixed_fee: Some(taker_fixed_fee.clone()),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (n1_resaddr.clone(),
                                          (fee1_resaddr.clone(), 111.into()))
                                     ])),
                                 per_payment_bps_fee: Some(1.into()),
                             }),
                             Some(HashSet::from([n1_resaddr])),
                             Some(HashSet::from([f2_resaddr])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    // Sell side order book:
    //
    // uuid6: user1 sells 99 N1 for 4950 F2 (= 50 per)       (only for user3)
    // uuid5: user1 sells 9  N1 for 550 F2  (= 61.1111 per)
    // uuid2: user1 sells 13 N1 for 800 F2  (= 61.5384 per)
    // uuid3: user1 sells 56 N1 for 3500 F2 (= 62.5000 per)  (only for user2)
    // uuid1: user1 sells 15 N1 for 1000 F2 (= 66.6667 per)
    // uuid4: user1 sells 1  N1 for 500 F2  (= 500 per)

    let ( receipt, _uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((1400..1415).collect())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(1000.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid2 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((1300..1313).collect())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(800.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid3 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user2_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((1200..1256).collect())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(3500.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid4 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids([1109].into())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(500.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid5 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((1100..1109).collect())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(550.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid6 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user3_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((1000..1099).collect())), None))].into(),
            [(f2_resaddr, AskingType::Fungible(4950.into()))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();



    // user2 buys out uuid5 (skipping uuid6 because it's earmarked for
    // user3) and 7 n1 from uuid2
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(user2_account).get(&n1_resaddr).is_none(),
            "user2 should start with no n1");
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.1")));
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("1776") + 3));



    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            None,
            (f2_resaddr, AskingType::Fungible(1000.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(dec!("16"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 16 n1");
    assert_dec_approx(pre_f2_balance - dec!("980.7692307700") - dec!("0.098076923077"),
                      *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be down ~980.77 f2 and fee");
    assert_eq!(pre_fee1_balance - 1776 - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 1779 fee1");
    

    // user2 buys one more from uuid2
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.1")));
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("111") + 3));



    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            None,
            (f2_resaddr, AskingType::Fungible(62.into())),
            taker_fee.into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_n1_balance + dec!("1"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 1 n1");
    assert_dec_approx(pre_f2_balance - dec!("61.5384615385") - dec!("0.00615384615385"),
                      *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be down ~61.538 f2 and fee");
    assert_eq!(pre_fee1_balance - 111 - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 114 fee1");


    // user2 buys the rest on the market, except the 500 cost ones
    // because of the max_price limit
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(f2_resaddr.clone(), AskingType::Fungible(dec!("0.6")));
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("10000") + 3));



    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            Some(dec!("70")),
            (f2_resaddr, AskingType::Fungible(6000.into())),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();


    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_n1_balance + dec!("76"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 76 n1");
    assert_dec_approx(pre_f2_balance - dec!("4807.6923076900") - dec!("0.48076923076900"),
                      *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "user2 should be down ~61.538 f2 and fee");
    assert_eq!(pre_fee1_balance - 8436 - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 8439 fee1");


    
    // Buy side order book
    //
    //            named   random
    // uuid16: buys   0 + 999 N1 for 55k  F2 (= 55.0551 per) (only for user3)
    // uuid12: buys  44 +  44 N1 for 4400 F2 (= 50 per)                         ##5003-5046
    // uuid15: buys   1 +   0 N1 for   49 F2 (= 49 per)                          #5000
    // uuid11: buys   0 +  50 N1 for 2400 F2 (= 48 per)
    // uuid14: buys   0 + 133 N1 for 6200 F2 (= 46.6165 per) (only for user1)
    // uuid13: buys   2 +  50 N1 for 2400 F2 (= 46.1538 per)                    ##5001,5002


    let ( receipt, _uuid11 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(2400.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                None, Some(50)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid12 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(4400.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5003..5047).collect())), Some(44)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid13 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(2400.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5001..5003).collect())), Some(50)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid14 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some(NonFungibleGlobalId::new(user1_nftres, 1.into())),
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(6200.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                None, Some(133)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid15 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            None,
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(49.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                Some([5000.into()].into()), None))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid16 ) =
        make_generic_proposal(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some(NonFungibleGlobalId::new(user3_nftres, 1.into())),
            ProposalType::Barter,
            &user2_nftres,
            1,
            true,
            [(f2_resaddr, AskingType::Fungible(55000.into()))].into(),
            [(n1_resaddr, AskingType::NonFungible(
                None, Some(999)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    

    let price_limit = Some(Decimal::ONE / dec!("46.2"));
    
    // user1 sells 7 of the 44 named NFTs wanted by uuid12 plus 10 randos
    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    assert!(test_runner.get_component_resources(user1_account).get(&f2_resaddr).is_none(),
            "user1 should start with no f2");
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("1887") + 3));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some((user1_nftres, 1)),
            price_limit,
            (n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5003..5010).chain(5500..5510).collect())),
                None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();
    
    assert_eq!(pre_n1_balance - 17,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "n1 balance should be down by 17");
    assert_eq!(dec!("850"),
               *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
               "f2 balance should be up by 850");
    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 19");
    assert_eq!(pre_fee1_balance - 1887 - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "fee balance should be down by 1890");


    // user1 sells 100 rando NFTs to uuid12, uuid11, uuid14
    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("11100") + 3));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some((user1_nftres, 1)),
            price_limit,
            (n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5510..5610).collect())),
                None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();
    
    assert_eq!(pre_n1_balance - 100,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "n1 balance should be down by 100");
    assert_dec_approx(pre_f2_balance + dec!("4845.8646616500"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "f2 balance should be up by 4845.86-ish");
    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 19");
    assert_eq!(pre_fee1_balance - 11100 - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "fee balance should be down by 11103");

    // Buy side order book after this
    //
    //            named   random
    // uuid16: buys   0 + 999 N1 for F2 (55.0551 per) (only for user3)
    // uuid12: buys  37 +   0 N1 for F2 (50 per)                         ##5010-5046
    // uuid15: buys   1 +   0 N1 for F2 (49 per)                          #5000
    // uuid11: buys   0 +   0 N1 for F2 (48 per)
    // uuid14: buys   0 + 117 N1 for F2 (46.6165 per) (only for user1)
    // uuid13: buys   2 +  50 N1 for F2 (46.1538 per)                    ##5001,5002



    // user1 sells 5000, 5010-5045 and 5700-5749 which should clear
    // out uuid15, remove 50 randos from uuid14 and leave only 5046 in
    // uuid12

    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("9657") + 3));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some((user1_nftres, 1)),
            price_limit,
            (n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5000..5001).chain(5010..5046).chain(5700..5750).collect())),
                None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();
    
    assert_eq!(pre_n1_balance - 87,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "n1 balance should be down by 87");
    assert_dec_approx(pre_f2_balance + dec!("4179.8270676700"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "f2 balance should be up by 4179.83-ish");
    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 19");
    assert_eq!(pre_fee1_balance - 9657 - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "fee balance should be down by 9660");

    // Buy side order book after this
    //
    //            named   random
    // uuid16: buys   0 + 999 N1 for F2 (55.0551 per) (only for user3)
    // uuid12: buys   1 +   0 N1 for F2 (50 per)                         ##5010-5046
    // uuid15: buys   0 +   0 N1 for F2 (49 per)                          #5000
    // uuid11: buys   0 +   0 N1 for F2 (48 per)
    // uuid14: buys   0 +  67 N1 for F2 (46.6165 per) (only for user1)
    // uuid13: buys   2 +  50 N1 for F2 (46.1538 per)                    ##5001,5002


    // user1 piles on the randos to clear out the order book to their
    // price limit - this will leave only uuid16 which is earmarked
    // for user3 and uuid13 which is too low priced


    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap().clone();
    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee1_resaddr.clone(), AskingType::Fungible(dec!("20000") + 3));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some((user1_nftres, 1)),
            price_limit,
            (n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((5800..5999).collect())),
                None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();
    
    assert_eq!(pre_n1_balance - 67,
               *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
               "n1 balance should be down by 67");
    assert_dec_approx(pre_f2_balance + dec!("3123.3082706800"),
                      *test_runner.get_component_resources(user1_account).get(&f2_resaddr).unwrap(),
                      dec!("0.00000001"),
                      "f2 balance should be up by 3123.31-ish");
    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "XRD balance should be down by 19");
    assert_eq!(pre_fee1_balance - 7437 - 3,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "fee balance should be down by 7440");
}


/// Sets up a non-fungible-to-non-fungible trading pair and tests the
/// sweep operation on it.
#[test]
fn test_sweep_nf2nf() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Create user3
    let (_user3_pubk, _user3_privk, user3_account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());


    // Create the n1 NFT resource, with nflids 10000-19999
    let n1_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        9999,
        10000);

    // Create the n2 NFT resource, with nflids 20000-29999
    let n2_resaddr = create_nft_resource(
        &mut test_runner,
        &user2_nfgid,
        &user2_account,
        19999,
        10000);


    // Create the fee1 resource
    let fee1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee1_resaddr,
                100000);

    // Create the fee2 resource
    let fee2_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee2_resaddr,
                100000);


    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let user3_nftres =
        test_runner.create_non_fungible_resource(user3_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());


    let maker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(7.into())),
            (fee1_resaddr, AskingType::Fungible(2.into())),
        ]);
    let taker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(19.into())),
            (fee1_resaddr, AskingType::Fungible(3.into())),
        ]);

    // Call the `instantiate_kaupa` function with a
    // non-fungible/non-fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(maker_fixed_fee.clone()),
                                 per_tx_taker_fixed_fee: Some(taker_fixed_fee.clone()),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (n1_resaddr.clone(),
                                          (fee2_resaddr.clone(), 111.into())),
                                         (n2_resaddr.clone(),
                                          (fee2_resaddr.clone(), 7.into()))
                                     ])),
                                 per_payment_bps_fee: None,
                             }),
                             Some(HashSet::from([n1_resaddr])),
                             Some(HashSet::from([n2_resaddr])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];



    
    let ( receipt, _uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user2_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((10600..10700).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20200..20300).collect())), Some(250)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid2 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids([10999].into())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20100..20102).collect())), None))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid3 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((11000..12000).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                None, Some(4000)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid4 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((10100..10126).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20000..20070).collect())), Some(10)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid5 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((12000..12999).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                None, Some(4995)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid6 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            Some(NonFungibleGlobalId::new(user3_nftres, 1.into())),
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((10000..10100).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                None, Some(200)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();
    let ( receipt, _uuid7 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::Barter,
            &user1_nftres,
            1,
            true,
            [(n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((10700..10703).collect())), None))].into(),
            [(n2_resaddr, AskingType::NonFungible(
                Some(to_nflids([20300].into())), Some(1)))].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    // Note uuid7 where the fraction between sell side (3x) and buy
    // side (2x) means that you cannot partially buy it: If you tried
    // to pay 1 N2 into this proposal it would net you 1.5 N1s and we
    // don't allow you to buy halfs of NFTs. Therefore the "random"
    // NFT here can effectively only be filled into the proposal if
    // you are also able to deliver the named NFT it requests.
    //
    // uuid4 is similar.
    //
    // Those proposals should therefore stick around unchanged for a
    // while, until we provide the named NFTs to them (e.g. N2 #20300
    // for uuid7).
    //
    // The same is true for uuid2 which only asks for named NFTs.
    //
    // Then there is uuid1 which effectively only accepts N2s in
    // certain batch sizes, e.g. 7 N2s is ok but 8 is not; and uuid3
    // has this sort of thing going on as well.
    //
    // Overflow from rounding in all these order book entries passes
    // on to uuid5 which has a 5-to-1 ratio, but we have a price limit
    // that prevents us from dipping into this one.
    

    // Sell side order book now
    //
    //                       named    random                                   named ids
    // uuid7: sells    3 N1 for    1 +    1 N2 (= 0.67 per) awkward fraction   #20300
    // uuid6: sells  100 N1 for         200 N2 (= 2    per) (only for user3)
    // uuid2: sells    1 N1 for    2 +    0 N2 (= 2    per)                    ##20100,20101
    // uuid4: sells   26 N1 for   70 +   10 N2 (= 3.1  per) awkward fraction   ##20000-20069
    // uuid1: sells  100 N1 for  100 +  250 N2 (= 3.5  per) (only for user2)   ##20200-20299
    // uuid3: sells 1000 N1 for    0 + 4000 N2 (= 4    per)
    // uuid5: sells  999 N1 for        4995 N2 (= 5    per)

    
    // user2 puts 5 named and 9 randos into uuid1 (for 4 in return),
    // and 4 randos into uuid3 (for 1 in return), leaving 2 unspent
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(user2_account).get(&n1_resaddr).is_none(),
            "user2 should start with no n1");
    let pre_n2_balance =
        test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee2_resaddr.clone(), AskingType::Fungible(dec!("111") * 5 + 7 * 18));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            Some(dec!("4")),
            (n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20200..20205).chain(21000..21015).collect())), None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();

    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(dec!("5"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 5 n1");
    assert_eq!(pre_n2_balance - 18,
            *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
            "user2 should be down 18 n2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");
    assert_eq!(pre_fee2_balance - 111 * 5 - 7 * 18,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down fee2 for 5x n1 and 18x n2");


    // Sell side order book now
    //
    //                       named    random                                   named ids
    // uuid7: sells    3 N1 for    1 +    1 N2 (= 0.67 per) awkward fraction   #20300
    // uuid6: sells  100 N1 for         200 N2 (= 2    per) (only for user3)
    // uuid2: sells    1 N1 for    2 +    0 N2 (= 2    per)                    ##20100,20101
    // uuid4: sells   26 N1 for   70 +   10 N2 (= 3.1  per) awkward fraction   ##20000-20069
    // uuid1: sells   96 N1 for   95 +  241 N2 (= 3.5  per) (only for user2)   ##20205-20299
    // uuid3: sells  999 N1 for    0 + 3996 N2 (= 4    per)
    // uuid5: sells  999 N1 for        4995 N2 (= 5    per)

    
    // user2 puts 1 named and 1 rando into uuid7 (clearing it out),
    // and 4 randos into uuid3 (for 1 in return), leaving none unspent
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap().clone();
    let pre_n2_balance =
        test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee2_resaddr.clone(), AskingType::Fungible(dec!("111") * 4 + 7 * 6));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            Some(dec!("4")),
            (n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20300..20301).chain(21100..21105).collect())), None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();

    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_n1_balance + dec!("4"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 4 n1");
    assert_eq!(pre_n2_balance - 6,
            *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
            "user2 should be down 6 n2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");
    assert_eq!(pre_fee2_balance - 111 * 4 - 7 * 6,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down fee2 for 4x n1 and 6x n2");

    // Sell side order book now
    //
    //                       named    random                                   named ids
    // uuid7: sells    0 N1 for    0 +    0 N2 (= 0.67 per) gone
    // uuid6: sells  100 N1 for         200 N2 (= 2    per) (only for user3)
    // uuid2: sells    1 N1 for    2 +    0 N2 (= 2    per)                    ##20100,20101
    // uuid4: sells   26 N1 for   70 +   10 N2 (= 3.1  per) awkward fraction   ##20000-20069
    // uuid1: sells   96 N1 for   95 +  241 N2 (= 3.5  per) (only for user2)   ##20205-20299
    // uuid3: sells  998 N1 for    0 + 3992 N2 (= 4    per)
    // uuid5: sells  999 N1 for        4995 N2 (= 5    per)

    
    
    // user2 decided to try to completely buy out uuid4.
    
    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap().clone();
    let pre_n2_balance =
        test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();
    let pre_fee2_balance =
        test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap().clone();
    
    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee2_resaddr.clone(), AskingType::Fungible(dec!("111") * 26 + 7 * 80));

    let receipt =
        sweep_proposals(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            Some((user2_nftres, 1)),
            Some(dec!("4")),
            (n2_resaddr, AskingType::NonFungible(
                Some(to_nflids((20000..20070).chain(21200..21210).collect())), None)),
            taker_fee.clone().into_iter().collect(),
        );
    receipt.expect_commit_success();

    assert_eq!(pre_xrd_balance - 19,
               *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_n1_balance + dec!("26"),
            *test_runner.get_component_resources(user2_account).get(&n1_resaddr).unwrap(),
            "user2 should be up 26 n1");
    assert_eq!(pre_n2_balance - 80,
            *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
            "user2 should be down 80 n2");
    assert_eq!(pre_fee1_balance - 3,
               *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");
    assert_eq!(pre_fee2_balance - 111 * 26 - 7 * 80,
               *test_runner.get_component_resources(user2_account).get(&fee2_resaddr).unwrap(),
               "user2 should be down fee2 for 26x n1 and 80x n2");

    // Sell side order book now
    //
    //                       named    random                                   named ids
    // uuid7: sells    0 N1 for    0 +    0 N2 (= 0.67 per) gone
    // uuid6: sells  100 N1 for         200 N2 (= 2    per) (only for user3)
    // uuid2: sells    1 N1 for    2 +    0 N2 (= 2    per)                    ##20100,20101
    // uuid4: sells    0 N1 for    0 +    0 N2 (= 3.1  per) gone
    // uuid1: sells   96 N1 for   95 +  241 N2 (= 3.5  per) (only for user2)   ##20205-20299
    // uuid3: sells  998 N1 for    0 + 3992 N2 (= 4    per)
    // uuid5: sells  999 N1 for        4995 N2 (= 5    per)

}


/// This tests a basic flash loan.
///
/// I have disabled this test because there appears to be a bug in 0.8
/// Scrypto where the transaction manifest sees there is a transient
/// bucket with zero tokens in it and panics when it should just ignore
/// that situation.
///
/// While this means that our flash loans cannot possibly work at this
/// time I have still implemented them and their tests in the
/// expectation that this will start working when the above problem
/// gets fixed (for RCnet 1?)
///
/// Note, if you want to pretend that flash loans are currently
/// working you can comment out a line in the `instantiate_kaupa`
/// function as specified there and then run the flash loan tests.
#[test]
#[ignore]
fn test_flash_loans() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create owner
    let (owner_pubk, _owner_privk, owner_account) = test_runner.new_allocated_account();
    let owner_nfgid = NonFungibleGlobalId::from_public_key(&owner_pubk);

    // Create user1
    let (user1_pubk, _user1_privk, user1_account) = test_runner.new_allocated_account();
    let user1_nfgid = NonFungibleGlobalId::from_public_key(&user1_pubk);
    
    // Create user2
    let (user2_pubk, _user2_privk, user2_account) = test_runner.new_allocated_account();
    let user2_nfgid = NonFungibleGlobalId::from_public_key(&user2_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());


    // Create the n1 NFT resource, with nflids 10000-19999
    let n1_resaddr = create_nft_resource(
        &mut test_runner,
        &user1_nfgid,
        &user1_account,
        9999,
        10000);

    // Create the n2 NFT resource, with nflids 20000-29999
    let n2_resaddr = create_nft_resource(
        &mut test_runner,
        &user2_nfgid,
        &user2_account,
        19999,
        10000);

    // Create the f1 resource
    let f1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);

    // Create the f2 resource
    let f2_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user2_account);

    // Create the fee1 resource
    let fee1_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee1_resaddr,
                100000);

    // Create the fee2 resource
    let fee2_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, user1_account);
    give_tokens(&mut test_runner,
                &user1_account,
                &user1_nfgid,
                &user2_account,
                &fee2_resaddr,
                100000);


    // Only need a few of these so do it simple
    let user1_nftres =
        test_runner.create_non_fungible_resource(user1_account);
    let _user2_nftres =
        test_runner.create_non_fungible_resource(user2_account);
    let owner_nftres =
        test_runner.create_non_fungible_resource(owner_account);
    let kaupa_admin_badge =
        NonFungibleGlobalId::new(owner_nftres, 1.into());


    let maker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(7.into())),
            (fee1_resaddr, AskingType::Fungible(2.into())),
        ]);
    let taker_fixed_fee =
        HashMap::<ResourceAddress, AskingType>::from([
            (RADIX_TOKEN, AskingType::Fungible(19.into())),
            (fee1_resaddr, AskingType::Fungible(3.into())),
        ]);

    // Call the `instantiate_kaupa` function with a
    // non-fungible/non-fungible trading pair
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa_admin_badge,
                             None::<String>,
                             None::<String>,
                             None::<String>,
                             Some(Fees{
                                 per_tx_maker_fixed_fee: Some(maker_fixed_fee.clone()),
                                 per_tx_taker_fixed_fee: Some(taker_fixed_fee.clone()),
                                 per_nft_flat_fee: Some(HashMap::from(
                                     [
                                         (n1_resaddr.clone(),
                                          (fee2_resaddr.clone(), 111.into())),
                                         (n2_resaddr.clone(),
                                          (fee2_resaddr.clone(), 7.into()))
                                     ])),
                                 per_payment_bps_fee: None,
                             }),
                             None::<String>,
                             None::<String>,
                             false,
                             false,
                             true))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![owner_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let component = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    let transient_resource = receipt.output::<(ComponentAddress, Option<ResourceAddress>)>(1)
        .1.unwrap();

    // user1 offers a flash loan

    let pre_xrd_balance =
        test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap().clone();
    let pre_n1_balance =
        test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap().clone();
    let pre_f1_balance =
        test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap().clone();

    let loan_size =
        HashMap::<ResourceAddress, AskingType>::from([
            (n1_resaddr, AskingType::NonFungible(
                Some(to_nflids((10000..10100).collect())), None)),
            (f1_resaddr, AskingType::Fungible(dec!("20000"))),
        ]);
    
    let ( receipt, uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &user1_nfgid,
            &component,
            &user1_account,
            None,
            ProposalType::FlashLoan,
            &user1_nftres,
            1,
            true,
            loan_size.clone().into_iter().collect(),
            [
                (n2_resaddr, AskingType::NonFungible(
                    Some(to_nflids([20000].into())), Some(1))),
                (f2_resaddr, AskingType::Fungible(dec!("200"))),
            ].into(),
            maker_fixed_fee.clone().into_iter().collect());
    receipt.expect_commit_success();

    assert_eq!(pre_xrd_balance - 7,
               *test_runner.get_component_resources(user1_account).get(&RADIX_TOKEN).unwrap(),
               "user2 should be down 19 XRD");
    assert_eq!(pre_n1_balance - 100,
            *test_runner.get_component_resources(user1_account).get(&n1_resaddr).unwrap(),
            "user2 should be down 100 n1");
    assert_eq!(pre_f1_balance - 20000,
            *test_runner.get_component_resources(user1_account).get(&f1_resaddr).unwrap(),
            "user2 should be up 20k f1");
    assert_eq!(pre_fee1_balance - 2,
               *test_runner.get_component_resources(user1_account).get(&fee1_resaddr).unwrap(),
               "user2 should be down 3 fee1");



    // user2 takes out the flash loan (and just asserts that they got
    // it)

    let pre_xrd_balance =
        test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap().clone();
    assert!(test_runner.get_component_resources(user2_account).get(&n1_resaddr).is_none(),
            "user2 should start with no n1 tokens");
    assert!(test_runner.get_component_resources(user2_account).get(&f1_resaddr).is_none(),
            "user2 should start with no f1 tokens");
    let pre_n2_balance =
        test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap().clone();
    let pre_f2_balance =
        test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap().clone();
    let pre_fee1_balance =
        test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap().clone();

    let mut taker_fee = taker_fixed_fee.clone();
    taker_fee.insert(fee2_resaddr, AskingType::Fungible(dec!("14")));
    
    let receipt =
        accept_flash_loan(
            &mut test_runner,
            &user2_nfgid,
            &component,
            &user2_account,
            uuid1,
            None,
            [
                (n2_resaddr, AskingType::NonFungible(
                    Some(to_nflids((20000..20002).collect())), None)),
                (f2_resaddr, AskingType::Fungible(dec!("200"))),
            ].into(),
            taker_fee.into_iter().collect(),
            loan_size.clone().into_iter().collect(),
            transient_resource);
    receipt.expect_commit_success();


    // As a curiosity, it turns out that when your account has had n1
    // tokens in it (however briefly, such as in our flash loan) the
    // get_component_resources method now returns a Some({}) instead
    // of a None like it did above in the pre-checking. So here I use
    // unwrap_or instead of is_none().
    
    assert_eq!(dec!("0"),
               *test_runner.get_component_resources(user2_account).get(&n1_resaddr)
               .unwrap_or(&dec!("0")),
               "user2 should still have no n1 tokens");
    assert_eq!(dec!("0"),
               *test_runner.get_component_resources(user2_account).get(&f1_resaddr)
               .unwrap_or(&dec!("0")),
               "user2 should still have no f1 tokens");
    assert_eq!(dec!("0"),
               *test_runner.get_component_resources(user2_account).get(&transient_resource)
               .unwrap_or(&dec!("0")),
               "user2 should definitely not have any transient tokens");
    assert_eq!(pre_xrd_balance - 19,
            *test_runner.get_component_resources(user2_account).get(&RADIX_TOKEN).unwrap(),
            "user2 should be down 19 XRD");
    assert_eq!(pre_n2_balance - 2,
            *test_runner.get_component_resources(user2_account).get(&n2_resaddr).unwrap(),
            "user2 should be down 2 n2");
    assert_eq!(pre_f2_balance - 200,
            *test_runner.get_component_resources(user2_account).get(&f2_resaddr).unwrap(),
            "user2 should be down 200 f2");
    assert_eq!(pre_fee1_balance - 3,
            *test_runner.get_component_resources(user2_account).get(&fee1_resaddr).unwrap(),
            "user2 should be down 3 fee1");
}



/// Story time!
///
/// Alice was an early high-profile player in Gold/Land, an online
/// game where players compete to acquire the two main resources Gold
/// and Land. Those are represented by the on-ledger tokens GOLD and
/// LAND.
///
/// She set up a trading pair for those two back then and this has
/// remained the primary venue players use to swap GOLD for LAND and
/// vice versa. Alice has since stopped playing the game and has no
/// interest in following it. She therefore no longer has any need for
/// the GOLD and LAND tokens that the trading pair earns her in
/// fees. Ideally she would like to simply sell those tokens off every
/// now and then but the company behind the game has been very
/// dilligent in pursuing and shutting down anyone who tries to set up
/// GOLD and LAND trading pairs vs real currencies. (Maybe they're
/// terrified of their game tokens becoming securities.)
///
/// Instead of forcing herself to keep up to date on game events and
/// the fluctuating values of those two currencies, Alice therefore
/// instead sets up a flash loan on a different service where she
/// offers her trading pair admin token for a small fee of 10
/// XRD. This way, she figures, whenever the fees in the trading pair
/// happen to be worth about that much some ambitious player or other
/// will come along and borrow her admin token and use it to pull the
/// fees from the trading pair to use in the game.
///
/// This allows Alice to completely ignore the trading pair altogether
/// (she is *really* done with that game) and just rely on market
/// forces to ensure that she'll get 10 XRD coming in to her flash
/// loan at appropriate times.
///
/// In this test we will set up the above scenario and we will follow
/// Bob as he decides the time has come to put 10 XRD into buying a
/// little bit of game power, netting him 25 LAND and 10 GOLD he can
/// use to crush his enemies.
///
/// Note that while it may seem dangerous to loan out an admin token
/// like this, in Kaupa's case the only thing your admin token does is
/// allow you to collect fees. If it was also used to configure your
/// Kaupa instance, turn it on and off, etc., then we should likely
/// offer a separate "fee collecting badge" you would use for this
/// flash loan use case instead.
///
/// This test takes a long time to run (~7 minutes on the author's
/// system) because this scenario does a large number of transactions
/// on a trading pair, just to give the thing a good workout, before
/// it gets to the flash loan part. Because of this it is usually set
/// to ignore.
///
/// Also see the note about the flash loan bug in the comment to the
/// the test_flash_loans function.
#[test]
#[ignore]
fn test_goldland_scenario() {
    // Setup the environment
    let mut test_runner = TestRunner::builder().build();

    // Create alice, an OG player who is now out of the game
    let (alice_pubk, _alice_privk, alice_account) = test_runner.new_allocated_account();
    let alice_nfgid = NonFungibleGlobalId::from_public_key(&alice_pubk);
    
    // Create bob, a current player with a lot of LAND production
    let (bob_pubk, _bob_privk, bob_account) = test_runner.new_allocated_account();
    let bob_nfgid = NonFungibleGlobalId::from_public_key(&bob_pubk);

    // Create charlie, a current player with a lot of GOLD production
    let (charlie_pubk, _charlie_privk, charlie_account) = test_runner.new_allocated_account();
    let charlie_nfgid = NonFungibleGlobalId::from_public_key(&charlie_pubk);

    // Create victor, who knows nothing of the game but runs a general
    // bartering Kaupa
    let (victor_pubk, _victor_privk, victor_account) = test_runner.new_allocated_account();
    let victor_nfgid = NonFungibleGlobalId::from_public_key(&victor_pubk);

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());


    // Create the LAND resource
    let land_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, bob_account);

    // Create the GOLD resource
    let gold_resaddr =
        test_runner.create_fungible_resource(
            1000000.into(), 18, charlie_account);


    // Only need a few of these so do it simple
    let alice_nftres =
        test_runner.create_non_fungible_resource(alice_account);
    let bob_nftres =
        test_runner.create_non_fungible_resource(bob_account);
    let charlie_nftres =
        test_runner.create_non_fungible_resource(charlie_account);
    let victor_nftres =
        test_runner.create_non_fungible_resource(victor_account);


    // Create the LAND/GOLD trading pair Kaupa
    let kaupa1_admin_badge =
        NonFungibleGlobalId::new(alice_nftres, 1.into());
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa1_admin_badge,
                             Some("Goldland marketplace"),
                             Some("Trade your land and gold here!"),
                             Some("https://___goldlandex___.com"),
                             Some(Fees{
                                 per_tx_maker_fixed_fee: None,
                                 per_tx_taker_fixed_fee: None,
                                 per_nft_flat_fee: None,
                                 per_payment_bps_fee: Some(dec!("10")),
                             }),
                             Some(HashSet::from([land_resaddr])),
                             Some(HashSet::from([gold_resaddr])),
                             true,
                             true,
                             false))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![alice_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let kaupa1 = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];


    
    // Create the otherwise unrelated general barter Kaupa
    let kaupa2_admin_badge =
        NonFungibleGlobalId::new(victor_nftres, 1.into());
    let manifest = ManifestBuilder::new()
        .call_function(package_address, "Kaupa", "instantiate_kaupa",
                       args!(&kaupa2_admin_badge,
                             Some("Barters R Us"),
                             Some("The premier marketplace for any bartering ever"),
                             Some("https://---barters-r-us---.com"),
                             Some(Fees{
                                 per_tx_maker_fixed_fee: None,
                                 per_tx_taker_fixed_fee: None,
                                 per_nft_flat_fee: None,
                                 per_payment_bps_fee: Some(dec!("10")),
                             }),
                             None::<String>,
                             None::<String>,
                             false,
                             false,
                             true))
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![victor_nfgid.clone()],
    );

    receipt.expect_commit_success();
    let kaupa2 = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];
    
    let transient_resource = receipt.output::<(ComponentAddress, Option<ResourceAddress>)>(1)
        .1.unwrap();


    // Alice puts her NFT1 in a flash loan for 10 XRD per borrow
    let ( receipt, uuid1 ) =
        make_generic_proposal(
            &mut test_runner,
            &alice_nfgid,
            &kaupa2,
            &alice_account,
            None,
            ProposalType::FlashLoan,
            &alice_nftres,
            2,
            false,
            [(alice_nftres, AskingType::NonFungible(Some(to_nflids([1].into())), None))].into(),
            [(RADIX_TOKEN, AskingType::Fungible(dec!("10")))].into(),
            [].into());
    receipt.expect_commit_success();



    // There is now a flurry of trading activity on the LAND/GOLD
    // trading pair, generating LAND and GOLD fees for its owner.
    for count in 0..100 {
        let ( receipt, _ ) =
            make_generic_proposal(
                &mut test_runner,
                &bob_nfgid,
                &kaupa1,
                &bob_account,
                None,
                ProposalType::Barter,
                &bob_nftres,
                1,
                true,
                [(land_resaddr, AskingType::Fungible(dec!("1000")))].into(),
                [(gold_resaddr, AskingType::Fungible(dec!("100") + count))].into(),
                Vec::new());
        receipt.expect_commit_success();
    }

    for _ in 0..50 {
        let receipt =
            sweep_proposals(
                &mut test_runner,
                &charlie_nfgid,
                &kaupa1,
                &charlie_account,
                None,
                None,
                (gold_resaddr, AskingType::Fungible(dec!("200"))),
                [(gold_resaddr, AskingType::Fungible(dec!("1")))].into(),
            );
        receipt.expect_commit_success();
    }

    for count in 0..100 {
        let ( receipt, _ ) =
            make_generic_proposal(
                &mut test_runner,
                &charlie_nfgid,
                &kaupa1,
                &charlie_account,
                None,
                ProposalType::Barter,
                &charlie_nftres,
                1,
                true,
                [(gold_resaddr, AskingType::Fungible(dec!("100")))].into(),
                [(land_resaddr, AskingType::Fungible(dec!("1000") + count))].into(),
                Vec::new());
        receipt.expect_commit_success();
    }

    for _ in 0..50 {
        let receipt =
            sweep_proposals(
                &mut test_runner,
                &bob_nfgid,
                &kaupa1,
                &bob_account,
                None,
                None,
                (land_resaddr, AskingType::Fungible(dec!("500"))),
                [(land_resaddr, AskingType::Fungible(dec!("1")))].into(),
            );
        receipt.expect_commit_success();
    }


    // Bob collects all the GOLD his trades have been receiving
    let receipt = collect_funds(&mut test_runner,
                                &bob_nfgid,
                                &kaupa1,
                                &bob_account,
                                false,
                                &bob_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();

    
    // Charlie collects all the LAND his trades have been receiving
    let receipt = collect_funds(&mut test_runner,
                                &charlie_nfgid,
                                &kaupa1,
                                &charlie_account,
                                false,
                                &charlie_nftres,
                                1,
                                None,);
    receipt.expect_commit_success();

    assert!(*test_runner.get_component_resources(bob_account).get(&gold_resaddr).unwrap()
            > 100.into(),
            "bob should now have some GOLD");
    
    assert!(*test_runner.get_component_resources(charlie_account).get(&land_resaddr).unwrap()
            > 100.into(),
            "charlie should now have some LAND");



    let pre_xrd_balance =
        *test_runner.get_component_resources(bob_account).get(&RADIX_TOKEN).unwrap();
    let pre_land_balance =
        *test_runner.get_component_resources(bob_account).get(&land_resaddr).unwrap();
    let pre_gold_balance =
        *test_runner.get_component_resources(bob_account).get(&gold_resaddr).unwrap();

    // Bob pounces on the flash loan and collects fees from the
    // trading pair. We write this out in full here to give a clear
    // idea how a flash loan manifest can be built.

    let manifest = ManifestBuilder::new()
        // This is 10 XRD payment to Alice + 0.01 XRD fee to Victor.
        .withdraw_from_account_by_amount(bob_account, dec!("10.01"), RADIX_TOKEN)
    
        .take_from_worktop_by_amount(
            dec!("10"), RADIX_TOKEN,
            |builder, payment_bucket_id| {
                builder.take_from_worktop_by_amount(
                    dec!("0.01"), RADIX_TOKEN,
                    |builder, fee_bucket_id| {
                        builder

                        // Here Bob pays 10 XRD to loan Alice's NFT #1,
                        // and also 0.01 XRD fee to Victor.
                            .call_method(
                                kaupa2, "accept_proposal",
                                args!(
                                    None::<String>,
                                    uuid1,
                                    false,
                                    Vec::<ManifestBucket>::from([payment_bucket_id]),
                                    Vec::<ManifestBucket>::from([fee_bucket_id])))

                        // Just to confirm we have Alice's NFT #1 at
                        // this point. This isn't necessary but useful
                        // for debugging.
                            .assert_worktop_contains_by_amount(1.into(), alice_nftres)

                        // We now create a bucket with NFT #1, and
                        // then create a proof from that bucket. (If
                        // there is a more direct way of creating the
                        // proof I could not find it.)
                            .take_from_worktop_by_ids(
                                &BTreeSet::from([1.into()]),
                                alice_nftres,
                                |builder, loaned_proof_bucket_id| {
                                    builder.create_proof_from_bucket(
                                        &loaned_proof_bucket_id,
                                        |builder, loaned_proof_id| {

                                            // And now we call the
                                            // meat of this whole
                                            // operation:
                                            // collect_funds on Kaupa1
                                            // to get at Alice's fees
                                            builder
                                                .call_method(
                                                    kaupa1, "collect_funds",
                                                    args!(
                                                        loaned_proof_id,
                                                        true,
                                                        true,
                                                        None::<String>
                                                    )
                                                )
                                        })

                                        // At this point it is useful
                                        // for Bob to ensure that he
                                        // actually received what he
                                        // thought he would from that
                                        // call. After all, someone
                                        // else may have beaten him to
                                        // it, and he doesn't want to
                                        // pay 10 XRD for the flash
                                        // loan if he's left with
                                        // empty hands after. So the
                                        // following two asserts are
                                        // very important since they
                                        // will abort the transaction
                                        // if they fail, saving Bob
                                        // his 10.01 XRD.
                                        .assert_worktop_contains_by_amount(dec!("10"), gold_resaddr)
                                        .assert_worktop_contains_by_amount(dec!("25"), land_resaddr)

                                        // Now we need to put Alice's
                                        // NFT #1 back onto the
                                        // worktop (we still have in
                                        // the bucket we made earlier)
                                        // so we can return it to the
                                        // flash loan.
                                        .return_to_worktop(loaned_proof_bucket_id)
                                })

                            // Grab the transient token we received,
                            // as well as NFT #1 which we need to
                            // return.
                            .take_from_worktop(
                                transient_resource,
                                |builder, transient_bucket_id| {
                                    builder.take_from_worktop_by_ids(
                                        &BTreeSet::from([1.into()]),
                                        alice_nftres,
                                        |builder, repayment_bucket_id| {
                                            builder

                                                // This repays the
                                                // loan so now we lose
                                                // Alice's NFT #1 and
                                                // also the transient
                                                // token that binds us
                                                // to the loan.
                                                .call_method(
                                                    kaupa2, "repay_flash_loan",
                                                    args!(
                                                        transient_bucket_id,
                                                        Vec::from([repayment_bucket_id])
                                                    ))
                                        })
                                })
                    })
            })

        // And this returns to us any excess funds we put into the
        // transaction (if we overpaid some XRD, it comes back here),
        // and the GOLD and LAND that we got out of Alice's Kaupa1
        // fees.
        .call_method(bob_account,
                     "deposit_batch",
                     args!(ManifestExpression::EntireWorktop))
        .build();

    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![bob_nfgid.clone()],
    );
    
    receipt.expect_commit_success();


    let post_xrd_balance =
        *test_runner.get_component_resources(bob_account).get(&RADIX_TOKEN).unwrap();
    let post_land_balance =
        *test_runner.get_component_resources(bob_account).get(&land_resaddr).unwrap();
    let post_gold_balance =
        *test_runner.get_component_resources(bob_account).get(&gold_resaddr).unwrap();

    println!("xrd: {} to {} - earned: {}",
             pre_xrd_balance,
             post_xrd_balance,
             post_xrd_balance - pre_xrd_balance);
    println!("gold: {} to {} - earned: {}",
             pre_gold_balance,
             post_gold_balance,
             post_gold_balance - pre_gold_balance);
    println!("land: {} to {} - earned: {}",
             pre_land_balance,
             post_land_balance,
             post_land_balance - pre_land_balance);

    assert_eq!(dec!("10"), post_gold_balance - pre_gold_balance,
               "Bob should be up 10 GOLD");
    assert_eq!(dec!("25"), post_land_balance - pre_land_balance,
               "Bob should be up 25 LAND");
    assert_eq!(dec!("0"),
               *test_runner.get_component_resources(bob_account).get(&transient_resource)
               .unwrap_or(&dec!("0")),
               "Bob should not have any transient tokens");
}
