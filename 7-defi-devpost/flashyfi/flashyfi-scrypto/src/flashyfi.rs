use derive_new::new;
use scrypto::prelude::*;

#[blueprint]
mod flashyfi {
    use super::*;

    struct Flashyfi {
        /// A badge with a dual purpose:
        /// * acts as a minting/burning authority for the resources that are managed by this component
        /// * acts as an access badge for the resources in the accounts of user's who have enabled
        /// flash loans for those accounts.
        ///
        /// As per the code of this blueprint, the badge can never leave this vault and will only
        /// ever be used in the confines of the component's methods. This makes it safe for users to
        /// authorize this badge for withdrawals on their accounts.
        ///
        flashyfi_badge: Vault,

        /// A collection of resource addresses that are managed by this component
        addresses: FlashyfiAddresses,

        /// Whether or not normal components can be registered just like accounts. This will work
        /// if the component exposes the following account-style methods:
        /// * `withdraw_by_amount`
        /// * `withdraw_by_ids`
        /// * `deposit_batch`
        allow_regular_components: bool,
    }

    impl Flashyfi {
        /// Instantiates a new Flashyfi component.
        ///
        /// # Arguments:
        /// * `allow_regular_components` - Whether or not normal components can be registered just like accounts
        ///
        /// # Returns:
        /// * The address of the newly instantiated component
        /// * A collection with the addresses of all resources that are managed by the component
        pub fn instantiate_global(allow_regular_components: bool) -> (ComponentAddress, FlashyfiAddresses) {
            // Create a badge that will be
            // - used for minting/burning all other resources handled by this component
            // - used to safely withdraw funds from user's accounts
            let flashyfi_badge = ResourceBuilder::new_fungible().divisibility(DIVISIBILITY_NONE).mint_initial_supply(1);

            // Create a badge resource that
            // - keeps track of what tokens can be borrowed from a user's account (account config)
            // - enables the account owner to change the account config
            let account_config_badge_resource = ResourceBuilder::new_bytes_non_fungible()
                .metadata("name", "Flashyfi Account Badge")
                .mintable(rule!(require(flashyfi_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(flashyfi_badge.resource_address())), LOCKED)
                // IMPORTANT: Do not let users transfer this badge out of there account. This ties the badge to
                // the account it has been minted for and prevents users from accidentally passing it to a bad actor
                // which would be dangerous.
                .restrict_withdraw(AccessRule::DenyAll, LOCKED)
                .create_with_no_initial_supply();

            // Create a loan receipt resource that will be used to keep track of loans
            let loan_receipt_resource = ResourceBuilder::new_uuid_non_fungible()
                .mintable(rule!(require(flashyfi_badge.resource_address())), LOCKED)
                .burnable(rule!(require(flashyfi_badge.resource_address())), LOCKED)
                // IMPORTANT: Restrict users from depositing the receipt into their account. We thereby force them
                // to return it to this component which gives us the chance to check if they have repaid the loan
                // and any due fees in full. If they haven't we can let the transaction fail.
                .restrict_deposit(AccessRule::DenyAll, LOCKED)
                .create_with_no_initial_supply();

            let addresses = FlashyfiAddresses {
                flashyfi_badge_resource: flashyfi_badge.resource_address(),
                account_config_badge_resource,
                loan_receipt_resource,
            };
            let component = Flashyfi {
                flashyfi_badge: Vault::with_bucket(flashyfi_badge), //
                addresses: addresses.clone(),
                allow_regular_components,
            }
                .instantiate()
                .globalize();

            (component, addresses)
        }

        /// Registers this account with the Flashyfi component, so that other users can take out
        /// flash loans from it. An account config badge that allows its holder to administer
        /// which tokens and NFTs may be borrowed, is automatically issued to the account.
        ///
        /// Before calling this method the account's access rules must be modified so that the
        /// flashyfi_badge (managed by this component) may be used to withdraw tokens from it.
        /// Specifically, it must be possible for the FlashyFi component to call the following two
        /// methods by presenting the flashyfi_badge:
        /// * `withdraw_by_amount`
        /// * `withdraw_by_ids`
        ///
        /// **Warning:** It is important to a) authorize the flashyfi_badge on the above two
        /// methods and b) register it with the Flashyfi component within the same transaction!
        /// Failing to do so will result in a failed transaction!
        ///
        /// For security reasons the account config badge is deposited directly into the registered
        /// account so that no unauthorized party may obtain it and use it to configure the account
        /// without the account owners permission.
        ///
        /// # Arguments:
        /// * `account_address` - The address of the account for which flash loans should be enabled
        pub fn flashyfi_account(&self, account_address: ComponentAddress) {
            use ComponentAddress::*;
            // Derive a (Flashyfi) account ID from the account's address
            let account_id = match account_address {
                Account(address) | EcdsaSecp256k1VirtualAccount(address) | EddsaEd25519VirtualAccount(address) => {
                    NonFungibleLocalId::bytes(address).unwrap()
                }
                Normal(address) => {
                    // Only allow normal components to be registered if the component is configured accordingly
                    assert!(
                        self.allow_regular_components,
                        "The component is not configured to allow registration of normal components as accounts"
                    );
                    NonFungibleLocalId::bytes(address).unwrap()
                }
                // Panic if a non-account address has been specified
                _ => panic!("{account_address:?} is not the address of an account component"),
            };

            // Verify that this component can withdraw from the specified account.
            // The user can of course change the account's access rules later on but it still makes
            // sense to verify withdrawals initially
            let mut account = AccountComponentTarget::at(account_address);
            self.flashyfi_badge.authorize(|| {
                let tokens = account.withdraw_by_amount(dec!("0.00000001"), RADIX_TOKEN);
                account.deposit_batch(vec![tokens]);
            });

            // Mint an account config badge
            // Start with an initial config where no tokens may be borrowed
            let account_config_badge = self.flashyfi_badge.authorize(|| {
                let rm = borrow_resource_manager!(self.addresses.account_config_badge_resource);
                rm.mint_non_fungible(
                    &account_id,
                    FlashyfiAccountConfig {
                        account_address,
                        fungible_fee_configs: HashMap::new(),
                        non_fungible_fee_configs: HashMap::new(),
                    },
                )
            });

            // Deposit the account config badge directly into the account that it belongs to
            // This is a security feature!
            let mut account = AccountComponentTarget::at(account_address);
            account.deposit_batch(vec![account_config_badge]);
        }

        /// Updates an account's configuration, i.e. which resources may be borrowed and for what fee.
        ///
        /// # Arguments:
        /// * `account_config_badge` - A proof of an Flashyfi account config badge. This identifies
        /// the account whose config should be changed as well as legitimizes the caller, because it
        /// proves their ownership of the account. (Account config badge can never be withdrawn from
        /// an account.)
        /// * `fungible_fee_configs` - A map containing a fee configuration for each fungible
        /// resources that other users may borrow from the specified account
        /// * `non_fungible_fee_configs` - A  map containing a fee configuration for each non-fungible
        /// resources that other users may borrow from the specified account
        ///
        pub fn update_account_config(
            &self,
            account_config_badge: Proof,
            fungible_fee_configs: HashMap<ResourceAddress, (bool, Fee)>,
            non_fungible_fee_configs: HashMap<ResourceAddress, (bool, FixedFee)>,
        ) {
            // Verify that the provided account config badge proof is authentic
            let account_config_badge = account_config_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.addresses.account_config_badge_resource,
                    dec!("1"),
                ))
                .unwrap();

