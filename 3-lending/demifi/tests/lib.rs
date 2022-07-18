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
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

const RADIX_TOKEN: &str = "030000000000000000000000000000000000000000000000000004";

#[derive(Debug)]
struct Account {
    address: String,
    _pubkey: String,
    privkey: String,
}

#[derive(Debug)]
struct ParticipantsComponent {
    address: String,
    nft_address: String,
    owner_nfid: String,
}

#[derive(Debug)]
struct RequestorComponent {
    address: String,
    nft_address: String,
    admin_badge_address: String,
    config_badge_address: String
}

#[derive(Debug)]
struct AcceptorComponent {
    address: String,
    nft_address: String,
}

/// Runs a command line program, panicking if it fails and returning
/// its stdout if it succeeds
fn run_command(command: &mut Command) -> String {
    let output = command
        .output()
        .expect("Failed to run command line");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    if !output.status.success() {
        println!("stdout:\n{}", stdout);
        panic!("{}", stderr);
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
        privkey: privkey.to_string()
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

/// Creates a new Participants catalog via
/// rtm/participants/instantiate_participant_catalog.rtm
///
/// Returns the catalog created.
fn instantiate_participant_catalog(account_addr: &str, package_addr: &str)
                                   -> ParticipantsComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/instantiate_participant_catalog.rtm")
                             .env("account", account_addr)
                             .env("package", &package_addr)
                             .env("admin_badge_name", "None")
                             .env("nft_resource_name", "None")
                             .env("root_participant_name", "None"));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#,
            r#"NonFungibleId\("(\w*)"\)"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_participant_catalog");

    ParticipantsComponent {
        address: matches[1].to_string(),
        nft_address: matches[2].to_string(),
        owner_nfid: matches[3].to_string(),
    }
}

/// Creates a new Participants catalog via
/// rtm/participants/new_participant.rtm
///
/// Returns the catalog created.
fn new_participant(component_addr: &str, account_addr: &str, name: &str, url: &str, id_ref: &str,
                   expect_sponsor: Option<&str> ) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/new_participant.rtm")
                             .env("component", component_addr)
                             .env("account", account_addr)
                             .env("name", &name)
                             .env("url", &url)
                             .env("id_ref", &id_ref)
                             .env("expect_sponsor",
                                  option_to_tm_string(expect_sponsor,
                                                      "NonFungibleId")));
    lazy_static! {
        static ref RE_NFID: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(.*"#,
            r#"NonFungibleId\("(\w*)"\)"#)).unwrap();
    }

    RE_NFID.captures(&output).expect("Failed to parse new_participant")[1].to_string()
}

/// Has one participant endorse another via
/// rtm/participants/endorse.rtm
fn endorse(account_addr: &str, component_addr: &str,
           nft_address: &str, endorser: &str, endorse_target: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/endorse.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("endorser_nfid", endorser)
                .env("endorse_target_nfid", endorse_target));
}

/// Has one participant unendorse another via
/// rtm/participants/unendorse.rtm
fn unendorse(account_addr: &str, component_addr: &str,
             nft_address: &str, endorser: &str, unendorse_target: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/unendorse.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("endorser_nfid", endorser)
                .env("unendorse_target_nfid", unendorse_target));
}

/// Checks if one participant endorses another, via
/// rtm/participants/do_i_endorse.rtm
fn do_i_endorse(component_addr: &str, endorser_nfid: &str, endorse_target_nfid: &str)
                -> bool
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/do_i_endorse.rtm")
                             .env("component", component_addr)
                             .env("endorser_nfid", endorser_nfid)
                             .env("endorse_target_nfid", endorse_target_nfid));

    lazy_static! {
        static ref RE_BOOL: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (\w*)"#)).unwrap();
    }

    RE_BOOL.captures(&output).expect("Failed to parse do_i_endorse")[1].parse().unwrap()
}

/// A participant sponsors another via rtm/participants/sponsor.rtm
fn sponsor(account_addr: &str, component_addr: &str,
           nft_address: &str, sponsor_nfid: &str, sponsorship_target_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/sponsor.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("sponsor_nfid", sponsor_nfid)
                .env("sponsorship_target_nfid", sponsorship_target_nfid));
}

/// Has one participant unsponsor another via
/// rtm/participants/unsponsor.rtm
fn unsponsor(account_addr: &str, component_addr: &str,
           nft_address: &str, sponsor_nfid: &str, sponsorship_target_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/unsponsor.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("sponsor_nfid", sponsor_nfid)
                .env("sponsorship_target_nfid", sponsorship_target_nfid));
}

/// Has one participant expect a sponsor, via
/// rtm/participants/expect_sponsor.rtm
fn expect_sponsor(account_addr: &str, component_addr: &str,
                  nft_address: &str, hopeful_nfid: &str, sponsor_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/expect_sponsor.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("hopeful_nfid", hopeful_nfid)
                .env("sponsor_nfid", sponsor_nfid));
}

/// Has one participant unendorse another via
/// rtm/participants/unendorse.rtm
fn participants_read_data(component_addr: &str, participant_nfid: &str)
                          -> (String, String, String, Option<String>, Option<String>)
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/read_data.rtm")
                             .env("component", component_addr)
                             .env("participant_nfid", participant_nfid));

    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\("#,
            r#""(.*)", "#,
            r#""(.*)", "#,
            r#""(.*)", "#,
            r#"(.*), "#,
            r#"(.*)\)"#,
        )).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect("Failed to parse Participant::read_data");
    
    (matches[1].to_string(),
     matches[2].to_string(),
     matches[3].to_string(),
     tm_string_to_option(&matches[4], "NonFungibleAddress"),
     tm_string_to_option(&matches[5], "NonFungibleAddress"))
}

/// Read a participant's endorsements via
/// rtm/participants/read_endorsements.rtm
fn read_endorsements(component_addr: &str, participant_nfid: &str)
                          -> HashSet<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/read_endorsements.rtm")
                             .env("component", component_addr)
                             .env("participant_nfid", participant_nfid));

    lazy_static! {
        static ref RE_SET: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashSet<NonFungibleId>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_ELEMENT: Regex = Regex::new(concat!(
            r#"NonFungibleId\("(\w*)"\)"#,
        )).unwrap();
    }

    // There absolutely has to be a better way to do this - but until
    // such an approach is found we parse the output piecemeal to
    // produce our result
    let hashset = RE_SET.captures(&output).expect("Failed to parse read_endorsements")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashset).collect();
    let mut nfids: HashSet<String> = HashSet::new();
    for element in elements {
        if element == "" { break; }
        let nfid = RE_ELEMENT.captures(&element).expect("Failed to parse nfid")[1].to_string();
        nfids.insert(nfid);
    }

    nfids
}

/// Finds the NonFungibleAddress of the catalog creator via
/// rtm/participants/read_catalog_creator.rtm
fn read_catalog_creator(component_addr: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/read_catalog_creator.rtm")
                             .env("component", component_addr));

    lazy_static! {
        static ref RE_CREATOR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ NonFungibleAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_CREATOR.captures(&output).expect("Failed to parse read_catalog_creator")[1].to_string()
}

/// Finds the ResourceAddress of Participants NFTs via
/// rtm/participants/read_participants_nft_addr.rtm
fn read_participants_nft_addr(component_addr: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/read_participants_nft_addr.rtm")
                             .env("component", component_addr));

    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_ADDR.captures(&output).expect("Failed to parse read_participants_nft_addr")[1].to_string()
}


/// Has a participant change its name via
/// rtm/participants/change_name.rtm
fn participants_change_name(account_addr: &str, component_addr: &str,
                            nft_address: &str, nfid: &str,
                            new_name: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/change_name.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("nfid", nfid)
                .env("new_name", new_name));
}

/// Has a participant change its URL via
/// rtm/participants/change_url.rtm
fn participants_change_url(account_addr: &str, component_addr: &str,
                           nft_address: &str, nfid: &str,
                           new_url: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/change_url.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("nfid", nfid)
                .env("new_url", new_url));
}

/// Has a participant change its ID ref via
/// rtm/participants/change_id_ref.rtm
fn participants_change_id_ref(account_addr: &str, component_addr: &str,
                              nft_address: &str, nfid: &str,
                              new_id_ref: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/participants/change_id_ref.rtm")
                .env("account", account_addr)
                .env("component", component_addr)
                .env("nft_address", nft_address)
                .env("nfid", nfid)
                .env("new_id_ref", new_id_ref));
}

/// Creates a new LoanRequestor instance, via
/// rtm/loanrequestor/instantiate_requestor.rtm
fn instantiate_requestor(account_addr: &str, package_addr: &str,
                         participants_nft_addr: &str)
                         -> RequestorComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/instantiate_requestor.rtm")
                             .env("account", account_addr)
                             .env("package", &package_addr)
                             .env("participants_nft_addr", &participants_nft_addr)
                             .env("admin_badge_name", "None")
                             .env("nft_resource_name", "None"));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\)"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_requestor");

    RequestorComponent {
        address: matches[1].to_string(),
        nft_address: matches[2].to_string(),
        admin_badge_address: matches[3].to_string(),
        config_badge_address: matches[4].to_string(),
    }
}

/// Ties a requestor to an acceptor, via
/// rtm/loanrequestor/set_loan_acceptor.rtm
fn set_loan_acceptor(component_addr: &str, account_addr: &str,
                     config_badge_addr: &str, acceptor_addr: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanrequestor/set_loan_acceptor.rtm")
                .env("component", component_addr)
                .env("account", account_addr)
                .env("config_badge_addr", &config_badge_addr)
                .env("acceptor_addr", &acceptor_addr));
}

