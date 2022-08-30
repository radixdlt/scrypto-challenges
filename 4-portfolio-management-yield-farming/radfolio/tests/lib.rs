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
    _privkey: String,
}

#[derive(Debug)]
struct ParticipantsComponent {
    address: String,
    nft_address: String,
    _owner_nfid: String,
}

#[derive(Debug)]
struct RadfolioComponent {
    address: String,
    admin_address: String,
    coupon_address: String,
    token_address: String,
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
        _privkey: privkey.to_string()
    }
}

/// Publishes the package by calling "resim publish ."
///
/// Returns the new blueprint's address
fn publish_package(path: Option<&str>) -> String {
    let path = path.unwrap_or(".");
    let output = run_command(Command::new("resim")
                             .arg("publish")
                             .arg(path));
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
        _owner_nfid: matches[3].to_string(),
    }
}

/// Creates a new Participants catalog via
/// rtm/participants/new_participant.rtm
///
/// Returns the catalog created.
fn new_participant(participants: &ParticipantsComponent,
                   account: &Account, name: &str, url: &str, id_ref: &str,
                   expect_sponsor: Option<&str> ) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/participants/new_participant.rtm")
                             .env("component", &participants.address)
                             .env("account", &account.address)
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

/// Creates a new Radfolio via
/// rtm/radfolio/instantiate_radfoltio.rtm
///
/// Returns the catalog created.
fn instantiate_radfolio(account_addr: &str, package_addr: &str,
                        investment_token: &str,
                        participants_nft_address: Option<&str>,
                        free_funds_target_percent: &str,
                        investment_update_interval_epochs: u64,
                        minimum_deposit: &str,
                        admin_badge_name: Option<&str>,
                        admin_badge_quantity: u64,
                        coupon_name: Option<&str>,
                        deposit_fee_bps: Option<&str>,
                        deposit_fee_partner_bps: Option<&str>,
                        withdraw_fee_bps: Option<&str>,
                        withdraw_fee_partner_bps: Option<&str>,
                        mint_badge_name: Option<&str>,
                        iv_control_badge_name: Option<&str>)
                                   -> RadfolioComponent
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/instantiate_radfolio.rtm")
                             .env("account", account_addr)
                             .env("package", &package_addr)
                             .env("investment_token", investment_token)
                             .env("participants_nft_address",
                                  option_to_tm_string(participants_nft_address, "ResourceAddress"))
                             .env("free_funds_target_percent", free_funds_target_percent)
                             .env("investment_update_interval_epochs", investment_update_interval_epochs.to_string())
                             .env("minimum_deposit", minimum_deposit)
                             .env("admin_badge_name", option_string_to_tm_string(admin_badge_name))
                             .env("admin_badge_quantity", admin_badge_quantity.to_string())
                             .env("coupon_name", option_string_to_tm_string(coupon_name))
                             .env("deposit_fee_bps", option_to_tm_string(deposit_fee_bps, "Decimal"))
                             .env("deposit_fee_partner_bps", option_to_tm_string(deposit_fee_partner_bps, "Decimal"))
                             .env("withdraw_fee_bps", option_to_tm_string(withdraw_fee_bps, "Decimal"))
                             .env("withdraw_fee_partner_bps", option_to_tm_string(withdraw_fee_partner_bps, "Decimal"))
                             .env("mint_badge_name", option_string_to_tm_string(mint_badge_name))
                             .env("iv_control_badge_name", option_string_to_tm_string(iv_control_badge_name)));
        
    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\).*"#,
            r#"ResourceAddress\("(\w*)"\)"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_radfolio");

    RadfolioComponent {
        address: matches[1].to_string(),
        admin_address: matches[2].to_string(),
        coupon_address: matches[3].to_string(),
        token_address: investment_token.to_string(),
    }
}

/// Finds the total funds under management, via
/// rtm/radfolio/read_total_funds.rtm
fn read_total_funds(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_total_funds.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FUNDS: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_FUNDS.captures(&output).expect("Failed to parse read_total_funds")[1].to_string()
}

/// Finds the total coupons in existence, via
/// rtm/radfolio/read_total_coupons.rtm
fn read_total_coupons(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_total_coupons.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_COUPONS: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_COUPONS.captures(&output).expect("Failed to parse read_total_coupons")[1].to_string()
}

/// Finds the current coupon value, via
/// rtm/radfolio/value_of_coupons.rtm
fn value_of_coupons(component: &RadfolioComponent, amount: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/value_of_coupons.rtm")
                             .env("component", &component.address)
                             .env("amount", amount));

    lazy_static! {
        static ref RE_DEC: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_DEC.captures(&output).expect("Failed to parse value_of_coupons")[1].to_string()
}

/// Finds the token we are managing, via
/// rtm/radfolio/read_investment_token.rtm
fn read_investment_token(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_investment_token.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_investment_token")[1].to_string()
}

/// Finds the token we use for coupons, via
/// rtm/radfolio/read_coupon_address.rtm
fn read_coupon_address(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_coupon_address.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_coupon_address")[1].to_string()
}

/// Finds the token we use for our admin badges, via
/// rtm/radfolio/read_admin_badge_address.rtm
fn read_admin_badge_address(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_admin_badge_address.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_admin_badge_address")[1].to_string()
}

/// Finds the token we use for our minting badges, via
/// rtm/radfolio/read_mint_badge_address.rtm
fn read_mint_badge_address(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_mint_badge_address.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_mint_badge_address")[1].to_string()
}

/// Finds the token we use for our iv control badges, via
/// rtm/radfolio/read_iv_control_badge_address.rtm
fn read_iv_control_badge_address(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_iv_control_badge_address.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_iv_control_badge_address")[1].to_string()
}

/// Finds the ResourceAddress of Participants NFTs via
/// rtm/radfolio/read_participants_nft_address.rtm
fn read_participants_nft_address(component: &RadfolioComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_participants_nft_address.rtm")
                             .env("component", &component.address));

    println!("{}", output);
    lazy_static! {
        static ref RE_ADDR: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_ADDR.captures(&output).expect("Failed to parse read_participants_nft_address")[1].to_string(),
        "ResourceAddress")
}

/// Reads the list of current investments, via
/// rtm/radfolio/read_investments.rtm
fn read_investments(component: &RadfolioComponent) -> HashMap<String, String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_investments.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_MAP: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashMap<ComponentAddress, Decimal>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_KEY: Regex = Regex::new(concat!(
            r#"ComponentAddress\("(\w*)"\)"#,
        )).unwrap();
        static ref RE_VALUE: Regex = Regex::new(concat!(
            r#"Decimal\("(.*)"\)"#,
        )).unwrap();
    }

    let hashmap = RE_MAP.captures(&output).expect("Failed to parse read_investments")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashmap).collect();
    let mut investments: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    for element in elements {
        if element == "" { break; }
        if key.is_none() {
            key = Some(RE_KEY.captures(&element).expect("Failed to parse key")[1].to_string());
        } else {
            investments.insert(
                key.unwrap(),
                RE_VALUE.captures(&element).expect("Failed to parse value")[1].to_string());
            key = None;
        }
    }

    investments
}

/// Reads the current free funds, via
/// rtm/radfolio/read_free_funds.rtm
fn read_free_funds(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_free_funds.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FUNDS: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_FUNDS.captures(&output).expect("Failed to parse read_free_funds")[1].to_string()
}

/// Changes the free funds target %, via
/// rtm/radfolio/set_free_funds_target_percent.rtm
fn set_free_funds_target_percent(component: &RadfolioComponent,
                                 account: &Account,
                                 target: &str) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_free_funds_target_percent.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("target", target));
}

/// Reads the free funds target %, via
/// rtm/radfolio/read_free_funds_target_percent.rtm
fn read_free_funds_target_percent(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_free_funds_target_percent.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FUNDS: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_FUNDS.captures(&output).expect("Failed to parse read_free_funds_target_percent")[1].to_string()
}

/// Changes the update interval, via
/// rtm/radfolio/set_investment_update_interval_epochs.rtm
fn set_investment_update_interval_epochs(component: &RadfolioComponent,
                                         account: &Account,
                                         interval: u64) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_investment_update_interval_epochs.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("interval", interval.to_string()));
}

/// Reads the update interval, via
/// rtm/radfolio/read_investment_update_interval_epochs.rtm
fn read_investment_update_interval_epochs(component: &RadfolioComponent) -> u64
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_investment_update_interval_epochs.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_INTERVAL: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)u64"#
        )).unwrap();
    }

    RE_INTERVAL.captures(&output).expect(
        "Failed to parse read_investment_update_interval_epochs")[1].parse().unwrap()
}

/// Reads last update epoch, via
/// rtm/radfolio/read_last_update_epoch.rtm
fn read_last_update_epoch(component: &RadfolioComponent) -> u64
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_last_update_epoch.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_EPOCH: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)u64"#
        )).unwrap();
    }

    RE_EPOCH.captures(&output).expect(
        "Failed to parse read_last_update_epoch")[1].parse().unwrap()
}

/// Changes the minimum deposit, via
/// rtm/radfolio/set_minimum_deposit.rtm
fn set_minimum_deposit(component: &RadfolioComponent,
                       account: &Account,
                       minimum: &str) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_minimum_deposit.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("minimum", minimum));
}

/// Reads the minimum deposit, via
/// rtm/radfolio/read_minimum_deposit.rtm
fn read_minimum_deposit(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_minimum_deposit.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_MINDEP: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_MINDEP.captures(&output).expect("Failed to parse read_minimum_deposit")[1].to_string()
}

/// Reads the deposit fee, via
/// rtm/radfolio/read_deposit_fee_bps.rtm
fn read_deposit_fee_bps(component: &RadfolioComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_deposit_fee_bps.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FEE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_FEE.captures(&output).expect("Failed to parse read_deposit_fee_bps")[1].to_string(),
        "Decimal")
}

