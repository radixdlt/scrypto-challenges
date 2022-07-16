use scrypto::prelude::*;
use crate::lending_pool::*;
use crate::radiswap::*;
use crate::collateral_pool::*;
use crate::user_management::*;
use crate::pseudopriceoracle::*;
use crate::structs::{User, FlashLoan, Loan};

blueprint! {
    /// This is the main component for this protocol. It can be considered as a router, taken inspiration from Omar's "RaDEX"
    /// DEX challenge submission. It essentially routes method calls to the right lending pool and collateral pool allowing
    /// for multi collateral support. It also facilitates the flash loan mechanism(s) and distributes Degen Tokens.
    struct DegenFi {
        // Contains all the lending pool addresses
        lending_pools: HashMap<ResourceAddress, LendingPool>,
        lending_pool_address: HashMap<ResourceAddress, ComponentAddress>,
        // Contains all the collateral pool addresses
        collateral_pools: HashMap<ResourceAddress, CollateralPool>,
        collateral_pool_address: HashMap<ResourceAddress, ComponentAddress>,
        // User Management component address
        user_management_address: ComponentAddress,
        // Price oracle component address
        pseudopriceoracle_address: ComponentAddress,
        // Radiswap component address
        radiswap_address: Option<ComponentAddress>,
        // Access Admin Badge used to mint/burn Access Tokens
        access_auth_vault: Vault,
        // Access Tokens are used to be able to make permissioned calls between Blueprints
        access_badge_vault: Vault,
        // The resource address of the Access Token
        access_badge_address: ResourceAddress,
        // Admin badge to mint/burn Degen Tokens
        degen_auth_vault: Vault,
        // Resource address of Degen Tokens
        degen_token_address: ResourceAddress,
        // Contains the initial supply of Degen Tokens
        degen_token_vault: Vault,
        // Resource address of the SBT
        sbt_address: Vec<ResourceAddress>,
        //Flash loan admin badge
        flash_loan_auth_vault: Vault,
        // Flash loan resource address
        flash_loan_address: ResourceAddress,
        // Data structure for the loan NFTs with a Health Factor below 1.
        bad_loans: HashMap<NonFungibleId, ResourceAddress>,
    }

    impl DegenFi {
        pub fn new(
        ) -> ComponentAddress 
        {
            
            // Creates badge to authorizie to mint/burn flash loan
            let flash_loan_token = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin authority for BasicFlashLoan")
                .metadata("symbol", "FLT")
                .metadata("description", "Admin authority to mint/burn flash loan tokens")
                .initial_supply(1);

            // Define a "transient" resource which can never be deposited once created, only burned
            let flash_loan_address = ResourceBuilder::new_non_fungible()
                .metadata(
                    "name",
                    "Promise token for BasicFlashLoan - must be returned to be burned!",
                )
                .mintable(rule!(require(flash_loan_token.resource_address())), LOCKED)
                .burnable(rule!(require(flash_loan_token.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(flash_loan_token.resource_address())), LOCKED)
                .restrict_deposit(rule!(deny_all), LOCKED)
                .no_initial_supply();

            // Creates badge to mint/burn access tokens
            let access_admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Access Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Admin authority to mint/burn Access Tokens")
                .initial_supply(1);   

            // Creates badge to allow permissioned method calls between Blueprints    
            let access_badge = ResourceBuilder::new_fungible()
                .metadata("name", "Access Token")
                .metadata("symbol", "AT")
                .metadata("description", 
                "Access Tokens are used to be able to make permissioned calls between Blueprints")
                .mintable(rule!(require(access_admin.resource_address())), LOCKED)
                .burnable(rule!(require(access_admin.resource_address())), LOCKED)
                .initial_supply(1);

            // Retrieves resource address of the Access Token to register as Access Rule
            let access_badge_address = access_badge.resource_address();

            // Creates admin badge to authorize minting/burning of Degen Tokens
            let degen_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Degen Admin Badge")
                .metadata("symbol", "DAB")
                .metadata("description", 
                "This is an admin badge that has the authority to mint and burn Degen Tokens")
                .initial_supply(1);  

            // The utility token for DegenFi (currently no use case)                 
            let degen_token = ResourceBuilder::new_fungible()
                .metadata("name", "Degen Token")
                .metadata("symbol", "DT")
                .metadata("description", "Degen Token is DegenFi's utility token. Earn Degen Tokens by interacting with the protocol!")
                .mintable(rule!(require(degen_badge.resource_address())), LOCKED)
                .burnable(rule!(require(degen_badge.resource_address())), LOCKED)
                .initial_supply(1000);

            return Self {
                lending_pools: HashMap::new(),
                lending_pool_address: HashMap::new(),
                collateral_pools: HashMap::new(),
                collateral_pool_address: HashMap::new(),
                user_management_address: UserManagement::new(access_badge.resource_address()),
                pseudopriceoracle_address: PseudoPriceOracle::new(),
                radiswap_address: None,
                access_auth_vault: Vault::with_bucket(access_admin),
                access_badge_vault: Vault::with_bucket(access_badge),
                access_badge_address: access_badge_address,
                degen_auth_vault: Vault::with_bucket(degen_badge),
                degen_token_address: degen_token.resource_address(),
                degen_token_vault: Vault::with_bucket(degen_token),
                sbt_address: Vec::new(),
                flash_loan_auth_vault: Vault::with_bucket(flash_loan_token),
                flash_loan_address: flash_loan_address,
                bad_loans: HashMap::new(),
            }
            .instantiate()
            .globalize();
        }

        /// Creates a new user for the lending protocol.
        /// 
        /// This method is used to create a new user for DegenFi. A "Soul Bound Token" (SBT) is
        /// created and sent to the user's wallet which cannot be transferred or burnt. The SBT tracks
        /// user interactions within the protocol. Its major use case is to attempt to create a borrowing
        /// track record to underwrite the user's credit worthines. The user has to submit their
        /// wallet's component address to prevent the creation of multiple SBTs. Most of the protocol's
        /// method will require users to submit a proof of their SBT in order to use the protocol. 
        /// 
        /// This method does not have any checks. The check(s) are done through the User Management component.
        /// 
        /// # Arguments: 
        /// 
        /// * `account_address` (ComponentAddress) - The user's wallet address to ensure the user cannot create multiple
        /// SBTs.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - This is the SBT the user receives from creating a new user.
        /// * `Bucket` - Users are rewarded 1 bonus Degen Tokens for the initial creation of their SBT.
        pub fn new_user(
            &mut self,
            account_address: ComponentAddress
        ) -> (Bucket, Bucket)
        {
            // Retrieves User Management component.
            let user_management: UserManagement = self.user_management_address.into();
            // Makes authorized method call to create a new user for the protocol.
            let new_user: Bucket = self.access_badge_vault.authorize(|| 
                user_management.new_user(account_address)
            );
            // User receives 1 Degen Token for creating a user
            let degen_token = self.degen_token_vault.take(dec!("1"));

            info!("User created! Your SBT resource address is {:?}", new_user.resource_address());
            info!(
                "Thank you for registering an account at DegenFi, here are {:?} Degen Tokens for you to start!", degen_token.amount());
            // Registers the resource address the SBT   
            self.sbt_address.push(new_user.resource_address());

            (new_user, degen_token)
        }

        /// Sets the collateral pool address for the lending pool.
        /// 
        /// This method is used so that the lending pool has permissioned access to move funds to and from
        /// the collateral pool.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `lendingpool_address` (ResourceAddress) - The requested lending pool to set the address for.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn set_address(
            &mut self,
            lendingpool_address: ResourceAddress,
        ) 
        {
            let lending_pool: &LendingPool = self.lending_pools.get(&lendingpool_address).unwrap();
            let collateral_pool_address = self.collateral_pool_address.get(&lendingpool_address).unwrap();
            lending_pool.set_address(*collateral_pool_address);
        }

        /// Gets the NonFungibleId of the SBT.
        /// 
        /// This method is used retrieve the NonFungibleId of the SBT. It can be used as a pseudocheck
        /// to make sure that the Proof of the SBT provided is from this protocol. NonFungibleId is retrieved
        /// so that the method calls can use to view and update the SBT data.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (&Proof) - A reference to the Proof of the SBT.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        fn get_user(
            &self, 
            user_auth: &Proof
        ) -> NonFungibleId 
        {
            let user_id = user_auth.non_fungible::<User>().id();
            return user_id
        }

        /// Checks if a lending pool for the given token exists or not.
        pub fn pool_exists(
            &self,
            address: ResourceAddress
        ) -> bool
        {
            return self.lending_pools.contains_key(&address);
        }

        /// Asserts that a lending pool for the given address exists
        pub fn assert_pool_exists(
            &self,
            address: ResourceAddress,
            label: String
        ) 
        {
            assert!(
                self.pool_exists(address), 
                "[{}]: No lending pool exists for the given address.", 
                label
            );
        }
        
        /// Asserts that a lending pool for the given address pair doesn't exist.
        pub fn assert_pool_doesnt_exists(&self, address: ResourceAddress, label: String) {
            assert!(
                !self.pool_exists(address), 
                "[{}]: A lending pool exists with the given address.", 
                label
            );
        }

        /// Checks if a collateral pool for the given pair of tokens exists or not.
        pub fn collateral_pool_exists(
            &self,
            address: ResourceAddress
        ) -> bool 
        {
            return self.collateral_pools.contains_key(&address);
        }

        /// Asserts that a collateral pool for the given address exists
        pub fn assert_collateral_pool_exists(
            &self,
            address: ResourceAddress,
            label: String
        )
        {
            assert!(
                self.pool_exists(address), 
                "[{}]: No collateral pool exists for the given address.", 
                label
            );
        }
        
        /// Asserts that a collateral pool for the given address doesn't exist.
        pub fn assert_collateral_pool_doesnt_exists(
            &self,
            address: ResourceAddress,
            label: String
        )
        {
        assert!(
            !self.collateral_pool_exists(address), 
            "[{}]: A collateral pool exists with the given address.", 
            label
        );
    }

        /// Sets the pricing of the asset.
        /// 
        /// This method is used to set the price of a given asset. It makes a call to the 
        /// Pseudo Price Oracle component.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the requested token to
        /// set the pricing.
        /// * `set_price` (Decimal) - The price changed to.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn set_price(
            &mut self,
            token_address: ResourceAddress,
            set_price: Decimal
        )
        {
            let pseudopriceoracle: PseudoPriceOracle = self.pseudopriceoracle_address.into();
            pseudopriceoracle.set_price(token_address, set_price);
        }

        /// Gets the price of the given asset.
        /// 
        /// This method is used to retrieve pricing information of the given asset.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the requested token to
        /// set the pricing.
        /// 
        /// # Returns:
        /// 
        /// This method does not return anything.
        pub fn get_price(
            &self,
            token_address: ResourceAddress
        ) -> Decimal
        {
            let pseudopriceoracle: PseudoPriceOracle = self.pseudopriceoracle_address.into();
            let price = pseudopriceoracle.get_price(token_address);
            return price
        }

        /// Instantiates the Radiswap Blueprint.
        /// 
        /// This method is used to instantiate the Radiswap Blueprint and sets the Radiswap
        /// component address. It is used so that the protocol can make calls to swap assets. It
        /// is mainly used as part of the "Folded Leverage" mechanic.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `xrd_token` (ResourceAddress) - The ResourceAddress of the token A. For demonstration
        /// purposes, it is meant to be the XRD token for clarity.
        /// * `usd_token` (ResourceAddress) - The ResourceAddress of the token B. For demonstration
        /// purposes, it is meant to be the USD token for clarity.
        /// * `lp_initial_supply` (Decimal) - The price changed to.
        /// * `lp_symbol` (String) -
        /// * `lp_name` (String)
        /// * `lp_url` (String)
        /// * `fee` (Decimal)
        /// # Returns:
        /// 
        /// * `Bucket` - The LP Tokens
        pub fn new_radiswap(
            &mut self,
            xrd_token: Bucket,
            usd_token: Bucket,
            lp_initial_supply: Decimal,
            lp_symbol: String,
            lp_name: String,
            lp_url: String,
            fee: Decimal,
        ) -> Bucket
        {
            let (radiswap, lp_tokens) = Radiswap::instantiate_pool(
                xrd_token,
                usd_token,
                lp_initial_supply,
                lp_symbol,
                lp_name,
                lp_url,
                fee
            )
            .into();

            self.radiswap_address.get_or_insert(radiswap);

            lp_tokens
        }

        /// Swaps Token A for Token B.
        /// 
        /// This method is used to swap Token A for Token B through the Radiswap component.
        /// 
        /// This method does not have any checks.
        /// 
        /// # Arguments: 
        /// 
        /// * `input_tokens` (Bucket) - The input amount of the token to be swapped for.
        /// set the pricing.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The output amount of the token received.
        pub fn swap(
            &mut self,
            input_tokens: Bucket
        ) -> Bucket
        {
            let radiswap: Radiswap = self.radiswap_address.unwrap().into();
            let return_bucket = radiswap.swap(input_tokens);
            return_bucket
        }

        /// Creates a new lending pool with the deposited asset.
        /// 
        /// This method is used to create a new lending pool of the deposited asset.
        /// 
        /// This method does a number of checks before a Lending Pool is created, these checks are:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// The majority of the checking is done in the `new` function of the LendingPool where it checks to ensure 
        /// that the buckets are not empty or that the given asset are fungibles. The checks done here
        /// are just lending checks to ensure that we don't create a lending pool for a lending pool that already  
        /// exist.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `deposit` (Bucket) - A bucket containing the amount of the first token used to initialize the pool.
        /// 
        /// # Returns:
        /// 
        /// * This method does not return anything.
        pub fn new_lending_pool(
            &mut self,
            user_auth: Proof,
            deposit_amount: Bucket
        ) -> Bucket
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves the User Management component.
            let user_management = self.user_management_address.into();
            // Retrieves the Pseudo Price Oracle component.
            let pseudopriceoracle = self.pseudopriceoracle_address.into();
            // Retrieves the resource address of the assets deposited in the bucket.
            let token_address: ResourceAddress = deposit_amount.resource_address();

            // Checking if a lending pool already exists for this token.
            self.assert_pool_doesnt_exists(
                deposit_amount.resource_address(), 
                String::from("New Liquidity Pool")
            );

            // Checking if user exists.
            let user_id = self.get_user(&user_auth);
            // Retrieves the amount data in the bucket.
            let amount = deposit_amount.amount();

            // Mints an access badge for the lending pool and collateral pool.
            let access_badge_token = self.access_auth_vault.authorize(|| borrow_resource_manager!(self.access_badge_address).mint(Decimal::one()));
            let access_badge_token2 = self.access_auth_vault.authorize(|| borrow_resource_manager!(self.access_badge_address).mint(Decimal::one()));
            
            // Instantiates the lending pool and collateral pool.
            let lending_pool: ComponentAddress = LendingPool::new(user_management, pseudopriceoracle, deposit_amount, access_badge_token);
            let collateral_pool: ComponentAddress = CollateralPool::new(user_management, lending_pool, token_address, access_badge_token2);
            
            // Retrieves User Management Component
            let user_management: UserManagement = self.user_management_address.into();
            // Authorizes balance update
            self.access_badge_vault.authorize(||
                user_management.add_deposit_balance(user_id.clone(), token_address, amount)
            );

            // Inserts into lending pool hashmap.
            self.lending_pools.insert(
                token_address,
                lending_pool.into()
            );

            self.lending_pool_address.insert(
                token_address,
                lending_pool
            );

            self.collateral_pool_address.insert(
                token_address,
                collateral_pool
            );

            // Inserts into collateral pool hashmap.
            self.collateral_pools.insert(
                token_address,
                collateral_pool.into()
            );

            // Retrieves Pseudo Price Oracle
            let pseudopriceoracle: PseudoPriceOracle = self.pseudopriceoracle_address.into();
            // Performs cross-blueprint call to register the token
            pseudopriceoracle.insert_resource(token_address);
            // Takes 5 Degen Token to give to the user for creating the lending pool.
            let degen_token = self.degen_token_vault.take(dec!("5"));
            
            info!("[DegenFi]: New lending pool for {:?} created!", token_address);
            info!("[DegenFi]: Depositing {:?} of {:?} as liquidity", amount, token_address);
            info!("[DegenFi]: You've received {:?} Degen Tokens", degen_token.amount());

            degen_token
        }

        /// Deposits supply of a given asset.
        /// 
        /// This method is used to add aditional liquidity to the lending pool. The user
        /// must first identify which
        /// 
        /// This method does a number of checks before supply is deposited, these checks are:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `deposit` (Bucket) - A bucket containing the amount of the first token used to initialize the pool.
        /// 
        /// # Returns:
        /// 
        /// * This method does not return anything.
        pub fn deposit_supply(
            &mut self,
            user_auth: Proof,
            deposit_amount: Bucket
        ) -> Bucket
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");
            // Retrieve resource address of the deposit
            let token_address: ResourceAddress = deposit_amount.resource_address(); 
            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            // Checks if the token resources are the same
            assert_eq!(token_address, deposit_amount.resource_address(), "Token requested and token deposited must be the same.");
            
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_address);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    info!("[DegenFi]: Depositing {:?} of {:?} as liquidity.", deposit_amount.amount(), token_address);
                    lending_pool.deposit(user_id, deposit_amount);
                    // Retrieves 1 supply of Degen Token to be given to the user for interacting with the protocol.
                    let degen_token = self.degen_token_vault.take(1);
                    degen_token
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist. Creating a new one.", token_address);
                    let degen_token = self.new_lending_pool(user_auth, deposit_amount);
                    degen_token
                }
            }
        }

        /// Deposits collateral of a given asset.
        /// 
        /// This method is used to add collateral of the given asset. Currently the collateral
        /// design locks up the asset. Future iterations may provide ability to redeploy collateral
        /// as supply to provide more liquidity and allows borrowers (who use their collateral)
        /// earn APY.
        /// 
        /// This method does a number of checks before collateral is deposited, these checks are:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `amount` (Bucket) - A bucket containing the amount of collateral token deposited.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn deposit_collateral(
            &mut self,
            user_auth: Proof,
            amount: Bucket
        ) -> Bucket
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");
            // Retrieves token address of the amount sent
            let token_address: ResourceAddress = amount.resource_address(); 
            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);
            
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&token_address);
            match optional_collateral_pool {
                Some (collateral_pool) => { // If it matches it means that the collateral pool exists.
                    info!("[DegenFi]: Depositing {:?} of {:?} as collateral.", amount.amount(), token_address);
                    collateral_pool.deposit(user_id, token_address, amount);
                    let degen_token = self.degen_token_vault.take(1);
                    degen_token
                }
                None => {
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_address);
                    let empty_bucket1 = self.access_auth_vault.take(0);
                    empty_bucket1
                }
            }
        }

        /// Tops off additional collateral for a given loan.
        /// 
        /// This method is used to add additionall collateral of a given loan.
        /// 
        /// This method does a number of checks before collateral is deposited, these checks are:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks whether the requested token to supply additional collateral and the token
        /// passed in the bucket are the same. 
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `loan_id` (NonFungibleId) - The NonFungibleId of the loan to modify the loan NFT data.
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the token the user wish to add as addiitonal
        /// collateral.
        /// * `amount` (Bucket) - A bucket containing the amount of collateral to be deposited.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn deposit_additional_collateral(
            &mut self, user_auth: Proof,
            loan_id: NonFungibleId,
            token_address: ResourceAddress,
            amount: Bucket
        ) -> Bucket
        { 
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID.
            let user_id = self.get_user(&user_auth);

            // Checks if the token resources are the same.
            assert_eq!(token_address, amount.resource_address(), "Token requested and token deposited must be the same.");
            
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&token_address);
            match optional_collateral_pool {
                Some (collateral_pool) => { // If it matches it means that the collateral pool exists.
                    info!("[DegenFi]: Depositing additional {:?} of {:?} as collateral towards your {:?} position.", amount.amount(), token_address, loan_id);
                    // Calls on the Collateral Pool component.   
                    collateral_pool.deposit_additional(user_id, loan_id, token_address, amount);
                    // Retrieves 1 supply of Degen Token to be given to the user for interacting with the protocol.
                    let degen_token = self.degen_token_vault.take(1);
                    degen_token
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_address);
                    // Since a bucket must be returned, this creates an empty bucket.
                    let empty_bucket1 = self.access_auth_vault.take(0);
                    empty_bucket1
                }
            }
        }

        /// Allows user to convert their supply liquidity as collateral.
        /// 
        /// This method is used to convert a user's supply liquidty as collateral for users to use
        /// as collateral.
        /// 
        /// This method does a number of checks before collateral is deposited, these checks are:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// Most of the checks are done through the Collateral Pool component.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the token the user wish to add as addiitonal
        /// collateral.
        /// * `amount` (Decimal) - The amount request to be converted to collateral supply.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn convert_to_collateral(
            &mut self,
            user_auth: Proof,
            token_requested: ResourceAddress,
            amount: Decimal
        )
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    info!("[DegenFi]: Converting {:?} of {:?} to collateral supply.", amount, token_requested);
                    self.set_address(token_requested);
                    self.access_badge_vault.authorize(|| 
                        lending_pool.convert_to_collateral(user_id, token_requested, amount));
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                }
            }
        }
        
        
        /// Allows user to convert their collateral to liquidity supply.
        /// 
        /// This method is used to convert a user's collateral to supply liquidity and earn APY.
        /// 
        /// This method does a number of checks before collateral is deposited, these checks are:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// Most of the checks are done through the Lending Pool component.
        /// 
        /// # Arguments: 
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `token_address` (ResourceAddress) - The ResourceAddress of the token the user wish to convert their collateral
        /// to liquidity supply.
        /// * `amount` (Decimal) - The amount request to be converted to liquidity supply.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn convert_to_deposit(
            &mut self,
            user_auth: Proof,
            token_requested: ResourceAddress,
            amount: Decimal
        )
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&token_requested);
            match optional_collateral_pool {
                Some (collateral_pool) => { // If it matches it means that the lending pool exists.
                    info!("[DegenFi]: Converting {:?} of {:?} to deposit supply", amount, token_requested);
                    self.access_badge_vault.authorize(|| 
                        collateral_pool.convert_to_deposit(user_id, token_requested, amount));
                }
                None => {
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                }
            }
        }

        /// Allows user to borrow funds from the pool.
        ///
        /// This method is used to allow users to borrow funds from the pool.
        /// 
        /// This method performs a number of checks before the borrow is made:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset to borrow.
        /// * `collateral_address` (ResourceAddress) - This is the resource address of the collateral the user wishes to use
        /// as collateral for this loan.
        /// * `amount` (Decimal) - This is the amount that the borrower wishes to borrow from the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - Returns a bucket of the borrowed funds from the pool.
        /// * `Bucket` - Returns the loan NFT to the user.
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn borrow(
            &mut self,
            user_auth: Proof,
            token_requested: ResourceAddress,
            collateral_address: ResourceAddress,
            amount: Decimal
        ) -> (Bucket, Bucket, Bucket)
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID.
            let user_id = self.get_user(&user_auth);

            // Attempting to get the lending pool component associated with the requested asset to borrowed.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    info!("[DegenFi]: Borrowing: {:?}, Amount: {:?}", token_requested, amount);
                    let (return_borrow, loan_nft): (Bucket, Bucket) = self.access_badge_vault.authorize(||
                    lending_pool.borrow(user_id, token_requested, collateral_address, amount));
                    let degen_token = self.degen_token_vault.take(1);
                    (return_borrow, loan_nft, degen_token)
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket2: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket3: Bucket = self.access_auth_vault.take(0);
                    (empty_bucket1, empty_bucket2, empty_bucket3)
                }
            }
        }

        /// Allows user to top off additional funds from the pool.
        ///
        /// This method is used to allow users to borrow additional funds from the pool.
        /// 
        /// This method performs a number of checks before the borrow is made:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `loan_id` (NonFungibleId) - The NonFungibleId of the loan the user wishes to top off on more funds.
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset to borrow.
        /// * `amount` (Decimal) - This is the amount that the borrower wishes to borrow from the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - Returns a bucket of the borrowed funds from the pool.
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn borrow_additional(
            &mut self,
            user_auth: Proof,
            loan_id: NonFungibleId,
            token_requested: ResourceAddress,
            amount: Decimal
        ) -> (Bucket, Bucket)
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    info!("[DegenFi]: Borrowing: {:?}, Amount: {:?}", token_requested, amount);
                    let return_borrow: Bucket = self.access_badge_vault.authorize(||
                    lending_pool.borrow_additional(user_id, loan_id, token_requested, amount));
                    let degen_token = self.degen_token_vault.take(1);
                    (return_borrow, degen_token)
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket2: Bucket = self.access_auth_vault.take(0);
                    (empty_bucket1, empty_bucket2)
                }
            }
        }

        
        /// Allows user to perform a flash loan.
        ///
        /// This method is used to allow users to perform a flash loan. A transient token is created to record the amount
        /// that was borrowed. The transient token must be burnt for the transaction to complete. Currently, there is no
        /// fee for performing flash loans. 
        /// 
        /// This method performs a number of checks before the borrow is made:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset to borrow.
        /// * `amount` (Decimal) - This is the amount that the borrower wishes to borrow from the pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - Returns a bucket of the borrowed funds from the pool.
        /// * `Bucket` - The transient token representing the flash loan.
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn flash_borrow(
            &mut self,
            token_requested: ResourceAddress,
            amount: Decimal
        ) -> (Bucket, Bucket, Bucket)
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    let return_borrow: Bucket = self.access_badge_vault.authorize(||
                    lending_pool.flash_borrow(amount));
                    // Mints the transient token
                    let transient_token = self.flash_loan_auth_vault.authorize(|| {
                        borrow_resource_manager!(self.flash_loan_address)
                        .mint_non_fungible(
                            &NonFungibleId::random(),
                            FlashLoan {
                                amount_due: amount,
                                asset: token_requested,
                                borrow_count: 1,
                            },
                        )
                    });
                    let degen_token = self.degen_token_vault.take(1);
                    (return_borrow, transient_token, degen_token)
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket2: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket3: Bucket = self.access_auth_vault.take(0);
                    (empty_bucket1, empty_bucket2, empty_bucket3)
                }
            }
        }

        /// Removes the percentage of the liquidity owed to this liquidity provider.
        /// 
        /// This method is used to calculate the amount of tokens owed to the liquidity provider and take them out of
        /// the lending pool and return them to the liquidity provider.
        /// 
        /// This method performs a number of checks before liquidity is removed from the pool:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `token_address` (ResourceAddress) - This is the token address of the requested amount to be redeemed.
        /// exchange for their share of the liquidity.
        /// * `amount` (Decimal) - This is the amount requested to redeem.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the tokens to be redeemed.
        pub fn redeem(
            &mut self, 
            user_auth: Proof, 
            token_requested: ResourceAddress, 
            amount: Decimal
        ) -> Bucket 
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists. 
                    info!("[DegenFi]: Redeeming {:?} of {:?}", amount, token_requested); 
                    let return_bucket: Bucket = self.access_badge_vault.authorize(|| 
                    lending_pool.redeem(user_id, token_requested, amount));
                    return_bucket
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    let empty_bucket: Bucket = self.access_auth_vault.take(0);
                    empty_bucket
                }
            }
        }

        /// Removes the collateral owed to the user.
        /// 
        /// This method is used to redeem the collateral the user deposited.
        /// 
        /// This method performs a number of checks before liquidity is removed from the pool:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `token_address` (ResourceAddress) - This is the token address of the requested amount to be redeemed.
        /// exchange for their share of the liquidity.
        /// * `amount` (Decimal) - This is the amount requested to redeem.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the tokens to be redeemed.
        pub fn redeem_collateral(
            &mut self,
            user_auth: Proof,
            collateral_address: ResourceAddress,
            amount: Decimal,
        ) -> Bucket
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&collateral_address);
            match optional_collateral_pool {
                Some (collateral_pool) => { // If it matches it means that the lending pool exists. 
                    info!("[DegenFi]: Redeeming {:?} of {:?}", amount, collateral_address); 
                    let return_bucket: Bucket = self.access_badge_vault.authorize(|| 
                    collateral_pool.redeem(user_id, collateral_address, amount));
                    return_bucket
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", collateral_address);
                    let empty_bucket: Bucket = self.access_auth_vault.take(0);
                    empty_bucket
                }
            }

        }

        /// Repays the loan in partial or in full.
        /// 
        /// This method is used to pay down or pay off the loan.
        /// 
        /// This method performs a number of checks before liquidity removed from the pool:
        /// 
        /// * **Check 1:** Checks if the user exist in this protocol.
        /// 
        /// * **Check 2:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `loan_id` (NonFungibleId) - The NonFungibleId of the loan the user wishes to top off on more funds.
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset to borrow.
        /// * `amount` (Bucket) - The bucket that contains the asset to repay the loan.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.
        pub fn repay(
            &mut self, 
            user_auth: Proof, 
            loan_id: NonFungibleId, 
            token_requested: ResourceAddress, 
            amount: Bucket
        ) -> (Bucket, Bucket) 
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            // Retrieves user NFT ID
            let user_id = self.get_user(&user_auth);

            // Checks if the token resources are the same
            assert_eq!(token_requested, amount.resource_address(), "Token requested and token deposited must be the same.");

            // Repay fully or partial?
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    let return_bucket: Bucket = lending_pool.repay(user_id, loan_id, token_requested, amount);
                    let degen_token = self.degen_token_vault.take(1);
                    (return_bucket, degen_token)
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket2: Bucket = self.access_auth_vault.take(0);
                    (empty_bucket1, empty_bucket2)
                }
            }
        }

        /// Allows user to repay the flash loan borrow.
        ///
        /// This method is used to allow users to repay their flash loan. The amount repaid must
        /// equal what was recorded in the flash loan token data structure.
        /// 
        /// This method performs a number of checks before the repayment is made:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// Most of the checks as it relates to the flash loan is done in the Lending Pool component.
        /// 
        /// # Arguments:
        /// 
        /// * `repay_amount` (Bucket) - The bucket that contains the asset to be repaid.
        /// * `flash_loan` (Bucket) - The bucket that contains the flash loan.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Degen Tokens received for interacting with the protocol.        
        pub fn flash_repay(
            &mut self,
            repay_amount: Bucket,
            flash_loan: Bucket
        ) -> Bucket 
        {
            let flash_loan_data: FlashLoan = flash_loan.non_fungible().data();
            // Asserts the amount passed is equal the amount borrowed.
            assert!(repay_amount.amount() >= flash_loan_data.amount_due, "Insufficient repayment given for your loan!");

            // Asserts if flash loan bucket is empty.
            assert_ne!(flash_loan.is_empty(), true, "Cannot be empty.");

            // Assets flash loan token belongs to this protocol
            assert_eq!(flash_loan.resource_address(), self.flash_loan_address, "Flash loan token must belong to this pool");

            // Retrieve flash loan data
            let flash_borrow_resource_address = flash_loan_data.asset;
            
            // Asserts
            assert_eq!(repay_amount.resource_address(), flash_borrow_resource_address, "The incorrect asset passed.");

            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&repay_amount.resource_address());
            match optional_lending_pool {
                Some (lending_pool) => { // If it matches it means that the lending pool exists.
                    self.access_badge_vault.authorize(|| 
                        lending_pool.flash_repay(repay_amount));
                    self.flash_loan_auth_vault.authorize(|| flash_loan.burn());
                    let degen_token = self.degen_token_vault.take(1);
                    degen_token
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", repay_amount.resource_address());
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    empty_bucket1
                }
            }
        }

        pub fn liquidate(
            &mut self,
            loan_id: NonFungibleId,
            repay_amount: Bucket
        ) -> (Bucket, Bucket)
        {  
            // Runs methods to update the bad loans data structure.
            self.insert_bad_loans();
            // Asserts that the loan exist in the bad loans data structure.
            assert_eq!(self.bad_loans.contains_key(&loan_id), true, "This is not a bad loan.");
            // Retrieves loan resource address.
            let loan_resource_address = self.bad_loans.get(&loan_id).unwrap();
            // Retrieves the collateral address of the loan NFT.
            let collateral_address = self.get_loan_collateral(&loan_id);
            // Retrieve the resource address of the repayment.
            let repayment_address = self.get_loan_asset(&loan_id);
            // Attempts to find the correct collateral pool.
            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&collateral_address);
            match optional_collateral_pool {
                Some (collateral_pool) => { // If it matches it means that the lending pool exists.
                    let lending_pool_address: ComponentAddress = *self.lending_pool_address.get(&repayment_address).unwrap();
                    let lending_pool: LendingPool = lending_pool_address.into();
                    let claim_liquidation: Bucket = self.access_badge_vault.authorize(|| 
                        collateral_pool.liquidate(loan_id, *loan_resource_address, collateral_address, lending_pool, repay_amount));
                    let degen_token = self.degen_token_vault.take(1);
                    (claim_liquidation, degen_token)
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", collateral_address);
                    let empty_bucket1: Bucket = self.access_auth_vault.take(0);
                    let empty_bucket2: Bucket = self.access_auth_vault.take(0);
                    (empty_bucket1, empty_bucket2)
                }
            }
        }

        fn get_loan_collateral(
            &self,
            loan_id: &NonFungibleId
        ) -> ResourceAddress
        {
            let loan_resource_address = self.bad_loans.get(loan_id).unwrap();
            let resource_manager = borrow_resource_manager!(*loan_resource_address);
            let loan_data: Loan = resource_manager.get_non_fungible_data(loan_id);
            let collateral_address = loan_data.collateral;

            collateral_address
        }

        fn get_loan_asset(
            &self,
            loan_id: &NonFungibleId
        ) -> ResourceAddress
        {
            let loan_resource_address = self.bad_loans.get(loan_id).unwrap();
            let resource_manager = borrow_resource_manager!(*loan_resource_address);
            let loan_data: Loan = resource_manager.get_non_fungible_data(loan_id);
            let asset_address = loan_data.asset;

            asset_address
        }

        
        /// Allows user to find loans that are below Health Factor of 1
        ///
        /// This method is used to display any loans that have a Health Factor of 1.
        /// It emits a message displaying the loan NFT ID and its Health Factor. In the future
        /// There will be more information that will be displayed.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset to borrow.
        /// 
        /// # Returns:
        /// 
        /// This method does not return any assets.
        pub fn find_bad_loans(
            &mut self,
            token_requested: ResourceAddress
        )
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { 
                    lending_pool.find_bad_loans();
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                }
            }
        }

        pub fn insert_bad_loans(
            &mut self,
        )
        {
            let lending_pools = self.lending_pools.iter();
            for (token_address, _lending_pool) in lending_pools {
                let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_address);
                match optional_lending_pool {
                    Some (lending_pool) => { 
                        let bad_pool_loans = lending_pool.bad_loans();
                        for (loans, loan_resource_address) in bad_pool_loans {
                            self.bad_loans.insert(loans, loan_resource_address);
                        }
                    }
                    None => { 
                        info!("[DegenFi]: Pool for doesn't exist.");
                    }
                }
                
            }
        }

        pub fn bad_loans(
            &mut self
        ) -> HashMap<NonFungibleId, ResourceAddress>
        {
            self.insert_bad_loans();
            return self.bad_loans.clone()
        }

        /// Allows user to check the liquidity of a given pool.
        ///
        /// This method is used to allow users check the liquidity of the given pool
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The liquidity of the pool.       
        pub fn check_liquidity(
            &mut self,
            token_requested: ResourceAddress
        ) -> Decimal
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { 
                    return lending_pool.check_liquidity(token_requested);
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                        return Decimal::zero()
                }
            }
        }

        
        /// Allows user to check the utilization rate of the pool.
        ///
        /// This method is used to allow users check the utilization rate of the pool. It is also
        /// used by the protocol to calculate the interest rate.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The utilization rate of the pool.
        pub fn check_utilization_rate(
            &mut self,
            token_requested: ResourceAddress
        ) -> Decimal
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { 
                    return lending_pool.check_utilization_rate();
                }
                None => {
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                        return Decimal::zero()
                }
            }
        }

        /// Allows user to check the total supplied to the pool.
        ///
        /// This method is used to allow users check the total supply of the pool.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The total supply of the pool.
        pub fn check_total_supplied(
            &mut self,
            token_requested: ResourceAddress
        ) -> Decimal
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => { 
                    return lending_pool.check_total_supplied();
                }
                None => {
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    return Decimal::zero()
                }
            }
        }

        /// Allows user to check the total collateral supplied to the pool.
        ///
        /// This method is used to allow users check the total collateral supplied to the pool.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The total collateral supply of the pool.
        pub fn check_total_collateral_supplied(
            &mut self, 
            token_requested: ResourceAddress
        ) -> Decimal
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_collateral_pool: Option<&CollateralPool> = self.collateral_pools.get(&token_requested);
            match optional_collateral_pool {
                Some (collateral_pool) => { 
                    return collateral_pool.check_total_collateral_supplied(token_requested);
                }
                None => {
                    info!("[Degenfi]: Collateral Pool for {:?} doesn't exist.", token_requested);
                    return Decimal::zero()
                }
            }
        }

        
        /// Allows user to check the total borrowed from the pool.
        ///
        /// This method is used to allow users check the total borrowed from the pool.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The total borrow of the pool.
        pub fn check_total_borrowed(
            &mut self,
            token_requested: ResourceAddress
        ) -> Decimal
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => {
                    return lending_pool.check_total_borrowed();
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                    return Decimal::zero()
                }
            }
        }

        /// Allows user to add to their credit score.
        ///
        /// This method is used to allow users add to their credit score for demonstration purpose.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// * `credit_score` (u64) - The credit score amount user wants to add.
        /// 
        /// # Returns:
        /// 
        /// This method does not return any assets.
        pub fn set_credit_score(
            &mut self,
            user_auth: Proof,
            credit_score: u64
        )
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");
            let user_id = self.get_user(&user_auth);
            let user_management: UserManagement = self.user_management_address.into();
            user_management.set_credit_score(user_id, credit_score);
        }

        /// Allows user to pull their SBT data.
        ///
        /// This method is used to allow users retrieve their SBT data. I suppose users cannot retrieve SBT data
        /// of other users yet.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `user_auth` (Proof) - A proof that proves that the depositer is a user that belongs to this protocol.
        /// 
        /// # Returns:
        /// 
        /// This method does not return any assets.
        pub fn get_sbt_info(
            &self,
            user_auth: Proof
        )
        {
            // Checks if user belongs to this protocol.
            assert_eq!(self.sbt_address.contains(&user_auth.resource_address()), true, "User does not belong to this protocol.");

            let user_id = self.get_user(&user_auth);
            let user_management: UserManagement = self.user_management_address.into();
            user_management.get_sbt_info(user_id);
        }

        /// Allows user to pull loan NFT data.
        ///
        /// This method is used to allow users retrieve any loan NFT data.
        /// 
        /// This method performs a number of checks before the information is pulled:
        /// 
        /// * **Check 1:** Checks that there does not already exist a lending pool for given token.
        /// 
        /// # Arguments:
        /// 
        /// * `token_requested` (ResourceAddress) - This is the token address of the requested asset.
        /// * `loan_id` (NonFungibleId) - The NFT ID of the loan wished to retrieve information on.
        /// 
        /// # Returns:
        /// 
        /// This method does not return any assets.
        pub fn get_loan_info(
            &self,
            token_requested: ResourceAddress,
            loan_id: NonFungibleId
        )
        {
            // Attempting to get the lending pool component associated with the provided address pair.
            let optional_lending_pool: Option<&LendingPool> = self.lending_pools.get(&token_requested);
            match optional_lending_pool {
                Some (lending_pool) => {
                    lending_pool.update_loan(loan_id.clone());
                    return lending_pool.get_loan_info(loan_id);
                }
                None => { 
                    info!("[DegenFi]: Pool for {:?} doesn't exist.", token_requested);
                }
            }
        }
    }
}