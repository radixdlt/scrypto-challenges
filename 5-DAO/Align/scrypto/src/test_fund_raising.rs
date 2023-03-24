/*!
This is a test blueprint to implement Test Fund Raising platform component.

The blueprint is only for testing the Align DAO blueprint package and should not be used on practice.

This blueprint doesn't have to be on the same package as the Align package
(this is just for saving time publish one package instead of two).
*/

use scrypto::prelude::*;

blueprint! {
    struct TestFundraising {
        /// Name of the fundraising
        name: String,
        /// Controller badge of the FundRaising component
        controller_badge: Vault,
        /// Store the fund
        fund_vault: Vault,
        /// Store the bond token address
        bond: ResourceAddress,
        /// Total funded
        funded: Decimal,
        /// Fee on fund management.
        fee: Decimal,
    }

    impl TestFundraising {
        pub fn new(
            name: String,
            owner: ResourceAddress,
            fund_resource: ResourceAddress,
            fee: Decimal,
        ) -> ComponentAddress {
            let controller_badge = ResourceBuilder::new_fungible().initial_supply(1);

            let bond = ResourceBuilder::new_fungible()
                .metadata("name", "Bond")
                .metadata("symbol", "BOND")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("use_fund", rule!(require(owner)))
                .method("report_loss", rule!(require(owner)))
                .method("change_fee", rule!(require(owner)))
                .default(rule!(allow_all));

            let mut comp = Self {
                name,
                controller_badge: Vault::with_bucket(controller_badge),
                fund_vault: Vault::new(fund_resource),
                bond: bond,
                funded: Decimal::ZERO,
                fee: fee / dec!(100),
            }
            .instantiate();

            comp.add_access_check(rules);

            let address = comp.globalize();

            info!("[TestFundraising]: TestFundraising Fee: {}%", fee);
            info!("[TestFundraising]: TestFundraising Bond: {}", bond);
            info!("[TestFundraising]: TestFundraising Component: {}", address);

            address
        }

        pub fn invest(&mut self, investment: Bucket) -> Bucket {
            let amount = investment.amount();

            let bond_amount = self.calc(amount);

            let bond = self
                .controller_badge
                .authorize(|| borrow_resource_manager!(self.bond).mint(bond_amount));

            self.fund_vault.put(investment);

            self.funded += amount;

            info!(
                "[TestFundraising]: invested {} reserve token for {} bond on {}",
                amount, bond_amount, self.name
            );

            bond
        }

        pub fn claim_profit(&mut self, bond: Bucket) -> Bucket {
            let bond_amount = bond.amount();

            let amount = self.calc2(bond_amount);

            assert!(
                amount < self.fund_vault.amount(),
                "[TestFundraising]: Not enough fund in the vault right now"
            );

            self.controller_badge.authorize(|| bond.burn());

            self.funded -= amount;

            info!(
                "[TestFundraising]: reclaimed {} reserve token by selling {} bond on {}",
                amount, bond_amount, self.name
            );

            self.fund_vault.take(amount)
        }

        pub fn bond(&self) -> ResourceAddress {
            self.bond
        }

        pub fn use_fund(&mut self, amount: Decimal) -> Bucket {
            assert!(
                amount < self.fund_vault.amount(),
                "[TestFundraising]: Not enough fund in the vault right now"
            );
            self.fund_vault.take(amount)
        }

        pub fn profit(&mut self, profit_bucket: Bucket) {
            let profit = profit_bucket.amount() - self.funded + self.fund_vault.amount();
            if profit > Decimal::ZERO {
                self.funded += profit;
            }
            self.fund_vault.put(profit_bucket)
        }

        pub fn report_loss(&mut self, amount: Decimal) {
            self.funded -= amount
        }

        pub fn change_fee(&mut self, fee: Decimal) {
            info!("[TestFundraising]: New protocol's fee: {}", fee);
            self.fee = fee / dec!(100)
        }

        fn calc(&self, amount: Decimal) -> Decimal {
            let mgr = borrow_resource_manager!(self.bond);
            let total_supply = mgr.total_supply();
            if total_supply == Decimal::ZERO {
                amount
            } else {
                amount / self.funded * total_supply * (dec!(1) - self.fee)
            }
        }

        fn calc2(&self, bond_amount: Decimal) -> Decimal {
            let mgr = borrow_resource_manager!(self.bond);
            let total_supply = mgr.total_supply();
            bond_amount / total_supply * self.funded * (dec!(1) - self.fee)
        }
    }
}