/// Sets the deposit partner fee, via
/// rtm/radfolio/set_deposit_fee_partner_bps.rtm
fn set_deposit_fee_partner_bps(component: &RadfolioComponent,
                               account: &Account,
                               fee: Option<&str>) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_deposit_fee_partner_bps.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("fee", option_to_tm_string(fee, "Decimal")));
}

/// Reads the deposit partner fee, via
/// rtm/radfolio/read_deposit_fee_partner_bps.rtm
fn read_deposit_fee_partner_bps(component: &RadfolioComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_deposit_fee_partner_bps.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FEE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_FEE.captures(&output).expect("Failed to parse read_deposit_fee_partner_bps")[1].to_string(),
        "Decimal")
}

/// Reads the withdraw fee, via
/// rtm/radfolio/read_withdraw_fee_bps.rtm
fn read_withdraw_fee_bps(component: &RadfolioComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_withdraw_fee_bps.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FEE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_FEE.captures(&output).expect("Failed to parse read_withdraw_fee_bps")[1].to_string(),
        "Decimal")
}

/// Sets the withdraw partner fee, via
/// rtm/radfolio/set_withdraw_fee_partner_bps.rtm
fn set_withdraw_fee_partner_bps(component: &RadfolioComponent,
                                account: &Account,
                                fee: Option<&str>) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_withdraw_fee_partner_bps.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("fee", option_to_tm_string(fee, "Decimal")));
}

/// Reads the withdraw partner fee, via
/// rtm/radfolio/read_withdraw_fee_partner_bps.rtm
fn read_withdraw_fee_partner_bps(component: &RadfolioComponent) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_withdraw_fee_partner_bps.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FEE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_FEE.captures(&output).expect("Failed to parse read_withdraw_fee_partner_bps")[1].to_string(),
        "Decimal")
}

/// Reads the maximum investment level for an investment vehicle, via
/// rtm/investmentvehicle/read_max_investable.rtm
fn iv_read_max_investable(component: &str) -> Option<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/investmentvehicle/read_max_investable.rtm")
                             .env("component", component));

    lazy_static! {
        static ref RE_DEC: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    tm_string_to_option(
        &RE_DEC.captures(&output).expect("Failed to parse read_max_investable")[1].to_string(),
        "Decimal")
}

/// Reads the current investment level in an investment vehicle, via
/// rtm/investmentvehicle/read_investment_value.rtm
fn iv_read_investment_value(component: &str) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/investmentvehicle/read_investment_value.rtm")
                             .env("component", component));

    lazy_static! {
        static ref RE_DEC: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_DEC.captures(&output).expect("Failed to parse read_investment_value")[1].to_string()
}

/// Reads the current stored fees, via
/// rtm/radfolio/read_fees_stored.rtm
fn read_fees_stored(component: &RadfolioComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_fees_stored.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_FUNDS: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ Decimal\("(.*)"\)"#
        )).unwrap();
    }

    RE_FUNDS.captures(&output).expect("Failed to parse read_fees_stored")[1].to_string()
}

/// Reads the current stored partner fees, via
/// rtm/radfolio/read_partner_fees_stored.rtm
fn read_partner_fees_stored(component: &RadfolioComponent,
                            partners: Option<HashSet<&str>>) -> HashMap<String, String>
{
    let mut prtm: String = "None".to_string();
    if partners.is_some() {
        prtm = "Some(HashSet<NonFungibleId>(".to_string();
        let mut first = true;
        for p in partners.unwrap() {
            if first {
                first = false;
            } else {
                prtm += ",";
            }
            prtm += &("NonFungibleId(\"".to_owned() + &p + "\")");
        }
        prtm += "))";
    }
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_partner_fees_stored.rtm")
                             .env("component", &component.address)
                             .env("partners", prtm));

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
            r#"Decimal\("([.\w]*)"\)"#,
        )).unwrap();
    }

    let hashmap = RE_MAP.captures(&output).expect(
        "Failed to parse read_partner_fees_stored")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashmap).collect();
    let mut fees: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    for element in elements {
        if element == "" { break; }
        if key.is_none() {
            key = Some(RE_KEY.captures(&element).expect("Failed to parse key")[1].to_string());
        } else {
            fees.insert(
                key.unwrap(),
                RE_VALUE.captures(&element).expect("Failed to parse value")[1].to_string());
            key = None;
        }
    }

    fees
}

/// Sets/clears the "allow any partner" flag, via
/// rtm/radfolio/set_allow_any_partner.rtm
fn set_allow_any_partner(component: &RadfolioComponent,
                         account: &Account,
                         allow: bool) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/set_allow_any_partner.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("allow", allow.to_string()));
}


/// Reads the "allow any partner" flag, via
/// rtm/radfolio/read_allow_any_partner.rtm
fn read_allow_any_partner(component: &RadfolioComponent) -> bool
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_allow_any_partner.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_BOOL: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    RE_BOOL.captures(&output).expect("Failed to parse read_allow_any_partner")[1].parse().unwrap()
}


/// Adds one or more partners to the approved list, via
/// rtm/radfolio/add_approved_partners.rtm
fn add_approved_partners(component: &RadfolioComponent,
                         account: &Account,
                         partners: HashSet<String>) {
    let mut prtm: String = "HashSet<NonFungibleId>(".to_string();
    let mut first = true;
    for p in partners {
        if first {
            first = false;
        } else {
            prtm += ",";
        }
        prtm += &("NonFungibleId(\"".to_owned() + &p + "\")");
    }
    prtm += ")";
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/add_approved_partners.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("partners", prtm));
}

/// Remove one or more partners from the approved list, via
/// rtm/radfolio/remove_approved_partners.rtm
fn remove_approved_partners(component: &RadfolioComponent,
                            account: &Account,
                            partners: HashSet<String>) {
    let mut prtm: String = "HashSet<NonFungibleId>(".to_string();
    let mut first = true;
    for p in partners {
        if first {
            first = false;
        } else {
            prtm += ",";
        }
        prtm += &("NonFungibleId(\"".to_owned() + &p + "\")");
    }
    prtm += ")";
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/remove_approved_partners.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("partners", prtm));
}

/// Clear all partners from the approved list, via
/// rtm/radfolio/clear_approved_partners.rtm
fn clear_approved_partners(component: &RadfolioComponent,
                            account: &Account) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/clear_approved_partners.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address));
}

/// Checks if a partner is currently approved, via
/// rtm/radfolio/is_partner_approved.rtm
fn is_partner_approved(component: &RadfolioComponent,
                       candidate: &str) -> bool
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/is_partner_approved.rtm")
                             .env("component", &component.address)
                             .env("candidate", &candidate));

    lazy_static! {
        static ref RE_BOOL: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ (.*)"#
        )).unwrap();
    }

    RE_BOOL.captures(&output).expect("Failed to parse is_partner_approved")[1].parse().unwrap()
}


/// Reads the list of approved partners, via
/// rtm/radfolio/read_approved_partners.rtm
fn read_approved_partners(component: &RadfolioComponent) -> HashSet<String>
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_approved_partners.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_SET: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashSet<NonFungibleId>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_PARTNER: Regex = Regex::new(concat!(
            r#"NonFungibleId\("(\w*)"\)"#,
        )).unwrap();
    }

    let hashset = RE_SET.captures(&output).expect(
        "Failed to parse read_approved_partners")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashset).collect();
    let mut partners: HashSet<String> = HashSet::new();
    for element in elements {
        if element == "" { break; }
        partners.insert(
            RE_PARTNER.captures(&element).expect("Failed to parse partner")[1].to_string());
    }

    partners
}


/// Deposits tokens into the fund, via
/// rtm/radfolio/deposit.rtm
fn deposit(radfolio: &RadfolioComponent, account: &Account,
           amount: &str, token: Option<&str>, partner: Option<&str>) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/deposit.rtm")
                .env("component", &radfolio.address)
                .env("account", &account.address)
                .env("amount", amount)
                .env("token", if token.is_some() { token.unwrap() } else { &radfolio.token_address })
                .env("partner", option_to_tm_string(partner, "NonFungibleId")));
}


/// Withdraws tokens from the fund, via
/// rtm/radfolio/withdraw.rtm
///
/// Note that "amount" is number of coupons to cash out.
fn withdraw(radfolio: &RadfolioComponent, account: &Account,
            amount: &str, token: Option<&str>, partner: Option<&str>) {
    let output = 
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/withdraw.rtm")
                .env("component", &radfolio.address)
                .env("account", &account.address)
                .env("amount", amount)
                .env("coupon", if token.is_some() { token.unwrap() } else { &radfolio.coupon_address })
                .env("partner", option_to_tm_string(partner, "NonFungibleId")));
    println!("{}", output);
}

/// Forces the fund to do a full maintenance cycle, via
/// rtm/radfolio/force_fund_maintenance.rtm
fn force_fund_maintenance(component: &RadfolioComponent, account: &Account)
{
    let output = 
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/force_fund_maintenance.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address));
    println!("{}", output);
}

/// Pulls out any protocol fees accrued, via
/// rtm/radfolio/withdraw_protocol_fees.rtm
fn withdraw_protocol_fees(component: &RadfolioComponent,
                          account: &Account) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/withdraw_protocol_fees.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address));
}

/// Pulls out partner fees accrued, via
/// rtm/radfolio/withdraw_partner_fees.rtm
fn withdraw_partner_fees(component: &RadfolioComponent,
                         account: &Account,
                         participants: &ParticipantsComponent,
                         partner: &str) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/withdraw_partner_fees.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("participants_nft_addr", &participants.nft_address)
                .env("partner_nfid", partner));
}

