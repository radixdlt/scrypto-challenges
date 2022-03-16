use std::fs;
use std::path::PathBuf;

// non-scrypto dependencies
use clap::{Parser, Subcommand, ArgEnum};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};

// scrypto dependencies
use radix_engine::engine::validate_data;
use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::buffer::scrypto_encode;
use scrypto::prelude::*;
use scrypto::types::EcdsaPublicKey;
use scrypto::utils::sha256;
use simulator::ledger::*;
use simulator::resim::*;
use radix_engine::model::{Instruction, Transaction};

// Only the imports needed to do off-ledger things, ie. the hareswap API
use hareswap::api::*;

/* Top Level */

/// hareswap CLI: Used for the off-ledger interactions supporting a swap
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

/// the available CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Command {
    NewKeyPair(NewKeyPair),
    RequestForQuote(RequestForQuote),
    MakeSignedOrder(MakeSignedOrder),
    TokenizeOrder(TokenizeOrder),
    Test(TestCommands),
}

/// testing CLI to aid with demos etc
#[derive(Parser, Debug)]
pub struct TestCommands {
    #[clap(subcommand)]
    command: TestCommand,
}

/// the available testing subcommands
#[derive(Subcommand, Debug)]
pub enum TestCommand {
    NFTSetup(NFTSetup),
}

/// custom errors which simply wrap explicit error types when the bubble up to the top
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ResimError(simulator::resim::Error),
    DecompileError(transaction_manifest::DecompileError),
    CompileError(transaction_manifest::CompileError),
    ParseAmountError(ParseBucketContentsError),
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
    GenerateEntrypointError(transaction_manifest::DecompileError),
    GenerateEntrypointFormatError,
    TestError,
    RuntimeError,
}

/// main CLI entry point
pub fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.command {
        Command::NewKeyPair(cmd) => cmd.run(),
        Command::RequestForQuote(cmd) => cmd.run(),
        Command::MakeSignedOrder(cmd) => cmd.run(),
        Command::TokenizeOrder(cmd) => cmd.run(),
        Command::Test(cmd) => {
            match cmd.command {
                TestCommand::NFTSetup(cmd) => cmd.run(),
            }
        }
    }
}

/* Subcommands */

/* Request For Quote */

