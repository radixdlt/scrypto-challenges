//! NOTE these tests use a global resource (the resim exectuable's
//! simulator) and therefore MUST be run single threaded, like this
//! from the command line:
//!
//! cargo test -- --test-threads=1
//!
//! Also note that if you run the tests with increased output
//! verbosity enabled you may see panics or stacktraces during a
//! successful run. This is expected behaviour as we use
//! std::panic::catch_unwind to test calls under conditions that
//! should make them panic. One way to see a lot of this sort of
//! output would be to run the tests like this (in a Unix-like shell):
//! 
//! RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

use std::process::Command;
use regex::Regex;
use lazy_static::lazy_static;

const RADIX_TOKEN: &str = "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag";

#[derive(Debug)]
struct Account {
    address: String,
    _pubkey: String,
    _privkey:String,
}

#[derive(Debug)]
struct SmorgasDaoComponent {
    address: String,
    admin_badge: String,
}

#[derive(Debug)]
struct ControlledComponent {
    address: String,
    admin_badge: String,
}


/// Runs a command line program, panicking if it fails and returning
/// its stdout if it succeeds
fn run_command(command: &mut Command) -> String {
    let output = command
        .output()
        .expect("Failed to run command line");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    lazy_static! {
        static ref RE_EXIT_FAIL: Regex =
            Regex::new(concat!(
                r#"^Transaction Status: COMMITTED FAILURE: (.*)\n"#,
            )).unwrap();
    }

    let errtext;
    if stderr.starts_with("Error") {
        errtext = stderr;
    } else if let Some(err) = &RE_EXIT_FAIL.captures(&stdout) {
        errtext = err[0].to_string();
    } else {
        errtext = String::default();
    }

    if errtext.len() > 0 {
        println!("stdout:\n{}", stdout);
        panic!("{}", errtext);
    }

    stdout
}

/// Calls "resim reset"
fn reset_sim() {
    run_command(Command::new("resim")
        .arg("reset"));
}

/// Calls "resim new-account"
///
/// Returns a tuple containing first the new account's address, then
/// its public key, and then last its private key.
fn create_account() -> Account {
    let output = run_command(Command::new("resim")
                             .arg("new-account"));

    lazy_static! {
        static ref RE_ADDRESS: Regex = Regex::new(r"Account component address: (\w*)").unwrap();
        static ref RE_PUBKEY:  Regex = Regex::new(r"Public key: (\w*)").unwrap();
        static ref RE_PRIVKEY: Regex = Regex::new(r"Private key: (\w*)").unwrap();
    }
    
    let address = &RE_ADDRESS.captures(&output).expect("Failed to parse new-account address")[1];
    let pubkey = &RE_PUBKEY.captures(&output).expect("Failed to parse new-account pubkey")[1];
    let privkey = &RE_PRIVKEY.captures(&output).expect("Failed to parse new-account privkey")[1];

    Account {
        address: address.to_string(),
        _pubkey: pubkey.to_string(),
        _privkey:privkey.to_string()
    }
}

/// Publishes the package by calling "resim publish ."
///
/// Returns the new blueprint's address
fn publish_package() -> String {
    let output = run_command(Command::new("resim")
                             .arg("publish")
                             .arg("."));
    lazy_static! {
        static ref RE_ADDRESS: Regex = Regex::new(r"New Package: (\w*)").unwrap();
    }
    
    RE_ADDRESS.captures(&output).expect("Failed to parse new blueprint address")[1].to_string()
}

/// Creates a new SmorgasDAO via
/// rtm/smorgasdao/instantiate_smorgasdao.rtm
///
/// Returns the SmorgasDAO created.
fn instantiate_smorgasdao(account: &Account, package_addr: &str,
                          proposal_duration: u64,
                          quorum: &str,
                          vote_token: &str,
                          id_token: Option<&str>,
                          vote_tally: &str,
                          vote_subsidy: &str)
                          -> SmorgasDaoComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/smorgasdao/instantiate_smorgasdao.rtm")
                             .env("account", &account.address)
                             .env("package", &package_addr)
                             .env("proposal_duration", proposal_duration.to_string())
                             .env("quorum", quorum)
                             .env("vote_token", vote_token)
                             .env("id_token", option_to_tm_string(id_token, "ResourceAddress"))
                             .env("vote_tally", vote_tally)
                             .env("vote_subsidy", vote_subsidy));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_smorgasdao");

    SmorgasDaoComponent {
        address: matches[1].to_string(),
        admin_badge: matches[2].to_string(),
    }
}