            // Load the account config with the ID obtained from the proof
            let config_id = account_config_badge.non_fungible_local_id();
            let mut config: FlashyfiAccountConfig = account_config_badge.non_fungible().data();
            // Update the config maps
            config.fungible_fee_configs = fungible_fee_configs;
            config.non_fungible_fee_configs = non_fungible_fee_configs;
            // Check that the config is valid, i.e. no negative fees where specified
            config.assert_valid();

            // Write the updated config back to the ledger
            let rm = borrow_resource_manager!(self.addresses.account_config_badge_resource);
            self.flashyfi_badge.authorize(|| rm.update_non_fungible_data(&config_id, config));
        }

        /// Borrows the specified [ResourceAmount] from the specified account.
        /// This method return the borrowed funds as well as a non-depositable receipt that
        /// forces the caller to return the fund in the same transaction.
        ///
        /// # Arguments
        /// * `to_borrow` - A definition of a resource and a amount that should be borrowed
        /// * `from_account` - The account from which the resources should be borrowed. Must be a
        /// "flashified" account.
        ///
        /// # Returns:
        /// * A bucket containing the borrowed funds
        /// * A bucket containing a loan receipt. This receipt must be returned within the same
        /// transaction together with the borrowed funds and a fee or the transaction will fail.
        pub fn borrow(&mut self, to_borrow: ResourceAmount, from_account: ComponentAddress) -> (Bucket, Bucket) {
            // Do not let users lend/borrow the account config badge as this would be a security violation!
            // Malicious actors would be able to borrow this badge and then use it to configure a user's Flashyfi account to their disadvantage
            if let ResourceAmount::NonFungibleAmount(NonFungibleAmount(resource, _)) = to_borrow {
                assert_ne!(
                    resource, self.addresses.account_config_badge_resource,
                    "Account config badges cannot be borrowed"
                )
            }

            // Derive the (Flashyfi) account ID from the account address
            let account_id = NonFungibleLocalId::bytes(from_account.raw()).unwrap();

            // Assert that the specified account is in fact a Flashyfi account
            let rm = borrow_resource_manager!(self.addresses.account_config_badge_resource);
            assert!(
                rm.non_fungible_exists(&account_id),
                "Not a FlashyFi account: cannot borrow from account {from_account:?}",
            );

            // Load the account's config
            let account_config: FlashyfiAccountConfig = rm.get_non_fungible_data(&account_id);

            // Create a loan receipt. The create_loan_receipt method will handle all required
            // security checks and calculate the fee that is owed by the borrower. It will then
            // return a loan receipt that must be given to the borrower
            let loan_receipt = self.create_loan_receipt(to_borrow.clone(), &account_config);

            // Withdraw the requested tokens from the lender's account
            let borrowed_tokens = self.withdraw_tokens_from_account(to_borrow, from_account);

            // Return the loan together with the loan receipt to the borrower
            (borrowed_tokens, loan_receipt)
        }