/// Opens up a new loan request, via
/// rtm/loanrequestor/request_loan.rtm
fn request_loan(requestor_addr: &str, account_addr: &str,
                participants_nft_addr: &str, borrower_nfid: &str,
                request_token: &str, request_amount: &str,
                minimum_share: &str,
                pledge_lock_epochs: u64,
                loan_filled_lock_epochs: u64,
                promise_payment_intervals: u64,
                promise_installments: u64,
                promise_amount_per_installment: &str,
                loan_purpose_summary: &str,
                loan_purpose_url: &str)
                -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/request_loan.rtm")
                             .env("component", requestor_addr)
                             .env("account", account_addr)
                             .env("participants_nft_addr", participants_nft_addr)
                             .env("borrower_nfid", borrower_nfid)
                             .env("request_token", request_token)
                             .env("request_amount", request_amount)
                             .env("minimum_share", minimum_share)
                             .env("pledge_lock_epochs", pledge_lock_epochs.to_string())
                             .env("loan_filled_lock_epochs", loan_filled_lock_epochs.to_string())
                             .env("promise_payment_intervals", promise_payment_intervals.to_string())
                             .env("promise_installments", promise_installments.to_string())
                             .env("promise_amount_per_installment", promise_amount_per_installment)
                             .env("loan_purpose_summary", loan_purpose_summary)
                             .env("loan_purpose_url", loan_purpose_url));

    lazy_static! {
        static ref RE_NFID: Regex = Regex::new(concat!(
            r#"Instruction Outputs:*\n.*\n.*\n"#,
            r#".─ Tuple\(.*NonFungibleId\("(\w*)"\)"#,
        )).unwrap();
    }

    RE_NFID.captures(&output).expect("Failed to parse request_loan")[1].to_string()
}

/// Pledges funds towards a loan request, via
/// rtm/loanrequestor/pledge_loan.rtm
fn pledge_loan(requestor_addr: &str, account_addr: &str,
               participants_nft_addr: &str, token_resource: &str,
               lender_nfid: &str, loanrequest_nfid: &str,
               amount: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanrequestor/pledge_loan.rtm")
                .env("component", requestor_addr)
                .env("account", account_addr)
                .env("participants_nft_addr", participants_nft_addr)
                .env("token_resource", token_resource)
                .env("lender_nfid", lender_nfid)
                .env("loanrequest_nfid", loanrequest_nfid)
                .env("amount", amount));
}

/// Rescinds a pledge towards a loan request, via
/// rtm/loanrequestor/rescind_loan.rtm
fn rescind_loan(requestor_addr: &str, account_addr: &str,
               participants_nft_addr: &str, 
               lender_nfid: &str, loanrequest_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanrequestor/rescind_loan.rtm")
                .env("component", requestor_addr)
                .env("account", account_addr)
                .env("participants_nft_addr", participants_nft_addr)
                .env("lender_nfid", lender_nfid)
                .env("loanrequest_nfid", loanrequest_nfid));
}

/// Starts a loan from a fully funded loan request, via
/// rtm/loanrequestor/start_loan.rtm
fn start_loan(requestor: &RequestorComponent, account_addr: &str,
              participants_nft_addr: &str, borrower_nfid: &str,
              loanrequest_nfid: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/start_loan.rtm")
                             .env("component", &requestor.address)
                             .env("account", account_addr)
                             .env("participants_nft_addr", participants_nft_addr)
                             .env("borrower_nfid", borrower_nfid)
                             .env("loanrequest_nft_addr", &requestor.nft_address)
                             .env("loanrequest_nfid", loanrequest_nfid));
    lazy_static! {
        static ref RE_NFID: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".*\n.*\n.*\n.*\n"#,
            r#".─ .*NonFungibleId\("(\w*)"\).*"#,
        )).unwrap();
    }

    RE_NFID.captures(&output).expect("Failed to parse start_loan")[1].to_string()
}

/// Cancels a loan request, via rtm/loanrequestor/cancel_request.rtm
fn cancel_request(requestor: &RequestorComponent, account_addr: &str,
                  participants_nft_addr: &str, 
                  borrower_nfid: &str, loanrequest_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanrequestor/cancel_request.rtm")
                .env("component", &requestor.address)
                .env("account", account_addr)
                .env("participants_nft_addr", participants_nft_addr)
                .env("borrower_nfid", borrower_nfid)
                .env("loanrequest_nft_addr", &requestor.nft_address)
                .env("loanrequest_nfid", loanrequest_nfid));
}

/// Burns a loan request, via
/// rtm/loanrequestor/burn.rtm
fn loan_request_burn(requestor: &RequestorComponent, account_addr: &str,
              participants_nft_addr: &str, borrower_nfid: &str,
              loanrequest_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanrequestor/burn.rtm")
                .env("component", &requestor.address)
                .env("account", account_addr)
                .env("participants_nft_addr", participants_nft_addr)
                .env("borrower_nfid", borrower_nfid)
                .env("loanrequest_nft_addr", &requestor.nft_address)
                .env("loanrequest_nfid", loanrequest_nfid));
}

/// Queries loan purpose, via
/// rtm/loanrequestor/read_loan_purpose.rtm
fn loan_request_read_loan_purpose(requestor: &RequestorComponent,
                                  loanrequest_nfid: &str)
                                  -> (String, String)
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_loan_purpose.rtm")
                             .env("component", &requestor.address)
                             .env("loanrequest_nfid", loanrequest_nfid));
    lazy_static! {
        static ref RE_PURPOSE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ Tuple\("(.*)", "(.*)"\)"#,
        )).unwrap();
    }
    let matches = RE_PURPOSE.captures(&output).expect(
        "Failed to parse loan_request_read_loan_purpose");

    (matches[1].to_string(), matches[2].to_string())
}

/// Queries loan cancellation status, via
/// rtm/loanrequestor/read_is_cancelled.rtm
fn read_is_cancelled(requestor: &RequestorComponent,
                     loanrequest_nfid: &str) -> bool
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_is_cancelled.rtm")
                             .env("component", &requestor.address)
                             .env("loanrequest_nfid", loanrequest_nfid));
    lazy_static! {
        static ref RE_PURPOSE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ (.*)"#,
        )).unwrap();
    }
    RE_PURPOSE.captures(&output).expect("Failed to parse read_is_cancelled")[1].parse().unwrap()
}

/// Queries borrower id, via
/// rtm/loanrequestor/read_is_cancelled.rtm
fn loan_request_read_borrower_id(requestor: &RequestorComponent,
                                 loanrequest_nfid: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_borrower_id.rtm")
                             .env("component", &requestor.address)
                             .env("loanrequest_nfid", loanrequest_nfid));
    lazy_static! {
        static ref RE_NFID: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ NonFungibleId\("(.*)"\)"#,
        )).unwrap();
    }
    RE_NFID.captures(&output).expect("Failed to parse read_borrower_id")[1].to_string()
}

/// Queries loan request NFT data, via
/// rtm/loanrequestor/read_data.rtm
fn loan_request_read_data(requestor: &RequestorComponent,
                          loanrequest_nfid: &str)
                          -> (String, String, String,
                              u64, u64, Option<u64>,
                              u64, u64, u64, String)
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_data.rtm")
                             .env("component", &requestor.address)
                             .env("loanrequest_nfid", loanrequest_nfid));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#"...Tuple\("#,
            r#"ResourceAddress\("(\w*)"\), "#,
            r#"Decimal\("(.*)"\), "#,
            r#"Decimal\("(.*)"\), "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"(.*), "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"Decimal\("(.*)"\)\)"#,
        )).unwrap();
    }
    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse read_data");

    (matches[1].to_string(),
     matches[2].to_string(),
     matches[3].to_string(),
     matches[4].parse().unwrap(),
     matches[5].parse().unwrap(),
     maybe_some_u64(&matches[6].to_string()),
     matches[7].parse().unwrap(),
     matches[8].parse().unwrap(),
     matches[9].parse().unwrap(),
     matches[10].to_string())
}

/// Queries loan request NFT address, via
/// rtm/loanrequestor/read_request_nft_addr.rtm
fn read_request_nft_addr(requestor: &RequestorComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_request_nft_addr.rtm")
                             .env("component", &requestor.address));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ ResourceAddress\("(.*)"\)"#,
        )).unwrap();
    }
    RE_ADDR.captures(&output).expect("Failed to parse read_request_nft_addr")[1].to_string()
}

/// Queries loan request Participants NFT address, via
/// rtm/loanrequestor/read_participants_nft_addr.rtm
fn loan_request_read_participants_nft_addr(requestor: &RequestorComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_participants_nft_addr.rtm")
                             .env("component", &requestor.address));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ ResourceAddress\("(.*)"\)"#,
        )).unwrap();
    }
    RE_ADDR.captures(&output).expect(
        "Failed to parse loan_request_read_participants_nft_addr")[1].to_string()
}

/// Queries the loan acceptor address, via
/// rtm/loanrequestor/read_loan_acceptor.rtm
fn read_loan_acceptor(requestor: &RequestorComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanrequestor/read_loan_acceptor.rtm")
                             .env("component", &requestor.address));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ (.*)"#,
        )).unwrap();
    }
    tm_string_to_option(
        &RE_ADDR.captures(&output).expect(
            "Failed to parse read_loan_acceptor")[1].to_string(),
        "ComponentAddress")
}

/// Creates a new LoanAcceptor instance, via
/// rtm/loanacceptor/instantiate_loan_acceptor.rtm
fn instantiate_loan_acceptor(account_addr: &str, package_addr: &str,
                             participants_nft_addr: &str,
                             requestor_admin_addr: &str,
                             facilitator: Option<&str>,
                             facilitator_fee: &str)
                             -> AcceptorComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/instantiate_loan_acceptor.rtm")
                             .env("package", &package_addr)
                             .env("account", account_addr)
                             .env("participants_nft_addr", &participants_nft_addr)
                             .env("requestor_admin_addr", &requestor_admin_addr)
                             .env("facilitator", option_to_tm_string(facilitator,
                                                                     "NonFungibleId"))
                             .env("facilitator_fee", facilitator_fee)
                             .env("admin_badge_name", "None")
                             .env("nft_resource_name", "None"));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#,
        )).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_loan_acceptor");

    AcceptorComponent {
        address: matches[1].to_string(),
        nft_address: matches[2].to_string(),
    }
}

