use scrypto::prelude::*;

#[derive(
    Debug, ScryptoCategorize, ScryptoEncode, ScryptoDecode, PartialEq, Eq, Clone, LegacyDescribe
)]

pub struct User {
    pub user_badge_resource_address: ResourceAddress,
    pub deposit_balances: HashMap<ResourceAddress, Deposit>,
    pub borrow_balances: HashMap<ResourceAddress, Borrow>,
}

impl User {

    pub fn get_asset_borrow_balance(&self, resource_address: ResourceAddress) -> Decimal {
        let balance = match self.borrow_balances.get(&resource_address) {
            Some(current_balance) => current_balance.balance,
            None => Decimal::from(0),
        };
        balance
    }

    pub fn get_asset_deposit_balance(&self, resource_address: ResourceAddress) -> Decimal {
        let balance = match self.deposit_balances.get(&resource_address) {
            Some(current_balance) => current_balance.balance,
            None => Decimal::from(0),
        };
        balance
    }

    pub fn on_deposit(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        info!(
            "Increasing deposit balance for asset `{:?}` by `{:?}`.",
            resource_address,
            amount
        );

        match self.deposit_balances.get_mut(&resource_address) {
            Some(_current_balance) => {}
            None => {
                let deposit = Deposit {
                    balance: amount, 
                    last_update: Runtime::current_epoch(),
                };
                self.deposit_balances.insert(resource_address, deposit);
                info!(
                    "Created new deposit balance for asset `{:?}` is `{:?}`",
                    resource_address, amount
                );
            }
        };
    }

    pub fn on_withdraw(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        info!(
            "Decreasing deposit balance for asset `{:?}` by `{:?}`.",
            resource_address,
            amount
        );
        match self.deposit_balances.get_mut(&resource_address) {
            Some(current_balance) => {
                assert!(
                    current_balance.balance >= amount,
                    "[User] Cannot create a negative balance"
                );

                let old_balance = current_balance.balance;
                let new_balance = current_balance.balance - amount;

                let deposit = Deposit {
                    balance: new_balance, 
                    last_update: Runtime::current_epoch(),
                };

                self.deposit_balances.insert(resource_address, deposit);
                info!(
                    "Updated deposit balance for asset `{:?}` from `{:?}` to `{:?}`",
                    resource_address, old_balance, new_balance
                );
            }
            None => {
                panic!(
                    "Can't decrease balance of asset `{:?}`.",
                    resource_address
                );
            }
        };
    }

    pub fn on_borrow(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        info!(
            "Increasing borrow balance for asset `{:?}` by `{:?}`.",
            resource_address,
            amount
        );
        match self.borrow_balances.get(&resource_address) {
            Some(_current_balance) => {}
            None => {
                let borrow = Borrow {
                    balance: amount, 
                    last_update: Runtime::current_epoch(),
                };

                self.borrow_balances.insert(resource_address, borrow);

                info!(
                    "Created new borrow balance for asset `{:?}` is `{:?}`",
                    resource_address, amount
                );
            }
        };
    }

    pub fn on_repay(&mut self, amount: Decimal, asset_address: ResourceAddress, interest_rate: Decimal) -> Decimal {

        match self.borrow_balances.get_mut(&asset_address) {
            Some(current_balance) => {

            // Increase borrow balance by interests accrued
            let interest = current_balance.balance * interest_rate * current_balance.borrow_time_elapsed();
            current_balance.balance += interest;
            current_balance.last_update = Runtime::current_epoch();

            // Repay the loan
            if current_balance.balance < amount {
                let to_return = amount - current_balance.balance;
                current_balance.balance = Decimal::zero();
                to_return
            } else {
                current_balance.balance -= amount;
                Decimal::zero()
            }
        }
            None => {
                Decimal::zero()
            }
        }
    }

    pub fn on_liquidate(&mut self, amount: Decimal, bonus_percent: Decimal, interest_rate: Decimal) -> Decimal {
        let changes = self.on_repay(amount, self.user_badge_resource_address, interest_rate);
        assert!(changes == 0.into());

        // TODO add exchange rate here when collaterals and borrows are different

        let to_return = amount * (bonus_percent + 1);
        match self.deposit_balances.get_mut(&self.user_badge_resource_address) {
            Some(current_balance) => {
                current_balance.balance -= to_return;
            }
                None => ()
        }
        
        to_return
    }

    pub fn get_collateral_ratio(&mut self, interest_rate: Decimal) -> Option<Decimal> {

        let loan: Decimal = match self.borrow_balances.get_mut(&self.user_badge_resource_address) {
            Some(current_balance) => {
                current_balance.balance + current_balance.balance * interest_rate * current_balance.borrow_time_elapsed()
        }
            None => {
                Decimal::zero()
            }
        };

        let collateral = match self.deposit_balances.get_mut(&self.user_badge_resource_address) {
            Some(current_balance) => {
                current_balance.balance + current_balance.balance * interest_rate * current_balance.deposit_time_elapsed()
        }
            None => {
                Decimal::zero()
            }
        };

        
        Some(collateral / loan)
    }

}

#[derive(
    Debug, ScryptoCategorize, ScryptoEncode, ScryptoDecode, PartialEq, Eq, Clone, LegacyDescribe
)]
pub struct Deposit {
    pub balance: Decimal, 
    //pub interest_rate: Decimal, // ne treba interest rate
    pub last_update: u64,
}

impl Deposit {
    pub fn deposit_time_elapsed(&self) -> u64 {
        // +1 is for demo purpose only
        Runtime::current_epoch() - self.last_update + 1
    }
}

#[derive(
    Debug, ScryptoCategorize, ScryptoEncode, ScryptoDecode, PartialEq, Eq, Clone, LegacyDescribe
)]
pub struct Borrow {
    pub balance: Decimal, 
    //pub interest_rate: Decimal, // ne treba interest rate
    pub last_update: u64,
}

impl Borrow {
    pub fn borrow_time_elapsed(&self) -> u64 {
        // +1 is for demo purpose only
        Runtime::current_epoch() - self.last_update + 1
    }
}