/// used by the taker: generate a request-for-quote (RFQ) to buy or sell an
/// exact amount of the resource "B" base asset with a to-be-determined amount of
/// the resource "Q" quote asset
#[derive(Parser, Debug)]
pub struct RequestForQuote {
    /// choose to buy or sell the base asset
    #[clap(arg_enum)]
    quote_type: QuoteType,
    /// path to file to store the request (for simulating sending or integrating with some RFQ protocol)
    output_path: PathBuf,
    /// amount of base asset
    resource_b_amount: String,
    /// resource of base asset
    resource_b: String,
    /// resource of the quoted asset
    resource_q: String,
    /// resource address for a badge to control what entity is allowed to submit
    /// a SignedOrder resulting from this RFQ (protects against frontrunning)
    /// ASSUMES requirement is a single fungible
    resource_taker_auth: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum QuoteType {
    SellBase,
    BuyBase,
}

impl RequestForQuote {
    pub fn run(&self) -> Result<(), Error> {
        // parse arguments
        let base_resource = ResourceDef::from(Address::from_str(&self.resource_b).map_err(Error::ParseAddressError)?);
        let base_contents = BucketContents::from_str(&self.resource_b_amount).map_err(Error::ParseAmountError)?;
        let quote_resource = ResourceDef::from(Address::from_str(&self.resource_q).map_err(Error::ParseAddressError)?);
        let taker_auth_resource =
            ResourceDef::from(Address::from_str(&self.resource_taker_auth).map_err(Error::ParseAddressError)?);
        let taker_auth_amount = Decimal::from_str("1").map_err(Error::ParseDecimalError)?;

        // combine the rosource_b information into a BucketRequirement
        let base_requirement = BucketRequirement {
            resource: base_resource,
            contents: base_contents,
        };

        // combine the taker_auth information into a BucketRequirement
        let taker_auth = BucketRequirement {
            resource: taker_auth_resource,
            contents: BucketContents::Fungible(taker_auth_amount),
        };

        // combine the above to create the PartialOrder which is the full RFQ
        let partial_order = PartialOrder {
            inverted: self.quote_type == QuoteType::SellBase,
            base_requirement,
            quote_resource,
            taker_auth,
        };

        // and encode it
        let partial_order_encoded = scrypto_encode(&partial_order);

        // not outputting a textual version because we can't parse it from Rust code since some transaction manifest compiler functions are private
        // so just writing the encoded bytes to file was cleaner
        // but this is how you would decompile it:
        //   let validated_arg =
        //      validate_data(&partial_order_encoded).map_err(transaction_manifest::DecompileError::DataValidationError).map_err(Error::DecompileError)?;
        //   print!("{}", validated_arg);
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
    /// Maker's "quote" in response to the request-for-quote, ie the amount of resource_q.  May be Decimal amount for Fungible (requires a ".") or a comma-seperated list of NonFungibleKeys
    resource_q_amount: String,
    /// component address for the Maker component which is the entry point to be called with the SignedOrder to complete the on-ledger order settlement
    maker_component_address: String,
    /// resource address identifying the resource a "Transporter" will mint when converting the SignedOrder to an on-ledger NonFungable token
    voucher_address: String,
    /// unique identifier for this order to avoid reply (used as the NonFungableKey when the Tranporter tokenizes this order)
    voucher_key: String,
    /// path to file containing the serialized private key which will sign the order - must match on-ledger public key
    private_key_file: PathBuf,
    /// integer value for the last epoch this order can be executed
    deadline_epoch: u64,
    /// optional callback to use instead of teh handle_order_default_callback.  This should be in the form of a CALL_METHOD instruction
    callback: Option<String>,
}

impl MakeSignedOrder {
    pub fn run(&self) -> Result<(), Error> {
        // parse arguments
        let partial_order_bytes = fs::read(&self.partial_order_file).map_err(Error::IoError)?;
        let resource_q_contents = BucketContents::from_str(&self.resource_q_amount).map_err(Error::ParseAmountError)?;
        let maker_component_address =
            Address::from_str(&self.maker_component_address).map_err(Error::ParseAddressError)?;
        let voucher_resource: ResourceDef = Address::from_str(&self.voucher_address)
            .map_err(Error::ParseAddressError)?
            .into();
        let voucher_key = NonFungibleKey::from_str(&self.voucher_key).map_err(Error::ParseNonFungibleKeyError)?;
        let private_key_bytes = fs::read(&self.private_key_file).map_err(Error::IoError)?;
        //let deadline = u64::from_str(&self.deadline_epoch)?;//.map_err(Error::ParseDeadline)?;
        let deadline = self.deadline_epoch;
        // set the callback by parsing the argument string or using the default
        let maker_callback = if self.callback.is_none() {
            // this is the default callback expected in the Maker Component
            // It is used for "simple" swaps where the Maker has a SharedAccount
            // managing the bought and sold assets.
            Callback::CallMethod {
                component_address: maker_component_address,
                method: "handle_order_default_callback".to_owned(),
                args: vec![],
            }
        } else {
            // parse the CallMethod Instruction into the Callback type with the same args
            let callback_str = self.callback.as_ref().unwrap(); // unwrap is safe on this branch
            let tx: Transaction = transaction_manifest::compile(&callback_str).map_err(Error::CompileError)?;
            assert_eq!(tx.instructions.len(), 1, "callback error, too many instructions"); // backend only supports 1 for now
            match tx.instructions[0].clone() {
                Instruction::CallMethod {
                    component_address,
                    method,
                    args,
                } => {
                    Callback::CallMethod {
                        component_address,
                        method,
                        args,
                    }
                },
                _ => panic!("callback did not contain a CallMethod")
            }
        };

        // decode the PartialOrder
        let partial_order_encoded = partial_order_bytes;
        let partial_order: PartialOrder = scrypto_decode(&partial_order_encoded).map_err(Error::SBORDecodeError)?;

        // create the MatchedOrder from the inputs
        let matched_order = MatchedOrder {
            partial_order,
            quote_contents: resource_q_contents,
            maker_callback,
            deadline,
        };

        // construct a Voucher for the MatchedOrder

        let voucher = Voucher::from_nfd(voucher_resource.clone(), Some(voucher_key.clone()), matched_order.clone());

        // and encode it
        let voucher_encoded = scrypto_encode(&voucher);

        // sign the voucher
        let signing_key = SigningKey::from_bytes(&private_key_bytes).map_err(Error::BadPrivateKeyError)?;
        let signature: Signature = signing_key.try_sign(&voucher_encoded).map_err(Error::SigningError)?;
        let sig_bytes = signature.to_der().to_bytes().to_vec();

        // double check that the sig verifies (all the format conversions are ok)
        let public_key = to_public_key(&signing_key);
        verify(&public_key, &voucher_encoded, &sig_bytes).map_err(Error::VerifyCheckError)?;

        // create the SignedOrder for consumption by the submitter.
        let signed_order = SignedOrder {
            order: matched_order,
            voucher_resource,
            voucher_key,
            signature: sig_bytes,
        };

        // and encode it
        let signed_order_encoded = scrypto_encode(&signed_order);

        // create the instruction the sender will use to execute this signed order (after adding the appropriate buckets)
        // this is not technically part of the signed order and not authenticated.  The taker/sender is free to send the
        // signed order (along with their badge) to the more advanced "tokenize_order" method.
        let execute_entrypoint = Instruction::CallMethod {
            component_address: maker_component_address,
            method: "execute_order".to_owned(), // method name matches the Maker blueprint implementation, hardcoding this here
            args: vec![signed_order_encoded],
        };
        // build the transaction so we can use the available API to easily print it
        let tx = Transaction {
            instructions: vec![execute_entrypoint], // following code (and the taker) assumes only a single instruction
        };
        let manifest = transaction_manifest::decompile(&tx).map_err(Error::GenerateEntrypointError)?;
        // drop the trailing semicolon and newline since extra args are required
        let (result, _) = manifest.rsplit_once(";").ok_or(Error::GenerateEntrypointFormatError)?;

        // print the instruction to stdout in Radix Transaction Manifest (rtm) format
        // so the sender can compose it with whatever they want
        print!("{}", result);

        // *SECURITY NOTICE*: care should be taken by the transaction submitter to
        // validate this upon receipt and not introduce "instruction injection" vulnerabilities
        // or any other "component redirection" vulnerabilities

        Ok(())
    }
}

/* TokenizeOrder */

/// used by the taker: Converts a SignedOrder (execute_order instruction) to a tokenize_order instruction instead, for "advanced usage"
/// results on stdout
/// 
/// This implementation is rough but good enough for a proof of concept.  It creates instructions intended to be used as a subset of
/// a larger transactions and outputs them in the manifest representation
#[derive(Parser, Debug)]
pub struct TokenizeOrder {
    /// path to file containing the CALL_METHOD instruction to be converted to call tokenize_order
    signed_order_file: PathBuf,
    /// BucketRef name to pass to tokenize_order as authorization
    auth_bucket: String,
    /// bucket name to store the order token
    order_bucket: String,
}

impl TokenizeOrder {
    pub fn run(&self) -> Result<(), Error> {
        // parse args
        let mut input_tx_str = fs::read_to_string(&self.signed_order_file).map_err(Error::IoError)?;
        // append ";" back onto the instruction so it can be compiled
        input_tx_str.push(';');
        // compile the transaction so the method name can be modified
        let mut tx: Transaction = transaction_manifest::compile(&input_tx_str).map_err(Error::CompileError)?;
        // modify the method name and add an instruction to store the result
        assert_eq!(tx.instructions.len(), 1, "signed_order_file had too many instructions");
        //let mut tx2: Transaction = Transaction { instructions}
        match &mut tx.instructions[0] {
            Instruction::CallMethod {
                ref mut method,
                ref mut args,
                ..
            } => {
                *method = "tokenize_order".to_string();
                args.push(scrypto_encode(&Rid(1))); // this will be the auth_bucket.  Luckly we can use 1 as the placeholder since it will always exist.  A bit of a hack
                let signed_order: SignedOrder = scrypto_decode(&args[0]).expect("arg[0] should be a SignedOrder");
                let new_inst = Instruction::TakeNonFungiblesFromWorktop {
                    resource_address: signed_order.voucher_resource.address(),
                    keys: BTreeSet::from([signed_order.voucher_key]),
                };
                tx.instructions.push(new_inst);
            },
            _ => panic!("signed_order_file did not contain a CallMethod")
        };

        // convert back to manifest text
        let manifest = transaction_manifest::decompile(&tx).map_err(Error::GenerateEntrypointError)?;

        // this next part is a little ugly, since we're using this as a template, splice in better text...
        let result = manifest;
        // replace the generated BucketRef name with the requested for the auth_bucket
        let result = result.replace("BucketRef(1u32)", &vec!["BucketRef(\"", &self.auth_bucket, "\")"].join(""));
        // replace the generated Bucket name with the requested for the order_bucket
        let result = result.replace("Bucket(\"bucket1\")", &vec!["Bucket(\"", &self.order_bucket, "\")"].join(""));

        // print the instruction to stdout in Radix Transaction Manifest (rtm) format
        // so the sender can compose it with whatever they want
        print!("{}", result);

        Ok(())
    }
}

/* New Key Pair */

/// used by the maker: generate new key pair for offline signing and online verification
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
        let validated_arg = validate_data(&public_bytes)
            .map_err(transaction_manifest::DecompileError::DataValidationError)
            .map_err(Error::DecompileError)?;
        print!("{}", validated_arg);

