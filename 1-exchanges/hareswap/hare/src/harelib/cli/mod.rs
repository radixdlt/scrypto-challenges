use clap::Parser;
use clap::Subcommand;
 use scrypto::buffer::scrypto_encode;
// use std::fs::read_to_string;
use std::fs;
use std::path::PathBuf;

use radix_engine::ledger::*;
use radix_engine::engine::validate_data;
//use radix_engine::transaction::*;
use scrypto::types::EcdsaPublicKey;
use scrypto::utils::sha256;
use simulator::resim::*;
use simulator::ledger::*;

use scrypto::prelude::*;

use hareswap::api::*;

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
    RequestForQuote(RequestForQuote),
    MakeSignedOrder(MakeSignedOrder),
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ResimError(simulator::resim::Error),
    DecompileError(transaction_manifest::DecompileError),
    ParseAddressError(ParseAddressError),
    ParseDecimalError(ParseDecimalError),
    SBORDecodeError(sbor::DecodeError),
    Utf8Error(std::str::Utf8Error),
    ManifestParserError(transaction_manifest::parser::ParserError),
    ParserNotEOFError,
}

pub fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.command {
        Command::NewKeyPair(cmd) => cmd.run(),
        Command::RequestForQuote(cmd) => cmd.run(),
        Command::MakeSignedOrder(cmd) => cmd.run(),
    }
}

/// generate a request for quote (RFQ) to buy resource "B" with amount of resource "A"
#[derive(Parser, Debug)]
pub struct RequestForQuote {
    output_path: PathBuf,
    resource_b_amount: String,
    resource_b: String,
    resource_a: String,
    resource_taker_auth: String,
}

impl RequestForQuote {
    pub fn run(&self) -> Result<(), Error> {

        let maker_resource = ResourceDef::from(Address::from_str(&self.resource_b).map_err(Error::ParseAddressError)?);
        let maker_amount = Decimal::from_str(&self.resource_b_amount).map_err(Error::ParseDecimalError)?;
        let taker_resource = ResourceDef::from(Address::from_str(&self.resource_a).map_err(Error::ParseAddressError)?);
        let taker_auth_resource = ResourceDef::from(Address::from_str(&self.resource_taker_auth).map_err(Error::ParseAddressError)?);
        let taker_auth_amount = Decimal::from_str("1").map_err(Error::ParseDecimalError)?;
    
        let maker_requirement = BucketRequirement {
            resource: maker_resource,
            contents: BucketContents::Fungible(maker_amount)
        };

        let taker_auth = BucketRequirement {
            resource: taker_auth_resource,
            contents: BucketContents::Fungible(taker_auth_amount)
        };

        let partial_order = PartialOrder {
            maker_requirement,
            taker_resource,
            taker_auth,
        };

        let partial_order_encoded = scrypto_encode(&partial_order);

        //let validated_arg =
        //    validate_data(&partial_order_encoded).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        //print!("{}", validated_arg);
        fs::write(&self.output_path, &partial_order_encoded).map_err(Error::IoError)?;

        Ok(())
    }
}

/// creates an order from a partial order and signs it
#[derive(Parser, Debug)]
pub struct MakeSignedOrder {
    partial_order_file: PathBuf,
    resource_a_amount: String,
    maker_component_address: String,
    private_key_file: PathBuf,
}

use k256::{
    ecdsa::{recoverable::Signature as RSignature, SigningKey, signature::Signer},
};
use hex;
// use transaction_manifest::parser::Parser as ManifestParser;
// use transaction_manifest::lexer::tokenize;
// use transaction_manifest::generator::generate_instruction; // crap, need to either keep sbor encoded not manifest ast string, or wrap the entire thing in a real instruction (which isn't the worst idea)
// use std::str;
use k256::{
    ecdsa::{VerifyingKey, Signature, signature::Verifier},
};
pub fn xverify(public_key: &EcdsaPublicKey, serialized: &[u8], signature: &[u8]) {
    let pub_bytes = public_key.to_vec();
    let verifying_key: VerifyingKey = VerifyingKey::from_sec1_bytes(&pub_bytes).expect("verify: failed to parse verifying public key");

    let rsignature = RSignature::try_from(signature).unwrap();
    let sig = Signature::from(rsignature);

    match verifying_key.verify(serialized, &sig) {
        Ok(_) => (), // GOOD!
        Err(_) => panic!("xverify: signature verification failed"),
    }
}