        /// Creates and returns a loan receipt.
        ///
        /// The loan receipt contains information about the borrowed amount and the fee due
        /// upon repayment.
        ///
        /// # Arguments
        /// * `borrow_amount` - The amount of resources borrowed.
        /// * `account_config` - A reference to the `FlashyfiAccountConfig` object for the account that the loan was taken out on.
        ///
        /// # Returns
        /// A bucket with the loan receipt
        fn create_loan_receipt(&self, borrow_amount: ResourceAmount, account_config: &FlashyfiAccountConfig) -> Bucket {
            // Ensure that the resource is borrowable
            assert!(
                account_config.is_resource_borrowable(&borrow_amount),
                "Resource cannot be borrowed: the target account is not configured to lend resource {:?}",
                borrow_amount.resource_address()
            );

            // Calculate the fee that must be paid by the borrower
            let fee_amount: FungibleAmount = match &borrow_amount {
                ResourceAmount::FungibleAmount(FungibleAmount(borrow_resource, borrow_amount)) => {
                    let fee = account_config.fungible_fee_configs.get(borrow_resource).map(|(_, fee)| fee).unwrap();
                    // When a fungible token is borrowed the fee can be...
                    match fee {
                        // ...calculated as a percentage of teh borrowed amount
                        // It must the be paid in the same resource that is borrowed
                        Fee::Percentage(fee_pct) => {
                            // Calculate the fee amount
                            let mut fee_amount = *borrow_amount * *fee_pct * dec!("0.01");

                            // Make sure the fee amount can be collected. If the token does not have
                            // maximum divisibility like e.g. the Radix token, it must be rounded
                            let rm = borrow_resource_manager!(*borrow_resource);
                            let divisibility = rm.resource_type().divisibility();
                            fee_amount =
                                fee_amount.round(divisibility.into(), RoundingMode::TowardsNearestAndHalfAwayFromZero);
                            FungibleAmount(*borrow_resource, fee_amount)
                        }
                        // ...a fixed fee payable in XRD only
                        Fee::Fixed(fixed_fee) => FungibleAmount::from(fixed_fee.clone()),
                    }
                }
                ResourceAmount::NonFungibleAmount(NonFungibleAmount(borrow_resource, ids)) => {
                    // If one or more non-fungible tokens are borrowed a per item "fixed" fee must be paid where the
                    // fee amount is calculated as the product of the fixed fee and the number of NFTs borrowed
                    let fee = account_config
                        .non_fungible_fee_configs
                        .get(borrow_resource)
                        .map(|(_, fixed_fee)| {
                            let fee_amount = fixed_fee.0;
                            let borrowed_tokens_count: Decimal = ids.len().try_into().unwrap();
                            FixedFee(fee_amount * borrowed_tokens_count)
                        })
                        .unwrap();
                    FungibleAmount::from(fee)
                }
            };

            // Mint the loan receipt and return it
            self.flashyfi_badge.authorize(|| {
                let rm = borrow_resource_manager!(self.addresses.loan_receipt_resource);
                rm.mint_uuid_non_fungible(LoanReceipt {
                    lender_account: account_config.account_address,
                    borrow_amount,
                    fee_amount,
                })
            })
        }

