use scrypto::prelude::*;
use crate::maple_finance_global::*;

// Allows approved Pool Delegate to manage pools.

blueprint! {
    struct InvestorDashboard {
        investor_address: ResourceAddress,
        maple_finance_global_address: ComponentAddress,
    }

    impl InvestorDashboard {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
            investor_address: ResourceAddress
        ) -> ComponentAddress
        {
            return Self {
                investor_address: investor_address,
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
            investor_badge: Proof,
            fund_name: String,
            xrd: Bucket,
        ) -> Bucket
        {
            assert_eq!(invest_badge.resource_address(), self.investor_address,
                "[Investor Dashboard]: This badge does not belong to this protocol."
            );

            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let fund_tokens: Bucket = index_fund.buy(xrd);

            fund_tokens
        }

        pub fn sell_fund_tokens(
            &mut self,
            invest_badge: Proof,
            fund_name: String,
            fund_token: Bucket,
        ) -> Bucket
        {
            assert_eq!(invest_badge.resource_address(), self.investor_address,
                "[Investor Dashboard]: This badge does not belong to this protocol."
            );

            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Bucket = index_fund.sell(fund_token);

            xrd_bucket
        }

    }
}