use radix_engine::errors::{RuntimeError, TransactionValidationError};
use radix_engine::ledger::{InMemorySubstateStore, Substate, SubstateStore};
use radix_engine::model::{Component, Receipt, ResourceManager, SignedTransaction, Vault};
use radix_engine::transaction::{TransactionBuilder, TransactionExecutor};

use scrypto::buffer::{scrypto_decode, scrypto_encode};
use scrypto::engine::types::{LazyMapId, VaultId};
use scrypto::math::Decimal;
use scrypto::prelude::{
    ComponentAddress, EcdsaPrivateKey, EcdsaPublicKey, PackageAddress, ResourceAddress, RADIX_TOKEN,
};
use scrypto::resource::Bucket;
use scrypto::values::ScryptoValue;
use scrypto::{args, dec};

use crate::utils::PackageCompileError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a test runner for the standard token sale package.
pub struct TokenSaleTestRunner {
    /// The substate store that will be used to run the tests against.
    substate_store: InMemorySubstateStore,

    /// The path of the token sale package.
    package_path: PathBuf,

    /// The address of the package after it has been published.
    package_address: PackageAddress,

    /// The account used in in the tests
    account: Account,
}

impl TokenSaleTestRunner {
    /// Creates a new `TokenSaleTestRunner` from the provided path
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self, PackageError> {
        // Creating a new substate store as well as transaction executor to publish and get the package address of the
        // package.
        let mut substate_store: InMemorySubstateStore = InMemorySubstateStore::with_bootstrap();
        let mut executor: TransactionExecutor<InMemorySubstateStore> =
            TransactionExecutor::new(&mut substate_store, true);

        // Compiling the package at the specified path
        let path: PathBuf = path.into();
        let compiled_package: Vec<u8> = crate::utils::compile_package(&path)
            .map_err(|err| PackageError::PackageCompileError(err))?;
        let package_address: PackageAddress = executor
            .publish_package(compiled_package)
            .map_err(|err| PackageError::TransactionRuntimeError(err, None))?;

        // Creating a new account for the tests
        let account: Account = executor.new_account().into();

        // Creating a new token sale runner object
        let token_sale_runner: Self = Self {
            account,
            substate_store,
            package_path: path.into(),
            package_address: package_address,
        };
        Ok(token_sale_runner)
    }

    // =======================
    // Substate Store Methods
    // =======================

    /// Gets the package at the `package_address` from the substate store
    pub fn package(&self) -> Package {
        self.substate_store
            .get_decoded_substate(&self.package_address)
            .unwrap()
            .0
    }

    /// Gets a resource manager from the substate store
    pub fn resource_manager(
        &self,
        resource_address: &ResourceAddress,
    ) -> radix_engine::model::ResourceManager {
        self.substate_store
            .get_decoded_substate(resource_address)
            .unwrap()
            .0
    }

    /// Gets a specific vault of a specific account from the substate store
    pub fn account_vault(
        &self,
        component_address: &ComponentAddress,
        resource_address: &ResourceAddress,
    ) -> Option<radix_engine::model::Vault> {
        // Getting the component and the LazyMap ID
        let component: Component = self
            .substate_store
            .get_decoded_substate(component_address)
            .unwrap()
            .0;
        let component_state: ScryptoValue = ScryptoValue::from_slice(component.state()).unwrap();
        let lazymap_id: LazyMapId = component_state.lazy_map_ids.iter().next().unwrap().clone();

        // Adding everything together to get the final address of the vault
        let mut substate_id: Vec<u8> = scrypto_encode(&lazymap_id);
        substate_id.extend(scrypto_encode(resource_address));
        let substate: Option<Substate> = self
            .substate_store
            .get_child_substate(component_address, &substate_id);

        match substate {
            Some(substate) => {
                let vault: scrypto::prelude::Vault = scrypto_decode(&substate.value).unwrap();
                let vault_id: VaultId = vault.0;

                Some(
                    self.substate_store
                        .get_decoded_child_substate(component_address, &vault_id)
                        .unwrap()
                        .0,
                )
            }
            None => None,
        }
    }