/// Creates a new Radfolio via
/// rtm/radfolio/instantiate_radfoltio.rtm
///
/// Returns the catalog created.
fn instantiate_interestbearing_mock(account: &Account, package_addr: &str,
                                    interest_percent_per_epoch: &str,
                                    treasury: &str,
                                    token: &str,
                                    admin_badge: &str,
                                    max_total_investment: Option<&str>)
                                   -> String
{
    
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/mock/instantiate_interestbearing_mock.rtm")
                             .env("account", &account.address)
                             .env("package", &package_addr)
                             .env("interest_percent_per_epoch", interest_percent_per_epoch)
                             .env("treasury", treasury)
                             .env("token", token)
                             .env("admin_badge", admin_badge)
                             .env("max_total_investment",
                                  option_to_tm_string(max_total_investment, "Decimal")));

    lazy_static! {
        static ref RE_TUPLE: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".*\n.*\n"#,
            r#".─ ComponentAddress\("(\w*)"\).*"#)).unwrap();
    }

    let matches = RE_TUPLE.captures(&output).expect(
        "Failed to parse instantiate_interestbearing_mock");

    matches[1].to_string()
}

/// Adds a new current investment vehicle, via
/// rtm/radfolio/add_investment_vehicle.rtm
fn add_investment_vehicle(component: &RadfolioComponent, account: &Account,
                          vehicle: &str, weight: &str) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/add_investment_vehicle.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("vehicle", vehicle)
                .env("weight", weight));
}

/// Change the weight of an investment vehicle, via
/// rtm/radfolio/modify_investment_vehicle.rtm
fn modify_investment_vehicle(component: &RadfolioComponent, account: &Account,
                             vehicle: &str, weight: &str) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/modify_investment_vehicle.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("vehicle", vehicle)
                .env("weight", weight));
}

/// Remove a number of investment vehicles, via
/// rtm/radfolio/remove_investment_vehicles.rtm
fn remove_investment_vehicles(component: &RadfolioComponent, account: &Account,
                              vehicles: HashSet<&str>) {
    let mut comps = "".to_owned();
    let mut first = true;
    for v in vehicles {
        if first {
            first = false;
        } else {
            comps += ",";
        }
        comps += &("ComponentAddress(\"".to_owned() + v + "\")");
    }
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/remove_investment_vehicles.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("vehicles", comps));
}

/// Remove all investment vehicles, via
/// rtm/radfolio/clear_investment_vehicles.rtm
fn clear_investment_vehicles(component: &RadfolioComponent, account: &Account) {
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/clear_investment_vehicles.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address));
}

/// Reads the list of current investment vehicles, via
/// rtm/radfolio/read_investment_vehicles.rtm
fn read_investment_vehicles(component: &RadfolioComponent)
                            -> HashMap<String, String> {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_investment_vehicles.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_MAP: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashMap<ComponentAddress, Decimal>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_KEY: Regex = Regex::new(concat!(
            r#"ComponentAddress\("(\w*)"\)"#,
        )).unwrap();
        static ref RE_VALUE: Regex = Regex::new(concat!(
            r#"Decimal\("([.\w]*)"\)"#,
        )).unwrap();
    }

    let hashmap = RE_MAP.captures(&output).expect(
        "Failed to parse read_investment_vehicles")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashmap).collect();
    let mut vehicles: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    for element in elements {
        if element == "" { break; }
        if key.is_none() {
            key = Some(RE_KEY.captures(&element).expect("Failed to parse key")[1].to_string());
        } else {
            vehicles.insert(
                key.unwrap(),
                RE_VALUE.captures(&element).expect("Failed to parse value")[1].to_string());
            key = None;
        }
    }

    vehicles
}

/// Halts one or more investment vehicles, via
/// rtm/radfolio/halt_investment_vehicles.rtm
fn halt_investment_vehicles(component: &RadfolioComponent, account: &Account,
                            vehicles: HashSet<&str>) {
    let mut comps = "".to_owned();
    let mut first = true;
    for v in vehicles {
        if first {
            first = false;
        } else {
            comps += ",";
        }
        comps += &("ComponentAddress(\"".to_owned() + v + "\")");
    }
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/halt_investment_vehicles.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("vehicles", comps));
}

/// Restarts one or more halted investment vehicles, via
/// rtm/radfolio/restart_investment_vehicles.rtm
fn restart_investment_vehicles(component: &RadfolioComponent, account: &Account,
                               vehicles: HashSet<&str>) {
    let mut comps = "".to_owned();
    let mut first = true;
    for v in vehicles {
        if first {
            first = false;
        } else {
            comps += ",";
        }
        comps += &("ComponentAddress(\"".to_owned() + v + "\")");
    }
    run_command(Command::new("resim")
                .arg("run")
                .arg("rtm/radfolio/restart_investment_vehicles.rtm")
                .env("component", &component.address)
                .env("account", &account.address)
                .env("admin_badge", &component.admin_address)
                .env("vehicles", comps));
}


/// Reads the list of halted investment vehicles, via
/// rtm/radfolio/read_halted_investment_vehicles.rtm
fn read_halted_investment_vehicles(component: &RadfolioComponent)
                                   -> HashSet<String> {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/radfolio/read_halted_investment_vehicles.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_SET: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashSet<ComponentAddress>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_VEHICLE: Regex = Regex::new(concat!(
            r#"ComponentAddress\("(\w*)"\)"#,
        )).unwrap();
    }

    let hashset = RE_SET.captures(&output).expect(
        "Failed to parse read_halted_investment_vehicles")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashset).collect();
    let mut vehicles: HashSet<String> = HashSet::new();
    for element in elements {
        if element == "" { break; }
        vehicles.insert(
            RE_VEHICLE.captures(&element).expect("Failed to parse vehicle")[1].to_string());
    }

    vehicles
}


/// Changes the default account by calling "resim set-default-account ..."
fn set_default_account(account: &Account) {
    run_command(Command::new("resim")
                .arg("set-default-account")
                .arg(&account.address)
                .arg(&account._privkey));
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
fn _maybe_some_u64(input: &str) -> Option<u64> {
    if input == "None" {
        return None;
    }
    lazy_static! {
        static ref RE_OPTION: Regex = Regex::new(r#"Some\((.*)u64\)"#).unwrap();
    }
    Some(RE_OPTION.captures(&input).expect("Invalid string-form Option")[1].parse().unwrap())
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

/// Helper function when debugging, dumps the current funds status to
/// console.
#[allow(dead_code)]
fn print_investments(radfolio: &RadfolioComponent) {
    let free_funds = read_free_funds(radfolio);
    let investments = read_investments(radfolio);

    let mut total: f64 = free_funds.parse().unwrap();
    println!("Free Funds: {}", free_funds);
    for (iv, funds) in investments {
        println!("{} in {}", funds, iv);
        total += funds.parse::<f64>().unwrap();
    }
    println!("Total funds: {}", total);
}

/// Asserts that two Decimal values (represented as strings) are
/// within a small delta of eachother.
fn assert_delta(left: &str, right: &str, msg: &str) {
    let left: f64 = left.parse().unwrap();
    let right: f64 = right.parse().unwrap();
    let delta: f64 = 0.0000001 * (left + right);
    assert!((left-right).abs() < delta,
            "{}: {} and {} are not within delta {}", msg, left, right, delta);
}

//
// Functionality tests follow below
//


/// Tests instantiation with default values where possible.
/// Incidentally also tests a bunch of read calls.
#[test]
fn test_instantiate_radfolio_with_defaults() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        None, // deposit fee bps
                                        None, // deposit fee partner bps
                                        None, // withdraw fee bps
                                        None, // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    assert_eq!("0", read_total_funds(&radfolio), "Total funds should be zero");
    assert_eq!("0", read_total_coupons(&radfolio), "Total coupons should be zero");
    assert_eq!(RADIX_TOKEN, read_investment_token(&radfolio), "Token should be XRD");
    assert_eq!(radfolio.coupon_address, read_coupon_address(&radfolio),
               "Coupon address should be consistent");
    assert_eq!(radfolio.admin_address, read_admin_badge_address(&radfolio),
               "Admin badge address should be consistent");
    assert_eq!(participants.nft_address, read_participants_nft_address(&radfolio).unwrap(),
               "Participants NFT address should be consistent");
    assert_eq!(0, read_investments(&radfolio).len(),
               "There shouldn't be any investments yet");
    assert_eq!("0", read_free_funds(&radfolio),
               "There shouldn't be any free funds yet");
    assert_eq!("10", read_free_funds_target_percent(&radfolio),
               "Free funds target should be as requested");
    assert_eq!(5, read_investment_update_interval_epochs(&radfolio),
               "Update interval should be as requested");
    assert_eq!(0, read_last_update_epoch(&radfolio),
               "Last update epoch should be zero");
    assert_eq!("1000", read_minimum_deposit(&radfolio),
               "Minimum deposit should be as requested");
    assert_eq!(None, read_deposit_fee_bps(&radfolio),
               "Deposit fee should be as requested");
    assert_eq!(None, read_deposit_fee_partner_bps(&radfolio),
               "Deposit partner fee should be as requested");
    assert_eq!(None, read_withdraw_fee_bps(&radfolio),
               "Withdraw fee should be as requested");
    assert_eq!(None, read_withdraw_fee_partner_bps(&radfolio),
               "Withdraw partner fee should be as requested");
    assert_eq!("0", read_fees_stored(&radfolio),
               "There shouldn't be any fees stored yet");
}


/// Tests instantiation without default values.
/// Incidentally also tests a bunch of read calls.
#[test]
fn test_instantiate_radfolio_with_overrides() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        Some("my admin badge"),
                                        1,    // admin badge quantity
                                        Some("my coupon"),
                                        Some("12"), // deposit fee bps
                                        Some("10"), // deposit fee partner bps
                                        Some("13"), // withdraw fee bps
                                        Some("11"), // withdraw fee partner bps
                                        Some("my mint badge"),
                                        Some("my iv control badge"));

    assert_eq!("0", read_total_funds(&radfolio), "Total funds should be zero");
    assert_eq!("0", read_total_coupons(&radfolio), "Total coupons should be zero");
    assert_eq!(RADIX_TOKEN, read_investment_token(&radfolio), "Token should be XRD");
    assert_eq!(radfolio.coupon_address, read_coupon_address(&radfolio),
               "Coupon address should be consistent");
    assert_eq!(radfolio.admin_address, read_admin_badge_address(&radfolio),
               "Admin badge address should be consistent");
    assert_eq!(participants.nft_address, read_participants_nft_address(&radfolio).unwrap(),
               "Participants NFT address should be consistent");
    assert_eq!(0, read_investments(&radfolio).len(),
               "There shouldn't be any investments yet");
    assert_eq!("0", read_free_funds(&radfolio),
               "There shouldn't be any free funds yet");
    assert_eq!("10", read_free_funds_target_percent(&radfolio),
               "Free funds target should be as requested");
    assert_eq!(5, read_investment_update_interval_epochs(&radfolio),
               "Update interval should be as requested");
    assert_eq!(0, read_last_update_epoch(&radfolio),
               "Last update epoch should be zero");
    assert_eq!("1000", read_minimum_deposit(&radfolio),
               "Minimum deposit should be as requested");
    assert_eq!("12", read_deposit_fee_bps(&radfolio).unwrap(),
               "Deposit fee should be as requested");
    assert_eq!("10", read_deposit_fee_partner_bps(&radfolio).unwrap(),
               "Deposit partner fee should be as requested");
    assert_eq!("13", read_withdraw_fee_bps(&radfolio).unwrap(),
               "Withdraw fee should be as requested");
    assert_eq!("11", read_withdraw_fee_partner_bps(&radfolio).unwrap(),
               "Withdraw partner fee should be as requested");
    assert_eq!("0", read_fees_stored(&radfolio),
               "There shouldn't be any fees stored yet");
}



