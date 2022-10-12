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
use scrypto::prelude::*;

const RADIX_TOKEN: &str = "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag";


#[derive(Debug)]
struct Account {
    address: String,
    _pubkey: String,
    _privkey: String,
}

impl Account
{
    pub fn get_amount_owned(&self, resource_address: &str) -> Option<Decimal>
    {
        let output = run_command(Command::new("resim")
            .arg("show")
            .arg(&self.address));

        let mut lines = output.split("\n").collect::<Vec<&str>>();

        loop
        {
            match lines.pop()
            {
                None => { break; }
                Some(line) =>
                    {
                        let words = line.split(" ").collect::<Vec<&str>>();
                        // Resource line is og the form
                        // ├─ { amount: 999.67126747, resource address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag, name: "Radix", symbol: "XRD" }
                        let word_before = words.get(5);
                        let resource_address_2 = words.get(6);
                        let amount = words.get(3);
                        match word_before
                        {
                            None => {}
                            Some(word) =>
                                {
                                    if *word == "address:"
                                    {
                                        match resource_address_2
                                        {
                                            None => {}
                                            Some(address) =>
                                                {
                                                    let address_without_comma = address.split(",").collect::<Vec<&str>>();
                                                    let correct_address = address_without_comma.get(0).unwrap();
                                                    if *correct_address == resource_address
                                                    {
                                                        let word = amount.unwrap().split(",").collect::<Vec<&str>>();
                                                        let number = word.get(0).unwrap();
                                                        return Some(Decimal::from(*number));
                                                    }
                                                }
                                        }
                                    }
                                }
                        }
                    }
            }
        }

        None
    }
}


#[derive(Debug)]
struct DAO_component {
    address: String,
    external_admin_address: String,
    internal_admin_address : String,
    styx_address: String,
    voter_card_address: String,
}

impl DAO_component
{
    pub fn get_amount_owned(&self, account_address: &str, resource_address: &str) -> Option<Decimal>
    {
        let output = amount_owned(account_address, &self.address, resource_address);
        let mut lines = output.split("\n").collect::<Vec<&str>>();

        loop
        {
            match lines.pop()
            {
                None => { break; }
                Some(line) =>
                    {
                        // We are looking for a line of the form ├─ Decimal("90")
                        let words = line.split(" ").collect::<Vec<&str>>();
                        match words.get(1)
                        {
                            None => {}
                            Some(word) =>
                                {
                                    // We split the word with "

                                    let subwords = word.split("\"").collect::<Vec<&str>>();
                                    let word_before = subwords.get(0);
                                    let amount = subwords.get(1);

                                    match word_before
                                    {
                                        None => {  }
                                        Some(word2) =>
                                            {
                                                if *word2 == "Decimal("
                                                {
                                                    return Some(Decimal::from(*amount.unwrap()))
                                                }
                                            }
                                    }
                                }
                        }
                    }
            }
        }
        None
    }

    pub fn get_locked_tokens(&self, account_address: &str) -> Decimal
    {
        let output = amount_locked(account_address, &self.address);
        let mut lines = output.split("\n").collect::<Vec<&str>>();
        let mut return_dec = dec!(0);
        loop
        {
            match lines.pop()
            {
                None => { break; }
                Some(line) =>
                    {
                        // We are looking for a line of the form ├─ Decimal("90")
                        let words = line.split(" ").collect::<Vec<&str>>();
                        match words.get(1)
                        {
                            None => {}
                            Some(word) =>
                                {
                                    // We split the word with "

                                    let subwords = word.split("\"").collect::<Vec<&str>>();
                                    let word_before = subwords.get(0);
                                    let amount = subwords.get(1);

                                    match word_before
                                    {
                                        None => {  }
                                        Some(word2) =>
                                            {
                                                if *word2 == "Decimal("
                                                {
                                                    return_dec = Decimal::from(*amount.unwrap())
                                                }
                                            }
                                    }
                                }
                        }
                    }
            }
        }
        return_dec
    }
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

// Create a token and return it's address
fn create_admin_badge() -> String {
    let output = run_command(Command::new("resim")
                            .arg("new-token-fixed")
                            .arg("--name")
                            .arg("admin_badge")
                            .arg("1")
                        );

    String::from(output.split("\n").collect::<Vec<&str>>()[13].split(" ").collect::<Vec<&str>>()[2])

}

fn show(address: &str) {

    let output = run_command(Command::new("resim")
                            .arg("show")
                            .arg(address)
                        );
    println!("{}", output);
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




/// Creates a new Dao catalog via
/// rtm/instantiate.rtm
///
/// Returns the dao created.
fn instantiate(account_addr: &str, package_addr: &str)
                                   -> DAO_component
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/instantiate.rtm")
                             .env("account", account_addr)
                             .env("package", &package_addr)
                             .env("initial_supply", "100"));




    println!("{}",output);

    let result = output.split("\n").collect::<Vec<&str>>();

    let i = 4 ; // for translation due to more info