/// Pays an installment on a loan, via
/// rtm/loanacceptor/pay_installment.rtm
fn pay_installment(acceptor: &AcceptorComponent, account: &Account,
                   loan_nfid: &str,
                   token: &str, amount: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/pay_installment.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("loan_nfid", loan_nfid)
                .env("token_resource", token)
                .env("amount", amount));
}

/// Checks if a loan is in arrears, via
/// rtm/loanacceptor/is_in_arrears.rtm
fn is_in_arrears(acceptor: &AcceptorComponent,
                 loan_nfid: &str) -> bool
{
    let output =
        run_command(Command::new("resim")
                    .arg("run")
                    .arg("rtm/loanacceptor/is_in_arrears.rtm")
                    .env("component", &acceptor.address)
                    .env("loan_nfid", loan_nfid));
    lazy_static! {
        static ref RE_BOOL: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ (.*)"#,
        )).unwrap();
    }
    RE_BOOL.captures(&output).expect("Failed to parse is_in_arrears")[1].parse().unwrap()
}

/// A lender approves the clearing of a loan's arrears status, via
/// rtm/loanacceptor/approve_clear_arrears.rtm
fn approve_clear_arrears(acceptor: &AcceptorComponent, account: &Account,
                        participants: &ParticipantsComponent,
                        lender_nfid: &str, loan_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/approve_clear_arrears.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("lender_nfid", &lender_nfid)
                .env("loan_nfid", loan_nfid));
}

/// A lender disapproves the clearing of a loan's arrears status, via
/// rtm/loanacceptor/disapprove_clear_arrears.rtm
fn disapprove_clear_arrears(acceptor: &AcceptorComponent, account: &Account,
                            participants: &ParticipantsComponent,
                            lender_nfid: &str, loan_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/disapprove_clear_arrears.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("lender_nfid", &lender_nfid)
                .env("loan_nfid", loan_nfid));
}

/// A lender requests an evaluation of a loan's arrears status, via
/// rtm/loanacceptor/update_arrears.rtm
fn update_arrears(acceptor: &AcceptorComponent, account: &Account,
                        participants: &ParticipantsComponent,
                        lender_nfid: &str, loan_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/update_arrears.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("lender_nfid", &lender_nfid)
                .env("loan_nfid", loan_nfid));
}

/// Claims facilitator rewards, via
/// rtm/loanacceptor/claim_facilitator_rewards.rtm
fn claim_facilitator_rewards(acceptor: &AcceptorComponent, account: &Account,
                             participants: &ParticipantsComponent,
                             facilitator_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/claim_facilitator_rewards.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("facilitator_nfid", &facilitator_nfid));
}

/// Claims lender rewards, via
/// rtm/loanacceptor/claim_lender_rewards.rtm
fn claim_lender_rewards(acceptor: &AcceptorComponent, account: &Account,
                        participants: &ParticipantsComponent,
                        lender_nfid: &str)
{
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/loanacceptor/claim_lender_rewards.rtm")
                .env("component", &acceptor.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("lender_nfid", &lender_nfid));
}

/// Queries loan NFT data, via
/// rtm/loanacceptor/read_loan_data.rtm
fn read_loan_data(acceptor: &AcceptorComponent,
                  loan_nfid: &str)
                  -> (String, String,
                      u64, u64, u64, u64,
                      String)
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_loan_data.rtm")
                             .env("component", &acceptor.address)
                             .env("loan_nfid", loan_nfid));
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#"...Tuple\("#,
            r#"Decimal\("(.*)"\), "#,
            r#"ResourceAddress\("(\w*)"\), "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"(.*)u64, "#,
            r#"Decimal\("(.*)"\)\)"#,
        )).unwrap();
    }
    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse read_data");

    (matches[1].to_string(),
     matches[2].to_string(),
     matches[3].parse().unwrap(),
     matches[4].parse().unwrap(),
     matches[5].parse().unwrap(),
     matches[6].parse().unwrap(),
     matches[7].to_string())
}

/// Queries loan purpose, via
/// rtm/loanacceptor/read_loan_purpose.rtm
fn loan_read_loan_purpose(acceptor: &AcceptorComponent,
                          loan_nfid: &str)
                          -> (String, String)
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_loan_purpose.rtm")
                             .env("component", &acceptor.address)
                             .env("loan_nfid", loan_nfid));
    lazy_static! {
        static ref RE_PURPOSE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ Tuple\("(.*)", "(.*)"\)"#,
        )).unwrap();
    }
    let matches = RE_PURPOSE.captures(&output).expect(
        "Failed to parse loan_read_loan_purpose");

    (matches[1].to_string(), matches[2].to_string())
}

/// Read the current arrears voting status for a loan, via
/// rtm/loanacceptor/read_loan_arrears_votes.rtm
fn read_loan_arrears_votes(acceptor: &AcceptorComponent, loan_nfid: &str)
                           -> HashSet<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_loan_arrears_votes.rtm")
                             .env("component", &acceptor.address)
                             .env("loan_nfid", loan_nfid));

    lazy_static! {
        static ref RE_SET: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashSet<NonFungibleId>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_ELEMENT: Regex = Regex::new(concat!(
            r#"NonFungibleId\("(\w*)"\)"#,
        )).unwrap();
    }

    let hashset = RE_SET.captures(&output).expect("Failed to parse read_endorsements")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashset).collect();
    let mut nfids: HashSet<String> = HashSet::new();
    for element in elements {
        if element == "" { break; }
        let nfid = RE_ELEMENT.captures(&element).expect("Failed to parse nfid")[1].to_string();
        nfids.insert(nfid);
    }

    nfids
}

/// Queries borrower id, via rtm/loanacceptor/read_borrower.rtm
fn loan_read_borrower(acceptor: &AcceptorComponent,
                          loan_nfid: &str)
                          -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_borrower.rtm")
                             .env("component", &acceptor.address)
                             .env("loan_nfid", loan_nfid));
    lazy_static! {
        static ref RE_NFADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ NonFungibleAddress\("(.*)"\)"#,
        )).unwrap();
    }
    RE_NFADDR.captures(&output).expect("Failed to parse loan_read_borrower")[1].to_string()
}

/// Read the lenders for a loan, via
/// rtm/loanacceptor/read_lenders.rtm
fn read_lenders(acceptor: &AcceptorComponent, loan_nfid: &str)
                -> HashMap<String, String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_lenders.rtm")
                             .env("component", &acceptor.address)
                             .env("loan_nfid", loan_nfid));

    lazy_static! {
        static ref RE_MAP: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashMap<NonFungibleId, Decimal>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_KEY: Regex = Regex::new(concat!(
            r#"NonFungibleId\("(\w*)"\)"#,
        )).unwrap();
        static ref RE_VALUE: Regex = Regex::new(concat!(
            r#"Decimal\("(\w*)"\)"#,
        )).unwrap();
    }

    let hashmap = RE_MAP.captures(&output).expect("Failed to parse read_lenders")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashmap).collect();
    let mut lenders: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    for element in elements {
        if element == "" { break; }
        if key.is_none() {
            key = Some(RE_KEY.captures(&element).expect("Failed to parse key")[1].to_string());
        } else {
            lenders.insert(
                key.unwrap(),
                RE_VALUE.captures(&element).expect("Failed to parse value")[1].to_string());
            key = None;
        }
    }

    lenders
}

/// Queries the Participants NFT resource address, via
/// rtm/loanacceptor/read_participants_nft_addr.rtm
fn loan_read_participants_nft_addr(acceptor: &AcceptorComponent)
                                   -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_participants_nft_addr.rtm")
                             .env("component", &acceptor.address));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ ResourceAddress\("(.*)"\)"#,
        )).unwrap();
    }
    RE_ADDR.captures(&output).expect(
        "Failed to parse loan_read_participants_nft_addr")[1].to_string()
}

/// Queries the Loan NFT resource address, via
/// rtm/loanacceptor/read_loan_nft_addr.rtm
fn loan_read_loan_nft_addr(acceptor: &AcceptorComponent)
                           -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_loan_nft_addr.rtm")
                             .env("component", &acceptor.address));
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ ResourceAddress\("(.*)"\)"#,
        )).unwrap();
    }
    RE_ADDR.captures(&output).expect(
        "Failed to parse loan_read_loan_nft_addr")[1].to_string()
}

/// Queries facilitator id, via rtm/loanacceptor/read_facilitator.rtm
fn read_facilitator(acceptor: &AcceptorComponent)
                      -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_facilitator.rtm")
                             .env("component", &acceptor.address));
    lazy_static! {
        static ref RE_NFADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ (.*)"#,
        )).unwrap();
    }
    tm_string_to_option(
        &RE_NFADDR.captures(&output).expect("Failed to parse loan_read_borrower")[1],
        "NonFungibleAddress")
}

/// Queries facilitator fee, via
/// rtm/loanacceptor/read_facilitator_fee.rtm
fn read_facilitator_fee(acceptor: &AcceptorComponent)
                        -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/loanacceptor/read_facilitator_fee.rtm")
                             .env("component", &acceptor.address));
    lazy_static! {
        static ref RE_DEC: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n"#,
            r#".─ Decimal\("(\w*)"\)"#,
        )).unwrap();
    }
    RE_DEC.captures(&output).expect("Failed to parse read_facilitator_fee")[1].to_string()
}

/// Changes the default account by calling "resim set-default-account ..."
fn set_default_account(account: &Account) {
    run_command(Command::new("resim")
                .arg("set-default-account")
                .arg(&account.address)
                .arg(&account.privkey));
}

/// Retreives a user's current balance for the requested asset by
/// calling "resim show ..."
fn get_balance(account: &Account, resource_addr: &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("show")
                             .arg(&account.address));
    let regexp = r#".─ \{ amount: ([\d.]*), resource address: "#.to_string() + resource_addr + ",";
    let re_balance: Regex = Regex::new(&regexp).unwrap();

    re_balance.captures(&output).expect("Failed to parse balance")[1].to_string()
}