/// Creates a new Intermediary via
/// rtm/intermediary/instantiate_intermediary.rtm
///
/// Returns the address of the intermediary created.
fn instantiate_intermediary(account: &Account, package_addr: &str,
                            dao: &SmorgasDaoComponent,
                            controlled: &ControlledComponent)
                            -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/intermediary/instantiate_intermediary.rtm")
                             .env("account", &account.address)
                             .env("package", &package_addr)
                             .env("dao_addr", &dao.address)
                             .env("controlled_addr", &controlled.address)
                             .env("dao_admin_badge", &dao.admin_badge));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ComponentAddress\("(\w*)"\)"#)).unwrap();
    }

    let matches = RE_ADDR.captures(&output).expect(
        "Failed to parse instantiate_intermediary");

    matches[1].to_string()
}

/// Creates a new Controlled component via
/// rtm/controlled/instantiate_controlled.rtm
///
/// Returns the address of the controlled created and the address of
/// its admin badge.
fn instantiate_controlled(account: &Account, package_addr: &str)
                            -> ControlledComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/controlled/instantiate_controlled.rtm")
                             .env("account", &account.address)
                             .env("package", &package_addr));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_controlled");

    ControlledComponent {
        address: matches[1].to_string(),
        admin_badge: matches[2].to_string(),
    }
}


/// Creates a new proposal via
/// rtm/smorgasdao/create_proposal.rtm
///
/// Returns the proposal id created.
fn create_proposal(account: &Account, dao: &SmorgasDaoComponent,
                   ptype: &str,
                   options: &str,
                   title: &str,
                   pitch: &str,
                   deadline: u64,
                   target_component: Option<&str>,
                   target_method: Option<&str>,
                   target_proofs: &str,
                   target_buckets: &str,
                   target_funding: Option<&str>)
                          -> u64
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/smorgasdao/create_proposal.rtm")
                             .env("account", &account.address)
                             .env("component", &dao.address)
                             .env("authority", "None")
                             .env("ptype", ptype)
                             .env("options", options)
                             .env("title", title)
                             .env("pitch", pitch)
                             .env("deadline", deadline.to_string())
                             .env("target_component", option_to_tm_string(target_component, "ComponentAddress"))
                             .env("target_method", option_string_to_tm_string(target_method))
                             .env("target_proofs", target_proofs)
                             .env("target_buckets", target_buckets)
                             .env("target_funding", option_to_tm_string(target_funding, "Decimal")));
    lazy_static! {
        static ref RE_U64: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)u64.*"#)).unwrap();
    }

    let matches = RE_U64.captures(&output).expect(
        "Failed to parse create_proposal");

    matches[1].parse().unwrap()
}

/// Reads the counter in the Controlled component via
/// rtm/controlled/read_count.rtm
///
/// Returns the count.
fn controlled_read_count(account: &Account, comp: &str)
                          -> u64
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/controlled/read_count.rtm")
                             .env("account", &account.address)
                             .env("component", &comp));
    lazy_static! {
        static ref RE_U64: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)u64.*"#)).unwrap();
    }

    let matches = RE_U64.captures(&output).expect(
        "Failed to parse controlled_read_count");

    matches[1].parse().unwrap()
}

/// Reads proposal duration of a DAO via
/// rtm/smorgasdao/read_proposal_duration.rtm
///
/// Returns the duration.
fn read_proposal_duration(account: &Account, comp: &SmorgasDaoComponent)
                          -> u64
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/smorgasdao/read_proposal_duration.rtm")
                             .env("account", &account.address)
                             .env("component", &comp.address));
    lazy_static! {
        static ref RE_U64: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)u64.*"#)).unwrap();
    }

    let matches = RE_U64.captures(&output).expect(
        "Failed to parse read_proposal_duration");

    matches[1].parse().unwrap()
}

/// Adds external badges to the DAO via
/// rtm/smorgasdao/add_external_badges.rtm
///
/// Returns the proposal id created.
fn add_external_badges(account: &Account, dao: &SmorgasDaoComponent,
                       badge_addr: &str,
                       badge_amount: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/smorgasdao/add_external_badges.rtm")
                .env("account", &account.address)
                .env("component", &dao.address)
                .env("badge_addr", badge_addr)
                .env("badge_amount", badge_amount));
}