/// Tests instantiation with invalid values.
#[test]
fn test_instantiate_radfolio_with_invalid_values() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "1000",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 Some("10"), // deposit fee bps
                                 Some("12"), // deposit fee partner bps
                                 None, // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Partner fee higher than protocol fee should fail");

    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "1000",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 None, // deposit fee partner bps
                                 Some("10"), // withdraw fee bps
                                 Some("12"), // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Partner fee higher than protocol fee should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "-10",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 None, // deposit fee partner bps
                                 None, // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative minimum deposit should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "0",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 Some("-1"), // deposit fee bps
                                 None, // deposit fee partner bps
                                 None, // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative deposit fee should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "0",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 Some("-1"), // deposit fee partner bps
                                 None, // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative deposit partner fee should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "0",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 None, // deposit fee partner bps
                                 Some("-1"), // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative withdraw fee should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "10", // free funds target %
                                 5,    // investment update interval epochs
                                 "0",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 None, // deposit fee partner bps
                                 None, // withdraw fee bps
                                 Some("-1"), // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative withdraw partner fee should fail");
    
    let result =
        std::panic::catch_unwind(|| {
            instantiate_radfolio(&user.address, &package_addr,
                                 RADIX_TOKEN,
                                 Some(&participants.nft_address),
                                 "-10",// free funds target %
                                 5,    // investment update interval epochs
                                 "0",// minimum deposit
                                 None, // admin badge name
                                 1,    // admin badge quantity
                                 None, // coupon name
                                 None, // deposit fee bps
                                 None, // deposit fee partner bps
                                 None, // withdraw fee bps
                                 None, // withdraw fee partner bps
                                 None, // mint badge name
                                 None);// iv control badge name
        });
    assert!(result.is_err(),
            "Negative free funds target % should fail");

}

/// Tests management of the approved_partners list
#[test]
fn test_partner_list_management() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);
    let alice_p = new_participant(&participants, &user,
                                  "Alice", "", "", None);
    let bob_p = new_participant(&participants, &user,
                                "Bob", "", "", None);
    let charlie_p = new_participant(&participants, &user,
                                    "Charlie", "", "", None);
    let debbie_p = new_participant(&participants, &user,
                                   "Debbie", "", "", None);
    let elsa_p = new_participant(&participants, &user,
                                 "Elsa", "", "", None);

    let radfolio =
        instantiate_radfolio(&user.address, &package_addr,
                             RADIX_TOKEN,
                             Some(&participants.nft_address),
                             "10", // free funds target %
                             5,    // investment update interval epochs
                             "1000",// minimum deposit
                             None, // admin badge name
                             1,    // admin badge quantity
                             None, // coupon name
                             None, // deposit fee bps
                             None, // deposit fee partner bps
                             None, // withdraw fee bps
                             None, // withdraw fee partner bps
                             None, // mint badge name
                             None);// iv control badge name

    assert!(!read_allow_any_partner(&radfolio),
            "allow_any_partner flag should start as false");
    assert_eq!(0, read_approved_partners(&radfolio).len(),
               "Approved partners list should start empty");
    set_allow_any_partner(&radfolio, &user, true);
    assert!(read_allow_any_partner(&radfolio),
            "allow_any_partner flag should now be true");
    assert!(is_partner_approved(&radfolio, &charlie_p),
            "Everyone should now be approved");
    assert_eq!(0, read_approved_partners(&radfolio).len(),
               "Approved partners list should still be empty");
    set_allow_any_partner(&radfolio, &user, false);
    assert!(!read_allow_any_partner(&radfolio),
            "allow_any_partner flag should now be false again");

    let mut partners: HashSet<String> = HashSet::new();
    partners.insert(alice_p.clone());
    partners.insert(bob_p.clone());
    add_approved_partners(&radfolio, &user, partners);

    assert_eq!(2, read_approved_partners(&radfolio).len(),
               "Approved partners list should now have members");
    assert!(is_partner_approved(&radfolio, &alice_p),
            "Alice should now be approved");
    assert!(is_partner_approved(&radfolio, &bob_p),
            "Bob should now be approved");
    assert!(!is_partner_approved(&radfolio, &charlie_p),
            "Charlie should not be approved");

    let partners = read_approved_partners(&radfolio);
    assert!(partners.contains(&alice_p),
            "Alice should be in the approved partners list");
    assert!(partners.contains(&bob_p),
            "Bob should be in the approved partners list");
    
    let mut partners: HashSet<String> = HashSet::new();
    partners.insert(debbie_p.clone());
    partners.insert(elsa_p.clone());
    add_approved_partners(&radfolio, &user, partners);

    assert_eq!(4, read_approved_partners(&radfolio).len(),
               "Approved partners list should be bigger");
    assert!(is_partner_approved(&radfolio, &alice_p),
            "Alice should still be approved");
    assert!(is_partner_approved(&radfolio, &bob_p),
            "Bob should still be approved");
    assert!(is_partner_approved(&radfolio, &debbie_p),
            "Debbie should now be approved");
    assert!(is_partner_approved(&radfolio, &elsa_p),
            "Elsa should now be approved");

    let partners = read_approved_partners(&radfolio);
    assert!(partners.contains(&alice_p),
            "Alice should be in the approved partners list");
    assert!(partners.contains(&bob_p),
            "Bob should be in the approved partners list");
    assert!(partners.contains(&debbie_p),
            "Debbie should be in the approved partners list");
    assert!(partners.contains(&elsa_p),
            "Elsa should be in the approved partners list");

    let mut partners: HashSet<String> = HashSet::new();
    partners.insert(debbie_p.clone());
    partners.insert(bob_p.clone());
    remove_approved_partners(&radfolio, &user, partners);

    assert_eq!(2, read_approved_partners(&radfolio).len(),
               "Approved partners list should be smaller");
    assert!(is_partner_approved(&radfolio, &alice_p),
            "Alice should still be approved");
    assert!(!is_partner_approved(&radfolio, &bob_p),
            "Bob should no longer be approved");
    assert!(!is_partner_approved(&radfolio, &debbie_p),
            "Debbie should no longer be approved");
    assert!(is_partner_approved(&radfolio, &elsa_p),
            "Elsa should still be approved");

    let partners = read_approved_partners(&radfolio);
    assert!(partners.contains(&alice_p),
            "Alice should be in the approved partners list");
    assert!(!partners.contains(&bob_p),
            "Bob should not be in the approved partners list");
    assert!(!partners.contains(&debbie_p),
            "Debbie should not be in the approved partners list");
    assert!(partners.contains(&elsa_p),
            "Elsa should be in the approved partners list");
    
    
    clear_approved_partners(&radfolio, &user);
    assert_eq!(0, read_approved_partners(&radfolio).len(),
               "Approved partners list should be empty");
    assert!(!is_partner_approved(&radfolio, &alice_p),
            "Alice should no longer be approved");
    assert!(!is_partner_approved(&radfolio, &bob_p),
            "Bob should still not be approved");
    assert!(!is_partner_approved(&radfolio, &debbie_p),
            "Debbie should still not be approved");
    assert!(!is_partner_approved(&radfolio, &elsa_p),
            "Elsa should no longer be approved");
}