/// Given a string of the form "None" or "Some(string)" returns either
/// a None or a Some(string)
fn _maybe_some(input: &str) -> Option<String> {
    if input == "None" {
        return None;
    }
    lazy_static! {
        static ref RE_OPTION: Regex = Regex::new(r#"Some\((.*)\)"#).unwrap();
    }
    Some(RE_OPTION.captures(&input).expect("Invalid string-form Option")[1].to_string())
}

/// Given a u64 of the form "None" or "Some(Xu64)" returns either
/// a None or a Some(X)
fn maybe_some_u64(input: &str) -> Option<u64> {
    if input == "None" {
        return None;
    }
    lazy_static! {
        static ref RE_OPTION: Regex = Regex::new(r#"Some\((.*)u64\)"#).unwrap();
    }
    Some(RE_OPTION.captures(&input).expect("Invalid string-form Option")[1].parse().unwrap())
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

/// Converts an string of the type Option<T> to an actual Option<String>,
/// typically used to parse the output of the resim tool.
/// For example,
/// the string None -> None
/// the string Some(T("03000...04")) -> Some(T("03000...04"))
fn tm_string_to_option(input: &str, wrapped_type: &str) -> Option<String> {
    lazy_static! {
        static ref RE_SOME: Regex = Regex::new(concat!(
            r#"Some\((.*)\("(\w*)"\)\)"#)).unwrap();
    }
    if input == "None" { None } else
    {
        let matches = RE_SOME.captures(input).expect("Couldn't parse TM string");
        assert_eq!(wrapped_type, matches[1].to_string(),
                   "Wrong wrapped type in Some<T>");
        Some(matches[2].to_string())
    }
}

/// Calls "resim set-current-epoch ..." to change the epoch
fn set_current_epoch(epoch: u64) {
    run_command(Command::new("resim")
                .arg("set-current-epoch")
                .arg(epoch.to_string())
    );
}

/// Calls "resim new-badge-fixed ..." to create a new badge type.
/// Returns the resource address of the new badge.
fn _new_badge_fixed(name: &str, symbol: &str, supply: &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("new-badge-fixed")
                             .arg("--name")
                             .arg(&name)
                             .arg("--symbol")
                             .arg(&symbol)
                             .arg(&supply));
    lazy_static! {
        static ref RE_BADGE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W"#,
            r#".─ Tuple\(ResourceAddress\("(.*)""#)).unwrap();
    }

    RE_BADGE_ADDR.captures(&output).expect("Failed to parse new badge address")[1].to_string()
}

/// Calls "resim new-token-fixed ..." to create a new token.
/// Returns the resource address of the new token.
fn _new_token_fixed(name: &str, symbol: &str, supply: &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("new-token-fixed")
                             .arg("--name")
                             .arg(&name)
                             .arg("--symbol")
                             .arg(&symbol)
                             .arg(&supply));
    lazy_static! {
        static ref RE_TOKEN_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W"#,
            r#".─ Tuple\(ResourceAddress\("(.*)""#)).unwrap();
    }

    RE_TOKEN_ADDR.captures(&output).expect("Failed to parse new token address")[1].to_string()
}

/// Calls "resim transfer ..." to transfer tokens from the default
/// account to another.
fn _transfer_tokens(to: &Account, asset: &str, amount: &str) {
    run_command(Command::new("resim")
                .arg("transfer")
                .arg(&amount)
                .arg(&asset)
                .arg(&to.address));
}


/// Asserts the expected values of the three string fields in
/// Participant NFT data. Does not assert the NFT addresses
/// because it's gnarly: Those are tested through more extensive
/// sponsorship scenarios.
fn assert_participant_data(component_addr: &str, nfid: &str,
                           name: &str, url: &str, id_ref: &str,
                           some_sponsor: bool, some_expect_sponsor: bool)
{
    let (name_actual, url_actual, id_ref_actual,
         sponsor, expect_sponsor) =
        participants_read_data(component_addr, nfid);
    assert_eq!(name, name_actual, "Participant name mismatch");
    assert_eq!(url, url_actual, "Participant URL mismatch");
    assert_eq!(id_ref, id_ref_actual, "Participant id_ref mismatch");
    assert_eq!(some_sponsor, sponsor.is_some(), "Participant sponsor mismatch");
    assert_eq!(some_expect_sponsor, expect_sponsor.is_some(),
               "Participant expect_sponsor mismatch");
}

/// Utility function to create a Participants catalog
fn setup_catalog(user_addr: &str, package_addr: &str)
                 -> (ParticipantsComponent, String)
{
    let participants = instantiate_participant_catalog(
        user_addr, package_addr);

    assert_eq!(participants.nft_address,
               read_participants_nft_addr(&participants.address),
               "Participants instance should report correct NFT address");

    let user_p_nfid = new_participant(&participants.address,
                                      &user_addr,
                                      "C. A. Talog",
                                      "", "", None);
    (participants, user_p_nfid)
}


//
// Functionality tests follow below
//


/// Tests a participant's ability to change its text fields
#[test]
fn test_participants_change_data() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let participants = instantiate_participant_catalog(
        &alice.address, &package_addr);

    let alice_p_nfid = new_participant(&participants.address,
                                       &alice.address,
                                       "Alice",
                                       "file:alice.html",
                                       "no id",
                                       None);

    participants_change_name(&alice.address,
                             &participants.address,
                             &participants.nft_address,
                             &alice_p_nfid,
                             "Deborah");
    participants_change_url(&alice.address,
                            &participants.address,
                            &participants.nft_address,
                            &alice_p_nfid,
                            "http://debo.rah");
    participants_change_id_ref(&alice.address,
                               &participants.address,
                               &participants.nft_address,
                               &alice_p_nfid,
                               "I have zero knowledge of that");

    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Deborah",
                            "http://debo.rah",
                            "I have zero knowledge of that",
                            false,
                            false);
}

/// Runs through a basic scenario of creating a number of Participants
/// in a catalog and having them sponsor and endorse eachother.
///
/// Note that since we are not currently connected to an id system we
/// use random silly strings for the id_ref just to check that they
/// work.
///
/// Users here typically have two variables associated with them, one
/// is the Radix account owner (e.g. alice) and the other is their
/// Participant NFT id (e.g. alice_p_nfid). Some will have multiple of
/// the latter since the system itself does not force a 1:1
/// relationship on them. (In real use the problem arises for such a
/// user when these multiple aliases try to obtain sponsorships and
/// endorsements from other people, who may become suspicious of the
/// situtation.)
#[test]
fn test_participants_scenario_1() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let participants = instantiate_participant_catalog(
        &alice.address, &package_addr);

    assert_eq!(participants.nft_address,
               read_participants_nft_addr(&participants.address),
               "Participants instance should report correct NFT address");

    // Alice wants to have her own personal Participant that is
    // separate from the catalog owner
    let alice_p_nfid = new_participant(&participants.address,
                                       &alice.address,
                                       "Alice",
                                       "file:alice.html",
                                       "no id",
                                       Some(&participants.owner_nfid));

    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Alice",
                            "file:alice.html",
                            "no id",
                            false,
                            true);
    
    // The catalog owner endorses Alice of course, because they are
    // the same person
    endorse(&alice.address,
            &participants.address,
            &participants.nft_address,
            &participants.owner_nfid,
            &alice_p_nfid);

    assert!(do_i_endorse(&participants.address,
                         &participants.owner_nfid, &alice_p_nfid),
            "Root participant should now be endorsing Alice");

    // The catalog owner sponsors Alice as well for the same reason
    sponsor(&alice.address, &participants.address, &participants.nft_address,
            &participants.owner_nfid, &alice_p_nfid);

    // Check that sponsor is now set and expect_sponsor is cleared
    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Alice",
                            "file:alice.html",
                            "no id",
                            true,
                            false);

    // Alice fatfingers the front-end and accidentally unsponsors herself
    unsponsor(&alice.address, &participants.address, &participants.nft_address,
              &participants.owner_nfid, &alice_p_nfid);
    
    // Check that sponsor is now cleared
    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Alice",
                            "file:alice.html",
                            "no id",
                            false,
                            false);

    // Hurriedly trying to put things right, Alice forgets that she
    // needs to expect_sponsor before the root participant can sponsor
    // her again.
    let result = std::panic::catch_unwind(
        || sponsor(&alice.address, &participants.address, &participants.nft_address,
                   &participants.owner_nfid, &alice_p_nfid));
    assert!(result.is_err(),
            "Alice should not be able to get a sponsor she's not expecting");

    // Alice sees her error and calls expect_sponsor
    expect_sponsor(&alice.address, &participants.address, &participants.nft_address,
                   &alice_p_nfid, &participants.owner_nfid);
    
    // Check that expect_sponsor is now set
    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Alice",
                            "file:alice.html",
                            "no id",
                            false,
                            true);

    
    // While this has been going on, Alice's good friend Bob has been
    // setting up his own Participant
    let bob = create_account();
    set_default_account(&bob);
    let bob_p_nfid = new_participant(&participants.address,
                                     &bob.address,
                                     "Bob",
                                     "file:bob.html",
                                     "don't card me bro",
                                     None);

    assert_participant_data(&participants.address,
                            &bob_p_nfid,
                            "Bob",
                            "file:bob.html",
                            "don't card me bro",
                            false,
                            false);

    // Not understanding the system yet, Bob helpfully tries to
    // sponsor Alice
    let result = std::panic::catch_unwind(
        || sponsor(&bob.address, &participants.address, &participants.nft_address,
                   &bob_p_nfid, &alice_p_nfid));
    assert!(result.is_err(),
            "Bob should not be able to sponsor Alice who is not expecting him");


    // Alice finally gets around to putting her root sponsorship in
    // place again
    set_default_account(&alice);
    sponsor(&alice.address, &participants.address, &participants.nft_address,
            &participants.owner_nfid, &alice_p_nfid);

    // Check that expect_sponsor was cleared and sponsor was set
    assert_participant_data(&participants.address,
                            &alice_p_nfid,
                            "Alice",
                            "file:alice.html",
                            "no id",
                            true,
                            false);

    // A gaggle of Charlies shows up because why not.
    // They have even secured sponsorships.
    let mut charlie = Vec::new();
    for count in 1..11 {
        let account = create_account();
        set_default_account(&account);
        let count = count.to_string();
        let account_p_nfid = new_participant(&participants.address,
                                             &account.address,
                                             &("Charlie ".to_string() + &count),
                                             &("file:charlie".to_string() + &count + ".html"),
                                             "We Are All Individuals",
                                             None);
        expect_sponsor(&account.address, &participants.address, &participants.nft_address,
                       &account_p_nfid, &participants.owner_nfid);
        set_default_account(&alice);
        sponsor(&alice.address, &participants.address, &participants.nft_address,
                &participants.owner_nfid, &account_p_nfid);
        
        charlie.push((account, account_p_nfid));
    }

    // Now lets verify that Alice is correctly set up to sponsor them
    for (_, account_p_nfid) in &charlie {
        // We use Bob here just to verify that these are permissionless calls
        set_default_account(&bob);
        let (_, _, _, sponsor, _) =
            participants_read_data(&participants.address, &account_p_nfid);
        assert_eq!(read_catalog_creator(&participants.address),
                   sponsor.unwrap(),
                   "Root participant should now sponsor all the Charlies");
    }

    // Alice and Bob both endorse eachother
    set_default_account(&alice);
    endorse(&alice.address,
            &participants.address,
            &participants.nft_address,
            &alice_p_nfid,
            &bob_p_nfid);
    assert!(do_i_endorse(&participants.address,
                         &alice_p_nfid, &bob_p_nfid),
            "Alice should now be endorsing Bob");

    set_default_account(&bob);
    endorse(&bob.address,
            &participants.address,
            &participants.nft_address,
            &bob_p_nfid,
            &alice_p_nfid);
    assert!(do_i_endorse(&participants.address,
                         &bob_p_nfid, &alice_p_nfid),
            "Bob should now be endorsing Alice");

    // Alice and Bob also endorse all the Charlies
    for (_, account_p_nfid) in &charlie {
        set_default_account(&alice);
        endorse(&alice.address,
                &participants.address,
                &participants.nft_address,
                &alice_p_nfid,
                &account_p_nfid);
        assert!(do_i_endorse(&participants.address,
                             &alice_p_nfid, &account_p_nfid),
                "Alice should now be endorsing this Charlie");

        set_default_account(&bob);
        endorse(&bob.address,
                &participants.address,
                &participants.nft_address,
                &bob_p_nfid,
                &account_p_nfid);
        assert!(do_i_endorse(&participants.address,
                             &bob_p_nfid, &account_p_nfid),
                "Bob should now be endorsing this Charlie");
    }
    
    // In a fit of nominative fraternity the Charlies all endorse
    // eachother
    for (outer_charlie, outer_charlie_p_nfid) in &charlie {
        for (_, inner_charlie_p_nfid) in &charlie {
            if outer_charlie_p_nfid == inner_charlie_p_nfid { continue; }
            set_default_account(&outer_charlie);
            endorse(&outer_charlie.address,
                    &participants.address,
                    &participants.nft_address,
                    &outer_charlie_p_nfid,
                    &inner_charlie_p_nfid);
            assert!(do_i_endorse(&participants.address,
                                 &outer_charlie_p_nfid, &inner_charlie_p_nfid),
                    "Charlie should now be endorsing this other Charlie");
        }
    }

    // Verify the endorsement lists of the Charlies
    for (_, outer_charlie_p_nfid) in &charlie {
        let endorsements =
            read_endorsements(&participants.address, &outer_charlie_p_nfid);
        for (_, inner_charlie_p_nfid) in &charlie {
            if outer_charlie_p_nfid == inner_charlie_p_nfid { continue; }
            assert!(endorsements.contains(inner_charlie_p_nfid),
                    "A good Charlie should endorse his ol'pal Charlie");
        }
    }

    // The last Charlie just rugged his project, so Alice and Bob
    // unendorse him
    let (_, fraud_p_nfid) = charlie.last().unwrap();

    set_default_account(&alice);
    unendorse(&alice.address,
              &participants.address,
              &participants.nft_address,
              &alice_p_nfid,
              &fraud_p_nfid);
    assert!(!do_i_endorse(&participants.address,
                          &alice_p_nfid, &fraud_p_nfid),
            "Alice should no longer be endorsing this Charlie");

    set_default_account(&bob);
    unendorse(&bob.address,
              &participants.address,
              &participants.nft_address,
              &bob_p_nfid,
              &fraud_p_nfid);
    assert!(!do_i_endorse(&participants.address,
                          &bob_p_nfid, &fraud_p_nfid),
            "Bob should no longer be endorsing this Charlie");

    // At this point, Alice might wonder why she's still sponsoring
    // the fraudulent last Charlie, and the well behaving Charlies
    // still need to be convinced to stop endorsing the fraudulent one
    // - but that all is a different story for a different day.
}

