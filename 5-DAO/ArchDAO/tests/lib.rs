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

use core::arch;
use std::process::Command;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use scrypto::debug;
use scrypto::prelude::Component;
use scrypto::prelude::ComponentAddress;
use scrypto::prelude::Decimal;

const RADIX_TOKEN: &str = "030000000000000000000000000000000000000000000000000004";

#[derive(Debug)]
struct Account {
    address: String,
    _pubkey: String,
    _privkey: String,
}

#[derive(Debug)]
struct ArchDAOComponent {
    address: String,
    admin_address: String,
    vote_token_address: String,
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

/// Calls "resim new-account"
///
/// Returns a tuple containing first the new account's address, then
/// its public key, and then last its private key.
fn create_new_token() -> String {
    let output = run_command(Command::new("resim")
                             .arg("new-token-fixed")
                             .arg("400"));
    lazy_static! {
        static ref RE_RESOURCE: Regex = Regex::new(r"└─ Resource: (\w*)").unwrap();
    }
    
    RE_RESOURCE.captures(&output).expect("Failed to parse new-token resource address")[1].to_string()
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

/// Creates a new ArchDAO via
/// rtm/archdao/instantiate_archdao.rtm
///
/// Returns the catalog created.
fn instantiate_archdao(
    account_addr: &str, 
    package_addr: &str,
    proposal_token: &str,
    free_funds_target_percent: &str,
    proposal_update_interval_epochs: u64,
    minimum_deposit: &str,
    admin_badge_name: Option<&str>,
    admin_badge_quantity: u64,
    vote_name: Option<&str>,
    deposit_fee_bps: Option<&str>,
    withdraw_fee_bps: Option<&str>,
    vote_mint_badge_name: Option<&str>,
    proposal_control_badge_name: Option<&str>)
               -> ArchDAOComponent
{
let output = run_command(Command::new("resim")
         .arg("run")
         .arg("rtm/archdao/instantiate_archdao.rtm")
         .env("account", account_addr)
         .env("package", &package_addr)
         .env("investment_token", proposal_token)
         .env("free_funds_target_percent", free_funds_target_percent)
         .env("proposal_update_interval_epochs", proposal_update_interval_epochs.to_string())
         .env("minimum_deposit", minimum_deposit)
         .env("admin_badge_name", option_string_to_tm_string(admin_badge_name))
         .env("admin_badge_quantity", admin_badge_quantity.to_string())
         .env("vote_name", option_string_to_tm_string(vote_name))
         .env("deposit_fee_bps", option_to_tm_string(deposit_fee_bps, "Decimal"))
         .env("withdraw_fee_bps", option_to_tm_string(withdraw_fee_bps, "Decimal"))
         .env("vote_mint_badge_name", option_string_to_tm_string(vote_mint_badge_name))
         .env("proposal_control_badge_name", option_string_to_tm_string(proposal_control_badge_name)));

lazy_static! {
static ref RE_TUPLE: Regex = Regex::new(concat!(
r#"Instruction Outputs:\n\W*"#,
r#".─ Tuple\(ComponentAddress\("(\w*)"\).*"#,
r#"ResourceAddress\("(\w*)"\).*"#,
r#"ResourceAddress\("(\w*)"\)"#)).unwrap();
}

let matches = RE_TUPLE.captures(&output).expect(
"Failed to parse instantiate_archdao");

println!("ArchDAO address {} " , matches[1].to_string());
println!("Admin address {} " , matches[2].to_string());

ArchDAOComponent {
address: matches[1].to_string(),
admin_address: matches[2].to_string(),
vote_token_address: matches[3].to_string(),
token_address: proposal_token.to_string(),
}
}




/// Finds the token we use for our proposal control badges, via
/// rtm/archdao/read_proposal_control_badge_address.rtm
fn read_proposal_control_badge_address(component: &ArchDAOComponent) -> String
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/read_proposal_control_badge_address.rtm")
                             .env("component", &component.address));

    lazy_static! {
        static ref RE_TOK: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ ResourceAddress\("(.*)"\)"#
        )).unwrap();
    }

    RE_TOK.captures(&output).expect("Failed to parse read_proposal_control_badge_address")[1].to_string()
}