    /// Executes a transaction against the current substate store
    pub fn execute_transaction(
        &mut self,
        transaction: &SignedTransaction,
    ) -> Result<Receipt, PackageError> {
        // Getting a transaction executor to use for transaction
        let mut executor: TransactionExecutor<InMemorySubstateStore> =
            TransactionExecutor::new(&mut self.substate_store, false);

        // Running the transaction
        let receipt: Receipt = executor
            .validate_and_execute(transaction)
            .map_err(|err| PackageError::TransactionValidationError(err))?;

        if receipt.result.is_ok() {
            Ok(receipt)
        } else {
            Err(PackageError::TransactionRuntimeError(
                receipt.result.as_ref().err().unwrap().clone(),
                Some(receipt),
            ))
        }
    }

    // ===================
    // Validation Methods
    // ===================

    /// Validates that the token sale blueprint functions according to the rules of the challenge.
    pub fn validate_token_sale_blueprint(&mut self) -> Result<Vec<String>, PackageError> {
        let blueprint_name: String = self.validate_package_has_one_blueprint()?;
        let (
            token_sale_component_address,
            team_token_resource_address,
            seller_badge_resource_address,
        ): (ComponentAddress, ResourceAddress, ResourceAddress) =
            self.validate_function_new(&blueprint_name)?;
        let ticket_numbers: Vec<String> =
            self.validate_team_token_metadata(&team_token_resource_address)?;

        self.validate_buy_method_produces_correct_amount(
            &token_sale_component_address,
            &team_token_resource_address,
        )?;

        Ok(ticket_numbers)
    }

    /// Validates that the package has a single blueprint.
    fn validate_package_has_one_blueprint(&self) -> Result<String, PackageError> {
        let package: Package = self.package();
        match package.blueprints.len() {
            1 => Ok(package.blueprints.keys().next().unwrap().clone()),
            num => Err(PackageError::InvalidAmountOfBlueprints(num)),
        }
    }

    /// Validates that the `new` function works as expected.
    fn validate_function_new(
        &mut self,
        blueprint_name: &str,
    ) -> Result<(ComponentAddress, ResourceAddress, ResourceAddress), PackageError> {
        // Calling the `new` function on the package to instantiate a new component
        let instantiate_new_token_sale_transaction: SignedTransaction = TransactionBuilder::new()
            .call_function(
                self.package_address,
                blueprint_name,
                "new",
                args![dec!("10")],
            )
            .call_method_with_all_resources(self.account.component_address, "deposit_batch")
            .build(0)
            .sign([]);
        let instantiate_new_token_sale_receipt: Receipt =
            self.execute_transaction(&instantiate_new_token_sale_transaction)?;

        // Ensure that the instantiate method only creates a single component
        let component_address: ComponentAddress = match instantiate_new_token_sale_receipt
            .new_component_addresses
            .get(0)
        {
            Some(component_address) => Ok(*component_address),
            None => Err(PackageError::NoComponentAddressAtIndex),
        }?;

        // Ensure that the instantiation created only two resources, then attempt to determine which resource is which
        let (team_token_resource_address, seller_badge_resource_address): (
            ResourceAddress,
            ResourceAddress,
        ) = match (
            instantiate_new_token_sale_receipt
                .new_resource_addresses
                .get(0),
            instantiate_new_token_sale_receipt
                .new_resource_addresses
                .get(1),
        ) {
            (Some(resource_address_1), Some(resource_address_2)) => {
                // Getting the resource manager for both tokens
                let (resource_manager_1, resource_manager_2): (
                    radix_engine::model::ResourceManager,
                    radix_engine::model::ResourceManager,
                ) = (
                    self.resource_manager(resource_address_1),
                    self.resource_manager(resource_address_2),
                );

                if resource_manager_1.total_supply() == dec!("1000") {
                    Ok((*resource_address_1, *resource_address_2))
                } else if resource_manager_2.total_supply() == dec!("1000") {
                    Ok((*resource_address_2, *resource_address_1))
                } else {
                    Err(PackageError::NoResourceWithSpecifiedSupply)
                }
            }
            _ => Err(PackageError::NoResourceAddressAtIndex),
        }?;

        Ok((
            component_address,
            team_token_resource_address,
            seller_badge_resource_address,
        ))
    }