#[test]
fn test_loanrequestor_scenario_1() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let (participants, _) = setup_catalog(&alice.address,
                                          &package_addr);

    // And Alice creates the LoanRequestor
    let requestor =
        instantiate_requestor(&alice.address, &package_addr, &participants.nft_address);
    // She also needs a LoanAcceptor before the requestor can
    // start. This acceptor won't otherwise be used in this scenario.
    let acceptor =
        instantiate_loan_acceptor(&alice.address, &package_addr,
                                  &participants.nft_address,
                                  &requestor.admin_badge_address,
                                  None, "0");
    set_loan_acceptor(&requestor.address, &alice.address,
                      &requestor.config_badge_address, &acceptor.address);

    // Just in case she forgot, Alice does it again - which should cause panic
    let result = std::panic::catch_unwind(
        || set_loan_acceptor(&requestor.address, &alice.address,
                             &requestor.config_badge_address, &acceptor.address));
    assert!(result.is_err(),
            "Second call to set_loan_acceptor should fail");
    
    // Bob wants to loan money
    let bob = create_account();
    set_default_account(&bob);
    let bob_p_nfid = new_participant(&participants.address,
                                     &bob.address,
                                     "Bob",
                                     "file:bob.html",
                                     "don't card me bro",
                                     None);

    // Bob wants a loan to buy a lawnmower for a brilliant business
    // idea
    let bobs_request_nfid = 
        request_loan(&requestor.address, &bob.address,
                     &participants.nft_address, &bob_p_nfid,
                     RADIX_TOKEN,
                     "200", // amount
                     "15",  // minimum_share
                     250,   // pledge lock period
                     100,   // loan filled lock period
                     350,   // payment intervals
                     10,    // installments
                     "25",  // payment per installment
                     "I will buy a mower and mow peoples lawns this \
                      summer and make back everything and more.",
                     "https://bobsamazinglawnservice.hopeful/financeme.html");

    // Bullish on mowing, the Charlies all ape in on this amazing
    // scheme
    let mut charlie = Vec::new();
    for count in 1..11 {
        let account = create_account();
        set_default_account(&account);
        let count = count.to_string();
        let account_p_nfid = new_participant(&participants.address,
                                             &account.address,
                                             &("Charlie ".to_string() + &count),
                                             &("file:charlie".to_string() + &count + ".html"),
                                             "We Are All Individuals",
                                             None);

        pledge_loan(&requestor.address, &account.address,
                    &participants.nft_address, RADIX_TOKEN,
                    &account_p_nfid, &bobs_request_nfid,
                    "19");
        assert_eq!("999981", get_balance(&account, RADIX_TOKEN),
                   "Charlies should now be down 19 XRD each");
        charlie.push((account, account_p_nfid));
    }

    // Debbie also wants in on this.
    let debbie = create_account();
    set_default_account(&debbie);
    let debbie_p_nfid = new_participant(&participants.address,
                                        &debbie.address,
                                        "Debbie",
                                        "http://deb.rah/index.html",
                                        "you can't spell debit without Debbi",
                                        None);
    // Unfortunately Debbie is a bit short right now and pledges too
    // little
    let result = std::panic::catch_unwind(
        ||
            pledge_loan(&requestor.address, &debbie.address,
                        &participants.nft_address, RADIX_TOKEN,
                        &debbie_p_nfid, &bobs_request_nfid,
                        "5"));
    assert!(result.is_err(),
            "Debbie should be turned away with this small a pledge");

    // The last Charlie discovered some XRD between the couch cushions
    // and adds it to his pledge. Even though this new amount alone is
    // below the minimum limit, together with his existing 19 XRD
    // pledge it is well above and so it is accepted by the system.
    let (last_charlie, last_charlie_p_nfid) = charlie.last().unwrap();
    set_default_account(&last_charlie);
    pledge_loan(&requestor.address, &last_charlie.address,
                &participants.nft_address, RADIX_TOKEN,
                &last_charlie_p_nfid, &bobs_request_nfid,
                "2");
    assert_eq!("999979", get_balance(&last_charlie, RADIX_TOKEN),
               "The last Charlie should now be down 21 XRD");

    // Instant regret! Last Charlie wants his two coins back.
    let result = std::panic::catch_unwind(
        ||
            rescind_loan(&requestor.address, &last_charlie.address,
                         &participants.nft_address, 
                         &last_charlie_p_nfid, &bobs_request_nfid));
    // But he can't have them.
    assert!(result.is_err(),
            "It should be too soon for rescinding loans, wait a few epochs");

    // Time passes. Thorin sits down and starts singing about $XRD.
    set_current_epoch(249);
    // Now!, last Charlie thinks, Now! Finally I can have my two coins back!
    let result = std::panic::catch_unwind(
        ||
            rescind_loan(&requestor.address, &last_charlie.address,
                         &participants.nft_address, 
                         &last_charlie_p_nfid, &bobs_request_nfid));
    // But nope, he is off by one
    assert!(result.is_err(),
            "It should still be too soon for rescinding loans");

    set_current_epoch(250);
    // But then, at long last
    rescind_loan(&requestor.address, &last_charlie.address,
                 &participants.nft_address, 
                 &last_charlie_p_nfid, &bobs_request_nfid);
    assert_eq!("1000000", get_balance(&last_charlie, RADIX_TOKEN),
               "The last Charlie should have his full stash back");

    // Having got his two coins back, last Charlie puts the original
    // 19 back in
    pledge_loan(&requestor.address, &last_charlie.address,
                &participants.nft_address, RADIX_TOKEN,
                &last_charlie_p_nfid, &bobs_request_nfid,
                "19");
    assert_eq!("999981", get_balance(&last_charlie, RADIX_TOKEN),
               "The last Charlie should now be down 21 XRD");

    // In the meantime, Debbie had a windfall and now wants to pledge
    // 12 coins. This usually would be below the limit but since it
    // completes the loan request it is allowed.
    set_default_account(&debbie);
    pledge_loan(&requestor.address, &debbie.address,
                &participants.nft_address, RADIX_TOKEN,
                &debbie_p_nfid, &bobs_request_nfid,
                "12");
    assert_eq!("999990", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should now be down 10 XRD");

    // The loan now being full starts the "loan filled" lock period so
    // that, once more, nobody can rescind loans for a bit.

    // Last Charlie tries anyway.
    set_default_account(&last_charlie);
    let result = std::panic::catch_unwind(
        ||
            rescind_loan(&requestor.address, &last_charlie.address,
                         &participants.nft_address, 
                         &last_charlie_p_nfid, &bobs_request_nfid));
    assert!(result.is_err(),
            "Should not be able to rescind during loan filled lock");
    
    // Debbie made some more money and wants to go all in on the loan
    set_default_account(&debbie);
    let result = std::panic::catch_unwind(
        ||
            pledge_loan(&requestor.address, &debbie.address,
                        &participants.nft_address, RADIX_TOKEN,
                        &debbie_p_nfid, &bobs_request_nfid,
                        "100"));
    // But the loan is already fully pledged
    assert!(result.is_err(),
            "Should not be able to pledge to a filled loan");

    // A couple days pass, Bob is nowhere to be seen
    set_current_epoch(349);

    // Last Charlie tries to get his money back again
    set_default_account(&last_charlie);
    let result = std::panic::catch_unwind(
        ||
            rescind_loan(&requestor.address, &last_charlie.address,
                         &participants.nft_address, 
                         &last_charlie_p_nfid, &bobs_request_nfid));
    // But he's off by one - again
    assert!(result.is_err(),
            "Should still not be able to rescind during loan filled lock");

    // Another half hour passes ...
    set_current_epoch(350);

    // And last Charlie gets his money back out
    set_default_account(&last_charlie);
    rescind_loan(&requestor.address, &last_charlie.address,
                 &participants.nft_address, 
                 &last_charlie_p_nfid, &bobs_request_nfid);
    assert_eq!("1000000", get_balance(&last_charlie, RADIX_TOKEN),
               "The last Charlie has his full stash back");

    // Bob is back from mountaineering and eagerly tries to claim his
    // loan, not having noticed last Charlie paper handing it at the
    // last minute
    set_default_account(&bob);
    let result = std::panic::catch_unwind(
        ||
            start_loan(&requestor, &bob.address,
                       &participants.nft_address, 
                       &bob_p_nfid, &bobs_request_nfid));
    assert!(result.is_err(),
            "Loan request should no longer be filled so can't be converted");

    // This is Debbie's chance to get a bigger piece of the pie
    set_default_account(&debbie);
    pledge_loan(&requestor.address, &debbie.address,
                &participants.nft_address, RADIX_TOKEN,
                &debbie_p_nfid, &bobs_request_nfid,
                "100");
    assert_eq!("999971", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should now be down 29 XRD");

    // First Charlie now has cold feet as well and pulls out
    let (first_charlie, first_charlie_p_nfid) = charlie.first().unwrap();
    set_default_account(&first_charlie);
    rescind_loan(&requestor.address, &first_charlie.address,
                 &participants.nft_address, 
                 &first_charlie_p_nfid, &bobs_request_nfid);
    assert_eq!("1000000", get_balance(&first_charlie, RADIX_TOKEN),
               "The first Charlie has his full stash back");

    // Verify a query method
    assert!(!read_is_cancelled(&requestor, &bobs_request_nfid),
            "Bob's loan should not be cancelled yet");

    // This is too much nonsense for Bob, he discards the whole thing
    // intending to start over again
    set_default_account(&bob);
    cancel_request(&requestor, &bob.address,
                   &participants.nft_address, 
                   &bob_p_nfid, &bobs_request_nfid);

    // Verify a query method
    assert!(read_is_cancelled(&requestor, &bobs_request_nfid),
            "Bob's loan should now be cancelled");
    
    // Last Charlie is eager to get back in
    set_default_account(&last_charlie);
    let result = std::panic::catch_unwind(
        ||
            pledge_loan(&requestor.address, &last_charlie.address,
                        &participants.nft_address, RADIX_TOKEN,
                        &last_charlie_p_nfid, &bobs_request_nfid,
                        "19"));
    // But he can't
    assert!(result.is_err(),
            "Should not be able to pledge to a cancelled loan");

    // Debbie claws her money back
    set_default_account(&debbie);
    rescind_loan(&requestor.address, &debbie.address,
                 &participants.nft_address, 
                 &debbie_p_nfid, &bobs_request_nfid);
    assert_eq!("1000000", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should now have all her money");
    
    // Bob tries to get rid of this failed first foray into the
    // wonderful world of micro finance
    set_default_account(&bob);
    let result = std::panic::catch_unwind(
        ||
            loan_request_burn(&requestor, &bob.address,
                              &participants.nft_address, 
                              &bob_p_nfid, &bobs_request_nfid));
    // But he can't, yet
    assert!(result.is_err(),
            "Should not be able to burn a request with pledges on it");

    // The Charlies now all recover their funds
    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);
        rescind_loan(&requestor.address, &account.address,
                     &participants.nft_address, 
                     &account_p_nfid, &bobs_request_nfid);
        assert_eq!("1000000", get_balance(&account, RADIX_TOKEN),
                   "Charlies should now have all their money");
    }

    // Verify some query methods while we still can
    {
        let (request_token, request_amount, minimum_share, request_start_epoch,
             pledge_lock_epochs, loan_filled_epoch, loan_filled_lock_epochs,
             promise_payment_intervals, promise_installments,
             promise_amount_per_installment)
            = loan_request_read_data(&requestor, &bobs_request_nfid);
        assert_eq!(RADIX_TOKEN, request_token);
        assert_eq!("200", request_amount);
        assert_eq!("15", minimum_share);
        assert_eq!(0, request_start_epoch);
        assert_eq!(250, pledge_lock_epochs);
        assert_eq!(Some(250), loan_filled_epoch);
        assert_eq!(100, loan_filled_lock_epochs);
        assert_eq!(350, promise_payment_intervals);
        assert_eq!(10, promise_installments);
        assert_eq!("25", promise_amount_per_installment);
    }

    // And NOW it can burn
    set_default_account(&bob);
    loan_request_burn(&requestor, &bob.address,
                      &participants.nft_address, 
                      &bob_p_nfid, &bobs_request_nfid);


    // Bob now arranges a new loan request with slightly better terms
    // and monitors it closely to guide it home without too many
    // complications

    let bobs_2nd_request_nfid = 
        request_loan(&requestor.address, &bob.address,
                     &participants.nft_address, &bob_p_nfid,
                     RADIX_TOKEN,
                     "200", // amount
                     "15",  // minimum_share
                     250,   // pledge lock period
                     100,   // loan filled lock period
                     350,   // payment intervals
                     10,    // installments
                     "27",  // payment per installment
                     "I WILL buy a mower and mow peoples lawns this \
                      summer and make back everything and more.",
                     "https://bobsamazinglawnservice.hopeful/financemeagain.html");

    // Let's verify some of our query methods
    {
        let (summary, url) = 
            loan_request_read_loan_purpose(&requestor, &bobs_2nd_request_nfid);
        assert_eq!("I WILL buy a mower and mow peoples lawns this \
                    summer and make back everything and more.",
                   summary);
        assert_eq!("https://bobsamazinglawnservice.hopeful/financemeagain.html",
                   url);

        assert_eq!(&bob_p_nfid,
                   &loan_request_read_borrower_id(&requestor, &bobs_2nd_request_nfid),
                   "Bob should be the borrower and the borrower should be Bob");

        assert_eq!(&requestor.nft_address,
                   &read_request_nft_addr(&requestor));

        assert_eq!(&participants.nft_address,
                   &loan_request_read_participants_nft_addr(&requestor));

        assert_eq!(&acceptor.address,
                   &read_loan_acceptor(&requestor).unwrap());
    }    
    
    // Charlies fomo in again, this time filling the full request
    // within minutes - their fervor fueled in no small part by a very
    // very silly loanmower meme that Bob came up with
    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);

        pledge_loan(&requestor.address, &account.address,
                    &participants.nft_address, RADIX_TOKEN,
                    &account_p_nfid, &bobs_2nd_request_nfid,
                    "20");
        assert_eq!("999980", get_balance(&account, RADIX_TOKEN),
                   "Charlies should now be down 20 XRD each");
    }

    // Immediately, Bob finalizes the request
    set_default_account(&bob);
    start_loan(&requestor, &bob.address,
               &participants.nft_address, 
               &bob_p_nfid, &bobs_2nd_request_nfid);
    assert_eq!("1000200", get_balance(&bob, RADIX_TOKEN),
               "Bob should now have his loan!");

    // Last Charlie wants out again
    set_default_account(&last_charlie);
    let result = std::panic::catch_unwind(
        ||
            rescind_loan(&requestor.address, &last_charlie.address,
                         &participants.nft_address, 
                         &last_charlie_p_nfid, &bobs_2nd_request_nfid));
    // But the loan request no longer exists, he'll have to wait for
    // first installment to see any of his money back
    assert!(result.is_err(),
            "Should not be able to rescind after loan starts");
}

