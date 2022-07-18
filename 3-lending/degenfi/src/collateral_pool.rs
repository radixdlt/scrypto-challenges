
use scrypto::prelude::*;
use crate::user_management::*;
use crate::lending_pool::*;
use crate::structs::{User, Loan, Status};

blueprint! {
    /// The collateral pool is where collateral deposits are locked. It essentially mimicks a lot of the vault 
    /// manipulation (i.e deposit, withdraw, etc.) from the lending pool component. I've been debating whether 
    /// the collateral could just be integrated with the lending pool component as some method calls here became 
    /// somewhat complex, such as the liquidation method. Due to the multiple pool design, the liquidation requires 
    /// visibility of other lending pools that it may need to route repayments to. 
    struct CollateralPool {
        // Vault for lending pool
        collateral_vaults: HashMap<ResourceAddress, Vault>,
        user_management: ComponentAddress,
        access_badge_vault: Vault,
        lending_pool: ComponentAddress,
        close_factor: Decimal,
    }

    impl CollateralPool {
        pub fn new(
            user_component_address: ComponentAddress,
            lending_pool_address: ComponentAddress,
            token_address: ResourceAddress,
            access_badge: Bucket
        ) -> ComponentAddress 
        {
            let access_rules: AccessRules = AccessRules::new()
            .method("convert_from_deposit", rule!(require(access_badge.resource_address())))
            .method("convert_to_deposit", rule!(require(access_badge.resource_address())))
            .method("redeem", rule!(require(access_badge.resource_address())))
            .method("withdraw_vault", rule!(require(access_badge.resource_address())))
            .method("liquidate", rule!(require(access_badge.resource_address())))
            .default(rule!(allow_all));

            assert_ne!(
                borrow_resource_manager!(token_address).resource_type(), ResourceType::NonFungible,
                "[Pool Creation]: Asset must be fungible."
            );

            let user_management_address: ComponentAddress = user_component_address;
            let lending_pool_address: ComponentAddress = lending_pool_address;

            //Inserting pool info into HashMap
            let mut collateral_vaults: HashMap<ResourceAddress, Vault> = HashMap::new();
            collateral_vaults.insert(token_address, Vault::new(token_address));

            //Instantiate lending pool component
            let collateral_pool: ComponentAddress = Self {
                collateral_vaults: collateral_vaults,
                user_management: user_management_address,
                access_badge_vault: Vault::with_bucket(access_badge),
                lending_pool: lending_pool_address,
                close_factor: dec!("0.5"),
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize();
            return collateral_pool
        }

        // This method is also being used in the lending pool component as a convertion from deposit supply to collateral supply
        // Is it important to distinguish between regular supply and conversions?
        pub fn deposit(
            &mut self,
            user_id: NonFungibleId,
            token_address: ResourceAddress,
            deposit_amount: Bucket
        ) 
        {
            assert_eq!(token_address, deposit_amount.resource_address(), "Tokens must be the same.");
            
            let user_management: UserManagement = self.user_management.into();

            let dec_deposit_amount = deposit_amount.amount();

            self.access_badge_vault.authorize(|| {user_management.add_collateral_balance(user_id, token_address, dec_deposit_amount);});

            // Deposits collateral into the vault
            self.collateral_vaults.get_mut(&deposit_amount.resource_address()).unwrap().put(deposit_amount);
        }

        pub fn deposit_additional(
            &mut self,
            user_id: NonFungibleId,
            loan_id: NonFungibleId,
            token_address: ResourceAddress,
            deposit_amount: Bucket
        ) 
        {
            assert_eq!(token_address, deposit_amount.resource_address(), "Tokens must be the same.");
            
            let user_management: UserManagement = self.user_management.into();
            
            let lending_pool: LendingPool = self.lending_pool.into();

            // Finds the loan NFT
            let loan_nft_resource = lending_pool.loan_nft();
            let resource_manager = borrow_resource_manager!(loan_nft_resource);
            let mut loan_nft_data: Loan = resource_manager.get_non_fungible_data(&loan_id);

            let dec_deposit_amount = deposit_amount.amount();

            // Updates the states
            self.access_badge_vault.authorize(|| {user_management.add_collateral_balance(user_id, token_address, dec_deposit_amount);});
            loan_nft_data.collateral_amount += dec_deposit_amount;

            // Deposits collateral into the vault
            self.collateral_vaults.get_mut(&deposit_amount.resource_address()).unwrap().put(deposit_amount);
        }

        /// Converts the user's supply deposit to collateral.
        ///
        /// This method converts the user's supply deposit to collateral deposit. It first checks whether the requested token to
        /// convert belongs to this pool. Takes the SBT data to view whether the user has deposits to convert to collateral.
        /// It performs another check to ensure the requested conversion is enough. The lending protocol then moves fund to the collateral
        /// component to be locked up.
        /// 
        /// This method performs a number of checks before the borrow is made:
        /// 
        /// * **Check 1:** Checks whether the resquested token to convert belongs to this lending pool.
        /// * **Check 2:** Checks whether the user has enough deposit supply to convert to collateral.
        /// 
        /// # Arguments:
        /// 
        /// * `user_id` (NonFungibleId) - The NonFungibleId that identifies the specific NFT which represents the user. It is used 
        /// to update the data of the NFT.
        /// * `token_address` (ResourceAddress) - This is the token address of the requested collateral to be converted back to supply.
        /// * `collateral_amount` (Bucket) - The bucket with the amount of collateral supply to be deposited.
        /// 
        /// # Returns:
        /// 
        /// * `None` - Nothing is returned.
        pub fn convert_from_deposit(
            &mut self,
            user_id: NonFungibleId,
            token_address: ResourceAddress,
            collateral_amount: Bucket
        ) 
        {
            assert_eq!(token_address, collateral_amount.resource_address(), "Tokens must be the same.");
            
            let user_management: UserManagement = self.user_management.into();

            let dec_collateral_amount = collateral_amount.amount();

            self.access_badge_vault.authorize(|| {user_management.convert_deposit_to_collateral(user_id, token_address, dec_collateral_amount)});
            // Deposits collateral into the vault
            self.collateral_vaults.get_mut(&collateral_amount.resource_address()).unwrap().put(collateral_amount);
        }

        /// Gets the resource addresses of the tokens in this liquidity pool and returns them as a `Vec<ResourceAddress>`.
        /// 
        /// # Returns:
        /// 
        /// `Vec<ResourceAddress>` - A vector of the resource addresses of the tokens in this liquidity pool.
        pub fn addresses(
            &self
        ) -> Vec<ResourceAddress> 
        {
            return self.collateral_vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
        }

        /// Checks if the given address belongs to this pool or not.
        /// 
        /// This method is used to check if a given resource address belongs to the token in this lending pool
        /// or not. A resource belongs to a lending pool if its address is in the addresses in the `vaults` HashMap.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (ResourceAddress) - The address of the resource that we wish to check if it belongs to the pool.
        /// 
        /// # Returns:
        /// 
        /// * `bool` - A boolean of whether the address belongs to this pool or not.
        pub fn belongs_to_pool(
            &self, 
            address: ResourceAddress
        ) -> bool
        {
            return self.collateral_vaults.contains_key(&address);
        }

        /// Asserts that the given address belongs to the pool.
        /// 
        /// This is a quick assert method that checks if a given address belongs to the pool or not. If the address does
        /// not belong to the pool, then an assertion error (panic) occurs and the message given is outputted.
        /// 
        /// # Arguments:
        /// 
        /// * `address` (ResourceAddress) - The address of the resource that we wish to check if it belongs to the pool.
        /// * `label` (String) - The label of the method that called this assert method. As an example, if the swap 
        /// method were to call this method, then the label would be `Swap` so that it's clear where the assertion error
        pub fn assert_belongs_to_pool(
            &self, 
            address: ResourceAddress, 
            label: String
        ) 
        {
            assert!(
                self.belongs_to_pool(address), 
                "[{}]: The provided resource address does not belong to the pool.", 
                label
            );
        }

        /// Withdraws tokens from the collateral pool.
        /// 
        /// This method is used to withdraw a specific amount of tokens from the lending pool. 
        /// 
        /// This method performs a number of checks before the withdraw is made:
        /// 
        /// * **Check 1:** Checks that the resource address given does indeed belong to this lending pool.
        /// * **Check 2:** Checks that the there is enough liquidity to perform the withdraw.
        /// 
        /// # Arguments:
        /// 
        /// * `resource_address` (ResourceAddress) - The address of the resource to withdraw from the liquidity pool.
        /// * `amount` (Decimal) - The amount of tokens to withdraw from the liquidity pool.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A bucket of the withdrawn tokens.
        fn withdraw(
            &mut self,
            resource_address: ResourceAddress,
            amount: Decimal
        ) -> Bucket 
        {
            // Performing the checks to ensure tha the withdraw can actually go through
            self.assert_belongs_to_pool(resource_address, String::from("Withdraw"));
            
            // Getting the vault of that resource and checking if there is enough liquidity to perform the withdraw.
            let vault: &mut Vault = self.collateral_vaults.get_mut(&resource_address).unwrap();
            assert!(
                vault.amount() >= amount,
                "[Withdraw]: Not enough liquidity available for the withdraw. The liquidity is {:?}", vault.amount()
            );
            

            return vault.take(amount);
        }

        /// Converts the collateral back to supply deposit.
        /// 
        /// This method is used in the event that the user may change their mind of using their deposit supply as collateral 
        /// (which will become locked/illiquid) or if the loan has been paid off with the remaining collateral to be used
        /// as supply liquidity and earn rewards. This method is called first from the router component which is routed
        /// to the correct collateral pool.
        /// 
        /// This method does not perform any checks, but Access Rules are enforced and ultimately only callable by the DegenFi component.
        /// 
        /// # Arguments:
        /// 
        /// * `user_id` (NonFungibleId) - The NonFungibleId that identifies the specific NFT which represents the user. It is used 
        /// to update the data of the NFT.
        /// * `token_address` (ResourceAddress) - This is the token address of the requested collateral to be converted back to supply.
        /// * `deposit_amount` (Decimal) - This is the amount of the deposit supply.
        /// 
        /// # Returns:
        /// 
        /// * `None` - Nothing is returned.

        pub fn convert_to_deposit(
            &mut self, 
            user_id: NonFungibleId, 
            token_address: ResourceAddress, 
            deposit_amount: Decimal
        ) 
        {
            // Check if the NFT belongs to this lending protocol.
            let user_management: UserManagement = self.user_management.into();

            // Gets the user badge ResourceAddress
            let nft_resource = user_management.get_sbt();
            let resource_manager = borrow_resource_manager!(nft_resource);
            let nft_data: User = resource_manager.get_non_fungible_data(&user_id);
            let user_loans = nft_data.open_loans.iter();

            {
                // Looping through loans in the User SBT
                for (_token_address, loans) in user_loans {
                    let lending_pool: LendingPool = self.lending_pool.into();
                    let loan_resource = lending_pool.loan_nft();
                    let resource_manager = borrow_resource_manager!(loan_resource);
                    // Retrieve loan data for every loans in the User SBT
                    let loan_data: Loan = resource_manager.get_non_fungible_data(&loans);
                    let loan_status = loan_data.loan_status;
                    match loan_status {
                        Status::Current => assert!(loan_status != Status::Current, "Cannot have outstanding loans"),
                        _ => break,
                    }
                }
            }

            // Check if the user has enough collateral supply to convert to deposit supply
            assert!(*nft_data.collateral_balance.get(&token_address).unwrap() >= deposit_amount, "Must have enough deposit supply to use as a collateral");

            // Withdrawing the amount of tokens owed to this lender
            let addresses: Vec<ResourceAddress> = self.addresses();
            let bucket: Bucket = self.withdraw(addresses[0], deposit_amount);
            let lending_pool: LendingPool = self.lending_pool.into();
            self.access_badge_vault.authorize(|| 
                lending_pool.convert_from_collateral(user_id, token_address, bucket));
        }

        /// Redeems collateral of the borrower.
        /// 
        /// This method performs a number of checks before liquidity removed from the pool:
        /// 
        /// * **Check 1:** Checks to ensure that there are no loans outstanding using this asset as collateral.
        /// 
        /// # Arguments:
        /// 
        /// * `user_id` (NonFungibleId) - The NonFungibleId that identifies the specific NFT which represents the user. It is used 
        /// to update the data of the NFT.
        /// * `collateral_address` (ResourceAddress) - This is the collateral resource address of the requested asset to be redeemed.
        /// exchange for their share of the liquidity.
        /// * `redeem_amount` (Decimal) - This is the amount requested to redeem.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - A Bucket of the collateral asset redeemed.
        pub fn redeem(
            &mut self, 
            user_id: NonFungibleId, 
            collateral_address: ResourceAddress, 
            redeem_amount: Decimal
        ) -> Bucket 
        {
            // Check if the NFT belongs to this lending protocol.
            let user_management: UserManagement = self.user_management.into();
            let sbt_resource = user_management.get_sbt();
            let resource_manager = borrow_resource_manager!(sbt_resource);
            let sbt_data: User = resource_manager.get_non_fungible_data(&user_id);
            let user_loans = sbt_data.open_loans.keys();

            let lending_pool: LendingPool = self.lending_pool.into();
            let loan_resource_address = lending_pool.loan_nft();

            for address in user_loans {
                let loans = sbt_data.open_loans.get(address).unwrap();
                let resource_manager = borrow_resource_manager!(loan_resource_address);
                let loan_data: Loan = resource_manager.get_non_fungible_data(loans);
                let loan_collateral_address = loan_data.collateral;
                assert_eq!(loan_collateral_address, collateral_address, "Collateral address in the loan are not the same.");
                let loan_status = loan_data.loan_status;
                assert_ne!(loan_status, Status::Current, "Must pay off loans before redeeming.");
            }

            // Reduce collateral balance of the user
            self.access_badge_vault.authorize(|| 
                user_management.decrease_collateral_balance(user_id, collateral_address, redeem_amount)
            );

            // Withdrawing the amount of tokens owed to this lender
            let addresses: Vec<ResourceAddress> = self.addresses();
            let bucket: Bucket = self.withdraw(addresses[0], redeem_amount);
            return bucket;
        }

        pub fn liquidate(
            &mut self,
            loan_id: NonFungibleId,
            loan_resource_address: ResourceAddress,
            collateral_address: ResourceAddress,
            lending_pool: LendingPool,
            repay_amount: Bucket
        ) -> Bucket 
        {

            // Retrieve resource manager.
            let resource_manager = borrow_resource_manager!(loan_resource_address);
            // Retrieves loan NFT data.
            let mut loan_data: Loan = resource_manager.get_non_fungible_data(&loan_id);

            // Retrieve asset address.
            let repayment_address = loan_data.asset;

            // Asserts that the resource passed in must be the same as the collateral address. 
            assert_eq!(repayment_address, repay_amount.resource_address(), "Must pass the same resource.");

            // Retrieves health factor of the loan.
            let health_factor = loan_data.health_factor;

            let max_repay: Decimal = if health_factor >= self.close_factor {
                dec!("0.5")
            } else {
                dec!("1.0")
            };

            // Calculate amount returned
            assert!(repay_amount.amount() <= loan_data.remaining_balance * max_repay, "Max repay amount exceeded.");

            // Calculate owed to liquidator (amount paid + liquidation bonus fee of 5%)
            let amount_to_liquidator = repay_amount.amount() + (repay_amount.amount() * dec!("0.05"));

            let addresses: Vec<ResourceAddress> = self.addresses();
            let claim_liquidation: Bucket = self.withdraw(addresses[0], amount_to_liquidator);
            
            // Update loan
            loan_data.collateral_amount -= claim_liquidation.amount();
            loan_data.remaining_balance -= repay_amount.amount();
            //let new_collateral_amount = loan_data.collateral_amount;
            //let remaining_balance = loan_data.remaining_balance;
            //let health_factor = ( ( new_collateral_amount * self.xrd_usd ) * dec!("0.8") ) / remaining_balance;
            //loan_data.health_factor = health_factor;
            loan_data.loan_status = Status::Defaulted;

            self.access_badge_vault.authorize(|| resource_manager.update_non_fungible_data(&loan_id, loan_data));

            // Retrieve resource manager
            let loan_data: Loan = resource_manager.get_non_fungible_data(&loan_id);
            
            // Retrieve owner of the loan
            let user_id = loan_data.owner;

            // Update User State to record default amount
            let user_management: UserManagement = self.user_management.into();
            self.access_badge_vault.authorize(|| 
                user_management.inc_default(user_id.clone())
            );

            self.access_badge_vault.authorize(|| 
                user_management.decrease_borrow_balance(user_id.clone(), repayment_address, repay_amount.amount())
            );

            self.access_badge_vault.authorize(|| 
                user_management.decrease_collateral_balance(user_id.clone(), collateral_address, amount_to_liquidator)
            );

            let credit_score_decrease = 80;
            // Update User State to decrease credit score
            self.access_badge_vault.authorize(|| 
                user_management.dec_credit_score(user_id.clone(), credit_score_decrease)
            );

            // Update user collateral balance
            self.access_badge_vault.authorize(|| 
                user_management.decrease_collateral_balance(user_id.clone(), collateral_address, claim_liquidation.amount())
            );

            // Sends the repay amount to the lending pool
            lending_pool.repayment_deposit(repay_amount);

            return claim_liquidation
        }

        /// Allows user to check the total collateral supplied to the pool.
        ///
        /// This method is used to allow users check the total supply of the pool.
        /// 
        /// This method does not perform any checks.
        /// 
        /// # Arguments:
        /// 
        /// * `collateral_address` (ResourceAddress) - The requested collateral resource.
        /// 
        /// # Returns:
        /// 
        /// * `Decimal` - The total collateral supply of the pool.
        pub fn check_total_collateral_supplied(
            &self, 
            collateral_address: ResourceAddress
        ) -> Decimal 
        {
            let vault = self.collateral_vaults.get(&collateral_address).unwrap();
            info!("The total collateral supplied in this pool is {:?}", vault.amount());
            return vault.amount()
        }
    }
}