/// Places an anonymous vote via
/// rtm/smorgasdao/vote_with_receipt.rtm
fn vote_with_receipt(account: &Account, dao: &SmorgasDaoComponent,
                     proposal: u64,
                     vote_token: &str,
                     vote_amount: &str,
                     vote_for: u64)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/smorgasdao/vote_with_receipt.rtm")
                .env("account", &account.address)
                .env("component", &dao.address)
                .env("proposal", proposal.to_string())
                .env("vote_token", vote_token)
                .env("vote_amount", vote_amount)
                .env("vote_for", vote_for.to_string()));
}

/// Attempts to execute a proposal via
/// rtm/smorgasdao/execute_proposal.rtm
fn execute_proposal(account: &Account, dao: &SmorgasDaoComponent,
                     proposal: u64)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/smorgasdao/execute_proposal.rtm")
                .env("account", &account.address)
                .env("component", &dao.address)
                .env("proposal", proposal.to_string()));
}

/// Attempts to execute a proposal via
/// rtm/smorgasdao/execute_proposal_executive.rtm
fn execute_proposal_executive(account: &Account, dao: &SmorgasDaoComponent,
                              proposal: u64,
                              intermediary_addr: &str,
                              followup_method: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/smorgasdao/execute_proposal_executive.rtm")
                .env("account", &account.address)
                .env("dao_addr", &dao.address)
                .env("proposal", proposal.to_string())
                .env("intermediary_addr", intermediary_addr)
                .env("followup", followup_method));
}

/// Attempts to return the Admin token to the DAO
/// rtm/smorgasdao/execute_proposal_return.rtm
fn execute_proposal_return(account: &Account,
                              intermediary_addr: &str,
                              return_method: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/smorgasdao/execute_proposal_return.rtm")
                .env("account", &account.address)
                .env("intermediary_addr", intermediary_addr)
                .env("return_method", return_method));
}


