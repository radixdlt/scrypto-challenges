extern crate core;

// Some utils
mod utils;

// The three main system blueprints of the dao-kit toolkit
pub mod membership_system;
pub mod code_execution_system;
pub mod voting_system;

// A wrapper blueprint tying the above three systems together in a convinient way
pub mod simple_dao_system;

// A workaround for bug https://github.com/radixdlt/radixdlt-scrypto/issues/483
pub mod component_address_repo;

// Demo components showing how the dao-kit toolkit can be used
// I would have liked to put those into their own crates but had problems getting the import!
// macro to work -_-
pub mod demo_do_good_dao;
pub mod demo_flex_dao;