/// Deposits tokens into the fund, via
/// rtm/archdao/deposit.rtm
fn fund_approved_projects(archdao: &ArchDAOComponent, account: &Account,
    amount: &str, token: Option<&str>) {

    println!("fund_approved_projects START");  

run_command(Command::new("resim")
         .arg("run")
         .arg("rtm/archdao/fund_approved_project.rtm")
         .env("component", &archdao.address)
         .env("account", &account.address)
         .env("admin_badge", &archdao.admin_address)
         .env("amount", amount)
         .env("token", if token.is_some() { token.unwrap() } else { &archdao.token_address }));
}

/// Creates a new ArchDAO via
/// rtm/mock/instantiate_proposal_mock.rtm
///
/// Returns the catalog created.
fn instantiate_proposal_mock(account: &Account, package_addr: &str,
    vote_token: &str,
    funding_token: &str,
    proposal_description: &str,
    money_received: &str,
    admin_badge: &str)
   -> String
{

let output = run_command(Command::new("resim")
.arg("run")
.arg("rtm/mock/instantiate_proposal_mock.rtm")
.env("account", &account.address)
.env("package", &package_addr)
.env("proposal_description", proposal_description)
.env("treasury", money_received)
.env("vote_token", vote_token)
.env("funding_token", funding_token)
.env("admin_badge", admin_badge));

lazy_static! {
static ref RE_TUPLE: Regex = Regex::new(concat!(
r#"Instruction Outputs:\n\W*"#,
r#".*\n.*\n"#,
r#".─ ComponentAddress\("(\w*)"\).*"#)).unwrap();
}

let matches = RE_TUPLE.captures(&output).expect(
"Failed to parse instantiate_proposal_mock");

matches[1].to_string()
}


/// Register an account 
/// rtm/archdao/register.rtm
fn register(archdao: &ArchDAOComponent, account: &Account,
    amount: &str, token: Option<&str>) {
run_command(Command::new("resim")
         .arg("run")
         .arg("rtm/archdao/register.rtm")
         .env("component", &archdao.address)
         .env("account", &account.address)
         .env("amount", amount)
         .env("token", if token.is_some() { token.unwrap() } else { &archdao.token_address }));
}

/// Adds a new proposal for discussion, via
/// rtm/archdao/add_proposal.rtm
fn add_proposal(component: &ArchDAOComponent, account: &Account,
    proposal_project: &str, proposal: &str) {
run_command(Command::new("resim")
.arg("run")
.arg("rtm/archdao/add_proposal.rtm")
.env("component", &component.address)
.env("account", &account.address)
.env("admin_badge", &component.admin_address)
.env("proposal_project", proposal_project)
.env("proposal", proposal));
}

/// Reads the list of current proposals, via
/// rtm/archdao/read_proposal_for_approval.rtm
fn vote_proposal(component: &ArchDAOComponent,proposal_project: &str, account: &Account, amount: &str, token: Option<&str>)
                            -> HashMap<String, String> {
    println!("Vote proposal, amout put to vote: {}", amount);  
    println!("Vote proposal, ArchDAO address: {}", component.address);
    println!("Vote proposal, Proposal Project Address: {}", proposal_project);
    println!("Vote proposal, Token Resource Address: {:?}", token);

    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/vote_proposal.rtm")
                             .env("component", &component.address)
                             .env("proposal_project", &proposal_project)
                             .env("amount", amount)
                             .env("account", &account.address)
                             .env("token", if token.is_some() { token.unwrap() } else { &component.token_address })
                             );

    println!("Add vote: {}", output);   
    
    let mut proposals: HashMap<String, String> = HashMap::new();
    
    proposals
}

/// Reads the list of current proposals, via
/// rtm/archdao/read_proposal_for_approval.rtm
fn add_vote(component: &ArchDAOComponent,proposal_project: &str, voto: &str)
                            -> HashMap<String, String> {
    println!("Add vote, amout put to vote: {}", voto);  
    println!("Add vote, ArchDAO address: {}", component.address);
    println!("Add vote, Proposal Project Address: {}", proposal_project);

    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/add_vote.rtm")
                             .env("component", &component.address)
                             .env("proposal_project", &proposal_project)
                             .env("vote", voto)
                             );

    println!("Add vote: {}", output);   
    
    let mut proposals: HashMap<String, String> = HashMap::new();
    
    proposals
}

