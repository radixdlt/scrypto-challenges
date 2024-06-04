//! # Overview of main functions
//!
//! This is the list of all main functions. 
//!
//! ## Instantiation
//!
//! [instantiate()][tokenizer::Tokenizer::instantiate]
//! Creates a new Tokenizer instance.
//!
//! ## Register
//!
//! [register()][tokenizer::Tokenizer::register]
//! Function for registering an account to the platform
//! 
//! ## Unregister
//!
//! [unregister()][tokenizer::Tokenizer::unregister]
//! Function for being removed from the platform 
//! 
//! ## Supply
//!
//! [supply()][tokenizer::Tokenizer::supply]
//! Function for supplying fungible tokens to the platform                      
//! 
//! ## Takes Back
//!
//! [takes_back()][tokenizer::Tokenizer::takes_back]
//! Function for withdrawing your liquidity from the platform
//! 
//! ## Tokenize
//!
//! [tokenize_yield()][tokenizer::Tokenizer::tokenize_yield]
//! Function for tokenizing and block the liquidity already supplied into the platform
//! 
//! ## Redeem
//!
//! [redeem()][tokenizer::Tokenizer::redeem]
//! Function for withdrawing the locked liquidity before the maturity dat at the current market value as a function of the interest rate
//! 
//! ## Redeem from PT
//!
//! [redeem_from_pt()][tokenizer::Tokenizer::redeem_from_pt]
//! Function for withdrawing the locked liquidity after the maturity date
//! 
//! ## Claim Yield
//!
//! [claim_yield()][tokenizer::Tokenizer::claim_yield]
//! Function for withdrawing the accrued interest after the maturity date
//! 
//! # Overview of secondary functions
//!
//! This is the list of all the functions needed to setup, configure and manage the dApp functionalities
//! 
//! ## Set Reward
//!
//! [set_reward()][tokenizer::Tokenizer::set_reward]
//! Function for setting the reward for accounts providing liquidity
//! 
//! ## Set Extra Reward
//!
//! [set_extra_reward()][tokenizer::Tokenizer::set_extra_reward]
//! Function for setting the extra reward for accounts blocking their supplied liquidity
//! 
//! ## Set Reward Type
//!
//! [set_reward_type()][tokenizer::Tokenizer::set_reward_type]
//! Function for setting the reward type on the supplied liquidity
//! 
//! ## Extend Lending Pool
//!
//! [extend_lending_pool()][tokenizer::Tokenizer::extend_lending_pool]
//! Function for extending the maximum amount that a single account can supply in the platform
//! 
//! ## Fund Main Pool
//!
//! [fund_main_pool()][tokenizer::Tokenizer::fund_main_pool]
//! Function for funding the main pool at the beginning
//! 
//! ## Config
//!
//! [config()][tokenizer::Tokenizer::config]
//! Function for setting and modifying parameters within the life of the smart contract
//! 
//! ## Add Token
//!
//! [add_token()][tokenizer::Tokenizer::add_token]
//! Function for adding a new fungible token resource address and allow the contract to handle it 
//! 
//! ## Mint Staff Badge
//!
//! [mint_staff_badge()][tokenizer::Tokenizer::mint_staff_badge]
//! Function for minting a new badge for then allowing a staff member to modify the dApp configuration
//! 

use scrypto::prelude::*;
use scrypto_avltree::AvlTree;
use crate::utils::*;

/// this is to contain data about an account position over multiple supplied/tokenized liquidity
#[derive(ScryptoSbor, NonFungibleData)]
pub struct UserMultiPosition {
    #[mutable]
    liquidity_position: HashMap<ResourceAddress, LiquidityData>,
    #[mutable]
    yield_token_data: HashMap<ResourceAddress, YieldTokenData>
}

/// this is to contain data about account's provided liquidity 
#[derive(Copy, Clone, ScryptoSbor, NonFungibleData)]
 pub struct LiquidityData {
    #[mutable]
    start_supply_epoch: Epoch,
    #[mutable]
    end_supply_epoch: Epoch,
    #[mutable]
    amount: Decimal,
}

/// this is to contain data about account's tokenized liquidity 
#[derive(Copy, Clone, ScryptoSbor, NonFungibleData)]
pub struct YieldTokenData {
    extra_reward: Decimal,
    underlying_amount: Decimal,
    pub interest_totals: Decimal,
    pub yield_claimed: Decimal,
    maturity_date: Decimal,
    principal_returned: bool,
}

/// this is to contain the username of a Staff Member
#[derive(NonFungibleData, ScryptoSbor)]
struct StaffBadge {
    username: String
}


