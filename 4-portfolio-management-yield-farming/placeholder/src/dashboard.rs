use scrypto::prelude::*;
use radex::radex::*;
use degenfi::degenfi::*;
use crate::structs::*;
use crate::index_fund::*;
use crate::price_oracle::*;

blueprint! {
    struct MainDashboard {
        
        admin_vault: Vault,
        asset_manager_address: ResourceAddress,
        fund_lockers: HashMap<String, ComponentAddress>,
        price_oracle_address: ComponentAddress,
        radex_address: ComponentAddress,
        degenfi_address: ComponentAddress,
    }

    impl MainDashboard {
        

        pub fn new() -> ComponentAddress {
            
            let admin = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .metadata("symbol", "AB")
                .metadata("description", "Component Admin authority")
                .initial_supply(1);

            let asset_manager_address: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Asset Manager NFT")
                .metadata("symbol", "AM_NFT")
                .metadata("description", "Pool Delegate Admin Badge")
                .mintable(rule!(require(admin.resource_address())), LOCKED)
                .burnable(rule!(require(admin.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin.resource_address())), LOCKED)
                .no_initial_supply();


            Self {
                admin_vault: Vault::with_bucket(admin),
                asset_manager_address: asset_manager_address,
                fund_lockers: HashMap::new(),
                price_oracle_address: PriceOracle::new(),
                radex_address: RaDEX::new(),
                degenfi_address: DegenFi::new(),
            }
            .instantiate()
            .globalize()
        }

        pub fn create_am_badge(
            &mut self
        ) -> Bucket
        {

            let am_badge = self.admin_vault.authorize(|| {
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.asset_manager_address);
                resource_manager.mint_non_fungible(
                    // The User id
                    &NonFungibleId::random(),
                    // The User data
                    User {
                        funds_managed: HashMap::new(),
                        funds_invested: HashMap::new(),
                    },
                )
            });

            info!("[Maple Finance]: The resource address of your Asset Manager badge is: {:?}", am_badge.resource_address());

            am_badge
        }

        pub fn create_fund(
            &mut self,
            fund_type: FundType,
            fund_name: String,
            fund_ticker: String,
            starting_share_price: Decimal,
            leverage: bool,
            liquidity_provider: bool,
            governance: bool,
            tokens: HashMap<ResourceAddress, Decimal>,
        )
        {
            assert_ne!(self.fund_lockers.contains_key(&fund_ticker), true, 
                "The ticker for this fund already exist. Please choose another."
            );

            let fund_ticker2 = fund_ticker.clone();

            let price_oracle_address: ComponentAddress = self.price_oracle_address;
            let radex_address: ComponentAddress = self.radex_address;
            let degenfi_address: ComponentAddress = self.degenfi_address;

            let fund_locker: ComponentAddress = IndexFund::new(
                fund_name, 
                fund_ticker, 
                starting_share_price,
                leverage, 
                liquidity_provider,
                governance,
                tokens,
                price_oracle_address,
                radex_address,
                degenfi_address,
            );

            self.fund_lockers.insert(fund_ticker2, fund_locker);
        }

        pub fn issue_tokens(
            &mut self,
            fund_ticker: String,
            // issue_amount: Decimal,
            tokens: Vec<Bucket>,
        )
        {
            assert_eq!(self.fund_lockers.contains_key(&fund_ticker), true, 
                "The fund you are looking for does not exist."
            );

            let fund_locker_address = *self.fund_lockers.get_mut(&fund_ticker).unwrap();

            let fund_locker: IndexFund = fund_locker_address.into();

            fund_locker.issue_tokens(tokens);

        }

    }
}