/// Reads the result of a proposal via
/// rtm/smorgasdao/read_proposal_result.rtm
///
/// Returns the result of the proposal.
fn read_proposal_result(account: &Account, dao: &SmorgasDaoComponent,
                        prop_id: u64) -> Option<Option<u64>>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/smorgasdao/read_proposal_result.rtm")
                             .env("account", &account.address)
                             .env("component", &dao.address)
                             .env("prop_id", prop_id.to_string()));
    lazy_static! {
        static ref RE_RES: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#)).unwrap();
        static ref RE_OPT: Regex = Regex::new(concat!(
            r#"Some\((.*)\)"#)).unwrap();
        static ref RE_U64: Regex = Regex::new(concat!(
            r#"Some\((.*)u64\)"#)).unwrap();
    }

    let matches = RE_RES.captures(&output).expect(
        "Failed to parse read_proposal_result");

    let result = matches[1].to_string();
    if result == "None" { return None; }

    let matches = RE_OPT.captures(&result).expect(
        "Failed to parse inner option");

    let result = matches[1].to_string();
    if result == "None" { return Some(None); }

    let matches = RE_U64.captures(&result).expect(
        "Failed to parse option id");

    Some(Some(matches[1].parse().unwrap()))
}
    
/// Converts an Option<&str> where the str is a plain string into a
/// string that can be used inside a transaction manifest. For example,
/// None -> the string None
/// Some("foo") -> the string Some("foo")
fn option_string_to_tm_string(input: Option<&str>) -> String {
    if input.is_none()
    { "None".to_string() } else
    { "Some(\"".to_string() + input.unwrap() + "\")" }
}

/// Converts an Option<&str> where the str is a resource address into a
/// string that can be used inside a transaction manifest. For example,
/// None -> the string None
/// Some(03000...04) -> the string Some(ResourceAddress("03000...04"))
fn option_to_tm_string(input: Option<&str>, wrapped_type: &str) -> String {
    if input.is_none()
    { "None".to_string() } else
    { "Some(".to_string() + wrapped_type + "(\"" + input.unwrap() + "\"))" }
}

/// Calls "resim set-current-epoch ..." to change the epoch
fn set_current_epoch(epoch: u64) {
    run_command(Command::new("resim")
                .arg("set-current-epoch")
                .arg(epoch.to_string())
    );
}


//
// Functionality tests follow below
//


/// Tests the SmorgasDAO
#[test]
fn test_smorgasdao() {
    reset_sim();

    let alice = create_account();

    let package_addr = publish_package();

    // Note that we're voting with XRD here. In a production setting
    // this is absolutely not recommended for a variety of reasons,
    // but for testing purposes it's fine.
    let smorgasdao = instantiate_smorgasdao(&alice, &package_addr,
                                            10,
                                            "Any",
                                            RADIX_TOKEN,
                                            None,
                                            "Linear",
                                            "NoSubsidy");

    // We first do an advisory proposal

    let prop_id = create_proposal(&alice, &smorgasdao,
                                  "Advisory",
                                  r#""Disagree", "Agree", "Twerk""#,
                                  "My proposal",
                                  "I propose all of this blah blah blah (wall of text follows)",
                                  5,
                                  None,
                                  None,
                                  "",
                                  "",
                                  None);

    // Since we're voting anonymously in this test, Alice can place
    // all the votes and vote for different proposals. She will end up
    // with a bunch of different receipts NFTs but that is fine.
    
    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "100",
                      1);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "50",
                      2);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "100",
                      0);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "51",
                      2);

    set_current_epoch(6);


    execute_proposal(&alice, &smorgasdao,
                     prop_id);

    assert_eq!(2,
               read_proposal_result(&alice, &smorgasdao, prop_id).unwrap().unwrap(),
               "Option 2 should win");


    // We will now do an executive vote to change the DAO's max
    // proposal duration

    let controlled_comp = instantiate_controlled(&alice, &package_addr);
    
    let intermediary_component_address = instantiate_intermediary(&alice, &package_addr,
                                                                  &smorgasdao,
                                                                  &controlled_comp);


    let prop_id = create_proposal(&alice, &smorgasdao,
                                  "Executive",
                                  "",
                                  "Increase proposal duration",
                                  "I propose that we increase proposal duration to 100 epochs, as implemented in this call.",
                                  10,
                                  Some(&intermediary_component_address),
                                  Some("store_dao_admin_badge"),
                                  "",
                                  r#"Enum("AdminBadge")"#,
                                  None);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "10",
                      0);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "5",
                      1);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "10",
                      1);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "5.1",
                      1);

    set_current_epoch(11);

    assert_eq!(10,
               read_proposal_duration(&alice, &smorgasdao),
               "Proposal duration should start at 10");

    
    execute_proposal_executive(&alice, &smorgasdao,
                               prop_id,
                               &intermediary_component_address,
                               "execute_dao_call");
    execute_proposal_return(&alice,
                            &intermediary_component_address,
                            "return_dao_admin_token");

    assert_eq!(1,
               read_proposal_result(&alice, &smorgasdao, prop_id).unwrap().unwrap(),
               "Option 1 should win");

    assert_eq!(100,
               read_proposal_duration(&alice, &smorgasdao),
               "Proposal duration should now be 100");
    

    

    // Give the Controlled component badge to the DAO so it can start
    // controlling it.

    add_external_badges(&alice, &smorgasdao, &controlled_comp.admin_badge, "1");
    
    // We will now execute an executive vote to exert control over a
    // protected third-party component (the controlled component).

    let prop_id = create_proposal(&alice, &smorgasdao,
                                  "Executive",
                                  "",
                                  "Count them",
                                  "I propose that we increase the count of the controlled component.",
                                  15,
                                  Some(&intermediary_component_address),
                                  Some("call_controlled"),
                                  "",
                                  &format!(r#"Enum("ExternalFungibleBadge",ResourceAddress("{}"),Decimal("1"))"#,
                                           controlled_comp.admin_badge.to_string()),
                                  None);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "10",
                      0);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "5",
                      1);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "10",
                      1);

    vote_with_receipt(&alice, &smorgasdao,
                      prop_id,
                      RADIX_TOKEN,
                      "5.1",
                      1);

    set_current_epoch(16);

    assert_eq!(0,
               controlled_read_count(&alice, &controlled_comp.address),
               "Controlled component should start with a count of 0");

    execute_proposal(&alice, &smorgasdao,
                     prop_id);

    assert_eq!(1,
               read_proposal_result(&alice, &smorgasdao, prop_id).unwrap().unwrap(),
               "Option 1 should win");

    assert_eq!(1,
               controlled_read_count(&alice, &controlled_comp.address),
               "Controlled component should end with a count of 1");
}