        Ok(())
    }
}


/// generate (public, private) byte vectors for an ECDSA keypair using sha256(ledger.get_nonce) as the basis for the private key
///
/// WARNING this is insecure, used for testing with deterministic keys (similar to how new_public_key works)
pub fn new_public_private_pair(ledger: &mut RadixEngineDB) -> (Vec<u8>, Vec<u8>) {
    let mut private_raw = [0u8; 32];
    private_raw[..].copy_from_slice(sha256(ledger.get_nonce().to_string()).as_ref());
    ledger.increase_nonce();
    let signing_key =
        SigningKey::from_bytes(&private_raw).expect("unable to create signing key (this should not happen)");

    let public_key = to_public_key(&signing_key);
    let public_key_encoded = scrypto_encode(&public_key);

    (public_key_encoded, signing_key.to_bytes().to_vec())
}


/// Convert SigningKey to an EcdsaPublicKey.
///
/// WARNING: Makes some assumptions about the underlying type, curve paramaters etc for the SigningKey
fn to_public_key(signing_key: &SigningKey) -> EcdsaPublicKey {
    let verifying_key = signing_key.verifying_key();
    let compressed_point = verifying_key.to_bytes();
    let mut public_raw = [0u8; 33];
    public_raw[..].copy_from_slice(&compressed_point);
    EcdsaPublicKey(public_raw)
}