/// Tests some setter methods.
/// Incidentally also tests some read methods.
#[test]
fn test_setters() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        Some("10"), // deposit fee bps
                                        None, // deposit fee partner bps
                                        Some("20"), // withdraw fee bps
                                        None, // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name


    // We have no source of correctness for this value so we only test
    // that it can be called without issues.
    assert!(read_mint_badge_address(&radfolio).len() > 0);
    
    // Test set_free_funds_target_percent
    assert_eq!("10", read_free_funds_target_percent(&radfolio),
               "Free funds target should be as requested in instantiate");
    set_free_funds_target_percent(&radfolio, &user, "20");
    assert_eq!("20", read_free_funds_target_percent(&radfolio),
               "Free funds target should be as sent to setter");
    let result =
        std::panic::catch_unwind(
            || set_free_funds_target_percent(&radfolio, &user, "-1"));
    assert!(result.is_err(),
            "Should not be able to set negative free funds target %");

    let result =
        std::panic::catch_unwind(
            || set_free_funds_target_percent(&radfolio, &user, "101"));
    assert!(result.is_err(),
            "Should not be able to set free funds target above 100%");


    // Test set_investment_update_interval_epochs
    assert_eq!(5, read_investment_update_interval_epochs(&radfolio),
               "Update interval should be as requested");
    set_investment_update_interval_epochs(&radfolio, &user, 5000);
    assert_eq!(5000, read_investment_update_interval_epochs(&radfolio),
               "Update interval should be as requested");


    // Test set_minimum_deposit
    assert_eq!("1000", read_minimum_deposit(&radfolio),
               "Minimum deposit should be as requested");

    set_minimum_deposit(&radfolio, &user, "10000");
    assert_eq!("10000", read_minimum_deposit(&radfolio),
               "Minimum deposit should be as requested");

    let result = 
        std::panic::catch_unwind(
            || set_minimum_deposit(&radfolio, &user, "-1"));
    assert!(result.is_err(),
            "Setting minimum deposit to negative should fail");

    
    // Test set_deposit_fee_partner_bps
    assert!(read_deposit_fee_partner_bps(&radfolio).is_none(),
            "Deposit partner fee should start undefined");

    set_deposit_fee_partner_bps(&radfolio, &user, Some("8"));
    assert_eq!("8", read_deposit_fee_partner_bps(&radfolio).unwrap(),
            "Deposit partner fee should now be as set");

    set_deposit_fee_partner_bps(&radfolio, &user, Some("10"));
    assert_eq!("10", read_deposit_fee_partner_bps(&radfolio).unwrap(),
            "Deposit partner fee should have changed");

    set_deposit_fee_partner_bps(&radfolio, &user, None);
    assert!(read_deposit_fee_partner_bps(&radfolio).is_none(),
            "Deposit partner fee should be undefined again");

    let result = 
        std::panic::catch_unwind(
            || set_deposit_fee_partner_bps(&radfolio, &user, Some("11")));
    assert!(result.is_err(),
            "Setting deposit partner fee above protocol fee should fail");

    
    // Test set_withdraw_fee_partner_bps
    assert!(read_withdraw_fee_partner_bps(&radfolio).is_none(),
            "Withdraw partner fee should start undefined");

    set_withdraw_fee_partner_bps(&radfolio, &user, Some("18"));
    assert_eq!("18", read_withdraw_fee_partner_bps(&radfolio).unwrap(),
            "Withdraw partner fee should now be as set");

    set_withdraw_fee_partner_bps(&radfolio, &user, Some("20"));
    assert_eq!("20", read_withdraw_fee_partner_bps(&radfolio).unwrap(),
            "Withdraw partner fee should have changed");

    set_withdraw_fee_partner_bps(&radfolio, &user, None);
    assert!(read_withdraw_fee_partner_bps(&radfolio).is_none(),
            "Withdraw partner fee should be undefined again");

    let result = 
        std::panic::catch_unwind(
            || set_withdraw_fee_partner_bps(&radfolio, &user, Some("21")));
    assert!(result.is_err(),
            "Setting withdraw partner fee above protocol fee should fail");
}