#[blueprint]
mod tokenizer {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            staff => updatable_by: [admin, OWNER];
        },
        methods {
            register => PUBLIC;
            unregister => PUBLIC;
            supply => PUBLIC;
            takes_back => PUBLIC;
            set_reward => restrict_to: [staff, admin, OWNER];
            set_extra_reward => restrict_to: [admin, OWNER];
            set_reward_type => restrict_to: [admin, OWNER];
            extend_lending_pool => restrict_to: [admin, OWNER];
            fund_main_pool => restrict_to: [admin, OWNER];
            config => restrict_to: [staff, admin, OWNER];
            add_token => restrict_to: [admin, OWNER];
            mint_staff_badge => restrict_to: [staff, admin, OWNER];
            tokenize_yield  => PUBLIC;
            redeem => PUBLIC;
            redeem_from_pt => PUBLIC;
            claim_yield => PUBLIC;
        }
    }
    struct Tokenizer<> {
        tokenizer_vault: Vault,
        collected: HashMap<ResourceAddress, FungibleVault>,
        reward: Decimal,
        extra_reward: Decimal,
        tokenize_epoch_max_lenght: Decimal,
        tokenizer_manager: ResourceManager,
        nft_manager: ResourceManager,
        reward_type: String,
        interest_for_suppliers: AvlTree<Decimal, Decimal>,
        min_loan_limit: Decimal,
        max_loan_limit: Decimal,
        staff: AvlTree<u16, NonFungibleLocalId>,
        pt_resource_manager: ResourceManager,
        staff_badge_resource_manager: ResourceManager,
        resource_a: ResourceAddress,
        resource_b: ResourceAddress
    }

    impl Tokenizer {

        /// Creates a new ready-to-use Tokenizer, returning also an owner and an admin badge
        ///
        /// This create also the main container (Hashmap) for managing all the fungible vaults on which the contract is enabled.
        /// 
        /// This create also:
        ///   - A bucket of fungibles limited to 100.000tokens to be returned to accounts that supply liquidity (max size can be increased by the 'extend_pool' function)
        ///   - A resource manager to mint/update AccountData for suppliers
        ///   - A resource manager to mint/burn Principal Token for suppliers that decide to tokenize the liquidity provided
        ///   - A resource manager to mint/burn one or more Staff Badge for those who will be managing this dApp
        ///   - A balanced binary search tree for storing all the changes in the interest rate that occur during the epoch (period of time needed for the validators to participate in consensus)
        /// 
        /// It is important to note that the interest rate may accordingly change every epoch that is around 5 minutes
        ///
        /// The Tokenizer can be created by specifying the following parameters: 
        ///   - a reward level: This is the reward for accounts that will supply liquidity in
        ///   - a symbol name: This is the token symbol of the fungibles that each account will receive, together with the non fungible token, by supplying liquidity
        ///   - reward_type: The type of the reward is how the suppliers are rewarded. For this specific blueprint customized for this challenge the reward_type is always based on time.
        ///   - resource_a: For this specific blueprint customized for this challenge this is the first resource address managed by the blueprint
        ///   - resource_b: For this specific blueprint customized for this challenge this is the second resource address managed by the blueprint
        /// 
        /// Any new token addresses can be added to this blueprint by using the function 'add_token'
        /// 
        /// Returns a tuple containing:
        /// - The component address of the instantiated and globalized Tokenizer
        /// - An Owner badge 
        /// - An Admin badge 
        /// 
        pub fn instantiate(
            reward: Decimal,
            symbol: String,
            reward_type: String
            ,resource_a: ResourceAddress
            ,resource_b: ResourceAddress
        ) -> (Global<Tokenizer>, FungibleBucket, FungibleBucket) {

            //container for holding a FungibleVault for each FungibleToken managed by the blueprint
            let mut collected: HashMap<ResourceAddress, FungibleVault> = HashMap::new();
            collected.insert(resource_a, FungibleVault::new(resource_a));
            collected.insert(resource_b, FungibleVault::new(resource_b));
            
            //data struct for holding interest rates
            let mut lend_tree: AvlTree<Decimal, Decimal> = AvlTree::new();
            lend_tree.insert(Decimal::from(Runtime::current_epoch().number()), reward);
            //staff container
            let staff: AvlTree<u16, NonFungibleLocalId> = AvlTree::new();

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Tokenizer::blueprint_id());

            //owner badge
            let owner_badge = 
                ResourceBuilder::new_fungible(OwnerRole::None)
                    .metadata(metadata!(init{
                        "name"=>"Tokenizer Owner badge", locked;
                        "symbol" => "Tokenizer Owner", locked;
                        "description" => "A badge to be used for some extra-special administrative function", locked;
                    }))
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);

            //admin badge
            let admin_badge = 
                ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                .metadata(metadata!(init{
                    "name"=>"Tokenizer Admin badge", locked;
                    "symbol" => "Tokenizer Admin", locked;
                    "description" => "A badge to be used for some special administrative function", locked;
                }))
                .mint_roles(mint_roles! (
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => OWNER;
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);


            // create a new Fungible Bucket, with a fixed quantity of 100000
            let tokenizer_bucket = 
                ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(init{
                    "name" => "LiquidTokenizerUnit", locked;
                    "symbol" => symbol, locked;
                    "description" => "A token to use to receive back the loan", locked;
                }))
                .mint_roles(mint_roles! (
                         minter => rule!(require(global_caller(component_address)));
                         minter_updater => OWNER;
                ))
                .mint_initial_supply(100000);

            // Create a resourceManager to manage AccountData NFT
            // This NFT is also burnable in the scope of this specific blueprint customized for this challenge
            let nft_manager =
                ResourceBuilder::new_ruid_non_fungible::<UserMultiPosition>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(
                    init {
                        "name" => "Tokenizer AccountData NFT", locked;
                        "symbol" => "Tokenizer AccountData", locked;
                        "description" => "An NFT containing information about your liquidity", locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(require(global_caller(component_address)));
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => OWNER;
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => OWNER;
                ))           
                .create_with_no_initial_supply();
       

            // Create a resourceManager to manage Principal Token
            let pt_rm: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                require(owner_badge.resource_address())
                    || require(admin_badge.resource_address())
            )))
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => "Principal Token", locked;
                        "symbol" => "PT", locked;
                        "description" => "A Token containing the Principal Token", locked;
                        "yield_tokenizer_component" => GlobalAddress::from(component_address), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Create a resourceManager to manage Staff Badges
            let staff_resource_manager: ResourceManager = 
                ResourceBuilder::new_ruid_non_fungible::<StaffBadge>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(init{
                    "name" => "Tokenizer Staff_badge", locked;
                    "symbol" => "Tokenizer Staff", locked;
                    "description" => "A badge to be used for some administrative function", locked;
                }))
                .mint_roles(mint_roles! (
                         minter => rule!(require(global_caller(component_address)));
                         minter_updater => OWNER;
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(require(admin_badge.resource_address()));
                    burner_updater => OWNER;
                ))
                .recall_roles(recall_roles! {
                    recaller => rule!(require(admin_badge.resource_address()));
                    recaller_updater => OWNER;
                })
            .create_with_no_initial_supply();            

            // populate a Tokenizer struct and instantiate a new component
            // 
            // Some parameters are defined herein and can be then modified by the 'config' function
            //     - extra_reward: It is the extra reward for account that will block liquidity until a maturity date
            //     - tokenize_epoch_max_lenght: Max length of the tokenize operation (around 1 year)
            //     - min_loan_limit: Min amount of tokens that an account can supply in
            //     - max_loan_limit: Max amount of tokens that an account can supply in
            let component = 
                Self {
                    tokenizer_manager: tokenizer_bucket.resource_manager(),
                    tokenizer_vault: Vault::with_bucket(tokenizer_bucket.into()),
                    reward: reward,
                    extra_reward: dec!(10),
                    tokenize_epoch_max_lenght: dec!(518000),//how many days ??
                    nft_manager: nft_manager,
                    reward_type: reward_type,
                    interest_for_suppliers: lend_tree,
                    min_loan_limit: dec!(1),
                    max_loan_limit: dec!(10001),
                    staff: staff,
                    pt_resource_manager: pt_rm,
                    staff_badge_resource_manager: staff_resource_manager,
                    collected: collected
                    ,resource_a: resource_a
                    ,resource_b: resource_b
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                .enable_component_royalties(component_royalties! {
                    // The roles section is optional, if missing, all roles default to OWNER
                    // Herein we are specifyng who can set/update/loc/claim the royalties/commissions generated by this smart contract
                    roles {
                        royalty_setter => rule!(allow_all);
                        royalty_setter_updater => OWNER;
                        royalty_locker => OWNER;
                        royalty_locker_updater => rule!(deny_all);
                        royalty_claimer => OWNER;
                        royalty_claimer_updater => rule!(deny_all);
                    },
                    // Herein we are specifyng which functions generate a commission for the accounts
                    init {
                        register => Free, locked;
                        unregister => Free, locked;
                        supply => Xrd(10.into()), updatable;
                        takes_back => Xrd(10.into()), updatable;

                        set_reward => Free, locked;
                        set_extra_reward => Free, locked;
                        set_reward_type => Free, locked;
                        extend_lending_pool => Free, locked;
                        fund_main_pool => Free, locked;
                        config => Free, locked;
                        add_token => Free, locked;
                        mint_staff_badge => Free, locked;

                        tokenize_yield => Xrd(10.into()), updatable;
                        redeem => Xrd(10.into()), updatable;
                        redeem_from_pt => Xrd(10.into()), updatable;
                        claim_yield => Xrd(10.into()), updatable;
                    }
                })                
                .metadata(metadata!(
                    init {
                        "name" => "Tokenizer", locked;
                        "icon_url" => Url::of("https://test.Tokenizer.eu/images/logo3b.jpg"), locked;
                        "description" => "Tokenizer SmartContract for lending and tokenizer service", locked;
                        "claimed_websites" =>  ["https://test.Tokenizer.eu"], locked;
                    }
                ))
                //Herein we are specifying what does a role need to present a proof of itself
                .roles(roles!(
                    admin => rule!(require(admin_badge.resource_address()));
                    staff => rule!(require(staff_resource_manager.address()));
                ))
                .with_address(address_reservation)
                .globalize();
 
            return (component, admin_badge, owner_badge);
        }

        /// Add a new token resource address to those already managed
        /// For this specific blueprint customized for this challenge a token address cannot be removed
        /// 
        /// Arguments:
        /// - `token`: The resource address of the token you want to add 
        ///
        /// Returns 'None':
        ///
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///
        /// **Transaction manifest:**
        /// `rtm/add_token.rtm`
        /// ```text
        #[doc = include_str!("../rtm/add_token.rtm")]
        /// ```        
        pub fn add_token(&mut self, token: ResourceAddress)  {
            info!("Adding token type {:?}", token);
            self.collected.insert(token, FungibleVault::new(token));
        }

        //register to the platform
        /// Register the account by minting an NFT for holding the information about the supplied/tokenized liquidity
        /// 
        /// Returns:
        /// - 'userdata_multi_nft': An Nft of type UserMultiPosition containing an HashMap of LiquidityData and an HashMap of YieldTokenData for each Resource Address
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/register.rtm`
        /// ```text
        #[doc = include_str!("../rtm/register.rtm")]
        /// ```           
        pub fn register(&mut self) -> NonFungibleBucket {
            //mint an NFT for registering loan amount and starting/ending epoch
            let yield_token: YieldTokenData = self.init_yield();
            let liq_data = self.init_liq_data();

            let mut nft1: HashMap<ResourceAddress, LiquidityData> = HashMap::new();
            nft1.insert(self.resource_a, liq_data);
            nft1.insert(self.resource_b, liq_data);
            let mut nft2: HashMap<ResourceAddress, YieldTokenData> = HashMap::new();
            nft2.insert(self.resource_a, yield_token);
            nft2.insert(self.resource_b, yield_token);

            let userdata_multi_nft = self.nft_manager
            .mint_ruid_non_fungible(
                UserMultiPosition {
                    liquidity_position: nft1,
                    yield_token_data: nft2
                }
            );

            scrypto::prelude::NonFungibleBucket(userdata_multi_nft)
        }         


        /// Unregister the account from the component by burning the information about the supplied/tokenized liquidity
        /// 
        /// Be careful ! This has been used for testing this specific blueprint for this challenge
        /// 
        /// Arguments:
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        ///
        /// Returns 'None':
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/unregister.rtm`
        /// ```text
        #[doc = include_str!("../rtm/unregister.rtm")]
        /// ```                     
        pub fn unregister(&mut self, userdata_nft: Bucket) -> Option<Bucket> {
            //burn the NFT, be sure you'll lose all your tokens not reedemed in advance of this operation
            userdata_nft.burn();
            None
        }

        /// Supply some tokens of the specified type
        /// 
        /// Arguments:
        /// - `loan`: A Bucket of tokens that the account want to supply in
        /// - `userdata_nft`: A NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account want to supply in
        ///
        /// Returns a tuple with:
        /// - `token_received`: A bucket with the same number of tokens provided, but of a different type (specifically, we return the tokens managed by the component)
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// Be careful that all the information is contained in the NFT but the tokens are used to make the amount in the wallet more easily visible !
        /// 
        /// Panics if:
        /// - The provided Bucket and the ResourceAddress does not match.
        /// - The NFT is not of the requested type.
        /// - A supply operation is already in place (no subsequent supply is managed to make this simple)
        /// - The number of tokens provided meets the upper or lower limit
        /// - The tokens provided are not managed yet
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/supply_high.rtm`
        /// ```text
        #[doc = include_str!("../rtm/supply_high.rtm")]
        /// ```     
        pub fn supply(&mut self, loan: FungibleBucket, userdata_nft: NonFungibleBucket, token_type: ResourceAddress) -> (Bucket, NonFungibleBucket) {
            assert_resource(&userdata_nft.resource_address(), &self.nft_manager.address());
            assert_resource(&loan.resource_address(), &token_type);
            
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut liquidity_position = binding.liquidity_position;

            if let Some(mut data) = liquidity_position.remove(&token_type) {
                info!("Supplying liquidity of type  {:?}, amount {:?}", token_type, data.amount);

                //check amount and time limits
                let amount_lended = data.amount;
                lend_checks_time_based(amount_lended);
                let num_tokens = loan.amount();
                lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
                info!("Amount of token received: {:?} ", num_tokens);   
    
                //take the bucket as a new loan and put tokens in corresponding pool
                let vault = self.collected.get_mut(&token_type.clone());
                match vault {
                    Some(fung_vault) => {
                        info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                        fung_vault.put(loan);
                    }
                    None => {
                        assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                    }
                }
    
                //prepare a bucket with TKN tokens to give back to the user 
                let token_received = self.tokenizer_vault.take(num_tokens);
                info!("Returning some fungible token to the account, n° {:?}", token_received.amount());
    
                // Update the data on the network
                data.start_supply_epoch = Runtime::current_epoch();
                data.end_supply_epoch = Epoch::of(0);
                data.amount = num_tokens;
                // Insert the modified data back into the hashmap
                liquidity_position.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                return (token_received, userdata_nft)        
            } else {
                let num_tokens = loan.amount();
                lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
                info!("Amount of token received: {:?} ", num_tokens);   

                //take the bucket as a new loan and put tokens in corresponding pool
                let vault = self.collected.get_mut(&token_type.clone());
                match vault {
                    Some(fung_vault) => {
                        info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                        fung_vault.put(loan);
                    }
                    None => {
                        assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                    }
                }

                //prepare a bucket with TKN tokens to give back to the user 
                let token_received = self.tokenizer_vault.take(num_tokens);
                info!("Returning some fungible token to the account, n° {:?}", token_received.amount());

                // Update the data on the network
                let liq_data = self.new_liq_data(Runtime::current_epoch(),Epoch::of(0),num_tokens );
                // Insert the modified data back into the hashmap
                liquidity_position.insert(token_type.clone(), liq_data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                return (token_received, userdata_nft)                
            }
        }

        /// Returns the requested number of initial tokens provided and additionally returns the reward amount calculated by applying the variable interest that has been in effect in the epochs during which the liquidity provision has remained in the contract
        /// 
        /// Arguments:
        /// - `refund`: A Bucket of tokens that the account want to be returned back
        /// - `userdata_nft`: A NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account want to be returned back
        ///
        /// Returns a tuple with:
        /// - `token_received`: A bucket with the same number of tokens provided (specifically, we return those that were initially provided)
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// 
        /// Panics if:
        /// - The provided Bucket and the ResourceAddress does not match.
        /// - The NFT is not of the requested type.
        /// - The number of tokens requested back is at least 20% of those provided
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/takes_back.rtm`
        /// ```text
        #[doc = include_str!("../rtm/takes_back.rtm")]
        /// ```                
        pub fn takes_back(&mut self, refund: Bucket, userdata_nft: NonFungibleBucket, token_type: ResourceAddress) -> (Bucket, Option<NonFungibleBucket>) {
            assert_resource(&userdata_nft.resource_address(), &self.nft_manager.address());

            let non_fung_bucket = userdata_nft.as_non_fungible();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut liquidity_position = binding.liquidity_position;

            if let Some(mut data) = liquidity_position.remove(&token_type) {
                info!("Returning liquidity data of type  {:?}, amount {:?}", token_type, data.amount);

                // Verify the user has requested back at least 20% of its current loan
                take_back_checks(data.amount / 5, &refund.amount());

                // Update the amount field
                let remaining_amount_to_return = data.amount - refund.amount(); 
                info!("Remaining tokens to return: {:?} ", remaining_amount_to_return);   

                //take the bucket to close the loan, and returns corresponding tokens from the main pool
                let amount_to_be_returned = refund.amount();
                self.tokenizer_vault.put(refund);

                //calculate interest over the epochs
                let interest_totals = calculate_interests(
                    &self.reward_type, &self.reward,
                    data.start_supply_epoch.number(),
                    &amount_to_be_returned, &self.interest_for_suppliers);
                info!("Calculated interest {} ", interest_totals);

                //total amount to return 
                let amount_returned = amount_to_be_returned + interest_totals;
                info!("tokens to be given back: {:?} ", amount_returned);  
                
                //total net amount to return
                let vault = self.collected.get_mut(&token_type.clone());
                
                match vault {
                    Some(fung_vault) => {
                        info!("Returning tokens to the account, n°  {:?}", amount_returned);
                        //getting tokens to be returned
                        let bucket_returned = fung_vault.take(amount_returned);

                        // Update the data on the network
                        let nft_local_id: NonFungibleLocalId = userdata_nft.as_non_fungible().non_fungible_local_id();
                        if remaining_amount_to_return == dec!("0") {
                            info!("set the supply operation as 'closed'");
                            //here, we set the supply operation as 'closed' by setting 'end_supply_epoch'
                            data.end_supply_epoch = Runtime::current_epoch();
                            data.amount = remaining_amount_to_return;
                            // Insert the modified data back into the hashmap
                            liquidity_position.insert(token_type.clone(), data);
                            self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                            return (bucket_returned.into(),Some(userdata_nft))              
                        } else {
                            info!("set the supply operation as 'not closed', remaining {}", remaining_amount_to_return);
                            //here, the supply operation is not 'closed' because some tokens are supplied in yet 
                            data.amount = remaining_amount_to_return;
                            // Insert the modified data back into the hashmap
                            liquidity_position.insert(token_type.clone(), data);
                            self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                            return (bucket_returned.into(),Some(userdata_nft))                
                        }
                    }
                    None => {
                        assert_pair("Unavailable Vault".to_string());
                        let token_received = self.tokenizer_vault.take(dec!(0));
                        return (token_received, None)                            
                    }
                }
            } else {
                assert_pair("Unavailable liquidity_position of the specified token".to_string());
                let token_received = self.tokenizer_vault.take(dec!(0));
                return (token_received, None)      
            }

        }

        /// Returns the Principal Tokens bucket and also calculates the amount of reward that will be returned at the end of the 
        /// tokenize operation. 
        /// The reward will be returned after the maturity date but the data it needs to be calculated are already available 
        /// so this is why it is calculated in this function.
        /// 
        /// Arguments:
        /// - `tkn_token`: A Bucket of tokens that the account wants to tokenize
        /// - `tokenize_expected_length`: The duration of the tokenize operation, expressed in Epochs
        /// - `userdata_nft`: An NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account wants to tokenize
        ///
        /// Returns a tuple with:
        /// - `pt_bucket`: A bucket with the same number of the tokenized tokens (specifically, we return the PT token)
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// 
        /// Panics if:
        /// - The provided Bucket and the ResourceAddress does not match.
        /// - The NFT is not of the requested type.
        /// - The length of the tokenize period is too low/high
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/tokenize_yield.rtm`
        /// ```text
        #[doc = include_str!("../rtm/tokenize_yield.rtm")]
        /// ```     
        pub fn tokenize_yield(
            &mut self, 
            tkn_token: Bucket,
            tokenize_expected_length: Decimal,
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (FungibleBucket, NonFungibleBucket) {
            // assert_ne!(self.check_maturity(), true, "The expiry date has passed!");
            assert_eq!(tkn_token.resource_address(), self.tokenizer_manager.address());
            epoch_max_length_checks(self.tokenize_epoch_max_lenght,tokenize_expected_length);
            epoch_min(tokenize_expected_length);

            let zsu_amount = tkn_token.amount();
                
            //when you tokenize you fix the interest until the maturity date
            let extra_interest = self.extra_reward;
            info!("Tokenize for n° {} epoch with extra reward {} ", tokenize_expected_length, self.extra_reward); 
                    
            //mint some principal token
            let pt_bucket = self.pt_resource_manager.mint(zsu_amount).as_fungible();
            let maturity_epoch = Decimal::from(Runtime::current_epoch().number()) + tokenize_expected_length;
            //calculate interest
            let accumulated_interest = calculate_interest(tokenize_expected_length, extra_interest, zsu_amount);  
            info!("Simple Interest to be paied {} at epoch {} for the tokenized", accumulated_interest, maturity_epoch);
            
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                info!("Tokenize tokens of type  {:?}, amount {:?}", token_type, zsu_amount);
                info!("Principal returned = {:?}", data.principal_returned);
                if data.principal_returned==true {
                    //lock the tokens
                    self.tokenizer_vault.put(tkn_token);

                    //updates data on NFT       
                    data.extra_reward = self.extra_reward;
                    data.underlying_amount = zsu_amount;
                    data.interest_totals = accumulated_interest;
                    data.yield_claimed = Decimal::ZERO;
                    data.maturity_date = maturity_epoch;
                    data.principal_returned = false;
                    // Insert the modified data back into the hashmap
                    yield_data.insert(token_type.clone(), data);
                    self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                    return (pt_bucket, userdata_nft)
                } else {
                    assert_pair("You already have some tokenized liquidity ".to_string());
                    return (scrypto::prelude::FungibleBucket(tkn_token), userdata_nft)
                }
            } else {
                info!("No Yield Data available");
                //lock the tokens
                self.tokenizer_vault.put(tkn_token);                
                //updates data on NFT
                let strip = YieldTokenData {
                    extra_reward: self.extra_reward,
                    underlying_amount: zsu_amount,
                    interest_totals: accumulated_interest,
                    yield_claimed: Decimal::ZERO,
                    maturity_date: maturity_epoch,
                    principal_returned: false,
                };
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), strip);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);
                info!("New Yield Data has been creted in the account NFT");
                return (pt_bucket, userdata_nft)
            }
        }     

        /// Returns the required number of initial tokens provided by adding or subtracting the value resulting 
        /// from the calculation of the lesser or greater value due to the difference between the current interest rate and that of the initial moment
        /// 
        /// Arguments:
        /// - `pt_bucket`: A Bucket of tokens that the account wants to swap
        /// - `userdata_nft`: A NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account wants to swap
        ///
        /// Returns a tuple with:
        /// - `zsu_bucket`: A bucket with the same number of tokens provided (specifically, we return those that can be used to immediately withdraw the principal)
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// 
        /// Panics if:
        /// - The provided Bucket and the ResourceAddress does not match.
        /// - The NFT is not of the requested type.
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/redeem.rtm`
        /// ```text
        #[doc = include_str!("../rtm/redeem.rtm")]
        /// ```     
        pub fn redeem(
            &mut self, 
            pt_bucket: Bucket, 
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, Option<NonFungibleBucket>) {
            
            let pt_redeem_amount = pt_bucket.amount();
            assert_eq!(pt_bucket.resource_address(), self.pt_resource_manager.address());
            assert_eq!(userdata_nft.resource_address(), self.nft_manager.address());

            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(data) = yield_data.remove(&token_type) {
                info!("Swap tokens of type {:?}, amount locked {:?} until {:?}, now is epoch {} ", token_type, pt_redeem_amount, data.maturity_date, Runtime::current_epoch().number());
                info!("Tokenized amount {:?} and Extra reward at maturity {:?} ", data.underlying_amount, data.interest_totals);

                assert_eq!(pt_redeem_amount, data.underlying_amount, "You need to swap all your tokenized value!");
                // total at maturity * (1/(1+0.08)) -> 10,240 * 0.9259 -> 9,481
                // maucalay duration = 9,481/10,000 -> 0.948
                // modified duration = maucalay duration / (1+0.08) -> 0,948 / 1,08 = 0,877
                let fixed_reward = data.extra_reward/dec!(100);
                let total_at_maturity =  (data.underlying_amount+data.interest_totals) * (1/(1+fixed_reward));         //data.underlying_amount          
                info!("total at maturity {:?}", total_at_maturity);
                let maucalay_duration = total_at_maturity/data.underlying_amount; //data.underlying_amount
                info!("maucalay duration {:?}", maucalay_duration);
                let modified_duration = maucalay_duration / (1+fixed_reward);
                info!("modified duration {:?}", modified_duration);

                //differences in % from the time when the tokenize occurred and now
                //positive value means that % has been lowered -> PT value has risen
                //negative value means that % has been raised -> PT value has decreased
                let diff_reward = data.extra_reward - self.extra_reward;
                info!("Extra reward at the time of tokenize {}%, actual extra_reward {}%, diff_reward {:?}%", data.extra_reward, self.extra_reward, diff_reward);

                // Insert the cleaned data back into the hashmap for the next round of tokenize
                let yield_token = self.init_yield();
                yield_data.insert(token_type.clone(), yield_token);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                //burn principal token because they have been returned as an equivalent 
                pt_bucket.burn();
                //return back the amount priced at the current value
                let diff = diff_reward*modified_duration;
                info!("returned value is higher/lower of about {:?} %", diff);
                let priced_amount = (data.underlying_amount+data.interest_totals)*(dec!(100)+diff)/dec!(100);
                //The actualized price of the tokenized supply will be returned
                info!("tokens returned {:?}", priced_amount);
                //unlock the tokens                    
                let zsu_bucket = self.tokenizer_vault.take(priced_amount);

                return (zsu_bucket, Some(userdata_nft))
            } else {
                return (pt_bucket, Some(userdata_nft))
            }
        } 
                       
            

        /// This is for claiming principal token after maturity, you get back the principal that had been tozeniked
        /// 
        /// Returns the requested number of initial tokens provided (specifically, we return those tokens that can be used to immediately withdraw the principal)
        /// 
        /// Arguments:
        /// - `pt_bucket`: A Bucket of tokens that the account want to be returned back
        /// - `userdata_nft`: A NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account want to be returned back
        ///
        /// Returns a tuple with:
        /// - `token_received`: A bucket with the same number of tokens provided (specifically, we return those that were initially provided)
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// 
        /// Panics if:
        /// - The provided Bucket and the ResourceAddress does not match.
        /// - The maturity date has not been reached
        /// - The NFT is not of the requested type.
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/redeem_from_pt.rtm`
        /// ```text
        #[doc = include_str!("../rtm/redeem_from_pt.rtm")]
        /// ```     
        pub fn redeem_from_pt(
            &mut self,
            pt_bucket: FungibleBucket,
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, NonFungibleBucket) {

            info!("Returning PT amount {}", pt_bucket.amount());   
            assert_eq!(pt_bucket.resource_address(), self.pt_resource_manager.address());

            //update principal returned
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                // To redeem PT only, must wait until after maturity.
                assert_eq!(
                    check_maturity(data.maturity_date), 
                    false, 
                    "The Principal token has NOT reached its maturity!"
                );
                let zsu_amount = pt_bucket.amount();    
                info!("Reedem PT amount {:?} of type  {:?}", zsu_amount, token_type);

                //set that the principal has been returned
                data.principal_returned = true;
                data.extra_reward = dec!(0);
                data.underlying_amount = dec!(0);
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                //return the amount that was tokenized        
                let bucket_of_zsu = self.tokenizer_vault.take(zsu_amount);
                pt_bucket.burn();
   
                return (bucket_of_zsu, userdata_nft)
            } else {
                assert_pair("No PT available".to_string());
                return (pt_bucket.into(), userdata_nft)
            }
        }

        /// This is for claiming yield after maturity, you get back the interest calculated at the time of tozeniking
        /// 
        /// Returns the extra reward amount that has been calculated at the time of the tokenize operation
        /// 
        /// Arguments:
        /// - `userdata_nft`: A NFT for having it updated in its relevant fields
        /// - `token_type`: The ResourceAddress of the token that the account want to be returned back
        ///
        /// Returns a tuple with:
        /// - `net_returned`: A bucket with the extra reward 
        /// - `userdata_nft`: The NFT holding information about the account supplied/tokenized liquidity
        /// 
        /// 
        /// Panics if:
        /// - The maturity date has not been reached
        /// - The NFT is not of the requested type.
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/claim_yield.rtm`
        /// ```text
        #[doc = include_str!("../rtm/claim_yield.rtm")]
        /// ```     
        pub fn claim_yield(
            &mut self, 
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, NonFungibleBucket) {

            //update principal returned
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                // To claim yield only, must wait until after maturity.
                assert_eq!(
                    check_maturity(data.maturity_date), 
                    false, 
                    "The yield token has NOT reached its maturity!"
                );
                
                let interest_totals = data.interest_totals;
                info!("Paying back interest {} ", interest_totals);   
                //total net amount to return
                let net_returned = self.tokenizer_vault.take(interest_totals);
    
                //update claimed yield
                data.interest_totals = dec!(0);
                data.yield_claimed = interest_totals;
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                (net_returned, userdata_nft)
            } else {
                assert_pair("No Yield available".to_string());
                let net_returned = self.tokenizer_vault.take(dec!(0));
                return (net_returned, userdata_nft)
            }
        }
        
        /// Utility function: Set the reward for suppliers
        /// 
        /// Arguments:
        /// - `reward`: The new 'interest rate' for account supplying liquidity
        /// 
        /// The new 'interest rate' is stored in the balanced binary search tree for then be used for reward calculations
        /// 
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///
        /// **Transaction manifest:**
        /// `rtm/set_reward.rtm`
        /// ```text
        #[doc = include_str!("../rtm/set_reward.rtm")]
        /// ```             
        pub fn set_reward(&mut self, reward: Decimal) {
            self.reward = reward;
            self.interest_for_suppliers.insert(Decimal::from(Runtime::current_epoch().number()), reward);
        }

        /// Utility function: Set the extra reward for accounts locking their supplied liquidity
        /// 
        /// Arguments:
        /// - `extra_reward`: The new extra 'interest rate' for accounts that will tokenize their liquidity provided from now on
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///
        /// **Transaction manifest:**
        /// `rtm/set_extra.rtm`
        /// ```text
        #[doc = include_str!("../rtm/set_extra.rtm")]
        /// ```                     
        pub fn set_extra_reward(&mut self, extra_reward: Decimal) {
            self.extra_reward = extra_reward;
        }

        /// Utility function: Set the reward type, if fixed or timebased
        /// 
        /// For this specific blueprint customized for this challenge this function has not be used.
        /// No Transaction manifest has been provided
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///                 
        pub fn set_reward_type(&mut self, reward_type: String) {
            self.reward_type = reward_type
        }

        /// Utility function: Increases the level of liquidity that the contract is able to accept
        /// 
        /// Arguments:
        /// - `size_extended`: The number of additional tokens that the contract will accept from now on
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///
        /// **Transaction manifest:**
        /// `rtm/extend_lending_pool.rtm`
        /// ```text
        #[doc = include_str!("../rtm/extend_lending_pool.rtm")]
        /// ```                   
        pub fn extend_lending_pool(&mut self, size_extended: Decimal) {
            self.tokenizer_vault.put(self.tokenizer_manager.mint(size_extended));
        }

        /// Utility function: For funding the main pool
        /// 
        /// It is outside the scope of this specific challenge to describe how the liquidity received is used. 
        /// However for the specific purpose of this challenge this function is provided to allow the contract to 'pay out' rewards to accounts that provide or tokenized their liquidity
        /// 
        /// Arguments:
        /// - `fund`: A Bucket of tokens that the account want to be returned back
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///
        /// **Transaction manifest:**
        /// `rtm/fund.rtm`
        /// ```text
        #[doc = include_str!("../rtm/fund.rtm")]
        /// ```                   
        pub fn fund_main_pool(&mut self, fund: FungibleBucket)  {
            info!("Fund received to fund the main vault: {:?} ", fund.amount());  
            let token_type = fund.resource_address();

            //take the bucket and put tokens in corresponding pool
            let vault = self.collected.get_mut(&token_type.clone());
            match vault {
                Some(fung_vault) => {
                    info!("Receiving a supply of {:?}, amount {:?}", token_type, fund.amount());
                    info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                    fung_vault.put(fund);
                }
                None => {
                    info!("Unavailable pair ");
                }
            }
        }

        /// Utility function: Set parameters 
        /// 
        /// Arguments:
        /// - `reward`: It is the reward for account that supply liquidity in
        /// - `extra_reward`: It is the extra reward for account that will block liquidity until a maturity date
        /// - `tokenize_epoch_max_lenght`: Max length of the tokenize operation (around 1 year)
        /// - `min_loan_limit`: Min amount of tokens that an account can supply in
        /// - `max_loan_limit`: Max amount of tokens that an account can supply in
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin or the Staff only.            
        pub fn config(&mut self, reward: Decimal, extra_reward: Decimal
                , tokenize_epoch_max_lenght: Decimal
                , min_loan_limit: Decimal, max_loan_limit: Decimal ) {                
            self.set_reward(reward);
    
            //without methods
            self.tokenize_epoch_max_lenght = tokenize_epoch_max_lenght;
            self.extra_reward = extra_reward;
            self.min_loan_limit = min_loan_limit; //min limit 
            self.max_loan_limit = max_loan_limit; //max limit 
       
            assert!(
                max_loan_limit > min_loan_limit,
                "Maximum number of tokens must be higher than Min limit"
            );                                           
        }      

        /// Internal function: Initialized YieldTokenData Struct
        fn init_yield(&mut self) -> YieldTokenData {
            return YieldTokenData {
                extra_reward: dec!(0),
                underlying_amount: dec!(0),
                interest_totals: dec!(0),
                yield_claimed: dec!(0),
                maturity_date: dec!(0),
                principal_returned: true,
            };    
        }

        /// Internal function: Initialized LiquidityData Struct (at the time of registering the account)
        fn init_liq_data(&mut self) -> LiquidityData {
            return LiquidityData {
                start_supply_epoch: Epoch::of(0),
                end_supply_epoch: Epoch::of(0),
                amount: dec!("0"),
            };
        }   

        /// Internal function: Initialized LiquidityData Struct (at the time of the first supply of a new token)
        fn new_liq_data(&mut self,start_supply_epoch: Epoch,end_supply_epoch: Epoch,amount: Decimal) -> LiquidityData {
            return LiquidityData {
                start_supply_epoch: start_supply_epoch,
                end_supply_epoch: end_supply_epoch,
                amount: amount
            };
        }         

        /// Utility function: Mint a staff badge for a new staff member
        /// 
        /// Arguments:
        /// - `username`: Username that will be registered in the NFT
        /// ---
        ///
        /// **Access control:** Can be called by the Owner or the Admin only.
        ///                    
        pub fn mint_staff_badge(&mut self, username: String) -> Bucket {
            let staff_badge_bucket: Bucket = self
                .staff_badge_resource_manager
                .mint_ruid_non_fungible(StaffBadge {
                    username: username.clone(),
                });

            let id = staff_badge_bucket.as_non_fungible().non_fungible_local_id();
            let key = self.staff.get_length().to_u16().unwrap()+1; 
            info!("Saving staff badge with key : {:?} and id {:?} for the username: {:?}  ",key, id, username);
            self.staff.insert(key, id);

            staff_badge_bucket
        }
    
    }
}