/// populates accounts with some NonFungible resources
#[derive(Parser, Debug)]
pub struct NFTSetup {
    /// account to deposit newly minted NFT
    account: String,
    /// symbol for the NFT
    symbol: String,
    /// Set of keys to create, in comma-seperated hex values
    amount: String,
    /// path to helper package
    helper: PathBuf,
}

impl NFTSetup {
    pub fn run(&self) -> Result<(), Error> {
        let account = Address::from_str(&self.account).map_err(Error::ParseAddressError)?;
        let amount = BucketContents::from_str(&self.amount).map_err(Error::ParseAmountError)?;

        let keys = match amount {
            BucketContents::NonFungible(keys) => keys,
            _ => return Err(Error::TestError)
        };
        let args = vec![scrypto_encode(&self.symbol), scrypto_encode(&keys)];

        // get the on-disk ledger the same way resim does
        let mut ledger = RadixEngineDB::with_bootstrap(get_data_dir().map_err(Error::ResimError)?);
        let mut executor = TransactionExecutor::new(&mut ledger, false);

        // inefficient to publish this every time, but this is just for demo setup
        let package = executor.publish_package(&compile(&self.helper.to_string_lossy(), "helper")).unwrap();

        // call function Helper "new_nft" keys
        let key = executor.new_public_key(); // doesn't actually matter
        let transaction1 = TransactionBuilder::new(&executor)
            //.call_function(package, "Helper", "new_nft", scrypto_encode(args), None)
            .add_instruction(Instruction::CallFunction {
                package_address: package,
                blueprint_name: "Helper".to_owned(),
                function: "new_nft".to_owned(),
                args
            }).0
            .call_method_with_all_resources(account, "deposit_batch")
            .build(vec![key])
            .unwrap();
        let receipt1 = executor.run(transaction1).unwrap();
//        println!("{:?}\n", receipt1);
        //assert!(receipt1.result.is_ok());
        receipt1.result.as_ref().map_err(|_|Error::RuntimeError)?;

        println!("{:?}\n", receipt1.resource_def(0).unwrap());

        Ok(())
    }
}

/// compiles a package at path with name in the same way scrypto build
/// Copied from some scyrypto tests
pub fn compile(path: &str, name: &str) -> Vec<u8> {
    std::process::Command::new("cargo")
        .current_dir(format!("{}", path))
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
        .status()
        .unwrap();
    fs::read(format!(
        "{}/target/wasm32-unknown-unknown/release/{}.wasm",
        path,
        name.replace("-", "_")
    ))
    .unwrap()
}
