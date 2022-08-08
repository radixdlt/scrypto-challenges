use scrypto::prelude::*;
use crate::maple_finance_global::*;
use crate::{structs::*, fundinglocker::FundingLocker};

// Allows approved Pool Delegate to manage pools.

blueprint! {
    struct InvestorDashboard {
        borrower_admin_address: ResourceAddress,
        borrower_id: NonFungibleId,

        funding_lockers: HashMap<NonFungibleId, ComponentAddress>,
        maple_finance_global_address: ComponentAddress,
    }

    impl InvestorDashboard {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
            borrower_admin_address: ResourceAddress,
            borrower_id: NonFungibleId,
            loan_request_nft_admin: Bucket) -> ComponentAddress
        {

            return Self {
                borrower_admin_address: borrower_admin_address,
                borrower_id: borrower_id,
                funding_lockers: HashMap::new(),
                maple_finance_global_address: maple_finance_global_address,
            }
            .instantiate()
            .globalize();
        }

        pub fn retrieve_index_funds_lists(
            &mut self,
        ) -> HashMap<(String, String), ComponentAddress>
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            return maple_finance_global.index_fund_list();
        }

        pub fn buy_fund_tokens(
            &mut self,
            fund_name: String,
            amount: Bucket,
        )
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);   
        }

    }
}