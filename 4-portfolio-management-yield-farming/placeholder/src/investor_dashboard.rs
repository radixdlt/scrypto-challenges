use scrypto::prelude::*;
use crate::maple_finance_global::*;
use crate::index_fund::*;
use crate::debt_fund::*;

// Allows approved Pool Delegate to manage pools.
// Even if there's a rogue actor who inputs the ResourceAddress of the badge to this component.
// They can't interact with the protocol as they need proof of the badge, which is created and claimed
// via global. 
blueprint! {
    struct InvestorDashboard {
        maple_finance_global_address: ComponentAddress,
    }

    impl InvestorDashboard {

        pub fn new(
            maple_finance_global_address: ComponentAddress,
        ) -> ComponentAddress
        {
            return Self {
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
            xrd: Bucket,
        ) -> Bucket
        {

            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let fund_tokens: Bucket = index_fund.buy(xrd);

            fund_tokens
        }

        pub fn sell_fund_tokens(
            &mut self,
            fund_name: String,
            fund_token: Bucket,
        ) -> Bucket
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Bucket = index_fund.sell(fund_token);

            xrd_bucket
        }

        pub fn issue_tokens(
            &mut self,
            fund_name: String,
            tokens: Vec<Bucket>,
        )
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let _xrd_bucket: Bucket = index_fund.issue_tokens(tokens);
        }

        pub fn redeem_fund_tokens(
            &mut self,
            fund_name: String,
            fund_tokens: Bucket,
        ) -> Vec<Bucket>
        {
            
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let index_fund_address: ComponentAddress = maple_finance_global.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Vec<Bucket> = index_fund.redeem(fund_tokens);

            xrd_bucket
        }

        pub fn retrieve_debt_funds_list(
            &self
        ) -> HashMap<String, ComponentAddress>
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            maple_finance_global.debt_fund_list()
        }

        pub fn supply_liquidity(
            &mut self,
            fund_name: String,
            liquidity_amount: Bucket,
        ) -> Bucket
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let debt_fund: DebtFund = maple_finance_global.get_debt_fund(fund_name).into();
            let tracking_tokens: Bucket = debt_fund.supply_liquidity(liquidity_amount);

            tracking_tokens
        }

        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: Bucket
        ) -> Bucket
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let fund_name: String = maple_finance_global.get_tracking_tokens_mapping(tracking_tokens.resource_address());
            let debt_fund: DebtFund = maple_finance_global.get_debt_fund(fund_name).into();
            let return_liquidity: Bucket = debt_fund.remove_liquidity(tracking_tokens);

            return_liquidity
        }

        pub fn claim_fees(
            &mut self,
            tracking_tokens: Bucket
        ) -> (Vec<Bucket>, Bucket)
        {
            let maple_finance_global: MapleFinance = self.maple_finance_global_address.into();
            let fund_name: String = maple_finance_global.get_tracking_tokens_mapping(tracking_tokens.resource_address());
            let debt_fund: DebtFund = maple_finance_global.get_debt_fund(fund_name).into();
            let tracking_tokens_proof: Proof = tracking_tokens.create_proof();
            let fees: Vec<Bucket> = debt_fund.claim_fees(tracking_tokens_proof);

            return (fees, tracking_tokens);
        }

    }
}