use clap::Parser;
use clap::Subcommand;
 use scrypto::buffer::scrypto_encode;
// use std::fs::read_to_string;
use std::fs;
use std::path::PathBuf;

use radix_engine::ledger::*;
//use radix_engine::transaction::*;
use scrypto::types::EcdsaPublicKey;
use scrypto::utils::sha256;
use simulator::resim::*;
use simulator::ledger::*;

//use hareswap::api::*;

/// hareswap CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    NewKeyPair(NewKeyPair),
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ResimError(simulator::resim::Error)
}

pub fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.command {
        Command::NewKeyPair(cmd) => cmd.run()
    }
}

/// generate new key pair for offline signing and online verification
#[derive(Parser, Debug)]
pub struct NewKeyPair {
    public_key: PathBuf,
    private_key: PathBuf,
}

impl NewKeyPair {
    pub fn run(&self) -> Result<(), Error> {
        let mut ledger = RadixEngineDB::with_bootstrap(get_data_dir().map_err(Error::ResimError)?);

        let (public_bytes, private_bytes) = new_public_private_pair(&mut ledger);

        fs::write(&self.public_key, public_bytes).map_err(Error::IoError)?;
        fs::write(&self.private_key, private_bytes).map_err(Error::IoError)?;

        Ok(())
    }
}

use k256::{
    ecdsa::SigningKey,
};
//use rand_core::OsRng; // requires 'getrandom' feature

pub fn new_public_private_pair(ledger: &mut RadixEngineDB) -> (Vec<u8>, Vec<u8>) {
    // WARNING this is insecure, used for testing with deterministic keys (similar to how new_public_key works)
    let mut private_raw = [0u8; 32];
    private_raw[..].copy_from_slice(sha256(ledger.get_nonce().to_string()).as_ref());
    ledger.increase_nonce();
    let signing_key = SigningKey::from_bytes(&private_raw).expect("unable to create signing key (this should not happen)");

    let verifying_key = signing_key.verifying_key();
    let compressed_point = verifying_key.to_bytes();
    let mut public_raw = [0u8; 33];
    public_raw[..].copy_from_slice(&compressed_point);
    let public_key: EcdsaPublicKey = EcdsaPublicKey(public_raw);
    let public_key_encoded = scrypto_encode(&public_key);

    (
        public_key_encoded,
        signing_key.to_bytes().to_vec(),
    )
}