    let dao_address = result[13+i];
    let external_admin_address = result[14+i];
    let internal_admin_address = result[15+i];
    let styx_address = result[16+i];
    let voter_card_address = result[17+i];

    let dao_address = dao_address.split(" ").collect::<Vec<&str>>()[2];
    let external_admin_address = external_admin_address.split(" ").collect::<Vec<&str>>()[2];
    let internal_admin_address = internal_admin_address.split(" ").collect::<Vec<&str>>()[2];
    let styx_address = styx_address.split(" ").collect::<Vec<&str>>()[2];
    let voter_card_address = voter_card_address.split(" ").collect::<Vec<&str>>()[2];




    let dao = DAO_component {
        address: String::from(dao_address),
        external_admin_address: String::from(external_admin_address),
        internal_admin_address : String::from(internal_admin_address),
        styx_address: String::from(styx_address),
        voter_card_address: String::from(voter_card_address),
    };
    dao
}


/// Creates a new Dao catalog via
/// rtm/instantiate_custom.rtm
///
/// Returns the dao created.
fn instantiate_custom(account_addr: &str, package_addr: &str, admin_badge_addr: &str)
                                   -> DAO_component
{
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/instantiate.rtm")
                             .env("account", account_addr)
                             .env("package", &package_addr)
                             .env("admin_badge", admin_badge_addr)
                             .env("initial_supply", "100"));




    //println!("{}",output);

    let result = output.split("\n").collect::<Vec<&str>>();

    let i = 4 ; // for translation due to more info

    let dao_address = result[13+i];
    let external_admin_address = result[14+i];
    let internal_admin_address = result[15+i];
    let styx_address = result[16+i];
    let voter_card_address = result[17+i];

    let dao_address = dao_address.split(" ").collect::<Vec<&str>>()[2];
    let external_admin_address = external_admin_address.split(" ").collect::<Vec<&str>>()[2];
    let internal_admin_address = internal_admin_address.split(" ").collect::<Vec<&str>>()[2];
    let styx_address = styx_address.split(" ").collect::<Vec<&str>>()[2];
    let voter_card_address = voter_card_address.split(" ").collect::<Vec<&str>>()[2];




    let dao = DAO_component {
        address: String::from(dao_address),
        external_admin_address: String::from(external_admin_address),
        internal_admin_address : String::from(internal_admin_address),
        styx_address: String::from(styx_address),
        voter_card_address: String::from(voter_card_address),
    };
    dao
}


fn mint_voter_card_with_bucket(account_addr: &str,dao_address : &str , styx_address : &str, bucket_amount : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/mint_voter_card_with_bucket.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("styx", styx_address)
                             .env("amount", bucket_amount));
    output
}


fn withdraw(account_addr: &str,dao_address : &str , external_badge_address : &str, amount : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/withdraw.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("admin_badge", external_badge_address)
                             .env("amount", amount));
    output
    //println!("{}", output);
}


fn emit(account_addr: &str,dao_address : &str , external_badge_address : &str, amount : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/emit.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("admin_badge", external_badge_address)
                             .env("amount", amount));
    output
    //println!("{}", output);
}


fn lock(account_addr: &str, dao_address : &str , voter_card_address : &str, styx_address : &str, bucket_amount : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/lock.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("styx", styx_address)
                             .env("voter_card", voter_card_address)
                             .env("amount", bucket_amount));
    output
}


fn unlock(account_addr: &str, dao_address : &str , voter_card_address : &str, bucket_amount : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/unlock.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address)
                             .env("amount", bucket_amount));
    output
}


fn simple_transfer(account1_addr: &str, account2_addr: &str , asset_address : &str, amount : &str) -> String {
    let output = run_command(Command::new("resim")
        .arg("run")
        .arg("rtm/simple_transfer.rtm")
        .env("account1", account1_addr)
        .env("account2", account2_addr)
        .env("asset", &asset_address)
        .env("amount", amount));
    output
}

fn unlock_all(account_addr: &str, dao_address : &str , voter_card_address : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/unlock_all.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address));
    output
}


fn support_proposal(account_addr: &str, dao_address : &str , voter_card_address : &str, proposal_id : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/support_proposal.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address)
                             .env("proposal_id", proposal_id));
    output
}

fn advance_with_proposal(account_addr: &str, dao_address : &str , proposal_id : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/advance_with_proposal.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("proposal_id", proposal_id));
    output
}

fn delegate_for_proposal(account_addr: &str, dao_address : &str, voter_card_address : &str , proposal_id : &str, deleguate_to : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/delegate_for_proposal.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address)
                             .env("proposal_id", proposal_id)
                             .env("deleguate_to", deleguate_to));
    output
}

fn vote_for_proposal(account_addr: &str, dao_address : &str, voter_card_address : &str , proposal_id : &str, vote : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/delegate_for_proposal.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address)
                             .env("proposal_id", proposal_id)
                             .env("vote", vote));
    output
}


fn gift_asset(account_addr: &str, dao_address : &str , amount : &str, asset_address : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/gift_asset.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("asset", asset_address)
                             .env("amount", amount));
    output
}