/// Reads the list of current proposal, via
/// rtm/archdao/read_proposal_for_approval.rtm
fn read_proposal_for_approval(component: &ArchDAOComponent)
                            -> HashMap<String, String> {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/read_proposal_for_approval.rtm")
                             .env("component", &component.address));

    println!("Proposal for approval: {}", output);                             

    lazy_static! {
        static ref RE_MAP: Regex = Regex::new(concat!(
            r#"Instruction Outputs:\n\W*"#,
            r#".─ HashMap<ComponentAddress, Struct>\(([^\n]*)\)"#,
        )).unwrap();
        static ref RE_SPLIT: Regex = Regex::new(", ").unwrap();
        static ref RE_KEY: Regex = Regex::new(concat!(
            r#"ComponentAddress\("(\w*)"\)"#,
        )).unwrap();
        static ref RE_VALUE: Regex = Regex::new(concat!(
            r#"Struct\("([.\w]*)"\)"#,
        )).unwrap();
    }

    let hashmap = RE_MAP.captures(&output).expect(
        "Failed to parse read_proposal_for_approval")[1].to_string();
    let elements: Vec<&str> = RE_SPLIT.split(&hashmap).collect();
    println!("re map {}" , hashmap );
    println!("elements {:?}" , elements );
    let mut proposals: HashMap<String, String> = HashMap::new();
    let mut key: Option<String> = None;
    for element in elements {
        if element == "" { break; }
        if key.is_none() {
            debug!("Key of proposal: {}", element); 
            // TODO parse key needed
            // key = Some(RE_KEY.captures(&element).expect("Failed to parse key")[1].to_string());      
        } else {
            debug!("Element of proposal: {}", element); 
            proposals.insert(
                key.unwrap(),
                element.to_string());
                // TODO parse what needed
                // RE_VALUE.captures(&element).expect("Failed to parse value")[1].to_string());
            key = None;
        }
    }

    proposals
}


/// Reads the list of current proposal, via
/// rtm/archdao/list_proposal.rtm
fn list_proposal(component: &ArchDAOComponent)
                            -> HashMap<String, String> {
    println!("list_proposal");

    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/list_proposal.rtm")
                             .env("component", &component.address)
                             );

    println!("list_proposal result : {}", output);   
    
    let mut proposals: HashMap<String, String> = HashMap::new();
    
    proposals
}

/// Reads the list of current proposal, via
/// rtm/archdao/list_proposal.rtm
fn approve_proposal(component: &ArchDAOComponent, account: &Account)
                            -> HashMap<String, String> {
    println!("approve_proposal");

    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/archdao/approve_proposal.rtm")
                             .env("account", &account.address)
                             .env("admin_badge", &component.admin_address)
                             .env("component", &component.address)
                             );

    println!("approve_proposal result : {}", output);   
    
    let mut proposals: HashMap<String, String> = HashMap::new();
    
    proposals
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

    println!("Account balance: \n {}", output);
                             
    let regexp = r#".─ \{ amount: ([\d.]*), resource address: "#.to_string() + resource_addr + ",";
    let re_balance: Regex = Regex::new(&regexp).unwrap();

    re_balance.captures(&output).expect("Failed to parse balance")[1].to_string()
}

/// Retreives a user's current balance for the requested asset by
/// calling "resim show ..."
fn get_component(account: &ArchDAOComponent, resource_addr: &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("show")
                             .arg(&account.address));
    println!("ArchDAO  balance: \n {}", output);
    let regexp = r#".─ \{ amount: ([\d.]*), resource address: "#.to_string() + resource_addr + ",";
    let re_balance: Regex = Regex::new(&regexp).unwrap();

    re_balance.captures(&output).expect("Failed to parse balance")[1].to_string()
}

/// Retreives a user's current balance for the requested asset by
/// calling "resim show ..."
fn get_proposal(account: &str, resource_addr: &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("show")
                             .arg(&account));
    println!("Proposal balance: \n {}", output);
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


