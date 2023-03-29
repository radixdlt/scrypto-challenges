use scrypto::prelude::*;
use crate::user_management::*;
use crate::price_oracle::*;
use crate::structs::*;

#[blueprint]
mod scrilla_module {

    struct Scrilla {

        admin_badge: ResourceAddress,  // given to founder of lootbox component
        auth_badge: ResourceAddress, // stored in component, allows for access
        auth_badge_vault: Vault, // component vault that holds the admin mint badge

        scrl: ResourceAddress, // Scrilla token
        usds: ResourceAddress, // USDS stablecoin pegged to USD and redeemable for XRD
        usds_shield_pool: Vault, // Pool that stores USDS to absorb liquidations and is rewarded in XRD
        xrd_shield_reward_pool: Vault, // Pool that stores the XRD rewards for USDS liquidations
        xrd_fee_vault: Vault, // Vault to store platform fees
        xrd_collateral_pool: Vault, // Vault that stores all collateral
        scrl_stake_pool: Vault, // Vault that stores scrl tokens for staking
        scrl_summation: Decimal, // S: S = S + (E/D) holds running total of summation

        event_number: u128, // Index of deposit events
        product: Decimal, // P = P(1-(Q/D)) holds running total of product
        summation: Decimal, // S: S = S + (E/D)P holds running total of summation
        
        // Key = XRD price to liquidate, value = NFT ID that needs to be updated to liquidate
        liquidation_book: HashMap<NonFungibleLocalId, Decimal>,
        shield_depositors: Vec<NonFungibleLocalId>,
        last_xrd_price: Decimal, // stores most recent price of XRD from oracle

        current_usds_issued: Decimal, // stores an amount of USDS currently circulating
        collateral_deposit_fee: Decimal, // platform fee that could be updated in future
        collateral_withdrawal_fee: Decimal, // platform fee that could be updated in future

        user_management_address: ComponentAddress, // User Management component address
        price_oracle_address: ComponentAddress, // Price oracle component address

        platform_ratio: Decimal, // collateralization ratio of entire platform
        
        // Operational mode that can be switched from Normal to Recovery mode depending on
        // the total platform ratio
        platform_mode: OperationalMode, 
    }

    impl Scrilla {

        pub fn instantiate_scrilla_module() -> (ComponentAddress, Bucket) {
            
            // stored in component, allows for access, authentication, token minting, etc
            let auth_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)    
                .metadata("name", "Auth Badge")
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .mint_initial_supply(1);
            let auth_badge_address = auth_badge.resource_address();
            
            // Create an admin badge for instantiator of component
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)    
                .mintable(rule!(require(auth_badge.resource_address())), LOCKED)
                .burnable(rule!(allow_all), LOCKED)
                .metadata("name", "Scrilla Admin")
                .metadata("symbol", "SA")
                .mint_initial_supply(1);

            // USDS stablecoin pegged to USD and redeemable for XRD
            let usds: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(18)    
                .mintable(rule!(require(auth_badge.resource_address())), LOCKED)
                .burnable(rule!(require(auth_badge.resource_address())), LOCKED)
                .metadata("name", "USD-Scrilla")
                .metadata("symbol", "USDS")
                .create_with_no_initial_supply();

            // Scrilla token
            let scrl: ResourceAddress = ResourceBuilder::new_fungible()
                .divisibility(18)    
                .mintable(rule!(require(auth_badge.resource_address())), LOCKED)
                .burnable(rule!(require(auth_badge.resource_address())), LOCKED)
                .metadata("name", "Scrilla Token")
                .metadata("symbol", "SCRL")
                .create_with_no_initial_supply();

            let access_rules: AccessRules = AccessRules::new() // require admin badge for protected methods
                // .method("add_fungible_loot", rule!(require(admin_badge.resource_address())), AccessRule::DenyAll)
                .default(rule!(allow_all), AccessRule::DenyAll);
            