/// Tests some deposit/withdraw sequences on a fund with no 
/// deposit/withdraw fees.
#[test]
fn test_deposit_withdraw_a_no_fees() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        None, // deposit fee bps
                                        None, // deposit fee partner bps
                                        None, // withdraw fee bps
                                        None, // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    assert_eq!("100", value_of_coupons(&radfolio, "100"),
              "Starting coupon value should be at unity");
    
    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("995000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down 5k");
    assert_eq!("5000", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up 5k coupons");
    assert_eq!("5000", read_total_funds(&radfolio),
               "Fund should be up 5k");

    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("990000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another 5k");
    assert_eq!("10000", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up another 5k coupons");
    assert_eq!("10000", read_total_funds(&radfolio),
               "Fund should be up by another 5k");

    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("985000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another 5k");
    assert_eq!("15000", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up another 5k coupons");
    assert_eq!("15000", read_total_funds(&radfolio),
               "Fund should be up by another 5k");

    withdraw(&radfolio, &user, "2000", None, None);
    assert_eq!("987000", get_balance(&user, RADIX_TOKEN),
               "Investor should be back up by 2k");
    assert_eq!("13000", get_balance(&user, &radfolio.coupon_address),
               "Investor should be down by 2k coupons");
    assert_eq!("13000", read_total_funds(&radfolio),
               "Fund should be down by 2k");

    assert_eq!("100", value_of_coupons(&radfolio, "100"),
              "Coupon value without profit or losses should be at unity");

    let result =
        std::panic::catch_unwind(
            || deposit(&radfolio, &user, "999", None, None));
    assert!(result.is_err(),
            "Deposit below minimum should fail");

    let result =
        std::panic::catch_unwind(
            || deposit(&radfolio, &user, "5000", Some(&radfolio.coupon_address), None));
    assert!(result.is_err(),
            "Deposit of wrong token type should fail");
    
    let result =
        std::panic::catch_unwind(
            || withdraw(&radfolio, &user, "5000", Some(RADIX_TOKEN), None));
    assert!(result.is_err(),
            "Withdraw with wrong token type should fail");

    let result =
        std::panic::catch_unwind(
            || withdraw(&radfolio, &user, "15000", None, None));
    assert!(result.is_err(),
            "Withdraw with more coupons than we have should fail");

    withdraw(&radfolio, &user, "13000", None, None);
    assert_eq!("1000000", get_balance(&user, RADIX_TOKEN),
               "Investor should have all their money back");
    assert_eq!("0", get_balance(&user, &radfolio.coupon_address),
               "Investor should have no coupons left");
    assert_eq!("0", read_total_funds(&radfolio),
               "Fund should be emptied out");

    assert_eq!("100", value_of_coupons(&radfolio, "100"),
              "Coupon value should still be at unity");

}


/// Tests some deposit/withdraw sequences on a fund with regular
/// deposit/withdraw fees but not partner fees.
#[test]
fn test_deposit_withdraw_b_no_partner_fees() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        Some("10"), // deposit fee bps
                                        None, // deposit fee partner bps
                                        Some("20"), // withdraw fee bps
                                        None, // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("995000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down ~5k");
    assert_eq!("4995", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("4995", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("5", read_fees_stored(&radfolio),
               "Fund fees should be up by 5");

    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("990000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another ~5k");
    assert_eq!("9990", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up another ~5k coupons");
    assert_eq!("9990", read_total_funds(&radfolio),
               "Fund should be up by another ~5k");
    assert_eq!("10", read_fees_stored(&radfolio),
               "Fund fees should be up by 5");

    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("985000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another ~5k");
    assert_eq!("14985", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up another ~5k coupons");
    assert_eq!("14985", read_total_funds(&radfolio),
               "Fund should be up by another ~5k");
    assert_eq!("15", read_fees_stored(&radfolio),
               "Fund fees should be up by 5");

    withdraw(&radfolio, &user, "2000", None, None);
    assert_eq!("986996", get_balance(&user, RADIX_TOKEN),
               "Investor should be back up by ~2k");
    assert_eq!("12985", get_balance(&user, &radfolio.coupon_address),
               "Investor should be down by 2k coupons");
    assert_eq!("12985", read_total_funds(&radfolio),
               "Fund should be down by 2k");
    assert_eq!("19", read_fees_stored(&radfolio),
               "Fund fees should be up by 4");

    withdraw(&radfolio, &user, "12985", None, None);
    assert_eq!("999955.03", get_balance(&user, RADIX_TOKEN),
               "Investor should have most of their money back");
    assert_eq!("0", get_balance(&user, &radfolio.coupon_address),
               "Investor should have no coupons left");
    assert_eq!("0", read_total_funds(&radfolio),
               "Fund should be emptied out");
    assert_eq!("44.97", read_fees_stored(&radfolio),
               "Fund fees should be up by ~26");

    withdraw_protocol_fees(&radfolio, &user);
    assert_eq!("1000000", get_balance(&user, RADIX_TOKEN),
               "User should now have all funds back");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should have been grabbed");
}


/// Tests some deposit/withdraw sequences on a fund with partner
/// deposit/withdraw fees but no protocol fees.
#[test]
fn test_deposit_withdraw_c_only_partner_fees() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        None, // deposit fee bps
                                        Some("10"), // deposit fee partner bps
                                        None, // withdraw fee bps
                                        Some("20"), // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    let bob_p = new_participant(&participants, &user,
                                "Bob", "", "", None);
    let charlie_p = new_participant(&participants, &user,
                                    "Charlie", "", "", None);

    let result =
        std::panic::catch_unwind(
            || deposit(&radfolio, &user, "5000", None, Some(&bob_p)));
    assert!(result.is_err(),
            "Should not be able to use non-approved partner");

    let result =
        std::panic::catch_unwind(
            || set_deposit_fee_partner_bps(&radfolio, &user, Some("1")));
    assert!(result.is_err(),
            "Should not be able to change partner deposit fee when no protocol fee");
    
    let result =
        std::panic::catch_unwind(
            || set_withdraw_fee_partner_bps(&radfolio, &user, Some("1")));
    assert!(result.is_err(),
            "Should not be able to change partner withdraw fee when no protocol fee");

    // We are testing "allow any partner" setup here
    set_allow_any_partner(&radfolio, &user, true);

    // Bob is partner
    deposit(&radfolio, &user, "5000", None, Some(&bob_p));
    assert_eq!("995000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down 5k");
    assert_eq!("4995", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("4995", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should be zero");
    assert_eq!("5", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should be 5");

    // Bob is partner again
    deposit(&radfolio, &user, "5000", None, Some(&bob_p));
    assert_eq!("990000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another 5k");
    assert_eq!("9990", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("9990", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should be zero");
    assert_eq!("10", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should up by 5");

    // No partner this time
    deposit(&radfolio, &user, "5000", None, None);
    assert_eq!("985000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another 5k");
    assert_eq!("14990", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up 5k coupons");
    assert_eq!("14990", read_total_funds(&radfolio),
               "Fund should be up 5k");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should be zero");
    assert_eq!("10", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should have stood still");

    // Charlie is partner
    deposit(&radfolio, &user, "5000", None, Some(&charlie_p));
    assert_eq!("980000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down another 5k");
    assert_eq!("19985", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("19985", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should be zero");
    assert_eq!("10", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should have stood still");
    assert_eq!("5", read_partner_fees_stored(&radfolio, None).get(&charlie_p).unwrap(),
               "Charlie's accrued fees should by up by 5");

    // Charlie is partner
    withdraw(&radfolio, &user, "2000", None, Some(&charlie_p));
    assert_eq!("981996", get_balance(&user, RADIX_TOKEN),
               "Investor should be back up by ~2k");
    assert_eq!("17985", get_balance(&user, &radfolio.coupon_address),
               "Investor should be down by 2k coupons");
    assert_eq!("17985", read_total_funds(&radfolio),
               "Fund should be down by 2k");
    assert_eq!("0", read_fees_stored(&radfolio),
               "Fund fees should be zero");
    assert_eq!("10", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should have stood still");
    assert_eq!("9", read_partner_fees_stored(&radfolio, None).get(&charlie_p).unwrap(),
               "Charlie's accrued fees should by up by 4");

    withdraw_partner_fees(&radfolio, &user, &participants, &bob_p);
    assert_eq!("982006", get_balance(&user, RADIX_TOKEN),
               "User should be up by Bob's fees");
    assert!(read_partner_fees_stored(&radfolio, None).get(&bob_p).is_none(),
            "Bob should have retrieved all his fees");
    assert_eq!("9", read_partner_fees_stored(&radfolio, None).get(&charlie_p).unwrap(),
               "Charlie's accrued fees should be unchanged");

    withdraw_partner_fees(&radfolio, &user, &participants, &charlie_p);
    assert!(read_partner_fees_stored(&radfolio, None).get(&charlie_p).is_none(),
            "Charlie should have retrieved all his fees");
    withdraw_protocol_fees(&radfolio, &user);
    withdraw(&radfolio, &user, "17985", None, None);
    assert_eq!("1000000", get_balance(&user, RADIX_TOKEN),
               "User should now have all funds back");
}

/// Tests some deposit/withdraw sequences on a fund with both partner
/// and protocol deposit/withdraw fees
#[test]
fn test_deposit_withdraw_d_partner_and_protocol_fees() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        Some("15"), // deposit fee bps
                                        Some("5"), // deposit fee partner bps
                                        Some("30"), // withdraw fee bps
                                        Some("20"), // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    let bob_p = new_participant(&participants, &user,
                                "Bob", "", "", None);
    let charlie_p = new_participant(&participants, &user,
                                    "Charlie", "", "", None);
    let debbie_p = new_participant(&participants, &user,
                                   "Debbie", "", "", None);

    // We are testing the "only approved partners" setup here
    assert!(!read_allow_any_partner(&radfolio),
            "Fund should default to only allowing approved partners");

    add_approved_partners(&radfolio, &user,
                          vec![bob_p.clone(),
                               charlie_p.clone()]
                          .into_iter().collect());

    let result =
        std::panic::catch_unwind(
            || deposit(&radfolio, &user, "5000", None, Some(&debbie_p)));
    assert!(result.is_err(),
            "Debbie should not be allowed to partner");
    
    // Bob is partner
    deposit(&radfolio, &user, "5000", None, Some(&bob_p));
    assert_eq!("995000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down 5k");
    assert_eq!("4992.5", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("4992.5", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("5", read_fees_stored(&radfolio),
               "Fund fees should be 5");
    assert_eq!("2.5", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should be 2.5");

    // Charlie is partner
    deposit(&radfolio, &user, "10000", None, Some(&charlie_p));
    assert_eq!("985000", get_balance(&user, RADIX_TOKEN),
               "Investor should be down 5k");
    assert_eq!("14977.5", get_balance(&user, &radfolio.coupon_address),
               "Investor should be up ~5k coupons");
    assert_eq!("14977.5", read_total_funds(&radfolio),
               "Fund should be up ~5k");
    assert_eq!("15", read_fees_stored(&radfolio),
               "Fund fees should be up 10");
    assert_eq!("5", read_partner_fees_stored(&radfolio, None).get(&charlie_p).unwrap(),
               "Charlie's accrued fees should be 5");

    // Test correct handling of a removed partner
    remove_approved_partners(&radfolio, &user,
                          vec![bob_p.clone()]
                             .into_iter().collect());
    assert!(!read_approved_partners(&radfolio).contains(&bob_p),
            "Bob should no longer be partner");

    let result =
        std::panic::catch_unwind(
            || deposit(&radfolio, &user, "5000", None, Some(&bob_p)));
    assert!(result.is_err(),
            "Bob should no longer be able to be partner");

    // He should be able to claim his fees however
    assert_eq!("2.5", read_partner_fees_stored(&radfolio, None).get(&bob_p).unwrap(),
               "Bob's accrued fees should still be 2.5");
    withdraw_partner_fees(&radfolio, &user, &participants, &bob_p);
    assert!(read_partner_fees_stored(&radfolio, None).get(&bob_p).is_none(),
               "Bob's accrued fees should be cleared out");
    assert_eq!("985002.5", get_balance(&user, RADIX_TOKEN),
               "Bob's partner fees should have been returned");

    // Clearing out remaining stored fees is already adequately
    // covered in other deposit_withdraw tests so we end it here.
    
}

/// Tests some deposit/withdraw sequences on a fund with both partner
/// and protocol deposit/withdraw fees
#[test]
fn test_investment_vehicle_management() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        Some("15"), // deposit fee bps
                                        Some("5"), // deposit fee partner bps
                                        Some("30"), // withdraw fee bps
                                        Some("20"), // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    let iv_control_address = read_iv_control_badge_address(&radfolio);

    // The Xaviers are providing treasury funds for our mocks

    let xavier1 = create_account();
    set_default_account(&xavier1);
    let mock1 = instantiate_interestbearing_mock(&xavier1, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier2 = create_account();
    set_default_account(&xavier2);
    let mock2 = instantiate_interestbearing_mock(&xavier2, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier3 = create_account();
    set_default_account(&xavier3);
    let mock3 = instantiate_interestbearing_mock(&xavier3, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier4 = create_account();
    set_default_account(&xavier4);
    let mock4 = instantiate_interestbearing_mock(&xavier4, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier5 = create_account();
    set_default_account(&xavier5);
    let mock5 = instantiate_interestbearing_mock(&xavier5, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment

    set_default_account(&user);

    assert_eq!(0, read_investment_vehicles(&radfolio).len(),
               "Fund should start without any investment vehicles");

    add_investment_vehicle(&radfolio, &user, &mock1, "5");
    add_investment_vehicle(&radfolio, &user, &mock2, "6");
    add_investment_vehicle(&radfolio, &user, &mock3, "7");
    add_investment_vehicle(&radfolio, &user, &mock4, "8");

    assert_eq!(4, read_investment_vehicles(&radfolio).len(),
               "Fund should have four vehicles");

    assert_eq!("5", read_investment_vehicles(&radfolio).get(&mock1).unwrap(),
               "Fund should have correct mock1");
    assert_eq!("6", read_investment_vehicles(&radfolio).get(&mock2).unwrap(),
               "Fund should have correct mock2");
    assert_eq!("7", read_investment_vehicles(&radfolio).get(&mock3).unwrap(),
               "Fund should have correct mock3");
    assert_eq!("8", read_investment_vehicles(&radfolio).get(&mock4).unwrap(),
               "Fund should have correct mock4");

    let result =
        std::panic::catch_unwind(
            || add_investment_vehicle(&radfolio, &user, &mock1, "10"));
    assert!(result.is_err(),
            "Should not be able to add a vehicle twice");
    assert_eq!("5", read_investment_vehicles(&radfolio).get(&mock1).unwrap(),
               "We should still have a correct mock1");

    modify_investment_vehicle(&radfolio, &user, &mock3, "20");
    assert_eq!("20", read_investment_vehicles(&radfolio).get(&mock3).unwrap(),
               "Fund should have a modified mock3");
    
    let result =
        std::panic::catch_unwind(
            || modify_investment_vehicle(&radfolio, &user, &mock5, "15"));
    assert!(result.is_err(),
            "Should not be able to modify a vehicle we don't have");
    assert!(!read_investment_vehicles(&radfolio).contains_key(&mock5),
            "We should not have now acquired mock5");

    remove_investment_vehicles(&radfolio, &user,
                               vec![&*mock2,&*mock3,&*mock4].into_iter().collect());
    
    assert_eq!(1, read_investment_vehicles(&radfolio).len(),
               "Fund should have one vehicle left");
    assert_eq!("5", read_investment_vehicles(&radfolio).get(&mock1).unwrap(),
               "Fund should still have mock1");

    add_investment_vehicle(&radfolio, &user, &mock3, "7");
    add_investment_vehicle(&radfolio, &user, &mock4, "8");
    assert_eq!(3, read_investment_vehicles(&radfolio).len(),
               "Fund should be back up to three vehicles");

    clear_investment_vehicles(&radfolio, &user);
    assert_eq!(0, read_investment_vehicles(&radfolio).len(),
               "Fund should not have investment vehicles");

    add_investment_vehicle(&radfolio, &user, &mock1, "5");
    add_investment_vehicle(&radfolio, &user, &mock2, "6");
    add_investment_vehicle(&radfolio, &user, &mock3, "7");
    add_investment_vehicle(&radfolio, &user, &mock4, "8");

    assert_eq!(0, read_halted_investment_vehicles(&radfolio).len(),
               "Fund should not have any halted vehicles");

    halt_investment_vehicles(&radfolio, &user,
                             vec![&*mock2,&*mock3,&*mock4].into_iter().collect());
    assert_eq!(3, read_halted_investment_vehicles(&radfolio).len(),
               "Fund should have three halted vehicles");
    assert!(read_halted_investment_vehicles(&radfolio).contains(&mock2),
            "mock2 should be halted");
    assert!(read_halted_investment_vehicles(&radfolio).contains(&mock3),
            "mock3 should be halted");
    assert!(read_halted_investment_vehicles(&radfolio).contains(&mock4),
            "mock4 should be halted");
    
    restart_investment_vehicles(&radfolio, &user,
                                vec![&*mock2,&*mock4].into_iter().collect());
    
    assert_eq!(1, read_halted_investment_vehicles(&radfolio).len(),
               "Fund should have one halted vehicle");
    assert!(read_halted_investment_vehicles(&radfolio).contains(&mock3),
            "mock3 should still be halted");

    add_investment_vehicle(&radfolio, &user, &mock5, "9");
    halt_investment_vehicles(&radfolio, &user,
                             vec![&*mock1,&*mock5].into_iter().collect());
    assert_eq!(3, read_halted_investment_vehicles(&radfolio).len(),
               "Fund should have three halted vehicles");

    remove_investment_vehicles(&radfolio, &user,
                               vec![&*mock1,&*mock2,&*mock3].into_iter().collect());
    assert_eq!(1, read_halted_investment_vehicles(&radfolio).len(),
               "Removing vehicles should also have removed halted status");
    assert!(read_halted_investment_vehicles(&radfolio).contains(&mock5),
            "mock5 should still be halted");

    clear_investment_vehicles(&radfolio, &user);
    assert_eq!(0, read_investment_vehicles(&radfolio).len(),
               "Fund should not have investment vehicles");
    assert_eq!(0, read_halted_investment_vehicles(&radfolio).len(),
               "Clearing vehicles should also have removed halted status");
}


/// Puts actual funds into actual (mock) investment vehicles to
/// exercise the fund maintenance logic.
#[test]
fn test_investment_maintenance() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &user.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&user.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "10", // free funds target %
                                        50,   // investment update interval epochs
                                        "100",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // coupon name
                                        None, // deposit fee bps
                                        None, // deposit fee partner bps
                                        None, // withdraw fee bps
                                        None, // withdraw fee partner bps
                                        None, // mint badge name
                                        None);// iv control badge name

    let iv_control_address = read_iv_control_badge_address(&radfolio);

    // The Xaviers are providing treasury funds for our mocks

    let xavier1 = create_account();
    set_default_account(&xavier1);
    let mock1 = instantiate_interestbearing_mock(&xavier1, &package_addr,
                                                 "1", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier2 = create_account();
    set_default_account(&xavier2);
    let mock2 = instantiate_interestbearing_mock(&xavier2, &package_addr,
                                                 "2", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier3 = create_account();
    set_default_account(&xavier3);
    let mock3 = instantiate_interestbearing_mock(&xavier3, &package_addr,
                                                 "3", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 Some("25000")); // max investment
    let xavier4 = create_account();
    set_default_account(&xavier4);
    let mock4 = instantiate_interestbearing_mock(&xavier4, &package_addr,
                                                 "4", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 Some("50000"));   // max investment
    let xavier5 = create_account();
    set_default_account(&xavier5);
    let mock5 = instantiate_interestbearing_mock(&xavier5, &package_addr,
                                                 "5", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment

    set_default_account(&user);

    assert!(iv_read_max_investable(&mock1).is_none(),
            "mock1 shouldn't have a max investment level");
    assert_eq!("25000", iv_read_max_investable(&mock3).unwrap(),
               "mock3 should have a max investment level");
    
    add_investment_vehicle(&radfolio, &user, &mock1, "5");
    add_investment_vehicle(&radfolio, &user, &mock2, "6");
    add_investment_vehicle(&radfolio, &user, &mock3, "7");
    add_investment_vehicle(&radfolio, &user, &mock4, "8");
    add_investment_vehicle(&radfolio, &user, &mock5, "10");

    deposit(&radfolio, &user, "5000", None, None);
    deposit(&radfolio, &user, "5000", None, None);
    deposit(&radfolio, &user, "5000", None, None);

    // All of the above deposits should have resulted in fund
    // maintenance and resulting funding of the mocks.
    //
    // 15000 have been deposited, and the fund retains 10% so 13500
    // should have been distributed.
    //
    // Total weight is 36 => mock1 has 5/36 of the 13500 = 1875 etc.
    let investments = read_investments(&radfolio);
    assert_delta(investments.get(&mock1).unwrap(), "1875", "mock1");
    assert_delta("1875", &iv_read_investment_value(&mock1),"mock1");
    assert_delta(investments.get(&mock2).unwrap(), "2250", "mock2");
    assert_delta(investments.get(&mock3).unwrap(), "2625", "mock3");
    assert_delta(investments.get(&mock4).unwrap(), "3000", "mock4");
    assert_delta(investments.get(&mock5).unwrap(), "3750", "mock5");

    assert_delta("1500", &read_free_funds(&radfolio), "Free funds");


    // This is too small a deposit to push the fund into automatic
    // maintenance
    deposit(&radfolio, &user, "100", None, None);
    // So everything ends up in free funds
    assert_delta("1600", &read_free_funds(&radfolio), "Free funds");


    // Let's force the matter
    force_fund_maintenance(&radfolio, &user);

    // 15100 have been deposited, and the fund retains 10% so 13590
    // should have been distributed.
    let investments = read_investments(&radfolio);
    assert_delta(investments.get(&mock1).unwrap(), "1887.5", "mock1");
    assert_delta("1887.5", &iv_read_investment_value(&mock1),"mock1");
    assert_delta(investments.get(&mock2).unwrap(), "2265"  , "mock2");
    assert_delta(investments.get(&mock3).unwrap(), "2642.5", "mock3");
    assert_delta(investments.get(&mock4).unwrap(), "3020"  , "mock4");
    assert_delta(investments.get(&mock5).unwrap(), "3775"  , "mock5");

    assert_delta("1510", &read_free_funds(&radfolio), "Free funds");


    // Let's add enough so that mock3 reaches its max investment.
    deposit(&radfolio, &user, "150000", None, None);

    // At this point mock3 is overfull and should have stopped at 25k.
    // The others should have had the remainder divided between them
    // by weight, with the new total weight (excluding mock3) being 29.
    //
    // Total funds are 165100, 16510 are withheld as free funds,
    // and mock3 has 25000, leaving 123590 for the remaining ones.
    //
    // So mock1 has 5/29 of 123590 = ~21308.62 etc.
    let investments = read_investments(&radfolio);
    assert_delta(investments.get(&mock1).unwrap(), "21308.62"    , "mock1");
    assert_delta("21308.62", &iv_read_investment_value(&mock1)   , "mock1");
    assert_delta(investments.get(&mock2).unwrap(), "25570.3448"  , "mock2");
    assert_delta(investments.get(&mock3).unwrap(), "25000"       , "mock3");
    assert_delta(investments.get(&mock4).unwrap(), "34093.7931"  , "mock4");
    assert_delta(investments.get(&mock5).unwrap(), "42617.24138" , "mock5");

    assert_delta("16510", &read_free_funds(&radfolio), "Free funds");

    // Now we let an epoch pass so the investment vehicles can
    // generate some interest for us.
    set_current_epoch(1);

    // Total interest generated should be:
    // mock1: 1% of 21308.62 = 213
    // mock2: 2& of 25570.3448 = 511.4
    // mock3: 3% of 25000 = 750
    // mock4: 4% of 34093.7931 = 1363.7517
    // mock5: 5% of 42617.24138 = 2130.86
    // for a total of 4969.1069

    // Let's collect them
    force_fund_maintenance(&radfolio, &user);

    // Previous total funds of 165100 plus interest above is
    // 170069.1069
    //
    // 10% held as free funds = 17006.91069
    //
    // 25000 taken by capped mock3
    //
    // Leaves 128062.19621 to be distributed among the remaining 29
    // weight
    //
    // So mock3 = 128062.19621 * (5/29) = 22079.689 etc.
    let investments = read_investments(&radfolio);
    assert_delta(investments.get(&mock1).unwrap(), "22079.689" , "mock1");
    assert_delta("22079.689", &iv_read_investment_value(&mock1), "mock1");
    assert_delta(investments.get(&mock2).unwrap(), "26495.6268", "mock2");
    assert_delta(investments.get(&mock3).unwrap(), "25000"     , "mock3");
    assert_delta(investments.get(&mock4).unwrap(), "35327.5024", "mock4");
    assert_delta(investments.get(&mock5).unwrap(), "44159.378" , "mock5");

    assert_delta("17006.91069", &read_free_funds(&radfolio), "Free funds");
    

    // We now have 170069.1069 in the fund but we only ever put 165100
    // in. This means there is now profit and coupon value should have
    // increased accordingly.
    // 170069.1069 / 165100 = 103.009756
    assert_delta("103.009756", &value_of_coupons(&radfolio, "100"),
                 "Coupon value");


    // If we now pull out a small amount this should not affect the
    // investments but should reduce free funds.
    withdraw(&radfolio, &user, "2000", None, None);
    assert_delta("14946.7156", &read_free_funds(&radfolio), "Free funds");

    // This leaves a total of 168008.912 in the fund


    // We move time forwards again to generate some profits for us to
    // extract
    set_current_epoch(11);

    // Total interest generated should be:
    // mock1: 10% of 22079.689  =  2207.9689
    // mock2: 20& of 26495.6268 =  5299.1254
    // mock3: 30% of 25000      =  7500
    // mock4: 40% of 35327.5024 = 14131.001
    // mock5: 50% of 44159.378  = 22079.689
    // for a total of 51217.784

    // If we cash in 22k coupons this should give us 29570.74 XRD
    // which were all taken from profits. The remainder of profits
    // should be in free funds.
    withdraw(&radfolio, &user, "22000", None, None);
    assert_delta("36593.762", &read_free_funds(&radfolio), "Free funds");

    // Since our free funds are less than 2x the target this didn't
    // cause an automatic distribution of funds out to investment
    // vehicles.


    // The time has come to wind down the mock1 vehicle. We will want
    // to pull out any funds still in it and remove it from the
    // radfolio.
    modify_investment_vehicle(&radfolio, &user, &mock1, "0");
    force_fund_maintenance(&radfolio, &user);
    remove_investment_vehicles(&radfolio, &user, vec![&*mock1].into_iter().collect());

    // mock1 should now have zero funds left, and its funds
    // should have been redistributed to the other vehicles.

    // Total fund value now is 189655.96.
    //
    // After maintenance we expect this to have happened:
    //
    // 10% held as free funds = 18965.596
    //
    // 25000 taken by capped mock3
    //
    // Leaves 145690.364 to be distributed among the remaining 24
    // weight
    //
    // So mock4 = 145690.364 * (6/24) = 36422.591 etc.
    let investments = read_investments(&radfolio);
    assert!(investments.get(&mock1).is_none(), "mock1 should not be here");
    assert_delta(investments.get(&mock2).unwrap(), "36422.591", "mock2");
    assert_delta("36422.591", &iv_read_investment_value(&mock2),"mock2");
    assert_delta(investments.get(&mock3).unwrap(), "25000"    , "mock3");
    assert_delta(investments.get(&mock4).unwrap(), "48563.455", "mock4");
    assert_delta(investments.get(&mock5).unwrap(), "60704.318", "mock5");

    assert_delta("18965.596", &read_free_funds(&radfolio), "Free funds");
    


    // We've decided to shut down the fund so we'll set everything to
    // zero weight and cash out.
    modify_investment_vehicle(&radfolio, &user, &mock2, "0");
    modify_investment_vehicle(&radfolio, &user, &mock3, "0");
    modify_investment_vehicle(&radfolio, &user, &mock4, "0");
    modify_investment_vehicle(&radfolio, &user, &mock5, "0");
    force_fund_maintenance(&radfolio, &user);

    // Total interest has been 4969.1069 + 51217.784 = 56186.8909
    //
    // So we should now have our starting stash of 1 mill plus that
    // interest = 1056186.8909
    
    let my_coupons = get_balance(&user, &radfolio.coupon_address);
    withdraw(&radfolio, &user, &my_coupons, None, None);
    assert_delta("1056186.8909", &get_balance(&user, RADIX_TOKEN), "balance");
}


/// Sets up a Radfolio with all the options and puts it through a
/// flurry of activity to see if it can take the pressure. This tests
/// not correctness of results as much as it tests stability through
/// chaos.
#[test]
fn test_flurry_of_activity() {
    reset_sim();
    let owner = create_account();
    let package_addr = publish_package(None);
    let participants_package_addr = publish_package(Some("tests/demifi.wasm"));
    let participants = instantiate_participant_catalog(
        &owner.address, &participants_package_addr);

    let radfolio = instantiate_radfolio(&owner.address, &package_addr,
                                        RADIX_TOKEN,
                                        Some(&participants.nft_address),
                                        "7",  // free funds target %
                                        10,   // investment update interval epochs
                                        "10", // minimum deposit
                                        Some("admin!"), // admin badge name
                                        21,   // admin badge quantity
                                        Some("coupon!"), // coupon name
                                        Some("3"), // deposit fee bps
                                        Some("3"), // deposit fee partner bps
                                        Some("7"), // withdraw fee bps
                                        Some("1"), // withdraw fee partner bps
                                        Some("mint!"), // mint badge name
                                        Some("iv control!"));// iv control badge name

    let iv_control_address = read_iv_control_badge_address(&radfolio);

    // The Xaviers are providing treasury funds for our mocks

    let xavier1 = create_account();
    set_default_account(&xavier1);
    let mock1 = instantiate_interestbearing_mock(&xavier1, &package_addr,
                                                 "0.001", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier2 = create_account();
    set_default_account(&xavier2);
    let mock2 = instantiate_interestbearing_mock(&xavier2, &package_addr,
                                                 "0.002", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment
    let xavier3 = create_account();
    set_default_account(&xavier3);
    let mock3 = instantiate_interestbearing_mock(&xavier3, &package_addr,
                                                 "0.003", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 Some("2500")); // max investment
    let xavier4 = create_account();
    set_default_account(&xavier4);
    let mock4 = instantiate_interestbearing_mock(&xavier4, &package_addr,
                                                 "0.004", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 Some("5000"));   // max investment
    let xavier5 = create_account();
    set_default_account(&xavier5);
    let mock5 = instantiate_interestbearing_mock(&xavier5, &package_addr,
                                                 "0.005", // interest per epoch
                                                 "1000000", // treasury
                                                 RADIX_TOKEN,
                                                 &iv_control_address,
                                                 None);   // max investment

    set_default_account(&owner);

    set_allow_any_partner(&radfolio, &owner, true);

    add_investment_vehicle(&radfolio, &owner, &mock1, "1");
    add_investment_vehicle(&radfolio, &owner, &mock2, "100");
    add_investment_vehicle(&radfolio, &owner, &mock3, "75");
    add_investment_vehicle(&radfolio, &owner, &mock4, "20");

    // Alice is an investor
    let alice = create_account();

    // Bob is a highly regarded influencer
    let bob = create_account();
    let bob_p = new_participant(&participants, &bob, "Bob", "", "", None);

    // Charlie is an investor
    let charlie = create_account();

    // Debbie is an investor
    let debbie = create_account();

    // Eric is an investor
    let eric = create_account();
    
    // Fiona is an investor
    let fiona = create_account();

    // Geordi is an influencer
    let geordi = create_account();
    let geordi_p = new_participant(&participants, &geordi, "Geordi", "", "", None);

    
    // Investors put money in, some through referral links
    
    set_default_account(&alice);
    deposit(&radfolio, &alice, "100", None, Some(&bob_p));

    set_default_account(&charlie);
    deposit(&radfolio, &charlie, "500", None, None);
    
    set_default_account(&debbie);
    deposit(&radfolio, &debbie, "50", None, Some(&bob_p));

    set_default_account(&eric);
    deposit(&radfolio, &eric, "233.24", None, None);

    set_default_account(&fiona);
    deposit(&radfolio, &fiona, "999.99", None, Some(&geordi_p));

    // Time passes
    set_current_epoch(10);

    
    // Introducing a new investment vehicle
    set_default_account(&owner);
    add_investment_vehicle(&radfolio, &owner, &mock5, "110");
    // Balance it in immediately
    force_fund_maintenance(&radfolio, &owner);


    // Investors fomo in more monies

    set_default_account(&alice);
    deposit(&radfolio, &alice, "24", None, Some(&geordi_p));

    set_default_account(&charlie);
    deposit(&radfolio, &charlie, "167", None, None);
    
    set_default_account(&debbie);
    deposit(&radfolio, &debbie, "10", None, Some(&bob_p));

    set_default_account(&eric);
    deposit(&radfolio, &eric, "32", None, None);

    set_default_account(&fiona);
    deposit(&radfolio, &fiona, "199.99", None, Some(&geordi_p));


    // Time passes
    set_current_epoch(23);

    // An investment vehicle is halted for maintenance reasons
    set_default_account(&owner);
    halt_investment_vehicles(&radfolio, &owner, vec![&*mock3].into_iter().collect());

    // Time passes
    set_current_epoch(29);

    // mock3 was Debbie's favourite part of the fund, she's out
    set_default_account(&debbie);
    withdraw(&radfolio, &debbie,
             &get_balance(&debbie, &radfolio.coupon_address),
             None, Some(&bob_p));

    // There turns out to be serious problems with mock3, we remove it
    // completely from the fund.
    set_default_account(&owner);
    remove_investment_vehicles(&radfolio, &owner, vec![&*mock3].into_iter().collect());
    // Balance things immediately
    force_fund_maintenance(&radfolio, &owner);

    // Time passes
    set_current_epoch(39);

    // Eric is unhappy about the handling of mock3 and exits. He is a
    // whale and so he has to exit gradually in order to generate the
    // liquidity needed.
    set_default_account(&eric);
    withdraw(&radfolio, &eric,
             "100",
             None, None);
    withdraw(&radfolio, &eric,
             "75",
             None, None);
    withdraw(&radfolio, &eric,
             "75",
             None, None);
    withdraw(&radfolio, &eric,
             &get_balance(&eric, &radfolio.coupon_address),
             None, None);

    // Time passes
    set_current_epoch(84);
    

    // The operators of the mock5 investment are shutting down shop,
    // turns out their business model wasn't sustainable. We will do
    // an orderly removal of mock5.
    set_default_account(&owner);
    modify_investment_vehicle(&radfolio, &owner, &mock5, "0");
    force_fund_maintenance(&radfolio, &owner);
    remove_investment_vehicles(&radfolio, &owner, vec![&*mock5].into_iter().collect());


    // Time passes
    set_current_epoch(112);


    // Another round of fomo in from remaining investors

    set_default_account(&alice);
    deposit(&radfolio, &alice, "230", None, Some(&geordi_p));

    set_default_account(&charlie);
    deposit(&radfolio, &charlie, "438", None, None);
    
    set_default_account(&fiona);
    deposit(&radfolio, &fiona, "42", None, Some(&geordi_p));
    

    // Time passes
    set_current_epoch(142);

    // We decided to stop offering incentives for cashing out
    set_default_account(&owner);
    set_withdraw_fee_partner_bps(&radfolio, &owner, None);

    // Time passes
    set_current_epoch(161);

    // Charlie needs some money to pay a bill
    set_default_account(&charlie);
    withdraw(&radfolio, &charlie, "70", None, None);
    withdraw(&radfolio, &charlie, "73", None, None);

    // Time passes
    set_current_epoch(161);

    // Alice is going on vacation, needs some spending money
    set_default_account(&alice);
    withdraw(&radfolio, &alice, "90", None, Some(&bob_p));
    withdraw(&radfolio, &alice, "90", None, Some(&bob_p));
    withdraw(&radfolio, &alice, "70", None, Some(&bob_p));

    // Bob cashes out some sweet sweet shilling rewards
    set_default_account(&bob);
    withdraw_partner_fees(&radfolio, &bob, &participants, &bob_p);

    // Time passes
    set_current_epoch(211);

    // Fund managers need to eat too you know
    set_default_account(&owner);
    withdraw_protocol_fees(&radfolio, &owner);

    // Time passes
    set_current_epoch(231);

    // mock3 has been fixed - finally! - enough so that we can pull
    // out our trapped funds in it.
    add_investment_vehicle(&radfolio, &owner, &mock3, "0");
    force_fund_maintenance(&radfolio, &owner);
    remove_investment_vehicles(&radfolio, &owner, vec![&*mock3].into_iter().collect());
}