//
// Functionality tests follow below
//

/// Tests instantiation with default values where possible.
/// Incidentally also tests a bunch of read calls.
#[test]
fn test_instantiate_archdao_with_defaults() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(None);

    let archdao = instantiate_archdao(
                                        &user.address, 
                                        &package_addr,
                                        RADIX_TOKEN,
                                        "10", // free funds target %
                                        5u64,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1u64,    // admin badge quantity
                                        None, // vote name
                                        None, // deposit fee bps
                                        None, // withdraw fee bps
                                        None, // mint badge name
                                        None);// proposal control badge name
   
}


/// Tests some vote/leave proposale sequences on a proposal 
#[test]
fn test_proposal_management() {
    reset_sim();
    let admin_user = create_account();
    let mut vote_token = create_new_token();
    let package_addr = publish_package(None);

    let archdao = instantiate_archdao(&admin_user.address, &package_addr,
                                        RADIX_TOKEN,
                                        "10", // free funds target %
                                        5,    // investment update interval epochs
                                        "1000",// minimum deposit
                                        None, // admin badge name
                                        1,    // admin badge quantity
                                        None, // vote name
                                        Some("15"), // deposit fee bps
                                        Some("30"), // withdraw fee bps
                                        None, // mint badge name
                                        None);// proposal control badge name

    println!("archdao: {}", archdao.address);
    println!("vote token resource address: {}", vote_token);
    let mut vote_token = &archdao.vote_token_address;
    println!("vote token resource address minted: {}", vote_token);

    let proposal_control_address = read_proposal_control_badge_address(&archdao);
    println!("proposal_control_address: {} ", proposal_control_address);

    // The Admin are providing treasury funds for our mocks
    let admin1 = create_account();
    set_default_account(&admin1);
    let text_proposal: String = "Proposed something that could be used to teach Scrypto".to_string();
    let mock1 = instantiate_proposal_mock(&admin1, &package_addr,&vote_token,RADIX_TOKEN,                                                    
                                                 &text_proposal, // text proposal
                                                 "1000000", // treasury
                                                 &proposal_control_address);
    println!("mock1: {} ", mock1);

    let admin2 = create_account();
    set_default_account(&admin2);
    let mock2 = instantiate_proposal_mock(&admin2, &package_addr,&vote_token,RADIX_TOKEN,
        "Proposed something that could be used to teach Rust", // text proposal
        "1000000", // treasury
        &proposal_control_address);
    println!("mock2: {} ", mock2);

    let admin3 = create_account();
    set_default_account(&admin3);
    let mock3 = instantiate_proposal_mock(&admin3, &package_addr,&vote_token,RADIX_TOKEN,
        "Propose something that could be used to teach Web3 DApps", // text proposal
        "1000000", // treasury
        &proposal_control_address);
    println!("mock3: {} ", mock3);        
        
    let admin4 = create_account();
    set_default_account(&admin4);
    let mock4 = instantiate_proposal_mock(&admin4, &package_addr,&vote_token,RADIX_TOKEN,
        "Propose something that could be used to teach Network Gateway", // text proposal
        "1000000", // treasury
        &proposal_control_address);
    println!("mock4: {} ", mock4);        

    let admin5 = create_account();
    set_default_account(&admin5);
    let mock5 = instantiate_proposal_mock(&admin5, &package_addr,&vote_token,RADIX_TOKEN,
        "Propose something that could be used to teach how to run a full node", // text proposal
        "1000000", // treasury
        &proposal_control_address);
    println!("mock5: {} ", mock5);        

    set_default_account(&admin_user);

    //add proposal to the dao (only from user admin)
    add_proposal(&archdao, &admin_user, &mock1, "5");
    add_proposal(&archdao, &admin_user, &mock2, "6");
    add_proposal(&archdao, &admin_user, &mock3, "7");
    add_proposal(&archdao, &admin_user, &mock4, "8");
    add_proposal(&archdao, &admin_user, &mock5, "9");

    //TODO
    assert_eq!(0, read_proposal_for_approval(&archdao).len(),
               "ArchDAO should have five proposal");

    //Create another user and let him vote
    let user1 = create_account();
    set_default_account(&user1);               

    //get some right for voting
    register(&archdao, &user1, "5000", Some(RADIX_TOKEN));

    //let's vote
    vote_proposal(&archdao, &mock5, &user1, "10", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock4, &user1, "20", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock3, &user1, "50", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock2, &user1, "15", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock1, &user1, "5", Some(&archdao.vote_token_address));

    let mut balance = get_balance(&admin_user, RADIX_TOKEN);
    println!("Balance of current admin user {}", balance);   

    //user1 should have 100xrd less
    //he should have also some tokens from the proposal component //TO BE IMPLEMENTED
    balance = get_balance(&user1, RADIX_TOKEN);
    println!("Balance of current user1 {}", balance);   

    //Create another user and let him vote
    let user2 = create_account();
    set_default_account(&user2);
    //get some right for voting
    register(&archdao, &user2, "5000", Some(RADIX_TOKEN));
    //let's vote
    vote_proposal(&archdao, &mock5, &user2, "25", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock4, &user2, "20", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock3, &user2, "5", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock2, &user2, "40", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock1, &user2, "10", Some(&archdao.vote_token_address));

    balance = get_balance(&user2, RADIX_TOKEN);
    println!("Balance of current user 2 {}", balance);   

    //Create another user and let him vote
    let user3 = create_account();
    set_default_account(&user3);
    //get some right for voting
    register(&archdao, &user3, "5000", Some(RADIX_TOKEN));
    //let's vote
    vote_proposal(&archdao, &mock5, &user3, "40", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock4, &user3, "35", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock3, &user3, "5", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock2, &user3, "10", Some(&archdao.vote_token_address));
    vote_proposal(&archdao, &mock1, &user3, "10", Some(&archdao.vote_token_address));

    balance = get_balance(&user3, RADIX_TOKEN);
    println!("Balance of current user 3 {}", balance);   

    //move epoch to lool at vote values
    set_current_epoch(90);

    //calculate how is it going the approval process
    list_proposal(&archdao);

    println!("Balance of Proposal Project ");   
    get_proposal(&mock5, RADIX_TOKEN);
    get_proposal(&mock4, RADIX_TOKEN);
    get_proposal(&mock3, RADIX_TOKEN);
    get_proposal(&mock2, RADIX_TOKEN);
    get_proposal(&mock1, RADIX_TOKEN);
    get_proposal(&mock5, &archdao.vote_token_address);
    get_proposal(&mock4, &archdao.vote_token_address);
    get_proposal(&mock3, &archdao.vote_token_address);
    get_proposal(&mock2, &archdao.vote_token_address);
    get_proposal(&mock1, &archdao.vote_token_address);

    set_default_account(&admin_user);
    //fund the approval process
    fund_approved_projects(&archdao, &admin_user, "5000", None);
    
    balance = get_component(&archdao, RADIX_TOKEN);
    println!("Balance of ArchDAO before approval {} " , balance);       
    balance = get_proposal(&mock1, RADIX_TOKEN);    
    println!("Balance of Proposal Project 1 before approval {}" , balance);       
    balance = get_proposal(&mock5, RADIX_TOKEN);
    println!("Balance of Proposal Project 5 before approval {}" , balance);  

    let mut balance_vote = get_proposal(&mock5, &archdao.vote_token_address);
    println!("Balance of Proposal Project 5 before approval (XRD) {}" , balance);       
    println!("Balance of Proposal Project 5 before approval (VOTE) {}" , balance_vote);       

    //calculate how is it going the approval process
    approve_proposal(&archdao, &admin_user);

    balance = get_component(&archdao, RADIX_TOKEN);
    println!("Balance of ArchDAO after approval {}" , balance);   //ERROR (this is the output of the first vault)
    balance = get_proposal(&mock5, RADIX_TOKEN);
    balance_vote = get_proposal(&mock5, &archdao.vote_token_address);
    println!("Balance of Proposal Project 5 after approval (XRD) {}" , balance);   
    println!("Balance of Proposal Project 5 after approval (VOTE) {}" , balance_vote);           
    
   
}