        /// Withdraws the specified amount from the specified account.
        ///
        /// # Arguments:
        /// * `borrow_amount` - The resource and amount to borrow
        /// * `from_account` - The account to borrow from
        ///
        /// # Returns:
        /// A bucket with the requested tokens
        fn withdraw_tokens_from_account(
            &self,
            borrow_amount: ResourceAmount,
            from_account: ComponentAddress,
        ) -> Bucket {
            let mut account = AccountComponentTarget::at(from_account);
            // We use the flashyfi_badge to authorize the withdrawal from the user's account
            self.flashyfi_badge.authorize(|| match borrow_amount {
                ResourceAmount::FungibleAmount(FungibleAmount(resource, amount)) => {
                    account.withdraw_by_amount(amount, resource)
                }
                ResourceAmount::NonFungibleAmount(NonFungibleAmount(resource, ids)) => {
                    account.withdraw_by_ids(ids, resource)
                }
            })
        }

        /// Repays the loan that is recorded in the supplied `loan_receipt`. This method must be
        /// called after the `borrow` method, within the same transaction, or the transaction cannot succeed.
        ///
        /// # Arguments:
        /// * `loan_receipt` - A [LoanReceipt] that tracks how much a user has borrowed from an account and what fee
        /// they owe the account owner.
        /// * `borrowed_tokens` - A bucket with the borrowed tokens. Any overpaid amount will be returned.
        /// * `fee_tokens` - A bucket with the tokens tha must be payed as a fee. Any overpaid amount will be returned.
        /// * `to_account` - The account to which the funds must be returned. This must correspond to the account that
        /// is referenced in the loan receipt.
        ///
        /// # Returns:
        /// A bucket with any overpaid tokens for the initial loan or the due fee respectively.
        pub fn repay_loan(
            &self,
            loan_receipt: Bucket,
            mut borrowed_tokens: Bucket,
            mut fee_tokens: Bucket,
            to_account: ComponentAddress,
        ) -> (Bucket, Bucket) {
            // Check that the given loan_receipt is of the correct resource type. This is not essential but helps to fail fast in case
            // this method is called erroneously
            assert_eq!(
                loan_receipt.resource_address(),
                self.addresses.loan_receipt_resource,
                "Invalid loan_receipt_token"
            );
            assert_eq!(loan_receipt.amount(), dec!("1"), "Only one loan can be rapid at a time");

            // Get the non fungible from the bucket
            let LoanReceipt { lender_account, borrow_amount, fee_amount } = loan_receipt.non_fungible().data();
            assert_eq!(to_account, lender_account, "Parameter to_account must be the lender's account");

            // Create an account stub so we can call methods on it
            let mut account = AccountComponentTarget::at(to_account);

            // Create a vec with buckets that we will (re)deposit into the lender's account
            let mut to_deposit = vec![];

            // Assert that the borrowed tokens bucket that is returned by the borrower contains the correct resource
            assert_eq!(
                borrowed_tokens.resource_address(),
                borrow_amount.resource_address(),
                "Invalid borrowed_tokens: expected resource {:?}",
                borrow_amount.resource_address()
            );
            // Then take only the fungible or non fungible tokens from that bucket that were borrowed
            to_deposit.push(match borrow_amount {
                ResourceAmount::FungibleAmount(FungibleAmount(_, amount)) => borrowed_tokens.take(amount),
                ResourceAmount::NonFungibleAmount(NonFungibleAmount(_, ids)) => {
                    borrowed_tokens.take_non_fungibles(&ids)
                }
            });

            // Assert that the fee tokens bucket that is send as payment by the borrower contains the correct resource
            assert_eq!(
                fee_tokens.resource_address(),
                fee_amount.resource_address(),
                "Invalid fee_tokens: expected resource {:?}",
                fee_amount.resource_address()
            );
            // Then only take the due fee amount
            to_deposit.push(fee_tokens.take(fee_amount.amount()));

            // (Re)deposit the two buckets into the lender's account
            account.deposit_batch(to_deposit);

            // Burn the loan receipt so the transaction can succeed
            self.flashyfi_badge.authorize(|| loan_receipt.burn());

            // Finally return any overpaid tokens to the borrower
            (borrowed_tokens, fee_tokens)
        }
    }
}