            let mut scrilla_component = Self {
                admin_badge: admin_badge.resource_address(),
                auth_badge: auth_badge.resource_address(),
                auth_badge_vault: Vault::with_bucket(auth_badge),
                usds,
                scrl,
                scrl_summation: dec!(0),
                usds_shield_pool: Vault::new(usds),
                xrd_collateral_pool: Vault::new(RADIX_TOKEN),
                xrd_shield_reward_pool: Vault::new(RADIX_TOKEN), 
                scrl_stake_pool: Vault::new(scrl),
                event_number: 1,
                product: dec!(1),
                summation: dec!(0),
                liquidation_book: HashMap::new(),
                shield_depositors: Vec::new(),
                last_xrd_price: dec!("0.05"),
                current_usds_issued: dec!(0),
                collateral_deposit_fee: dec!("0.005"),
                collateral_withdrawal_fee: dec!("0.005"),
                user_management_address: UserManagementComponent::new_usermanagement_module(auth_badge_address),
                price_oracle_address: PriceOracleComponent::new(),
                platform_ratio: dec!(0),
                platform_mode: OperationalMode::Normal,
                xrd_fee_vault: Vault::new(RADIX_TOKEN),
            }

            .instantiate();

            scrilla_component.add_access_check(access_rules); // require admin badge for protected methods

            let scrilla_component_add: ComponentAddress = scrilla_component.globalize();

