use clap::Parser;
use clap::Subcommand;
use scrypto::buffer::scrypto_encode;
use std::fs;
use std::path::PathBuf;

use radix_engine::ledger::*;
use radix_engine::engine::validate_data;
use scrypto::types::EcdsaPublicKey;
use scrypto::utils::sha256;
use simulator::resim::*;
use simulator::ledger::*;

use scrypto::prelude::*;

use hareswap::api::*;


/* Top Level */

/// hareswap CLI: Used for the off-ledger interactions supporting a swap
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
    ParseNonFungibleKeyError(ParseNonFungibleKeyError),
    SBORDecodeError(sbor::DecodeError),
    Utf8Error(std::str::Utf8Error),
    ManifestParserError(transaction_manifest::parser::ParserError),
    SigningError(k256::ecdsa::Error),
    VerifyCheckError(VerifyError),
    BadPrivateKeyError(k256::ecdsa::Error),
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

/* Subcommands */

/* Request For Quote */

/// used by the taker: generate a request-for-quote (RFQ) to buy resource "B" with some amount of resource "A"
#[derive(Parser, Debug)]
pub struct RequestForQuote {
    /// path to file to store the request (for simulating sending or integrating with some RFQ protocol)
    output_path: PathBuf,
    /// amount to buy
    resource_b_amount: String,
    /// resource to buy
    resource_b: String,
    /// resource to sell
    resource_a: String,
    /// resource address for a badge to control what entity is allowed to submit a SignedOrder resulting from this RFQ (protects against frontrunning)
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

/* Make Signed Order */

/// used by the maker: creates an order from a partial order and signs it sending the result to stdout
#[derive(Parser, Debug)]
pub struct MakeSignedOrder {
    /// path to file containing the SBOR-encoded PartialOrder bytes (simulating receipt via some request-for-quote protocol)
    partial_order_file: PathBuf,
    /// Maker's "quote" in response the the request-for-quote, ie the amount of resource_a
    resource_a_amount: String,
    /// component address for the Maker component which is the entry point to be called with the SignedOrder to complete the on-ledger order settlement
    maker_component_address: String,
    /// resource address identifying the resource a "Transporter" will mint when converting the SignedOrder to an on-ledger NonFungable token
    voucher_address: String,
    /// unique identifier for this order to avoid reply (used as the NonFungableKey when the Tranporter tokenizes this order)
    voucher_key: String,
    /// path to file containing the serialized private key which will sign the order - must match on-ledger public key
    private_key_file: PathBuf,
}

use k256::{
    ecdsa::{SigningKey, signature::Signer, /* VerifyingKey, signature::Verifier, */ Signature},
};

impl MakeSignedOrder {
    pub fn run(&self) -> Result<(), Error> {
        // parse arguments
        let partial_order_bytes = fs::read(&self.partial_order_file).map_err(Error::IoError)?;
        let resource_a_amount = Decimal::from_str(&self.resource_a_amount).map_err(Error::ParseDecimalError)?; // FUTURE: support NonFungibleKey to trade NonFungibles too
        let maker_component_address = Address::from_str(&self.maker_component_address).map_err(Error::ParseAddressError)?;
        let voucher_resource: ResourceDef = Address::from_str(&self.voucher_address).map_err(Error::ParseAddressError)?.into();
        let voucher_key = NonFungibleKey::from_str(&self.voucher_key).map_err(Error::ParseNonFungibleKeyError)?;
        let private_key_bytes = fs::read(&self.private_key_file).map_err(Error::IoError)?;

        // decode the PartialOrder
        let partial_order_encoded = partial_order_bytes;
        let partial_order: PartialOrder = scrypto_decode(&partial_order_encoded).map_err(Error::SBORDecodeError)?;

        // create the MatchedOrder from the inputs
        let matched_order = MatchedOrder {
            partial_order,
            taker_contents: BucketContents::Fungible(resource_a_amount), // FUTURE: support at least a single key instead to trade NonFungible tokens
            // this is the default callback expected in the Maker Component
            maker_callback: Callback::CallMethod {
                component_address: maker_component_address,
                method: "handle_order_default_callback".to_owned(),
                args: vec![],
            },
        };

        // construct a Voucher for the MatchedOrder (and encode it)

        let nfd = matched_order.as_passthru();

        let voucher = Voucher {
            resource_def: voucher_resource.clone(),
            key: Some(voucher_key.clone()),
            nfd,
        };
    
        let voucher_encoded = scrypto_encode(&voucher);

        // test that the decode works properly

        let decoded_voucher = private_decode_with_type::<Voucher>(&voucher_encoded).unwrap();
        assert_eq!(voucher, decoded_voucher, "voucher decode error");

        // sign the voucher

        let signing_key = SigningKey::from_bytes(&private_key_bytes).map_err(Error::BadPrivateKeyError)?;
        let signature: Signature = signing_key.try_sign(&voucher_encoded).map_err(Error::SigningError)?;
        let sig_bytes = signature.to_der().to_bytes().to_vec();

        // double check that the sig verifies (all the format conversions are ok)
        let verifying_key = signing_key.verifying_key();
        let compressed_point = verifying_key.to_bytes();
        let mut public_raw = [0u8; 33];
        public_raw[..].copy_from_slice(&compressed_point);
        let public_key: EcdsaPublicKey = EcdsaPublicKey(public_raw);
        verify(&public_key, &voucher_encoded, &sig_bytes).map_err(Error::VerifyCheckError)?;

        // create the SignedOrder for consuption by the submitter.
        let signed_order = SignedOrder {
            order: matched_order,
            voucher_resource,
            voucher_key,
            signature: sig_bytes,
        };

        // and encode it
        let signed_order_encoded = scrypto_encode(&signed_order);

        // print signed order bytes to stdout in Radix Transaction Manifest (rtm) format
        // this is so it can be interpolated into a transaction
        // care should be taken by the transaction submitter to not introduce "instruction injection" vulnerabilities
        let validated_arg =
            validate_data(&signed_order_encoded).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        print!("{}", validated_arg);

        Ok(())
    }
}

/* New Key Pair */

/// generate new key pair for offline signing and online verification
/// WARNING: this is designed for testing and creates the private key deterministically from the current on-disk ledger state.
#[derive(Parser, Debug)]
pub struct NewKeyPair {
    /// path to store new public key
    public_key: PathBuf,
    /// path to store new private key
    private_key: PathBuf,
}

impl NewKeyPair {
    pub fn run(&self) -> Result<(), Error> {
        // get the on-disk ledger the same way resim does
        let mut ledger = RadixEngineDB::with_bootstrap(get_data_dir().map_err(Error::ResimError)?);

        // generate the keys
        let (public_bytes, private_bytes) = new_public_private_pair(&mut ledger);

        // write to disk
        fs::write(&self.public_key, &public_bytes).map_err(Error::IoError)?;
        fs::write(&self.private_key, private_bytes).map_err(Error::IoError)?;

        // print public_key bytes to stdout in Radix Transaction Manifest (rtm) format
        let validated_arg =
            validate_data(&public_bytes).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        print!("{}", validated_arg);

        Ok(())
    }
}

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