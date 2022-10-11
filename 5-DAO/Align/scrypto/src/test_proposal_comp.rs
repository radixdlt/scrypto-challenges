/*!
This is a test proposal blueprint to test the Align DAO blueprint package through all the TestFundRaising blueprint function and method.

Since there's no previous pratice of feeding a transaction manifest into Scrypto or using Worktop through Scrypto,
to call a complex proposal, we have to publish a proposal blueprint instead.
This blueprint doesn't have to be on the same package as
the Align package (this is just for saving time publish one package instead of two).

The blueprint is only for testing, should not use this on practice!
*/
use crate::test_fund_raising::TestFundraisingComponent;
use scrypto::prelude::*;

external_component! {

    GroundBusinessDAO {
        fn deposit(&mut self, a: Bucket);
        fn dao_withdraw_by_amount(&mut self, a: ResourceAddress, b: Decimal) -> Bucket;
    }

}

external_component! {

    Account {
        fn deposit(&mut self, a: Bucket);
    }

}

blueprint! {

    struct TestProposal {
        project_lead: ComponentAddress,
        dao_badge_address: ResourceAddress,
        primary_reserve_resource: ResourceAddress,
        dao: ComponentAddress,
        my_protocol: ComponentAddress,
    }

    impl TestProposal {
        pub fn new(
            project_lead: ComponentAddress,
            dao_badge_address: ResourceAddress,
            primary_reserve_resource: ResourceAddress,
            dao: ComponentAddress,
            name: String,
            fee: Decimal,
        ) {
            let address = TestFundraisingComponent::new(
                name.clone(),
                dao_badge_address,
                primary_reserve_resource,
                fee,
            );

            let address = Self {
                project_lead,
                dao_badge_address,
                primary_reserve_resource,
                dao,
                my_protocol: address,
            }
            .instantiate()
            .globalize();

            info!(
                "[TestProposal]: Created new test proposal component: {}",
                address
            )
        }

        pub fn invest_through_dao(&self, amount: Decimal) {
            let mut dao: GroundBusinessDAO = self.dao.into();
            let stable_coin = dao.dao_withdraw_by_amount(self.primary_reserve_resource, amount);
            let protocol: TestFundraisingComponent = self.my_protocol.into();
            let bond = protocol.invest(stable_coin);
            dao.deposit(bond);
        }

        pub fn assign_fund(&self, amount: Decimal) {
            let protocol: TestFundraisingComponent = self.my_protocol.into();
            let fund = protocol.use_fund(amount);
            let mut account: Account = self.project_lead.into();
            account.deposit(fund)
        }
    }
}