            (scrilla_component_add, admin_badge) // returns lootbox_admin badge to founder of LootBox
        }

        // Creates a new user for the lending protocol.
        // Returns a new user NFT
        pub fn new_user(&mut self) -> Bucket {
            
            // Retrieves User Management component.
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();

            // Makes authorized method call to create a new user for the protocol and returns a user NFT
            let new_user: Bucket = self.auth_badge_vault.authorize(|| user_management.new_user());

            new_user
        }
        
        /// Method for user to add XRD to collateral pool in order to borrow USDS against its value.
        /// Fee is taken out of collateral deposit.
        /// This method does not return anything.
        pub fn add_xrd_to_collateral(&mut self, mut xrd_collateral: Bucket, user_proof: Proof) {

            // Make sure bucket contains xrd
            assert_eq!(xrd_collateral.resource_address(), RADIX_TOKEN, "XRD is the only accepted collateral.");

            // get NFT data to manipulate
            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);

            // Collect XRD fee from collateral deposit and put it in the fee vault
            let fee_amount = xrd_collateral.amount()*self.collateral_deposit_fee;
            let fee: Bucket = xrd_collateral.take(fee_amount);
            self.take_platform_fee(fee);

            // calc the liquidation value for this user and insert it to liquidation_book 
            self.update_collateralization_and_liquidation_value(user_id.clone());

            // calls the user management component
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();

            // uses the user management component to do the actual updates to nft metadata
            self.auth_badge_vault.authorize(|| 
                user_management.increase_collateral_balance(user_id.clone(), xrd_collateral.amount(), data)
            );

            // Put the remaining new XRD into collateral pool
            self.xrd_collateral_pool.put(xrd_collateral);

            // for testing purposes only ******
            self.show_liquidation_book();
            self.show_info(user_id.clone());
        }

        // takes a proof, extracts the nonfungibleid, returns nonfungiblelocalid & data
        fn get_id_and_data_from_proof(&self, user_proof: Proof) -> (NonFungibleLocalId, User) {
            
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();

            let address = self.auth_badge_vault.authorize(|| 
                user_management.return_user_nft_address()
            );

            let validated_proof = user_proof
                .validate_proof(
                ProofValidationMode::ValidateResourceAddress(address))
                .expect("invalid proof");

            let user_id = validated_proof.non_fungible_local_id();

            let data = self.get_nft_data_from_id(user_id.clone());
            
            (user_id, data)
        }

        /// takes nonfungibleid and returns nonfungibleid, nft data
        fn get_nft_data_from_id(&self, user_id: NonFungibleLocalId) -> User {
            
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            let nft_data: User = user_management.get_data_from_id(user_id);
            nft_data
        }

        /// Method for user to remove XRD from collateral pool
        /// Fee is taken at collateral removal
        pub fn remove_xrd_from_collateral(&mut self, amount: Decimal, user_proof: Proof) -> Bucket {

            // get ID and NFT data to manipulate
            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);

            // Make sure you are only withdrawing only as much as you have deposited
            assert!(amount >= data.xrd_collateral, "You can't withdraw more than your collateral balance.");

            // Verify that the new collateral ratio is above liquidation threshold
            if data.usds_borrowed > dec!(0) {
                let xrd_price: Decimal = self.get_xrd_price(); 
                let new_collateralization_rate: Decimal = (xrd_price * (data.xrd_collateral - amount)) / data.usds_borrowed; 
                assert!(new_collateralization_rate > dec!(110), 
                    "You must pay back some of your loan before withdrawing this collateral"
                );
            }
                
            // Take xrd out of the pool and return to user
            let mut xrd_to_return: Bucket = self.xrd_collateral_pool.take(amount);
            let fee: Bucket = xrd_to_return.take(xrd_to_return.amount()*self.collateral_withdrawal_fee);
            self.take_platform_fee(fee);

            // calc the liquidation value for this loan and insert it to liquidation_book
            self.update_collateralization_and_liquidation_value(user_id.clone());

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.decrease_collateral_balance(user_id.clone(), amount, data)
            );

            // for testing purposes only ******
            self.show_liquidation_book();
            self.show_info(user_id.clone());

            // Return bucket of XRD to user
            xrd_to_return
        }

        /// Method used to borrow USDS against deposited XRD collateral.  This method returns
        /// a bucket of USDS borrowed against XRD collateral
        pub fn borrow_usds(&mut self, amount: Decimal, user_proof: Proof) -> Bucket {

            // get nft ID and data to manipulate
            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);
            
            // Does this user have borrow_amount available to borrow?
            let xrd_price = self.get_xrd_price();

            let number = (xrd_price * data.xrd_collateral) * dec!(100) / (data.usds_borrowed + amount);

            // // for testing purposes only *****
            info!("current XRD price is {}", xrd_price);
            
            assert!(
                 number > dec!(110), 
                "You must keep 110% collateral ratio when borrowing."
            );

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.increase_borrowed_usds_balance(user_id.clone(), amount, data)
            );

            self.current_usds_issued += amount;

            // mint usds to loan to user
            let borrowed_usds = self.auth_badge_vault.authorize(|| 
                borrow_resource_manager!(self.usds).mint(amount)
            );

            // calc the liquidation value for this loan and insert it to liquidation_book
            self.update_collateralization_and_liquidation_value(user_id.clone());

            // for testing purposes only ***
            self.show_liquidation_book();
            self.show_info(user_id.clone());

            borrowed_usds
        }

        /// Method used to repay a loan in usds.  This method takes in a bucket of USDS.  Data
        /// such as collateralization rate and liquidation prices are updated on the user's NFT
        pub fn repay_usds_loan(&mut self, mut amount: Bucket, user_proof: Proof) {

            // make sure amount contains usds
            assert_eq!(amount.resource_address(), self.usds, "You can only repay this loan with USDS");

            // get nft ID and data to manipulate
            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);

            // find how much usds is owed from data
            let owed_amount = data.usds_borrowed;
            
            let amount_repaid = amount.amount();
            let mut owed_tokens = Bucket::new(self.usds);

            // take just that amount needed from amount
            if amount_repaid > owed_amount {
                owed_tokens.put(amount.take(owed_amount));
            } else {
                owed_tokens.put(amount);
            }

            // update count of usds issued - tracked by component
            self.current_usds_issued -= owed_tokens.amount();

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.decrease_borrowed_usds_balance(user_id.clone(), owed_tokens.amount(), data)
            );

            // burn the amount of usds that is owed
            self.auth_badge_vault.authorize(|| owed_tokens.burn());

            // calc the liquidation value for this loan and insert it to liquidation_book
            self.update_collateralization_and_liquidation_value(user_id.clone());

            // for testing purposes only *****
            self.show_info(user_id.clone());
            self.show_liquidation_book();
        }

        /// Sets the pricing of the asset using oracle component.  This is a fake method used 
        /// just to simulate the changing or price of XRD since there is no oracle to pull from yet
        pub fn set_price(&mut self, set_price: Decimal) {
            
            // call method from user_management to update the data on user's NFT
            let price_oracle: PriceOracleGlobalComponentRef = self.price_oracle_address.into();
            price_oracle.set_price(set_price);

        }

        /// Helper function to determine if platform should switch into Safe Mode.  Future functionality
        /// will be provided that changes safety parameters to help the platform recover from a low
        /// total collateralization rate.
        fn find_platform_collateralization_ratio(&mut self) {
            self.platform_ratio = self.xrd_collateral_pool.amount() / self.current_usds_issued;

            if self.platform_ratio < dec!(150) {
                self.platform_mode = OperationalMode::Recovery;
            } else {
                self.platform_mode = OperationalMode::Normal;
            }
        }

        /// Helper function that takes the liquidation_book list and changes its type to 
        /// BTreeMap and inverts the value/user_id so that that list is sorted by 
        /// liquidation price instead of user_id.
        fn sort_liquidation_book(&mut self) ->  BTreeMap<Decimal, NonFungibleLocalId> {
            
            let mut sorted_liqudation_book: BTreeMap<Decimal, NonFungibleLocalId> = BTreeMap::new();
            let list = self.liquidation_book.clone();
            
            for (user_id, value) in list {
                sorted_liqudation_book.insert(value, user_id);
            }

            sorted_liqudation_book
        }

        /// Method used to deposit USDS into the shield pool to help guard the protocol
        /// against liquidations.  Those helping shield the liquidations get rewarded with
        /// greater values of XRD to replace their USDS deposits upon liquidation events.
        pub fn deposit_to_shield_pool(&mut self, deposit: Bucket, user_proof: Proof) {

            // check bucket contains usds.resource_address
            assert_eq!(deposit.resource_address(), self.usds, "You can only contribute USDS");

            let (user_id, mut data) = self.get_id_and_data_from_proof(user_proof);

            // Record product (P) snapshot on user_nft
            // Record summation (S) snapshot on user_nft
            // Record user deposit (dt) on user_nft
            let new_shield_deposit: ShieldDeposit = ShieldDeposit {
                event_number: self.event_number,
                usds_shield_deposit_amount: deposit.amount(),
                product_at_time_of_deposit: self.product.clone(),
                summation_at_time_of_deposit: self.summation.clone(),
            };

            // component keeps track of each event, and adds 1 to counter each event
            self.event_number += 1;

            data.shield_deposits.push(new_shield_deposit); 

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.increase_usds_shield_deposit(user_id.clone(), deposit.amount(), data)
            );

            // update total deposits: D2 = D1+ dt
            self.usds_shield_pool.put(deposit);

            // WORK IN PROGRESS *****
            self.shield_depositors.push(user_id.clone());

            self.show_info(user_id.clone());
        }

        /// This method is used to redeem any USDS from anyone (even someone that doesnt have a loan with the platform)
        /// This method will take in whatever amount of USDS to be redeemed 1:1 for current market rate of XRD
        /// and redeem this USDS directly from the least collateralized loans.  It will start by redeeming against the bottom 50% of
        /// collateralized loans and moving to a greater portion depending on the total amount of borrowed USDS in that bracket.
        /// This method is key for keeping USDS pegged to USD and ensuring it is always exchangable 1:1 for XRD.
        pub fn redeem_usds(&mut self, redemption: Bucket) -> Bucket {

            // Try to target the NFT IDs that represent the bottom 50% of all loans
            // in order to redeem USDS from the market.
            let liquidation_book_len = Decimal::from(self.liquidation_book.len());
            
            // find number of entries in bottom half, round down to whole number
            let bottom_50_percent_len = (liquidation_book_len/2).floor();

            // convert hashmap into an ordered BTreeMap
            let mut sorted_liquidation_book1: BTreeMap<_,_> = self.sort_liquidation_book();
            let mut sorted_liquidation_book2: BTreeMap<_,_> = self.sort_liquidation_book();
            info!("sorted_liquidation_book: {:?}", sorted_liquidation_book1);
            
            // calculate xrd value of USDS being redeemed in USD
            let xrd_price = self.get_xrd_price();
            let usds_amount = redemption.amount();
            let xrd_amount = usds_amount / xrd_price;

            let mut total_usds_borrowed_in_bottom_50: Decimal = dec!(0);

            // count the total amount of borrowed usds from bottom x % of loans closest to liqudation
            let mut count = bottom_50_percent_len;
            while count > dec!(0) {
                if let Some((_, user_id)) = sorted_liquidation_book1.pop_last() {
                let data = self.get_nft_data_from_id(user_id.clone());
                total_usds_borrowed_in_bottom_50 += data.usds_borrowed;
                info!("total_usd_borrowed_in_bottom_50 = {}", total_usds_borrowed_in_bottom_50);
                count -= dec!(1);
                }
            }

            info!("usds to redeem: {}", usds_amount);
            info!("xrd_amount to redeem: {}", xrd_amount);
            info!("bottom_50_percent_len: {}", bottom_50_percent_len);

            let mut count2 = bottom_50_percent_len;

            while count2 > dec!(0) {

                if let Some((_, user_id)) = sorted_liquidation_book2.pop_last() {
                    
                    info!("user_id: {}", user_id);

                    let mut data = self.get_nft_data_from_id(user_id.clone());

                    info!("data.usds_borrowed: {}", data.usds_borrowed);
                    info!("total_usds_borrowed_in_bottom_50: {}", total_usds_borrowed_in_bottom_50);
                    let individual_share = data.usds_borrowed / total_usds_borrowed_in_bottom_50;
                    info!("individual_share: {}", individual_share);

                    // find individual share of xrd for each user based on weight
                    let individual_xrd_share = individual_share * xrd_amount;
                    data.xrd_collateral -= individual_xrd_share;
                    info!("individual_xrd_share: {}", individual_xrd_share);

                    // find individual share of usds for each user based on weight
                    let individual_usds_share = individual_share * usds_amount;
                    data.usds_borrowed += individual_usds_share;
                    info!("individual_usds_share: {}", individual_usds_share);

                    // // Find new collateral rate
                    // data.loan_collateralization_rate = self.find_collateralization_rate(user_id.clone());

                    // calc the liquidation value for this loan and insert it to liquidation_book
                    self.update_collateralization_and_liquidation_value(user_id.clone());
                    
                    // call method from user_management to update the data on user's NFT
                    let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
                    self.auth_badge_vault.authorize(|| 
                        user_management.authorize_update(user_id.clone(), data)
                    );
                }

                count2 -= dec!(1);
            } // else take a larger amount of loans close to liquidation into the pool to redeem against***
            
            // burn the amount of usds that is owed
            self.auth_badge_vault.authorize(|| redemption.burn());

            // Take the total amount of XRD out of collateral pool to return
            let xrd_to_return = self.xrd_collateral_pool.take(xrd_amount);

            // return bucket of xrd
            info!("xrd_to_return: {}", xrd_to_return.amount());
            xrd_to_return
        }

        /// This method automatically withdraws all Sheild deposits and rewards so it does not need to take
        /// in an amount from user
        pub fn withdraw_shield_deposit_and_rewards(&mut self, user_proof: Proof) -> (Bucket, Bucket, Bucket) {

            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);
            
            // compute final compounded USDS deposit d: d = dt(P/Pt);
            // compute final corresponding XRD gain e: e = dt((S-St)/Pt)
            let shield_deposit_vec: Vec<&ShieldDeposit> = data.shield_deposits.iter().collect();

            let mut final_deposit_amount = dec!(0);

            let mut final_xrd_gain = dec!(0);

            for shield_deposit in shield_deposit_vec {

                final_deposit_amount += shield_deposit.usds_shield_deposit_amount*(self.product/shield_deposit.product_at_time_of_deposit);

                final_xrd_gain += shield_deposit.usds_shield_deposit_amount*((self.summation-shield_deposit.summation_at_time_of_deposit)/shield_deposit.product_at_time_of_deposit);
            }

            info!("Shield deposit amount to return is {}", final_deposit_amount);
            info!("xrd gain to return is {}", final_xrd_gain);
            info!("total usd in shield is {}", self.usds_shield_pool.amount());

            let final_xrd_gain_to_return = self.xrd_shield_reward_pool.take(final_xrd_gain);

            // Scrilla (SCRL) is the native token of this platform and is rewarded to users who use the platform
            // and deposit USDS into the shield pool.  This helps ensure speedy liquidations while rewarding
            // participants.  The SCRL token further rewards early adopters.  These SCRL tokens can later be
            // staked to earn a portion of all platform fees.
            let scrl_rewards = self.auth_badge_vault.authorize(|| 
                borrow_resource_manager!(self.scrl).mint(final_xrd_gain.clone()/dec!(10))
            );

            info!("scrl_rewards: {}", scrl_rewards.amount());


            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.decrease_usds_shield_deposit(user_id.clone(), data)
            );

            if final_deposit_amount > self.usds_shield_pool.amount() {
                final_deposit_amount = self.usds_shield_pool.amount();
            }
            // update total deposits: D = D - d
            let final_deposit_amount_to_return: Bucket = self.usds_shield_pool.take(final_deposit_amount);

            // return d & e to user
            (final_deposit_amount_to_return, final_xrd_gain_to_return, scrl_rewards)
        }

        /// This method allows the component to update the data stored on a user's NFT given new actions or changes 
        /// in price with respect to XRD.  This method is used to manually update NFTs in bash scripts for testing,
        /// but if this were live, users would be updating their own data when interacting with any relevent methods
        /// that would affect the collteralization rate of each loan.
        /// The method takes in a NonFungibleLocalId of integer type 
        pub fn update_collateralization_and_liquidation_value(&mut self, user_id: NonFungibleLocalId) {
            
            let mut data: User = self.get_nft_data_from_id(user_id.clone());
                
            if data.usds_borrowed == dec!(0) {

                // find and remove the old liquiation value if there is one
                self.liquidation_book.remove(&user_id);
                data.loan_collateralization_rate = None;

                // call method from user_management to update the data on user's NFT
                let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
                self.auth_badge_vault.authorize(|| 
                    user_management.authorize_update(user_id.clone(), data)
                );

            } else {

                // calc the liquidation value for this loan 
                let liquidation_value = dec!("1.10") * data.usds_borrowed / data.xrd_collateral;

                // insert this value to liquidation_book
                self.liquidation_book.insert(user_id.clone(), liquidation_value);

                // Get USD/XRD rate from oracle
                let new_xrd_price = self.get_xrd_price();  

                let new_collateralization_rate = (dec!(100) * new_xrd_price * data.xrd_collateral) / data.usds_borrowed;
                data.loan_collateralization_rate = Some(new_collateralization_rate);

                // call method from user_management to update the data on user's NFT
                let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
                self.auth_badge_vault.authorize(|| 
                    user_management.authorize_update(user_id.clone(), data)
                );
            }
        }

        /// Method for testing purposes only that shows the liquidation book.  This is a list that shows NFT ID for 
        /// a user of the platform along with that users price of XRD that would trigger a liquidation for this
        /// user's loan.
        pub fn show_liquidation_book(&mut self) {
            info!("liquidation book: {:?}", self.liquidation_book);
        }

        /// This method is for testing purposes.  It will show relevant information for both a given user
        /// and the component at any given time.  It is used in most methods to output testing info.
        pub fn show_info(&mut self, user_id: NonFungibleLocalId) {
            
            let data = self.get_nft_data_from_id(user_id.clone());

            info!("User {}: XRD collateral balance: {:?}", user_id.clone(), data.xrd_collateral);
            info!("User {}: USDS borrowed balance: {:?}", user_id.clone(), data.usds_borrowed);
            info!("User {}: Loan collateralization rate: {:?}", user_id.clone(), data.loan_collateralization_rate);
            info!("User {}: Current usds in shield pool: {:?}", user_id.clone(), data.current_usds_in_shield);
            info!("User {}: Current scrilla stake: {:?}", user_id.clone(), data.current_scrl_staked);
            info!("Scrilla Component: xrd price (usds): {:?}", self.get_xrd_price());
            info!("Scrilla Component: Total shield pool (usds): {:?}", self.usds_shield_pool.amount());
            info!("Scrilla Component: Total sheild reward pool (xrd): {:?}", self.xrd_shield_reward_pool.amount());
            info!("Scrilla Component: Total fees collected (xrd): {:?}", self.xrd_fee_vault.amount());
            info!("Scrilla Component: Total collateral (xrd): {:?}", self. xrd_collateral_pool.amount());
            info!("Scrilla Component: liquidation book: {:?}", self.liquidation_book);
            info!("Scrilla Component: summation: {:?}", self.summation);
            info!("Scrilla Component: product: {:?}", self.product);
            info!("Scrilla Component: Scrilla summation: {:?}", self.scrl_summation);
        }

        /// Gets the price of the given asset using the "fake" oracle component
        pub fn get_xrd_price(&mut self) -> Decimal {

            let price_oracle: PriceOracleGlobalComponentRef = self.price_oracle_address.into();
            let new_xrd_price = price_oracle.get_price();

            // convert hashmap into an ordered BTreeMap
            let sorted_liquidation_book: BTreeMap<_,_> = self.sort_liquidation_book();
            info!("sorted_liquidation_book: {:?}", sorted_liquidation_book);

            self.last_xrd_price = new_xrd_price;
            new_xrd_price
        }

        /// Method used to liquidate a users collateral after the loan collateralization ratio
        /// falls below 110% of the value of the held collateral.  Liquidaters are rewarded
        /// with a bucket of SCRL tokens based upon the number of XRD liquidated
        pub fn call_liquidation(&mut self, user_id: NonFungibleLocalId)  -> Bucket {
            
            self.update_collateralization_and_liquidation_value(user_id.clone());

            let data = self.get_nft_data_from_id(user_id.clone());
        
            // Check loan health is <110% collateral ratio
            let rate = data.loan_collateralization_rate;
            assert!(rate < Some(dec!(110)), "Cannot liquidate loans until below 110% collateralization");

	        // Check shield pool deposits (D) > USDS being liquidated (Q)
            assert!(self.usds_shield_pool.amount() >= data.usds_borrowed);
            
            // update S: S = S + (E/D)P
            self.summation += (data.xrd_collateral/self.usds_shield_pool.amount())*self.product;


            info!("data.usds_borrowed: {:?}", data.usds_borrowed);
            info!("self.usds_shield_pool.amount(): {:?}", self.usds_shield_pool.amount());

            // update P: P = P(1-(Q/D))
            let old_product = self.product.clone();
            self.product = old_product*(dec!(1)-(data.usds_borrowed/self.usds_shield_pool.amount()));

            // update total deposits: D = D - d  
            self.current_usds_issued -= data.usds_borrowed;

            // mint some SCRL tokens to give as reward to the person performing the liquidation
            let scrl_rewards = self.auth_badge_vault.authorize(|| 
                borrow_resource_manager!(self.scrl).mint(data.xrd_collateral * dec!("0.05"))
            );

            // Move user's liquidated collateral into the shield reward pool
            let deposit: Bucket = self.xrd_collateral_pool.take(data.xrd_collateral);
            self.xrd_shield_reward_pool.put(deposit);

	        // Burn loan amount from shield
            let burn_bucket: Bucket = self.usds_shield_pool.take(data.usds_borrowed);
            self.auth_badge_vault.authorize(|| 
                borrow_resource_manager!(self.usds).burn(burn_bucket)
            );
            
            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.liquidate_user_nft(user_id.clone(), data)
            );

            self.update_collateralization_and_liquidation_value(user_id.clone());

            // Update this to implement recovery mode vs normal mode ***
            self.find_platform_collateralization_ratio();

            scrl_rewards
        }

        /// Method used to stake Scrilla (SCRL) and earn a portion of platform fees
        /// proportional to your ownership of the pool
        pub fn stake_scrilla(&mut self, deposit: Bucket, user_proof: Proof) {

            // check bucket contains usds.resource_address
            assert_eq!(deposit.resource_address(), self.scrl, "You can only contribute Scrilla");

            let (user_id, mut data) = self.get_id_and_data_from_proof(user_proof);

            // Record summation (S) snapshot on user_nft
            // Record user deposit (dt) on user_nft
            let new_scrilla_deposit: ScrillaDeposit = ScrillaDeposit {
                event_number: self.event_number,
                scrilla_deposit_amount: deposit.amount(),
                scrilla_summation_at_deposit: self.scrl_summation.clone(),
            };

            data.scrilla_deposits.push(new_scrilla_deposit); 

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.increase_scrilla_stake(user_id.clone(), deposit.amount(), data)
            );

            // update total deposits: D2 = D1+ dt
            self.scrl_stake_pool.put(deposit);

            self.show_info(user_id.clone());
        }
        
        // This method automatically withdraws all scrilla stake deposits and rewards so it does not need to take
        // in an amount from user
        pub fn unstake_scrilla_and_claim_rewards(&mut self, user_proof: Proof) -> (Bucket, Bucket) {

            let (user_id, data) = self.get_id_and_data_from_proof(user_proof);
            
            // compute final corresponding XRD gain e: e = dt((S-St)/Pt)
            let scrilla_deposit_vec: Vec<&ScrillaDeposit> = data.scrilla_deposits.iter().collect();


            let mut final_deposit_amount = dec!(0);

            let mut final_xrd_gain = dec!(0);

            for scrilla_deposit in scrilla_deposit_vec {

                final_deposit_amount += scrilla_deposit.scrilla_deposit_amount;

                final_xrd_gain += scrilla_deposit.scrilla_deposit_amount*((self.scrl_summation - scrilla_deposit.scrilla_summation_at_deposit));
            }

            info!("final_deposit_amount {}", final_deposit_amount);
            info!("xrd gain from scrilla staking {}", final_xrd_gain);
            info!("total scrilla staked is {}", self.scrl_stake_pool.amount());

            let final_xrd_gain_to_return = self.xrd_fee_vault.take(final_xrd_gain);

            // call method from user_management to update the data on user's NFT
            let user_management: UserManagementGlobalComponentRef = self.user_management_address.into();
            self.auth_badge_vault.authorize(|| 
                user_management.decrease_scrilla_stake(user_id.clone(), data)
            );

            if final_deposit_amount > self.scrl_stake_pool.amount() {
                final_deposit_amount = self.scrl_stake_pool.amount();
            }

            // update total deposits: D = D - d
            let final_deposit_amount_to_return: Bucket = self.scrl_stake_pool.take(final_deposit_amount);

            // return d & e to user
            (final_deposit_amount_to_return, final_xrd_gain_to_return)
        }

        /// Helper function to keep track of the summation value for the Scrilla token staking pool
        /// (This is a different summation value than what is tracked for the shield pool)
        fn take_platform_fee(&mut self, fee_bucket: Bucket) {
    
            // update S: S = S + (E/D)
            if self.scrl_stake_pool.amount() == dec!(0) {
                self.scrl_summation += fee_bucket.amount()
            } else {
                self.scrl_summation += fee_bucket.amount()/self.scrl_stake_pool.amount();
            }
        
            self.xrd_fee_vault.put(fee_bucket);

        }
    }     
}

            