fn amount_owned(account_addr: &str, dao_address : &str , asset_address : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/amount_owned.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("asset", asset_address));
    output
}

fn amount_locked(account_addr: &str, dao_address: &str) -> String {
    let output = run_command(Command::new("resim")
                            .arg("run")
                            .arg("rtm/amount_locked.rtm")
                            .env("account", account_addr)
                            .env("dao", &dao_address));

    output

}

fn claim_asset(account_addr: &str, dao_address : &str , voter_card_address : &str) -> String {
    let output = run_command(Command::new("resim")
                             .arg("run")
                             .arg("rtm/claim_assets.rtm")
                             .env("account", account_addr)
                             .env("dao", &dao_address)
                             .env("voter_card", voter_card_address));
    output
}


#[test]
fn test_publish() {
    reset_sim();
    let _user = create_account();
    let package_addr = publish_package(Some("."));
    println!("User Package : {:?}", package_addr);
}



#[test]
fn test_instantiate() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    println!("dao component : {:#?}", dao);
}

#[test]
fn test_instantiate_custom() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let admin_badge_addr = create_admin_badge();
    let dao = instantiate_custom(&user.address, &package_addr, &admin_badge_addr );
    let owned = dao.get_amount_owned(&user.address, &dao.styx_address);
    assert_eq!(owned.unwrap(), dec!(100));
}


#[test]
fn test_withdraw()
{
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "10");
    let user_owned_stx = user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(user_owned_stx, dec!(10));
    let dao_owned = dao.get_amount_owned(&user.address, &dao.styx_address);
    assert_eq!(dao_owned.unwrap(), dec!(90));
}

#[test]
fn test_mint_voter_card()
{
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "10");
    let mut owned_stx = user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(owned_stx, dec!(10));

    mint_voter_card_with_bucket(&user.address, &dao.address, &dao.styx_address, "5");
    owned_stx =  user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(owned_stx, dec!(5));
}


#[test]
fn test_emit() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);

    // Withdraw all tokens from styx vault
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "100");
    let mut dao_styx = dao.get_amount_owned(&user.address, &dao.styx_address).unwrap();
    let owned_stx = user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(dao_styx, dec!(0));
    assert_eq!(owned_stx, dec!(100));

    // Now emit new tokens
    emit(&user.address, &dao.address, &dao.external_admin_address, "1000");
    dao_styx = dao.get_amount_owned(&user.address, &dao.styx_address).unwrap();
    assert_eq!(dao_styx, dec!(1000));
}


#[test]
fn test_lock() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "10");
    mint_voter_card_with_bucket(&user.address, &dao.address, &dao.styx_address, "5");
    lock(&user.address, &dao.address, &dao.voter_card_address, &dao.styx_address, "5");
    let locked = dao.get_locked_tokens(&user.address);
    assert_eq!(locked, dec!(5));
}

#[test]
fn test_unlock() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "10");
    mint_voter_card_with_bucket(&user.address, &dao.address, &dao.styx_address, "5");
    unlock(&user.address, &dao.address, &dao.voter_card_address, "3");
    let owned_stx = user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(owned_stx, dec!(8));
}

#[test]
fn test_unlock_all() {
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    withdraw(&user.address, &dao.address, &dao.external_admin_address, "10");
    mint_voter_card_with_bucket(&user.address, &dao.address, &dao.styx_address, "5");
    unlock_all(&user.address, &dao.address, &dao.voter_card_address);
    let owned_stx = user.get_amount_owned(&dao.styx_address).unwrap();
    assert_eq!(owned_stx, dec!(10));
}

#[test]
fn test_gift_asset()
{
    reset_sim();
    let user = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user.address, &package_addr);
    gift_asset(&user.address, &dao.address, "10", RADIX_TOKEN);
    let dao_rdx = dao.get_amount_owned(&user.address, RADIX_TOKEN).unwrap();
    assert_eq!(dao_rdx, dec!(10));
}

#[test]
fn test_transferable_styx(){
    reset_sim();
    let user1 = create_account();
    let user2 = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user1.address, &package_addr);
    withdraw(&user1.address, &dao.address, &dao.external_admin_address, "10");
    simple_transfer(&user1.address, &user2.address, &dao.styx_address, "5");
    assert_eq!(user1.get_amount_owned(&dao.styx_address).unwrap(), dec!(5));
    assert_eq!(user2.get_amount_owned(&dao.styx_address).unwrap(), dec!(5));
}

#[test]
fn test_untransferable_voter_card(){
    reset_sim();
    let user1 = create_account();
    let user2 = create_account();
    let package_addr = publish_package(Some("."));
    let dao = instantiate(&user1.address, &package_addr);
    withdraw(&user1.address, &dao.address, &dao.external_admin_address, "10");
    mint_voter_card_with_bucket(&user1.address, &dao.address, &dao.styx_address, "5");
    let transfer_output = simple_transfer(&user1.address, &user2.address, &dao.voter_card_address, "5");
    // Fails correctly
    println!("{}",transfer_output);
}