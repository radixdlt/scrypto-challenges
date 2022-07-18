use sbor::*;
use scrypto::prelude::*;

#[derive(TypeId, Encode, Decode, Describe)]
pub struct User {
    pub user_badge_resource_address: ResourceAddress,
    pub deposit_balances: HashMap<ResourceAddress, Decimal>,
    pub borrow_balances: HashMap<ResourceAddress, Decimal>,
}

impl User {
    pub fn get_borrow_balance_value(&self, resource_address: ResourceAddress) -> Decimal {
        let balance_value = match self.borrow_balances.get(&resource_address) {
            Some(current_balance) => *current_balance,
            None => Decimal(0),
        };
        balance_value
    }

    pub fn increase_deposit_balance(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        match self.deposit_balances.get(&resource_address) {
            Some(current_balance) => {
                let old_balance = *current_balance;
                let new_balance = *current_balance + amount;
                self.deposit_balances.insert(resource_address, new_balance);
                info!(
                    "[User][USER:{}] Updated deposit balance for asset {} from {} to {}",
                    self.user_badge_resource_address, resource_address, old_balance, new_balance
                );
            }
            None => {
                self.deposit_balances.insert(resource_address, amount);
                info!(
                    "[User][USER:{}] No existing balance - New deposit balance for asset {} is {}",
                    self.user_badge_resource_address, resource_address, amount
                );
            }
        };
    }

    pub fn decrease_deposit_balance(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        match self.deposit_balances.get(&resource_address) {
            Some(current_balance) => {
                assert!(
                    current_balance >= &amount,
                    "[User] Cannot create a negative balance"
                );

                let old_balance = *current_balance;
                let new_balance = *current_balance - amount;
                self.deposit_balances.insert(resource_address, new_balance);
                info!(
                    "[User][USER:{}] Updated deposit balance for asset {} from {} to {}",
                    self.user_badge_resource_address, resource_address, old_balance, new_balance
                );
            }
            None => {
                panic!(
                    "[User] Cannot decrease balance of asset {} as the user has no record of said asset.",
                    resource_address
                );
            }
        };
    }

    pub fn increase_borrowed_balance(&mut self, resource_address: ResourceAddress, amount: Decimal) {
        match self.borrow_balances.get(&resource_address) {
            Some(current_balance) => {
                let old_balance = *current_balance;
                let new_balance = *current_balance + amount;
                info!(
                    "[User][USER:{}] Updated borrow balance for asset {} from {} to {}",
                    self.user_badge_resource_address, resource_address, old_balance, new_balance
                );
                self.borrow_balances.insert(resource_address, new_balance)
            }
            None => self.borrow_balances.insert(resource_address, amount),
        };
    }

    pub fn decrease_borrowed_balance(&mut self, resource_address: ResourceAddress, amount: Decimal) -> Decimal {
        return match self.borrow_balances.get(&resource_address) {
            Some(current_balance) => {
                assert!(
                    *current_balance >= amount,
                    "[User] Cannot create a negative borrow balance"
                );
                let old_balance = *current_balance;
                let new_balance = *current_balance - amount;
                info!(
                    "[User][USER:{}] Updated borrow balance for asset {} from {} to {}",
                    self.user_badge_resource_address, resource_address, old_balance, new_balance
                );
                self.borrow_balances.insert(resource_address, new_balance);
                new_balance
            }
            None => {
                self.borrow_balances.insert(resource_address, amount);
                amount
            }
        };
    }
}