    /// Validates weather the team token contains the right ticket numbers of not
    fn validate_team_token_metadata(
        &self,
        resource_address: &ResourceAddress,
    ) -> Result<Vec<String>, PackageError> {
        // Getting the resource manager of the resource address and the corresponding metadata
        let resource_manager: ResourceManager = self.resource_manager(resource_address);
        let metadata = resource_manager.metadata();
        let elements: Vec<String> = (1..5)
            .map(|i| metadata.get(&format!("team-member-{}-ticket-number", i)))
            .filter(|element| element.is_some())
            .map(|element| element.unwrap().clone())
            .collect();

        match elements.len() {
            0 => Err(PackageError::InvalidMetadataOnTeamResource),
            _ => Ok(elements),
        }
    }

    /// Validates that a correct amount of tokens is produced based on how much tokens go in
    fn validate_buy_method_produces_correct_amount(
        &mut self,
        component_address: &ComponentAddress,
        team_token_resource_address: &ResourceAddress,
    ) -> Result<(), PackageError> {
        // Constructing the purchase transaction
        let buy_transaction: SignedTransaction = TransactionBuilder::new()
            .withdraw_from_account_by_amount(
                dec!("100"),
                RADIX_TOKEN,
                self.account.component_address,
            )
            .take_from_worktop(RADIX_TOKEN, |builder, bucket_id| {
                builder.call_method(component_address.clone(), "buy", args![Bucket(bucket_id)])
            })
            .call_method_with_all_resources(self.account.component_address, "deposit_batch")
            .build(0)
            .sign([&self.account.private_key]);
        let buy_receipt: Receipt = self.execute_transaction(&buy_transaction)?;

        let token_amount: Decimal = match self
            .account_vault(&self.account.component_address, team_token_resource_address)
        {
            Some(vault) => vault.total_amount(),
            None => Decimal::zero(),
        };

        if token_amount == dec!("10") {
            Ok(())
        } else {
            Err(PackageError::PriceBalanceMismatchError)
        }
    }
}

/// A collection of blueprints, compiled and published as a single unit.
#[derive(Debug, Clone, sbor::TypeId, sbor::Encode, sbor::Decode)]
pub struct Package {
    pub code: Vec<u8>,
    pub blueprints: HashMap<String, sbor::Type>,
}

/// Represents an account used by the test runner
pub struct Account {
    /// Represents the public key of the account
    public_key: EcdsaPublicKey,

    /// Represents the private key of the account
    private_key: EcdsaPrivateKey,

    /// Represents the component address of the account
    component_address: ComponentAddress,
}

impl From<(EcdsaPublicKey, EcdsaPrivateKey, ComponentAddress)> for Account {
    fn from(
        (public_key, private_key, component_address): (
            EcdsaPublicKey,
            EcdsaPrivateKey,
            ComponentAddress,
        ),
    ) -> Self {
        Self {
            public_key,
            private_key,
            component_address,
        }
    }
}

/// Represents an error or issue encountered when testing the blueprint.
#[derive(Debug)]
pub enum PackageError {
    /// Represents an error encountered while compiling the package.
    PackageCompileError(PackageCompileError),

    /// Represents an error encountered when running a transaction.
    TransactionRuntimeError(RuntimeError, Option<Receipt>),

    /// Represents an error the number of available blueprints is invalid.
    InvalidAmountOfBlueprints(usize),

    /// Represents an error encountered when validating a transaction
    TransactionValidationError(TransactionValidationError),

    /// Represents an error relating to component addresses
    NoComponentAddressAtIndex,

    /// Represents an error relating to resource addresses
    NoResourceAddressAtIndex,

    /// Represents an error where no resource could be found with the specified supply.
    NoResourceWithSpecifiedSupply,

    /// Represents an error where the team resource has invalid metadata
    InvalidMetadataOnTeamResource,

    /// Represents an error where the current price and the balance in the vault are not equal
    PriceBalanceMismatchError,
}
