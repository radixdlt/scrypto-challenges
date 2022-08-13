use scrypto::prelude::*;
use crate::maple_finance_global::*;
use crate::index_fund::*;

// Allows approved Pool Delegate to manage pools.
// Even if there's a rogue actor who inputs the ResourceAddress of the badge to this component.
// They can't interact with the protocol as they need proof of the badge, which is created and claimed
// via global. 
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
            assert_eq!(investor_badge.resource_address(), self.investor_address,
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
            investor_badge: Proof,
            fund_name: String,
            fund_token: Bucket,
        ) -> Bucket
        {
            assert_eq!(investor_badge.resource_address(), self.investor_address,
                "[Investor Dashboard]: This badge does not belong to this protocol."
            );

            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Bucket = index_fund.sell(fund_token);

            xrd_bucket
        }

        pub fn issue_tokens(
            &mut self,
            investor_badge: Proof,
            fund_name: String,
            tokens: Vec<Bucket>,
        )
        {
            assert_eq!(investor_badge.resource_address(), self.investor_address,
                "[Investor Dashboard]: This badge does not belong to this protocol."
            );
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let _xrd_bucket: Bucket = index_fund.issue_tokens(tokens);
        }

        pub fn redeem_fund_tokens(
            &mut self,
            investor_badge: Proof,
            fund_name: String,
            fund_tokens: Bucket,
        ) -> Vec<Bucket>
        {
            assert_eq!(investor_badge.resource_address(), self.investor_address,
                "[Investor Dashboard]: This badge does not belong to this protocol."
            );
            
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Vec<Bucket> = index_fund.redeem(fund_tokens);

            xrd_bucket
        }

    }
}