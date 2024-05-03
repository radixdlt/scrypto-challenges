use scrypto::prelude::*;

#[derive(Debug, ScryptoSbor, PartialEq, Eq, Clone)]

pub struct User {
    pub user_badge_resource_address: ResourceAddress,
    pub deposit_balances: HashMap<ResourceAddress, Deposit>,
}
impl User {
    pub fn on_deposit(
        &mut self,
        resource_address: ResourceAddress,
        amount: Decimal,
        yield_amount: Decimal,
    ) {
        let now = Clock::current_time_rounded_to_minutes();
        match self.deposit_balances.get_mut(&resource_address) {
            Some(_current_balance) => {
                let deposit = Deposit {
                    principal_balance: amount,
                    yield_balance: yield_amount,
                    deposited_at: now,
                };

                self.deposit_balances.insert(resource_address, deposit);
            }
            None => {
                let deposit = Deposit {
                    principal_balance: amount,
                    yield_balance: yield_amount,
                    deposited_at: now,
                };
                self.deposit_balances.insert(resource_address, deposit);
            }
        };
    }

    pub fn on_redeem(&mut self, resource_address: ResourceAddress) {
        let now = Clock::current_time_rounded_to_minutes();
        match self.deposit_balances.get_mut(&resource_address) {
            Some(_current_balance) => {
                let deposit = Deposit {
                    principal_balance: Decimal::ZERO,
                    yield_balance: Decimal::ZERO,
                    deposited_at: now,
                };

                self.deposit_balances.insert(resource_address, deposit);
            }
            None => {
                let deposit = Deposit {
                    principal_balance: Decimal::ZERO,
                    yield_balance: Decimal::ZERO,
                    deposited_at: now,
                };
                self.deposit_balances.insert(resource_address, deposit);
            }
        };
    }
}
#[derive(Debug, ScryptoSbor, PartialEq, Eq, Clone)]
pub struct Deposit {
    pub principal_balance: Decimal,
    pub yield_balance: Decimal,
    pub deposited_at: Instant,
}