#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone)]
pub struct FlashyfiAddresses {
    /// The resource address of the Flashyfi badge that must be authorized to withdraw from user's
    /// accounts.
    pub flashyfi_badge_resource: ResourceAddress,

    /// The resource address of the account config badges that will be issued to every account that
    /// that is enabled for flash loans. This badged may be used to configured which tokens can be borrowed
    /// by users and what fees must be paid.
    pub account_config_badge_resource: ResourceAddress,

    /// The address of the loan receipt resource
    pub loan_receipt_resource: ResourceAddress,
}

/// A config that keeps track of which resources may be borrowed from an account and what fees users
/// must pay when borrowing.
#[derive(NonFungibleData)]
struct FlashyfiAccountConfig {
    /// The address of the account this config belongs to
    account_address: ComponentAddress,

    /// A map that keeps track of the fungible tokens that may be borrowed from the account.
    /// The map's keys are addresses of fungible tokens and the values are tuple2s where the first value keeps track of
    /// whether borrowing the specific token is possible (enabled) and the second values specifies a fixed or percentage
    /// fee that must be paid when borrowing the token.
    #[mutable]
    fungible_fee_configs: HashMap<ResourceAddress, (bool, Fee)>,

    /// A map that keeps track of the non-fungible tokens that may be borrowed from the account.
    /// The map's keys are addresses of non-fungible tokens and the values are tuple2s where the first value keeps track of
    /// whether borrowing the specific token is possible (enabled) and the second values specifies a fixed fee that must be
    /// paid when borrowing the token.
    #[mutable]
    non_fungible_fee_configs: HashMap<ResourceAddress, (bool, FixedFee)>,
}

impl FlashyfiAccountConfig {
    /// Checks whether the specified resource may be borrowed from the account referenced in this config.
    ///
    /// Only returns `true` if the resource is listed in the config and is also enabled.
    fn is_resource_borrowable(&self, resource_amount: &ResourceAmount) -> bool {
        match resource_amount {
            ResourceAmount::FungibleAmount(FungibleAmount(resource, _)) => {
                self.fungible_fee_configs.get(resource).map(|(borrowable, _)| *borrowable).unwrap_or(false)
            }
            ResourceAmount::NonFungibleAmount(NonFungibleAmount(resource, _)) => {
                self.non_fungible_fee_configs.get(resource).map(|(borrowable, _)| *borrowable).unwrap_or(false)
            }
        }
    }

    /// Asserts that the config is valid by checking that there are no negative fee values used
    pub(crate) fn assert_valid(&self) {
        fn assert_positive(fee_value: Decimal) {
            assert!(fee_value >= Decimal::zero(), "The fee value must not be negative")
        }

        for (_, fee_config) in self.fungible_fee_configs.values() {
            match fee_config {
                Fee::Percentage(value) => assert_positive(*value),
                Fee::Fixed(FixedFee(value)) => assert_positive(*value),
            };
        }

        for (_, fee_config) in self.non_fungible_fee_configs.values() {
            assert_positive(fee_config.0);
        }
    }
}