/// Runs a scenario testing aspects around loan repayment and
/// management.
#[test]
pub fn test_loanacceptor_scenario_1() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let (participants, _) = setup_catalog(&alice.address,
                                          &package_addr);
    let alice_p_nfid = new_participant(&participants.address,
                                       &alice.address,
                                       "Alice",
                                       "file:alice.html",
                                       "id ont know about that",
                                       None);

    // Alice creates the requestor and acceptor
    let requestor =
        instantiate_requestor(&alice.address, &package_addr, &participants.nft_address);
    let acceptor =
        instantiate_loan_acceptor(&alice.address, &package_addr,
                                  &participants.nft_address,
                                  &requestor.admin_badge_address,
                                  Some(&alice_p_nfid), "5");
    set_loan_acceptor(&requestor.address, &alice.address,
                      &requestor.config_badge_address, &acceptor.address);

    // Bob wants to loan money
    let bob = create_account();
    set_default_account(&bob);
    let bob_p_nfid = new_participant(&participants.address,
                                     &bob.address,
                                     "Bob",
                                     "file:bob.html",
                                     "don't card me bro",
                                     None);

    // Bob wants to get a lawnmowing tractor to expand his business
    let bobs_request_nfid = 
        request_loan(&requestor.address, &bob.address,
                     &participants.nft_address, &bob_p_nfid,
                     RADIX_TOKEN,
                     "20000", // amount
                     "1000",  // minimum_share
                     250,     // pledge lock period
                     100,     // loan filled lock period
                     350,     // payment intervals
                     10,      // installments
                     "2500",  // payment per installment
                     "I will buy a BIGGER mower and mow huge lawns \
                      and be absolutely filthy rich.",
                     "https://bobsamazinglawnservice.hopeful/tothemoon.html");

    // A Charlie loves a good mower
    let mut charlie = Vec::new();
    for count in 1..11 {
        let account = create_account();
        set_default_account(&account);
        let count = count.to_string();
        let account_p_nfid = new_participant(&participants.address,
                                             &account.address,
                                             &("Charlie ".to_string() + &count),
                                             &("file:charlie".to_string() + &count + ".html"),
                                             "We Are All Individuals",
                                             None);

        pledge_loan(&requestor.address, &account.address,
                    &participants.nft_address, RADIX_TOKEN,
                    &account_p_nfid, &bobs_request_nfid,
                    "1500");
        charlie.push((account, account_p_nfid));
    }
    
    // Debbie has become a micro finance whale
    let debbie = create_account();
    set_default_account(&debbie);
    let debbie_p_nfid = new_participant(&participants.address,
                                        &debbie.address,
                                        "Debbie",
                                        "http://deb.rah/index.html",
                                        "you can't spell debit without Debbi",
                                        None);
    pledge_loan(&requestor.address, &debbie.address,
                &participants.nft_address, RADIX_TOKEN,
                &debbie_p_nfid, &bobs_request_nfid,
                "4500");

    // And Eric is dipping his toe into micro finance as well,
    // providing the final pledge
    let eric = create_account();
    set_default_account(&eric);
    let eric_p_nfid = new_participant(&participants.address,
                                      &eric.address,
                                      "Eric",
                                      "",
                                      "",
                                      None);
    pledge_loan(&requestor.address, &eric.address,
                &participants.nft_address, RADIX_TOKEN,
                &eric_p_nfid, &bobs_request_nfid,
                "500");

    // Epoch-based tests are better if we don't base them off 0
    set_current_epoch(50);
    // Bob converts the request into a loan
    set_default_account(&bob);
    let bobs_loan_nfid =
        start_loan(&requestor, &bob.address,
                   &participants.nft_address, 
                   &bob_p_nfid, &bobs_request_nfid);
    assert_eq!("1020000", get_balance(&bob, RADIX_TOKEN),
               "Bob should now have his loan!");

    // Test some query methods
    {
        let (loan_amount, loan_token,
             loan_start_epoch,
             installment_total_count, installments_remaining,
             epochs_per_installment, amount_per_installment) =
            read_loan_data(&acceptor, &bobs_loan_nfid);
        assert_eq!("20000", loan_amount);
        assert_eq!(RADIX_TOKEN, loan_token);
        assert_eq!(50, loan_start_epoch);
        assert_eq!(10, installment_total_count);
        assert_eq!(10, installments_remaining);
        assert_eq!(350, epochs_per_installment);
        assert_eq!("2500", amount_per_installment);

        let (purpose, url) = loan_read_loan_purpose(&acceptor, &bobs_loan_nfid);
        assert_eq!("I will buy a BIGGER mower and mow huge lawns \
                    and be absolutely filthy rich.",
                   purpose);
        assert_eq!("https://bobsamazinglawnservice.hopeful/tothemoon.html",
                   url);

        // We are not equipped to verify that the NFT addresses are
        // correct so we just check that they exist
        assert!(!loan_read_borrower(&acceptor, &bobs_loan_nfid).is_empty(),
                "There should be a borrower address");
        assert!(!read_facilitator(&acceptor).unwrap().is_empty(),
                "There should be a facilitator");

        assert_eq!("5", read_facilitator_fee(&acceptor));
        
        let lenders = read_lenders(&acceptor, &bobs_loan_nfid);
        assert_eq!(12, lenders.len());
        assert_eq!("500", lenders[&eric_p_nfid]);
        assert_eq!("4500", lenders[&debbie_p_nfid]);
        for (_, account_p_nfid) in &charlie {
            assert_eq!("1500", lenders[account_p_nfid]);
        }

        assert_eq!(participants.nft_address,
                   loan_read_participants_nft_addr(&acceptor));
        assert_eq!(acceptor.nft_address,
                   loan_read_loan_nft_addr(&acceptor));
    }
    
    // Bob tries to do an early payment but it's not enough for an
    // installment
    let result =  std::panic::catch_unwind(
        ||
            pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                            RADIX_TOKEN, "250"));
    assert!(result.is_err(),
            "Bob shouldn't be allowed to underpay an installment");

    // At this point the following payouts are expected from each installment
    // Installment size: 2500
    // Facilitator share: 0.05% of 2500 = 1.25
    // To be shared between lenders: 2500-1.25 = 2498.75
    // Charlies: 1500 pledged -> 2498.75 * (1500/20000) -> 187.40625 each
    // Debbie: 4500 pledged ->  2498.75 * (4500/20000) -> 562.21875
    // Eric: 500 pledged -> 2498.75 * (500/20000) -> 62.46875

    // Bob eagerly overpays an installment, but that is fine the excess
    // is returned to him.
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "3000");
    assert_eq!("1017500", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");

    // Test a subset of read_loan_data that may have changed
    {
        let (_,_,_,installment_total_count,installments_remaining,_,_) =
            read_loan_data(&acceptor, &bobs_loan_nfid);
        assert_eq!(10, installment_total_count);
        assert_eq!(9, installments_remaining);
    }

    // Alice pulls out her phat facilitator rewards
    set_default_account(&alice);
    claim_facilitator_rewards(&acceptor, &alice, &participants,
                              &alice_p_nfid);
    assert_eq!("1000001.25", get_balance(&alice, RADIX_TOKEN),
               "Alice should have her first taste of passive income");

    // The lenders all claim their first installment
    set_default_account(&debbie);
    claim_lender_rewards(&acceptor, &debbie, &participants,
                         &debbie_p_nfid);
    assert_eq!("996062.21875", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should have got her first installment");

    set_default_account(&eric);
    claim_lender_rewards(&acceptor, &eric, &participants,
                         &eric_p_nfid);
    assert_eq!("999562.46875", get_balance(&eric, RADIX_TOKEN),
               "Eric should have got his first installment");

    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);
        claim_lender_rewards(&acceptor, &account, &participants,
                             &account_p_nfid);
        assert_eq!("998687.40625", get_balance(&account, RADIX_TOKEN),
                   "Charlie should have got his first installment");
    }

    // Bob pays another installment
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "4000");
    assert_eq!("1015000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");

    // The novelty having died down, only Debbie claims this time
    set_default_account(&debbie);
    claim_lender_rewards(&acceptor, &debbie, &participants,
                         &debbie_p_nfid);
    assert_eq!("996624.4375", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should have got her second installment");

    // Having worked out a deal with Bob for getting her estate's lawn
    // mowed, Alice pays off an installment for him
    set_default_account(&alice);
    pay_installment(&acceptor, &alice, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("997501.25", get_balance(&alice, RADIX_TOKEN),
               "Alice should be 2500 XRD down");

    // The inexorable passage of time
    set_current_epoch(1450);
    
    // The loan isn't yet in arrears
    assert!(!is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob's loan should not be in arrears");

    // But half an hour later ...
    set_current_epoch(1451);

    // Things are looking bad for Bob, he's been sloppy with his
    // payments
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob's loan should now be in arrears");

    // He makes the payment
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1012500", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");

    // But it's still in arrears: a late payment isn't forgotten so
    // easily. Bob has let his lenders down.
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob's loan should still be in arrears");

    assert!(read_loan_arrears_votes(
        &acceptor, &bobs_loan_nfid).len() == 0,
            "Shouldn't be any arrears votes yet");
    
    // After pleading with them all Bob gets his creditors to forgive
    // him this slip. They start clearing the arrears status of the
    // loan.
    set_default_account(&eric);
    approve_clear_arrears(&acceptor, &eric, &participants,
                          &eric_p_nfid, &bobs_loan_nfid);
    assert!(read_loan_arrears_votes(&acceptor, &bobs_loan_nfid)
            .contains(&eric_p_nfid),
            "Eric should be voting for");

    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);
        approve_clear_arrears(&acceptor, &account, &participants,
                              &account_p_nfid, &bobs_loan_nfid);
        assert!(read_loan_arrears_votes(&acceptor, &bobs_loan_nfid)
                .contains(account_p_nfid),
                "Charlie should be voting for");
    }

    // Eric smells a rat and changes his mind
    set_default_account(&eric);
    disapprove_clear_arrears(&acceptor, &eric, &participants,
                             &eric_p_nfid, &bobs_loan_nfid);
    assert!(!read_loan_arrears_votes(&acceptor, &bobs_loan_nfid)
            .contains(&eric_p_nfid),
            "Eric should no longer be voting for");

    // Debbie was out parachuting and caught up with events last
    set_default_account(&debbie);
    approve_clear_arrears(&acceptor, &debbie, &participants,
                          &debbie_p_nfid, &bobs_loan_nfid);
    assert!(read_loan_arrears_votes(&acceptor, &bobs_loan_nfid)
            .contains(&debbie_p_nfid),
            "Debbie should be voting for");

    // Eric didn't tell Bob that he removed his approval so Bob thinks
    // all is now in order with the loan again.
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob's loan should still be in arrears");

    // Unhappy with the situation, Bob finally manages to talk Eric
    // into approving again.
    set_default_account(&eric);
    approve_clear_arrears(&acceptor, &eric, &participants,
                          &eric_p_nfid, &bobs_loan_nfid);
    
    // And now all should be good in Bob's little lawnmowing lairdship
    assert!(!is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should be good to go");

    assert!(read_loan_arrears_votes(
        &acceptor, &bobs_loan_nfid).len() == 0,
            "Arrears votes should have been cleared");

    // Bob remains forgetful and the loan slips into arrears again
    set_current_epoch(1801);
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should be in arrears again");

    // What's worse is, Eric makes it official
    set_default_account(&eric);
    update_arrears(&acceptor, &eric, &participants,
                   &eric_p_nfid, &bobs_loan_nfid);
    
    // Once more he manages to sweet-talk everyone into clearing it
    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);
        approve_clear_arrears(&acceptor, &account, &participants,
                              &account_p_nfid, &bobs_loan_nfid);
    }

    set_default_account(&debbie);
    approve_clear_arrears(&acceptor, &debbie, &participants,
                          &debbie_p_nfid, &bobs_loan_nfid);

    set_default_account(&eric);
    approve_clear_arrears(&acceptor, &eric, &participants,
                          &eric_p_nfid, &bobs_loan_nfid);

    // To his surprise this didn't put him out of arrears
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should still be in arrears");

    // Then he remembers he actually needs to pay his installment for
    // that to happen
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1010000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");

    // But he's still in arrears! Because the loan doesn't forget so
    // easily.
    assert!(is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should still be in arrears");

    // He gets everyone to bail him out again
    for (account, account_p_nfid) in &charlie {
        set_default_account(&account);
        approve_clear_arrears(&acceptor, &account, &participants,
                              &account_p_nfid, &bobs_loan_nfid);
    }

    set_default_account(&debbie);
    approve_clear_arrears(&acceptor, &debbie, &participants,
                          &debbie_p_nfid, &bobs_loan_nfid);

    set_default_account(&eric);
    approve_clear_arrears(&acceptor, &eric, &participants,
                          &eric_p_nfid, &bobs_loan_nfid);

    // And then everything is clear once more
    assert!(!is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should finally be out of the woods");

    // Bob dutifully pays off the rest of the loan
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1007500", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1005000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1002500", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("1000000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "2500");
    assert_eq!("997500", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 2500 XRD down");
    // The loan is now fully repaid. Note that Alice paid one of the
    // installments so Bob's balance is 2500 higher than it would have
    // been otherwise.

    // Bob absentmindedly makes an extra payment
    let result = 
        std::panic::catch_unwind(
            ||
                pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                                RADIX_TOKEN, "2500"));
    assert!(result.is_err(),
            "Fully repaid loan should not allow more payments");
    
    // And all is good in the world
    assert!(!is_in_arrears(&acceptor, &bobs_loan_nfid),
            "Bob should be done and good");

    // Do it again with only one lender and 0% fee
}


/// Runs a shorter scenario with a 1-lender / no facilitator loan
#[test]
pub fn test_loanacceptor_scenario_2() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let (participants, _) = setup_catalog(&alice.address,
                                          &package_addr);

    // Alice sets up a zero-fee loan service for her good friend (and
    // gardener) Bob
    let requestor =
        instantiate_requestor(&alice.address, &package_addr, &participants.nft_address);
    let acceptor =
        instantiate_loan_acceptor(&alice.address, &package_addr,
                                  &participants.nft_address,
                                  &requestor.admin_badge_address,
                                  None, "0");
    set_loan_acceptor(&requestor.address, &alice.address,
                      &requestor.config_badge_address, &acceptor.address);

    // Bob wants to loan money
    let bob = create_account();
    set_default_account(&bob);
    let bob_p_nfid = new_participant(&participants.address,
                                     &bob.address,
                                     "Bob",
                                     "file:bob.html",
                                     "don't card me bro",
                                     None);

    // Bob wants a turbocharger for his lawnmowing tractor, and has
    // already lined up a lender
    let bobs_request_nfid = 
        request_loan(&requestor.address, &bob.address,
                     &participants.nft_address, &bob_p_nfid,
                     RADIX_TOKEN,
                     "5000",  // amount
                     "5000",  // minimum_share
                     250,     // pledge lock period
                     100,     // loan filled lock period
                     500,     // payment intervals
                     2,       // installments
                     "3000",  // payment per installment
                     "turbocharge me bro",
                     "");

    // Debbie is so happy with her previous investment with Bob she
    // full finances this one
    let debbie = create_account();
    set_default_account(&debbie);
    let debbie_p_nfid = new_participant(&participants.address,
                                        &debbie.address,
                                        "Debbie",
                                        "http://deb.rah/index.html",
                                        "you can't spell debit without Debbi",
                                        None);
    pledge_loan(&requestor.address, &debbie.address,
                &participants.nft_address, RADIX_TOKEN,
                &debbie_p_nfid, &bobs_request_nfid,
                "5000");
    assert_eq!("995000", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should be 5000 down");

    // Bob converts the request into a loan
    set_default_account(&bob);
    let bobs_loan_nfid =
        start_loan(&requestor, &bob.address,
                   &participants.nft_address, 
                   &bob_p_nfid, &bobs_request_nfid);
    assert_eq!("1005000", get_balance(&bob, RADIX_TOKEN),
               "Bob should now have his loan!");

    assert!(read_facilitator(&acceptor).is_none(),
            "There shouldn't be a facilitator");
    assert_eq!("0", read_facilitator_fee(&acceptor));
    
    // Bob makes payment and Debbie collects immediately
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "3000");
    assert_eq!("1002000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 3000 XRD down");
    set_default_account(&debbie);
    claim_lender_rewards(&acceptor, &debbie, &participants,
                         &debbie_p_nfid);
    assert_eq!("998000", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should be 3000 XRD up");

    // Once more
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "3000");
    assert_eq!("999000", get_balance(&bob, RADIX_TOKEN),
               "Bob should be 3000 XRD down");

    // Alice accidentally tries to claim Debbie's rewards.
    // Accidentally.
    set_default_account(&alice);
    let result = std::panic::catch_unwind(
        ||
            claim_lender_rewards(&acceptor, &debbie, &participants,
                                 &debbie_p_nfid));
    assert!(result.is_err(),
            "Alice shouldn't be able to claim Debbie's rewards");

    // Then Debbie shows up and claims her last bit
    set_default_account(&debbie);
    claim_lender_rewards(&acceptor, &debbie, &participants,
                         &debbie_p_nfid);
    assert_eq!("1001000", get_balance(&debbie, RADIX_TOKEN),
               "Debbie should be 3000 XRD up");
}

/// Sets up a divide by three to verify our handling of rounding
/// artifacts in pay_installment
#[test]
pub fn test_loanacceptor_divide_by_three() {
    reset_sim();
    let package_addr = publish_package();

    // Alice owns the catalog
    let alice = create_account();
    let (participants, owner_p_nfid) = setup_catalog(&alice.address,
                                                     &package_addr);

    let requestor =
        instantiate_requestor(&alice.address, &package_addr, &participants.nft_address);
    let acceptor =
        instantiate_loan_acceptor(&alice.address, &package_addr,
                                  &participants.nft_address,
                                  &requestor.admin_badge_address,
                                  Some(&owner_p_nfid), "2");
    set_loan_acceptor(&requestor.address, &alice.address,
                      &requestor.config_badge_address, &acceptor.address);

    // Bob wants to loan money
    let bob = create_account();
    set_default_account(&bob);
    let bob_p_nfid = new_participant(&participants.address,
                                     &bob.address,
                                     "Bob",
                                     "file:bob.html",
                                     "don't card me bro",
                                     None);

    // Bob wants a turbocharger for his lawnmowing tractor, and has
    // already lined up a lender
    let bobs_request_nfid = 
        request_loan(&requestor.address, &bob.address,
                     &participants.nft_address, &bob_p_nfid,
                     RADIX_TOKEN,
                     "3000",  // amount
                     "100",   // minimum_share
                     250,     // pledge lock period
                     100,     // loan filled lock period
                     500,     // payment intervals
                     2,       // installments
                     "1100",  // payment per installment
                     "",
                     "");

    // Debbie pledges to the loan
    let debbie = create_account();
    set_default_account(&debbie);
    let debbie_p_nfid = new_participant(&participants.address,
                                        &debbie.address,
                                        "Debbie",
                                        "http://deb.rah/index.html",
                                        "you can't spell debit without Debbi",
                                        None);
    pledge_loan(&requestor.address, &debbie.address,
                &participants.nft_address, RADIX_TOKEN,
                &debbie_p_nfid, &bobs_request_nfid,
                "2000");

    // And so does Eric
    let eric = create_account();
    set_default_account(&eric);
    let eric_p_nfid = new_participant(&participants.address,
                                      &eric.address,
                                      "Eric",
                                      "",
                                      "",
                                      None);
    pledge_loan(&requestor.address, &eric.address,
                &participants.nft_address, RADIX_TOKEN,
                &eric_p_nfid, &bobs_request_nfid,
                "1000");

    // Bob converts the request into a loan
    set_default_account(&bob);
    let bobs_loan_nfid =
        start_loan(&requestor, &bob.address,
                   &participants.nft_address, 
                   &bob_p_nfid, &bobs_request_nfid);

    // Bob makes payment
    set_default_account(&bob);
    pay_installment(&acceptor, &bob, &bobs_loan_nfid,
                    RADIX_TOKEN, "1100");

    // The above resulted in a divide by three in the rewards
    // calculation in the blueprint, and it not panicing means that we
    // didn't lose any token fractions while processing it. Which is
    // the purpose of this test.
}
