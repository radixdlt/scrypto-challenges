use scrypto::prelude::*;
use crate::utilities::*;

blueprint! {
    /// This is a struct used to define a lending pool in the MescaLend decentralized lending protocol. 
    /// A lending pool is typically comprised of a pair of tokens, each stored in its own vault.
    /// One of the tokens is lent and borrowed, and the other is put up as collateral. 
    /// A number of methods are implemented for this struct to make it easier to add and remove 
    /// liquidity into and from the pool, lend and borrow assets, and calculate the k value in the 
    /// 3-variable constant product market maker function `x * y * z = k`.
    struct LendingPool {
        /// These are the vaults where the Asset and Collateral token reserves will be stored. 
        vaults: HashMap<ResourceAddress, Vault>,

        /// When liquidity providers, lenders, and borrowers interact with the lending pool,
        /// they are given a number of native tokens, depending on what action they take. 
        /// Liquidity providers receive fungible Liquidity tokens and a non-fungible Collateralized Debt token. 
        /// Lenders receive fungible Bond Principal, Bond Interest, Insurance Principle, Insurance Interest tokens. 
        /// Borrowers receive a non-fungible Collateralized Debt token. 
        native_token_resource_addresses: HashMap<String, ResourceAddress>,
        
        /// The native tokens are mutable supply tokens that may be minted and burned when liquidity is added or removed, 
        /// and when users lend or borrow.  This badge is the badge that has the authority to mint and burn the native tokens
        /// when needed.
        native_token_admin_badge: Vault,

        // Each lending pool has a interest rate that is subject to change, depending on lending and borrowing. 
        // The interest rate determines how much interest lenders get and how much borrowers have to pay. 
        interest_rate: Decimal,

        // Each lending pool has a collateral asset ratio (collateral factor) that determines how much collateral 
        // is needed to borrow 1 unit of asset.
        collateral_asset_ratio: Decimal,

        // This is the unix time in which the lending pool matures. 
        maturity_time: Decimal
    }

    impl LendingPool {
        /// Creates a new lending pool of two token types passed to this function.
        /// 
        /// This method is used to instantiate a new lending pool of the two token types that were passed to this
        /// function in the two buckets. The asset token of the lending pool may be lent and borrowed while
        /// the collateral of the lending pool may be put up, and taken in case of default 
        /// (method for lenders to take collateral when loan defaults not yet implemented). 
        /// 
        /// This function does a number of checks before a Lending Pool is created, these checks are:
        /// 
        /// * **Check 1:** Checks that `asset` and `collateral` are not of the same type.
        /// * **Check 2:** Checks that both `asset` and `collateral` are fungible tokens.
        /// * **Check 3:** Checks that neither of the buckets are empty.
        /// 
        /// If these checks are successful, then a new lending pool is created from the two buckets passed to this 
        /// function and fungible liquidity tokens and a non-fungible collateralized debt token are minted for the 
        /// creator of this lending pool. 
        /// 
        /// # Arguments: 
        /// 
        /// * `asset` (Bucket) - A bucket containing the amount of the asset token used to initialize the pool.
        /// * `collateral` (Bucket) - A bucket containing the amount of the collateral token used to initialize the pool.
        /// * `interest_rate` (Decimal) - A decimal value of the interest rate that is used in the constant... formula
        /// to determine how much a lender receives in interest, and how much a borrower owes in interest.  
        /// * `collateral_factor` (Decimal) - A decimal value of the collateral factor that is used in the constant... formula
        /// to determine how much collateral factor a lender requires for their asset they supply to be borrowed, and how much
        /// collateral factor a borrower is required to put up to borrow assets. 
        /// * `maturity_time` (Decimal) - A decimal value representing a maturity date/time in unix. 
        /// 
        /// # Returns:
        /// 
        /// * `Component` - A LendingPool component of the newly created lending pool.
        /// * `Bucket` - A bucket containing a collateral debt token issued to the creator of the lending pool.
        /// * `Bucket` - A bucket containing liquidity tokens issued to the creator of the lending pool.
        pub fn new(
            asset: Bucket, collateral: Bucket, 
            interest_rate: Decimal,
            maturity_time: Decimal
        ) -> (ComponentAddress, Bucket, Bucket) {
            // Performing checks to see if this lending pool may be created or not.
            assert_ne!(
                asset.resource_address(), collateral.resource_address(),
                "[Pool Creation]: Lending pools may only be created between two different tokens."
            );

            assert_ne!(
                borrow_resource_manager!(asset.resource_address()).resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Asset must be fungible."
            );

            assert_ne!(
                borrow_resource_manager!(collateral.resource_address()).resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Collateral must be fungible."
            );

            assert!(
                !asset.is_empty() & !collateral.is_empty(),
                "[Pool Creation]: Can't create a lending pool from an empty bucket."
            );


            let lp_id: String = format!("{}-{}", asset.resource_address(), collateral.resource_address());
            let pair_symbols: String = address_pair_symbol(asset.resource_address(), collateral.resource_address());

            info!(
                "[Pool Creation]: Creating new lending pool between tokens: {}, of name: {}, Ratio: {}:{}", 
                lp_id, pair_symbols, asset.amount(), collateral.amount()
            );

            // Calculating the amount of debt the initial liquidity provider owes and the collateral asset ratio.
            let debt_amount: Decimal = asset.amount()+(asset.amount()*interest_rate/dec!("31556926")*maturity_time);
            let collateral_amount: Decimal = collateral.amount();
            let collateral_asset_ratio: Decimal = collateral.amount()/asset.amount();

            // Creating hashmap of vaults to insert asset and collateral reserves in. 
            let mut vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            vaults.insert(asset.resource_address(), Vault::with_bucket(asset));
            vaults.insert(collateral.resource_address(), Vault::with_bucket(collateral));


            // Creating an admin badge of the lending pool which will be given the authority to mint and burn the
            // native tokens issued to the liquidity providers, lenders, and borrowers.
            let native_token_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Bond Principal Token Admin Badge")
                .metadata("symbol", "BPTAB")
                .metadata("description", "This is an admin badge that has the authority to mint and burn bond principal tokens")
                .metadata("lp_id", format!("{}", lp_id))
                .initial_supply(dec!("1"));

            // Bond Principal Token
            let bond_principal_tokens: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Bond Principal Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("BND-PRI-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Bond Interest Token
            let bond_interest_tokens: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Bond Interest Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("BND-INT-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Insurance Principal Token
            let insurance_principal_tokens: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Insurance Principal Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("INS-PRI-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "A token used to track the interest of providers of the lending pool")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Insurance Interest Token
            let insurance_interest_tokens: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Insurance Interest Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("INS-INT-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Collateralized Debt NFT
            let collateralized_debt_token_resource_address = ResourceBuilder::new_non_fungible()
                .metadata("name", format!("{} Collateralized Debt Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("CDT-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "NFT representing how much debt is owed")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            // Minting a collateralized debt NFT for the initial liquidity provider. 
            let collateralized_debt_token = native_token_admin_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(collateralized_debt_token_resource_address);
                resource_manager.mint_non_fungible(
                    // The NFT id
                    &NonFungibleId::random(),
                    // The NFT data
                    CollateralizedDebtToken {
                        name: "Collateralized Debt NFT".to_string(),
                        debt: debt_amount,
                        collateral_amount: collateral_amount, 
                    },
                )
            });

            // Liquidity Token
            let liquidity_tokens: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata("name", format!("{} Liquidity Token - {}", pair_symbols, maturity_time))
                .metadata("symbol", format!("LIQ-{}-{}",pair_symbols,maturity_time))
                .metadata("description", "A token used to track the liquidity of providers of the lending pool")
                .metadata("lp_id", format!("{}", lp_id))
                .mintable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .burnable(rule!(require(native_token_admin_badge.resource_address())), LOCKED)
                .initial_supply(dec!("100"));

            // Inserting resource addresses of native tokens as values in a HashMap and Strings for each native token as keys. 
            let mut native_token_resource_addresses: HashMap<String, ResourceAddress> = HashMap::new();
            native_token_resource_addresses.insert("BP".to_string(),bond_principal_tokens);
            native_token_resource_addresses.insert("BI".to_string(),bond_interest_tokens);
            native_token_resource_addresses.insert("IP".to_string(),insurance_principal_tokens);
            native_token_resource_addresses.insert("II".to_string(),insurance_interest_tokens);
            native_token_resource_addresses.insert("CD".to_string(),collateralized_debt_token_resource_address);
            native_token_resource_addresses.insert("LQ".to_string(),liquidity_tokens.resource_address());

            // Creating the lending pool component and instantiating it
            let lending_pool: ComponentAddress = Self { 
                vaults: vaults,
                native_token_resource_addresses: native_token_resource_addresses,
                native_token_admin_badge: Vault::with_bucket(native_token_admin_badge),
                interest_rate: interest_rate,
                collateral_asset_ratio: collateral_asset_ratio,
                maturity_time: maturity_time
            }
            .instantiate()
            .globalize();

            return (lending_pool, collateralized_debt_token, liquidity_tokens);

        }

        pub fn belongs_to_pool(&self, address: ResourceAddress) -> bool {
            return self.vaults.contains_key(&address);
        }

        pub fn assert_belongs_to_pool(&self, address: ResourceAddress, label: String) {
            assert!(
                self.belongs_to_pool(address), 
                "[{}]: The provided resource address does not belong to the pool.", 
                label
            );
        }

        // Method that deposits a bucket of tokens to the lending pool
        pub fn deposit(&mut self, token: Bucket) {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs_to_pool(token.resource_address(), String::from("Deposit"));
            assert!(token.amount() > Decimal::zero(), "[Deposit]: Bucket must not be empty.");
            return self.vaults.get_mut(&token.resource_address()).unwrap().put(token);

        }

        // Method that withdraws a bucket of tokens to the lending pool
        pub fn withdraw(&mut self, resource_address: ResourceAddress, amount: Decimal) -> Bucket {
            // Checking if the passed resource address belongs to this pool.
            self.assert_belongs_to_pool(resource_address, String::from("Withdraw"));
            let vault: &mut Vault = self.vaults.get_mut(&resource_address).unwrap();
            assert!(vault.amount() >= amount,"[Withdraw]: Not enough liquidity available for the withdraw.");
            return vault.take(amount);

        }

        // Method that provides liquidity to the lending pool by adding asset and collateral tokens.
        pub fn add_liquidity(&mut self, mut asset: Bucket, mut collateral: Bucket, current_time: Decimal) -> (Bucket, Bucket, Bucket, Bucket) {
            // Checking if the asset and collateral tokens belong to this lending pool.
            self.assert_belongs_to_pool(asset.resource_address(), String::from("Add Liquidity"));
            self.assert_belongs_to_pool(collateral.resource_address(), String::from("Add Liquidity"));

            // Checking that the buckets passed are not empty
            assert!(!asset.is_empty(), "[Add Liquidity]: Can not add liquidity from an empty bucket");
            assert!(!collateral.is_empty(), "[Add Liquidity]: Can not add liquidity from an empty bucket");
            
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();

            let y = self.interest_rate;
            let z = self.collateral_asset_ratio;
            let input_collateral_asset_ratio = collateral.amount()/asset.amount();
            let seconds_to_maturity: Decimal = self.maturity_time - current_time;

            // Determine the maximum amount of asset and collateral tokens that may be added to the lending pool. 
            let (input_asset_amount, input_collateral_amount): (Decimal, Decimal) = if z > input_collateral_asset_ratio {
                (asset.amount(), asset.amount()*self.collateral_asset_ratio)
            } else { 
                (collateral.amount()/z, collateral.amount())
            };

            let dx = input_asset_amount.clone();

            let debt_amount: Decimal = dx*y/dec!("31556926")*seconds_to_maturity;

            // Minting Native Tokens - Liquidity tokens and Collateralized Debt NFT
            let liquidity_token_address = self.native_token_resource_addresses["LQ"];
            let liquidity_tokens_manager: &ResourceManager = borrow_resource_manager!(liquidity_token_address);
            
            let liquidity_amount: Decimal = if liquidity_tokens_manager.total_supply() == Decimal::zero() { 
                dec!("100.00") 
            } else {
                dx/(self.vaults[&addresses[0]].amount()+dx)*liquidity_tokens_manager.total_supply()
            };

            let liquidity_tokens: Bucket = self.native_token_admin_badge.authorize(|| {
                liquidity_tokens_manager.mint(liquidity_amount)
            });

            let collateralized_debt_token_resource_address = self.native_token_resource_addresses["CD"];
            let collateralized_debt_token = self.native_token_admin_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(collateralized_debt_token_resource_address);
                resource_manager.mint_non_fungible(
                    // The NFT id
                    &NonFungibleId::random(),
                    // The NFT data
                    CollateralizedDebtToken {
                        name: "Collateralized Debt NFT".to_string(),
                        debt: debt_amount,
                        collateral_amount: self.collateral_asset_ratio*debt_amount,
                    },
                )
            });

            // Depositing the amount of tokens calculated into the lending pool
            self.deposit(asset.take(input_asset_amount));
            self.deposit(collateral.take(input_collateral_amount));
            
            // Returning the remaining tokens from `assset`, `collateral`, the liquidity tokens, and collateralized debt token
            return (asset, collateral, liquidity_tokens, collateralized_debt_token);
        }


        // Method that removes liquidity from the lending pool by subtracting asset and collateral tokens.
        pub fn remove_liquidity(&mut self,liquidity_tokens: Bucket, collateralized_debt_token: Bucket) -> (Bucket, Bucket, Bucket) {
            // Checking the resource address of the collateralized debt NFT passed to ensure that it belongs to this lending pool.
            assert_eq!(
                collateralized_debt_token.resource_address(),
                self.native_token_resource_addresses["CD"],
                "Collateralized Debt Token is from this lending pool"

            );

            let liquidity_token_address: ResourceAddress = self.native_token_resource_addresses["LQ"];

            assert_eq!(liquidity_tokens.resource_address(), liquidity_token_address,
                "[Remove Liquidity]: The tracking tokens given do not belong to this liquidity pool."
            );

            // Calculating the percentage ownership that the liquidity tokens amount corresponds to
            let liquidity_tokens_manager: &ResourceManager = borrow_resource_manager!(liquidity_token_address);

            let percentage: Decimal = liquidity_tokens.amount() / liquidity_tokens_manager.total_supply();

            // Burning the liquidity tokens
            self.native_token_admin_badge.authorize(|| {
                liquidity_tokens.burn();
            });

            // Withdrawing the amount of tokens owed to this liquidity provider
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();
            let asset: Bucket = self.withdraw(addresses[0], self.vaults[&addresses[0]].amount() * percentage);
            let collateral: Bucket = self.withdraw(addresses[1], self.vaults[&addresses[1]].amount() * percentage);

            // Changing the data of the passed collateralized debt NFT to reflect the change in debt
            let mut non_fungible_data: CollateralizedDebtToken = collateralized_debt_token.non_fungible().data();
            non_fungible_data.debt -= asset.amount();
            self.native_token_admin_badge.authorize(|| collateralized_debt_token.non_fungible().update_data(non_fungible_data));

            return (asset, collateral, collateralized_debt_token);
        }


        pub fn asset_collateral_addresses(&self) -> Vec<ResourceAddress> {
            return self.vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
        }

        // Calculate variable k of the constant product automated market maker function.
        pub fn k(&self) -> Decimal {
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();
            let x = self.vaults[&addresses[0]].amount();
            let y = self.interest_rate;
            let z = self.collateral_asset_ratio;
            return x*y*z;
        }

        // Calculate the maximum of variables y and z of the constant product automated market maker function
        // for a borrow action. 
        pub fn calculate_borrow_ymax_zmax(&self, asset_amount: Decimal) -> (Decimal, Decimal) {
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();
            let dx = asset_amount;
            let x: Decimal = self.vaults[&addresses[0]].amount();
            let y: Decimal = self.interest_rate;
            let z: Decimal = self.collateral_asset_ratio;
            let k: Decimal = self.k();

            let zmax = k/((x-dx)*(y));
            let ymax = k/((x-dx)*(z));

            return (ymax, zmax)
        }

        // Calculate the minimum of variables y and z of the constant product automated market maker function
        // for a lend action. 
        pub fn calculate_lend_ymin_zmin(&self, asset_amount: Decimal) -> (Decimal, Decimal) {
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();
            let dx = asset_amount;
            let x: Decimal = self.vaults[&addresses[0]].amount();
            let y: Decimal = self.interest_rate;
            let z: Decimal = self.collateral_asset_ratio;
            let k: Decimal = self.k();

            let dz_max = z - k/((x+dx)*(y));
            let dy_max = y - k/((x+dx)*(z));

            let zmin = z-dz_max;
            let ymin = y-dy_max;

            return (ymin, zmin)
        }

         // Method that borrows a bucket of asset tokens from the lending pool. The lending pool distributes
         // a Collateralized Debt Token indicating how much debt he owes when the loan matures. 
        pub fn borrow(&mut self, borrow_amount: Decimal, collateral: Bucket, current_time: Decimal) -> (Bucket, Bucket) {
            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();


            let dx: Decimal = borrow_amount;
            let dz: Decimal = (collateral.amount()/borrow_amount)-self.collateral_asset_ratio; // change in collateral_asset_ratio
            let x: Decimal = self.vaults[&addresses[0]].amount();
            let y: Decimal = self.interest_rate;
            let z: Decimal = self.collateral_asset_ratio;
            let k: Decimal = self.k();

            // assert dz is positive or 0.
            assert!(
                dz >= dec!("0"), 
                "The minimum collateral_asset ratio for a borrow must be whatever it was before, 
                therefore change in the collateral asset ratio must be either positive or 0."
            );
            
            let (ymax, zmax) = self.calculate_borrow_ymax_zmax(dx);

            assert!(
                z+dz <= zmax, 
                "The collateral asset ratio for a borrow must be equal to or below zmax"
            );

            let dy: Decimal = (k/((x-dx)*z)) - y;
            
            let interest_rate_change: Decimal = dy;

            info!(
                "x: {}, y: {}, z: {}, k: {}, dx: {}, dy: {}, dz: {}, interest_rate_change: {}", 
                x, y, z, k, dx, dy, dz, interest_rate_change
            );

            // Change lending pool's interest rate
            self.interest_rate += interest_rate_change;

            // Change lending pool's collateral asset ratio
            self.collateral_asset_ratio += dz;

            let seconds_to_maturity: Decimal = self.maturity_time - current_time;
            let interest: Decimal = dx*(y+interest_rate_change)/dec!("31556926")*seconds_to_maturity;

            // These are the bucket of borrowed asset tokens
            let borrowed_tokens: Bucket = self.withdraw(self.vaults[&addresses[0]].resource_address(),borrow_amount);

            let collateralized_debt_token_resource_address = self.native_token_resource_addresses["CD"];
            let collateralized_debt_token = self.native_token_admin_badge.authorize(|| {
                let resource_manager = borrow_resource_manager!(collateralized_debt_token_resource_address);
                resource_manager.mint_non_fungible(
                    // The NFT id
                    &NonFungibleId::random(),
                    // The NFT data
                    CollateralizedDebtToken {
                        name: "Collateralized Debt NFT".to_string(),
                        debt: dx+interest,
                        collateral_amount: (dx+interest)*(z+dz),
                    },
                )
            });

            return (borrowed_tokens, collateralized_debt_token);
        }

        // Method that pays back a certain amount of asset tokens and updates the data of a passed collateralized debt NFT.
        pub fn payback_debt(&mut self, asset: Bucket, collateralized_debt_token: Bucket) -> Bucket {

            let addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();
            
            let non_fungible_data: CollateralizedDebtToken = collateralized_debt_token.non_fungible().data();
            let total_debt = non_fungible_data.debt;

            let payback_amount = if asset.amount() < total_debt {
                asset.amount()
            } else if asset.amount() > total_debt {
                total_debt
            } else {
                asset.amount()
            };
            
            let payback_percentage = payback_amount/total_debt;
            let collateral_back = non_fungible_data.collateral_amount*payback_percentage;

            let mut non_fungible_data: CollateralizedDebtToken = collateralized_debt_token.non_fungible().data();
            non_fungible_data.debt -= asset.amount();
            non_fungible_data.collateral_amount -= collateral_back;
            self.native_token_admin_badge.authorize(|| collateralized_debt_token.non_fungible().update_data(non_fungible_data));

            //Pay back the asset
            self.deposit(asset);

            let returned_collateral = self.withdraw(addresses[1], collateral_back);

            return returned_collateral;

        }

        // Method that lends a bucket of asset tokens to the lending pool. The lending pool distributes
         // Bond Principle, Bond Interest, Insurance Principal and Insurance Interest tokens to the lender.
        pub fn lend(&mut self, lent_tokens: Bucket,interest_rate_change: Decimal, current_time: Decimal) -> (Bucket, Bucket, Bucket, Bucket) {
            let asset_collateral_addresses: Vec<ResourceAddress> = self.asset_collateral_addresses();

            let dx: Decimal = lent_tokens.amount();
            let dy: Decimal = interest_rate_change;
            let x: Decimal = self.vaults[&asset_collateral_addresses[0]].amount();
            let y: Decimal = self.interest_rate;
            let z: Decimal = self.collateral_asset_ratio;
            let k: Decimal = self.k();

            let dz: Decimal = (k/((x+dx)*(y-dy))) + z; 
            
            let (ymin, zmin) = self.calculate_lend_ymin_zmin(dx);

            let interest_rate_change: Decimal = dy;

            info!(
                "x: {}, y: {}, z: {}, k: {}, dx: {}, dy: {}, dz: {}, interest rate change: {}", 
                x, y, z, k, dx, dy, dz, interest_rate_change
            );

            // Change lending pool's interest rate
            self.interest_rate -= interest_rate_change;

            // Change lending pool's collateral asset ratio
            self.collateral_asset_ratio -= dz;

            let lent_tokens_amount = lent_tokens.amount().clone();
            self.deposit(lent_tokens);

            // Retrieve native token resource addresses for lenders
            let bond_principal_token_address = self.native_token_resource_addresses["BP"];
            let bond_interest_token_address = self.native_token_resource_addresses["BI"];
            let insurance_principal_token_address = self.native_token_resource_addresses["IP"];
            let insurance_interest_token_address = self.native_token_resource_addresses["II"];
            
            // Mint Bond Principal Tokens
            let bond_principal_tokens_manager: &ResourceManager = borrow_resource_manager!(bond_principal_token_address);
            let bond_principal_tokens: Bucket = self.native_token_admin_badge.authorize(|| {
                bond_principal_tokens_manager.mint(lent_tokens_amount)
             });
            info!("[Lent]: {} bond principal tokens minted", lent_tokens_amount);

            // Mint Bond Interest Tokens
            let seconds_to_maturity: Decimal = self.maturity_time - current_time;
            let interest: Decimal = dx*(y-interest_rate_change)/dec!("31556926")*seconds_to_maturity;

            let bond_interest_tokens_manager: &ResourceManager = borrow_resource_manager!(bond_interest_token_address);
            let bond_interest_tokens: Bucket = self.native_token_admin_badge.authorize(|| {
                bond_interest_tokens_manager.mint(interest)
             });
            info!("[Lent]: {} bond interest tokens minted", interest);

            // Mint Insurance Principal Tokens
            let total_asset_amount: Decimal = self.vaults[&asset_collateral_addresses[0]].amount();
            let total_collateral_amount: Decimal = self.vaults[&asset_collateral_addresses[1]].amount();
            let ratio: Decimal = total_collateral_amount/total_asset_amount;
            let collateral_amount: Decimal = lent_tokens_amount*ratio;
            
            let insurance_principal_tokens_manager: &ResourceManager = borrow_resource_manager!(insurance_principal_token_address);
            let insurance_principal_tokens: Bucket = self.native_token_admin_badge.authorize(|| {
                insurance_principal_tokens_manager.mint(collateral_amount)
                });
            info!("[Lent]: {} bond interest tokens minted", interest);

            // Mint Insurance Interest Tokens
            let insurance_interest: Decimal = collateral_amount*(y-dy)/dec!("31556926")*seconds_to_maturity;

            let insurance_interest_tokens_manager: &ResourceManager = borrow_resource_manager!(insurance_interest_token_address);
            let insurance_interest_tokens: Bucket = self.native_token_admin_badge.authorize(|| {
                insurance_interest_tokens_manager.mint(insurance_interest)
                });
            info!("[Lent]: {} bond interest tokens minted", interest);

            return (bond_principal_tokens, bond_interest_tokens, insurance_principal_tokens, insurance_interest_tokens);
        }

        
    }

}

#[derive(NonFungibleData)]
struct CollateralizedDebtToken {
    name: String,
    debt: Decimal,
    collateral_amount: Decimal,
}