impl MakeSignedOrder {
    pub fn run(&self) -> Result<(), Error> {
        let partial_order_bytes = fs::read(&self.partial_order_file).map_err(Error::IoError)?;
        let resource_a_amount = Decimal::from_str(&self.resource_a_amount).map_err(Error::ParseDecimalError)?;
        let maker_component_address = Address::from_str(&self.maker_component_address).map_err(Error::ParseAddressError)?;
        let private_key_bytes = fs::read(&self.private_key_file).map_err(Error::IoError)?;

        // parse parital_order_txt
        // TODO -- XXX
        // let partial_order_str = str::from_utf8(&partial_order_bytes).map_err(Error::Utf8Error)?.to_owned();
        // let mut parser = ManifestParser::new(tokenize(&partial_order_str).unwrap());
        // let partial_order_value = parser.parse_value().map_err(Error::ManifestParserError)?;
        // if !parser.is_eof() {
            // return Result::Err(Error::ParserNotEOFError);
        // }

        //let resolver = NameResolver::new();
        //let args = generate_instruction(vec![partial_order_value])?;
        let partial_order_encoded = partial_order_bytes;
        let partial_order: PartialOrder = scrypto_decode(&partial_order_encoded).map_err(Error::SBORDecodeError)?;

        let matched_order = MatchedOrder {
            partial_order,
            taker_contents: BucketContents::Fungible(resource_a_amount),
            maker_callback: Callback::CallMethod {
                component_address: maker_component_address,
                method: "default_swap".to_owned(),
                args: vec![],
            },
        };
    
        let matched_order_encoded = scrypto_encode(&matched_order);

        eprintln!("signing mached_order bytes:\n{}", hex::encode(&matched_order_encoded));

        let signing_key = SigningKey::from_bytes(&private_key_bytes).expect("unable to create signing key (this should not happen)");
        eprintln!("SigningKey: {:?}", signing_key);
        let rsignature: RSignature = signing_key.try_sign(&matched_order_encoded).unwrap(); // TODO map_err
        let rsignature_bytes: &[u8] = rsignature.as_ref();
        let signature = Vec::from(rsignature_bytes);

        eprintln!("result signature:\n{}", hex::encode(&signature));

        // double check sig verifies
        let verifying_key = signing_key.verifying_key();
        eprintln!("verifying_key: {:?}", verifying_key);
        let compressed_point = verifying_key.to_bytes();
        eprintln!("compressed_point: {:?}", compressed_point);
        let mut public_raw = [0u8; 33];
        public_raw[..].copy_from_slice(&compressed_point);
        let public_key: EcdsaPublicKey = EcdsaPublicKey(public_raw);
        xverify(&public_key, &matched_order_encoded, &signature);

        let signed_order = SignedOrder {
            order: matched_order,
            signature,
        };

        let signed_order_encoded = scrypto_encode(&signed_order);

        let validated_arg =
            validate_data(&signed_order_encoded).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        print!("{}", validated_arg);

        Ok(())
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

        fs::write(&self.public_key, &public_bytes).map_err(Error::IoError)?;
        fs::write(&self.private_key, private_bytes).map_err(Error::IoError)?;

        // print public_key bytes to stdout in rtm format
        let validated_arg =
            validate_data(&public_bytes).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        print!("{}", validated_arg);

        Ok(())
    }
}

// use k256::{
//     ecdsa::SigningKey,
// };
//use rand_core::OsRng; // requires 'getrandom' feature

pub fn new_public_private_pair(ledger: &mut RadixEngineDB) -> (Vec<u8>, Vec<u8>) {
    // WARNING this is insecure, used for testing with deterministic keys (similar to how new_public_key works)
    let mut private_raw = [0u8; 32];
    private_raw[..].copy_from_slice(sha256(ledger.get_nonce().to_string()).as_ref());
    ledger.increase_nonce();
    let signing_key = SigningKey::from_bytes(&private_raw).expect("unable to create signing key (this should not happen)");
    eprintln!("SigningKey: {:?}", signing_key);

    let verifying_key = signing_key.verifying_key();
    eprintln!("verifying_key: {:?}", verifying_key);
    let compressed_point = verifying_key.to_bytes();
    eprintln!("compressed_point: {:?}", compressed_point);
    let mut public_raw = [0u8; 33];
    public_raw[..].copy_from_slice(&compressed_point);
    let public_key: EcdsaPublicKey = EcdsaPublicKey(public_raw);
    let public_key_encoded = scrypto_encode(&public_key);

    (
        public_key_encoded,
        signing_key.to_bytes().to_vec(),
    )
}