/// A fee that must be paid when borrowing tokens
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone)]
pub enum Fee {
    /// A fee that is calculated as a percentage of the borrowed tokens.
    /// If a Fee `Percentage(dec!("1"))` is specified and `dec!("200")` of token X are borrowed, the
    /// fee will amount to `dec("2")` of token X.
    Percentage(Decimal),

    /// A fixed fee of n XRD
    Fixed(FixedFee),
}

/// A fixed fee of n XRD
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone)]
pub struct FixedFee(Decimal);

impl From<FixedFee> for FungibleAmount {
    fn from(value: FixedFee) -> Self {
        FungibleAmount::new(RADIX_TOKEN, value.0)
    }
}

/// keeps track of a loan that has been taken out
#[derive(NonFungibleData)]
struct LoanReceipt {
    /// The account from which the loan was obtained
    lender_account: ComponentAddress,
    /// The amount that was borrowed
    borrow_amount: ResourceAmount,
    /// The fee that must be paid when repaying the loan
    fee_amount: FungibleAmount,
}

/// Combination of a resource address and either a [Decimal] amount or a set of [NonFungibleLocalId]s
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone)]
pub enum ResourceAmount {
    FungibleAmount(FungibleAmount),
    NonFungibleAmount(NonFungibleAmount),
}

impl ResourceAmount {
    fn resource_address(&self) -> ResourceAddress {
        match self {
            ResourceAmount::FungibleAmount(FungibleAmount(resource, _)) => *resource,
            ResourceAmount::NonFungibleAmount(NonFungibleAmount(resource, _)) => *resource,
        }
    }
}

/// A fungible resource and a [Decimal] amount
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone, new)]
pub struct FungibleAmount(ResourceAddress, Decimal);

impl FungibleAmount {
    fn resource_address(&self) -> ResourceAddress {
        self.0
    }

    fn amount(&self) -> Decimal {
        self.1
    }
}

/// A non-fungible resource and a set of [NonFungibleLocalId]s
#[derive(ScryptoCategorize, ScryptoEncode, ScryptoDecode, LegacyDescribe, Clone)]
pub struct NonFungibleAmount(ResourceAddress, BTreeSet<NonFungibleLocalId>);

// Define the minimal interface for interacting with a standard radix account
external_component! {
    AccountComponentTarget {
         fn withdraw_by_amount(&mut self, amount: Decimal, resource_address: ResourceAddress) -> Bucket;
         fn withdraw_by_ids(&mut self, ids: BTreeSet<NonFungibleLocalId>, resource_address: ResourceAddress) -> Bucket;
         fn deposit_batch(&mut self, buckets: Vec<Bucket>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine::types::node::NetworkDefinition;
    use transaction::builder::ManifestBuilder;
    use transaction::manifest::decompile;

    #[test]
    fn test() {
        // FIXME REMOVE
        let mut fungible_fee_configs: HashMap<ResourceAddress, (bool, Fee)> = HashMap::new();
        fungible_fee_configs.insert(RADIX_TOKEN, (false, Fee::Percentage(dec!("0.1"))));
        fungible_fee_configs.insert(ResourceAddress::Normal([1; 26]), (false, Fee::Fixed(FixedFee(dec!("1")))));

        let mut non_fungible_fee_configs: HashMap<ResourceAddress, (bool, FixedFee)> = HashMap::new();
        non_fungible_fee_configs.insert(ResourceAddress::Normal([0; 26]), (false, FixedFee(dec!("1"))));

        let access_rules: BTreeMap<_, (_, AccessRule)> = BTreeMap::default();

        let manifest = ManifestBuilder::new()
            .call_method(
                ComponentAddress::Normal([0; 26]),
                "flashyfi_account",
                args!(ComponentAddress::Account([0; 26]), fungible_fee_configs, non_fungible_fee_configs),
            )
            .create_fungible_resource(DIVISIBILITY_MAXIMUM, BTreeMap::default(), access_rules, Some(dec!("1000000")))
            .build();
        println!("{}", decompile(&manifest.instructions, &NetworkDefinition::nebunet()).unwrap());
    }
}
