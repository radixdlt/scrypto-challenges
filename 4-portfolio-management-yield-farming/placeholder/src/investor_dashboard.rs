use scrypto::prelude::*;
use crate::farmers_market::*;
use crate::index_fund::*;
use crate::debt_fund::*;

// Allows approved Pool Delegate to manage pools.
// Even if there's a rogue actor who inputs the ResourceAddress of the badge to this component.
// They can't interact with the protocol as they need proof of the badge, which is created and claimed
// via global. 
blueprint! {
    struct InvestorDashboard {
        farmers_market_address: ComponentAddress,
    }

    impl InvestorDashboard {

        pub fn new(
            farmers_market_address: ComponentAddress,
        ) -> ComponentAddress
        {
            return Self {
                farmers_market_address: farmers_market_address,
            }
            .instantiate()
            .globalize();
        }

        /// This method is used to retrieve a list of all the Index Fund created in this protocol.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<(String, String), ComponentAddress>` - The HashMap of the fund_id (fund name, fund ticker) and the
        /// associated ComponentAddress.
        pub fn retrieve_index_funds_lists(
            &mut self,
        ) -> HashMap<(String, String), ComponentAddress>
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            return farmers_market.index_fund_list();
        }

        /// This method is used to allow investors to buy a stake of the Index Fund.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund the investor wishes to buy fund tokens from.
        /// * `xrd` (Bucket) - The Bucket that contains the XRD to purchase fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the fund tokens.
        pub fn buy_fund_tokens(
            &mut self,
            fund_name: String,
            xrd: Bucket,
        ) -> Bucket
        {

            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let index_fund_address: ComponentAddress = farmers_market.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let fund_tokens: Bucket = index_fund.buy(xrd);

            fund_tokens
        }

        /// This method allows investors to sell their fund tokens in exchange for equivalent value in XRD.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund the investor wishes to sell fund tokens from.
        /// * `fund_token` (Bucket) - The Bucket that contains the fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the XRD.
        pub fn sell_fund_tokens(
            &mut self,
            fund_name: String,
            fund_token: Bucket,
        ) -> Bucket
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let index_fund_address: ComponentAddress = farmers_market.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Bucket = index_fund.sell(fund_token);

            xrd_bucket
        }

        /// This method allows investors who already have the underlying asset of the Index Fund and convert them to equivalent
        /// value in fund tokens.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund the investor wishes to issue tokens from.
        /// * `tokens` (Vec<Bucket>) - The vector of Buckets that contains the underlying assets.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the fund tokens.
        pub fn issue_tokens(
            &mut self,
            fund_name: String,
            tokens: Vec<Bucket>,
        ) -> Bucket
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let index_fund_address: ComponentAddress = farmers_market.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let bucket: Bucket = index_fund.issue_tokens(tokens);

            bucket
        }

        /// This method allows investors to redeem the fund tokens for the underlying asset of the given Index Fund.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund the investor wishes to issue tokens from.
        /// * `fund_tokens` (Bucket) - The Bucket that contains the fund tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Vec<Bucket>` - The vector of Buckets that contains the underlying assets.
        pub fn redeem_fund_tokens(
            &mut self,
            fund_name: String,
            fund_tokens: Bucket,
        ) -> Vec<Bucket>
        {
            
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let index_fund_address: ComponentAddress = farmers_market.get_index_fund(fund_name);
            let index_fund: IndexFund = index_fund_address.into();
            let xrd_bucket: Vec<Bucket> = index_fund.redeem(fund_tokens);

            xrd_bucket
        }

        /// Retrieves all the debt funds created in this protocol.
        /// 
        /// This method does not perform any checks.
        /// 
        /// This method does not accept any arguments.
        /// 
        /// # Returns:
        /// 
        /// * `HashMap<String, ComponentAddress>` - The HashMap of the debt fund name and its associated
        /// ComponentAddress.
        pub fn retrieve_debt_funds_list(
            &self
        ) -> HashMap<String, ComponentAddress>
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            farmers_market.debt_fund_list()
        }

        /// This method allows investors to supply liquidity of the chosen Debt Fund.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `fund_name` (String) - The name of the fund the investor wishes to supply liquidity to.
        /// * `liquidity_amount` (Bucket) - The Bucket that contains the liquidity supply.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the tracking tokens.
        pub fn supply_liquidity(
            &mut self,
            fund_name: String,
            liquidity_amount: Bucket,
        ) -> Bucket
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let debt_fund: DebtFund = farmers_market.get_debt_fund(fund_name).into();
            let tracking_tokens: Bucket = debt_fund.supply_liquidity(liquidity_amount);

            tracking_tokens
        }

        /// This method allows investors to remove liquidity from the Debt Fund they supplied liquidity to.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_tokens` (Bucket) - The Bucket that contains the tracking tokens.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The Bucket that contains the liquidity supply.
        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: Bucket
        ) -> Bucket
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let fund_name: String = farmers_market.get_tracking_tokens_mapping(tracking_tokens.resource_address());
            let debt_fund: DebtFund = farmers_market.get_debt_fund(fund_name).into();
            let return_liquidity: Bucket = debt_fund.remove_liquidity(tracking_tokens);

            return_liquidity
        }

        /// This methid allows investors to claim fees from the Debt Fund they supplied liquidity to.
        /// 
        /// This method does not perform any checks. The checks are performed in the in the IndexFund
        /// component.
        /// 
        /// # Arguments:
        /// 
        /// * `tracking_tokens` (Bucket) - The Bucket that contains the traciking token.
        /// 
        /// # Returns:
        /// 
        /// * `(Vec<Bucket>, Bucket) - The vector of buckets of all the fees from the Debt Fund and the tracking tokens
        /// returned back to the investor.
        pub fn claim_fees(
            &mut self,
            tracking_tokens: Bucket
        ) -> (Vec<Bucket>, Bucket)
        {
            let farmers_market: FarmersMarket = self.farmers_market_address.into();
            let fund_name: String = farmers_market.get_tracking_tokens_mapping(tracking_tokens.resource_address());
            let debt_fund: DebtFund = farmers_market.get_debt_fund(fund_name).into();
            let tracking_tokens_proof: Proof = tracking_tokens.create_proof();
            let fees: Vec<Bucket> = debt_fund.claim_fees(tracking_tokens_proof);

            return (fees, tracking_tokens);
        }

    }
}