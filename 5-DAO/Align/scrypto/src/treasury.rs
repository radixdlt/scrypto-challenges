/*!
The blueprint implement Treasury Component for the DAO to store and manage its assets according to [Internal Treasury Function](crate::align_dao#internal-treasury-function).

Treasury is the main life-line of the DAO. Align DAO's internal treasury will contain these assets:
1. DAO's share liquidity: The DAO share token liquidity for the treasury treasury. This is for the implementation of the Internal decentralized share market.
2. Primary reserve currency: The choosen primary reserve currency on the DAO's launch, this could be stable coins or any other tokenized currency which the DAO believe that will have a stable value.
3. Internal assets: The DAO's internal assets, it could be tokenized real estates, computers, furnitures,...; protocol's ownership NFT; utilities token;...
4. Investment assets: The DAO's investment assets, it could be portfolio management, investment account NFT; loan ownership token;...

## Functions and Methods Overview
All Treasury's methods can only be called from the DAO's component.
- Function [new()](Treasury_impl::Treasury::new): Instantiate new Treasury Component.
- Method [deposit()](Treasury_impl::Treasury::deposit):
Allow anyone to deposit resources to the Treasury.
- Method [withdraw_by_amount()](Treasury_impl::Treasury::withdraw_by_amount):
Allow the DAO's collective action to withdraw resource from the Treasury.
- Method [withdraw()](Treasury_impl::Treasury::withdraw):
Allow the DAO's collective action to withdraw all one kind of resource from the Treasury.
- Method [rage_withdraw()](Treasury_impl::Treasury::rage_withdraw):
Allow the DAO Member/Delegator to do [Rage Withdraw](crate::align_dao)
- Method [auto_swap()](Treasury_impl::Treasury::auto_swap):
Allow anyone to swap between DAO share and the primary reserve resource.
- Method [amend_fee_policy()](Treasury_impl::Treasury::amend_fee_policy):
Allow DAO's collective actions to change the Treasury fee policy.
- Method [get_price()](Treasury_impl::Treasury::get_price):
Read only method to get current DAO share / primary reserve price.
- Method [get_rage_withdraw()](Treasury_impl::Treasury::get_rage_withdraw):
Read only method to get the treasury rage withdraw policy.
- Method [reset_period()](Treasury_impl::Treasury::reset_period):
Internal method to reset the withdraw period and withdraw threshold on the Treasury.
*/

use crate::policies::TreasuryPolicy;
use scrypto::prelude::*;

blueprint! {

    /// The DAO Local Treasury Component will store and manage DAO's assets. An AMM mechanism will be implemented here for
    /// people to easily exchange between DAO share token and the stable coin.
    /// 
    /// The Treasury Component is local to the DAO and it's methods can only be accessed through the DAO Component's methods
    pub struct Treasury {

        /// Vault contain DAO share resource.
        /// 
        /// The liquidity for DAO share cannot be added by any mean other than swap through the AMM
        share: Vault,
        /// Vault contain primary reserve resource.
        /// 
        /// Each DAO's collective action can withdraw resource from this vault but must smaller (or equal) than the withdraw threshold.
        primary_reserve_vault: Vault,
        /// The DAO's other vaults to contain other resources.
        other_vaults: KeyValueStore<ResourceAddress, Vault>,

        /// The DAO's [TreasuryPolicy](crate::policies::TreasuryPolicy).
        treasury_policy: TreasuryPolicy,

        /// The period end in which the withdraw threshold will be reset.
        period_end: u64,
        /// Current remain primary reserve resource can be withdrawed from the treasury on this period.
        /// 
        /// On period end, this value will be reset to the withdraw threshold.
        current_withdrawable: Decimal,
    }

    impl Treasury {

        /// This function instantiate new Treasury Component.
        ///
        /// # Input
        /// - inital_share_supply: Bucket contain initial DAO share supply for the Treasury.
        /// - primary_reserve_supply: Bucket contain initial primary reserve resource supply for the Treasury.
        /// - treasury_policy: The input [TreasuryPolicy](crate::policies::TreasuryPolicy)
        ///
        /// # Output
        /// The Treasury Component, the component will be stored on the DAO Component state, 
        /// field [treasury](crate::align_dao::DAO_impl::DAO::treasury)
        /// 
        /// # Access Rule
        /// The function should be called along when instantiate and globalize the DAO Component.
        pub fn new(
            inital_share_supply: Bucket,
            primary_reserve_supply: Bucket,
            treasury_policy: TreasuryPolicy,
        ) -> TreasuryComponent {

            let withdrawable = treasury_policy.withdraw_threshold * primary_reserve_supply.amount();

            Self {
                share: Vault::with_bucket(inital_share_supply),
                primary_reserve_vault: Vault::with_bucket(primary_reserve_supply),
                other_vaults: KeyValueStore::new(),
                treasury_policy,
                period_end: 0,
                current_withdrawable: withdrawable,
            }
            .instantiate()
        }

        /// This method will allow anyone to deposit resources to the Treasury.
        ///
        /// # Input
        /// - bucket: The resource bucket.
        /// # Access Rule
        /// Can only be called from the DAO's [deposit()](crate::align_dao::DAO_impl::DAO::deposit) method
        pub fn deposit(&mut self, bucket: Bucket) {
            let resource_address = bucket.resource_address();
            let amount = bucket.amount();

            if resource_address == self.primary_reserve_vault.resource_address() {
                info!("[Treasury]: Deposited {} stablecoins into treasury", amount);
                self.primary_reserve_vault.put(bucket)
            } else {
                info!(
                    "[Treasury]: Deposited {} resource of address {} into treasury",
                    amount, resource_address
                );
                if self.other_vaults.get(&resource_address).is_none() {
                    let v = Vault::with_bucket(bucket);
                    self.other_vaults.insert(resource_address, v);
                } else {
                    let mut v = self.other_vaults.get_mut(&resource_address).unwrap();
                    v.put(bucket);
                }
            }
        }

        /// This method will allow the DAO's collective action to withdraw resource from the Treasury.
        /// 
        /// # Input
        /// - amount: withdraw amount.
        /// - resource_address: the withdraw resource address
        /// - current: current time from [LocalOracle](crate::local_oracle) component
        /// 
        /// # Output
        /// The withdrawed resource bucket.
        /// 
        /// Return empty bucket if the treasury didn't have the resource.
        /// # Access Rule
        /// Can only be called from the DAO's [dao_withdraw_by_amount()](crate::align_dao::DAO_impl::DAO::dao_withdraw_by_amount) method
        pub fn withdraw_by_amount(
            &mut self,
            amount: Decimal,
            resource_address: ResourceAddress,
            current: u64,
        ) -> Bucket {
            if resource_address == self.primary_reserve_vault.resource_address() {
                self.reset_period(current);
                let withdrawable = self.current_withdrawable;
                if amount <= withdrawable {
                    self.current_withdrawable -= amount;
                    info!(
                        "[Treasury]: Withdrawed {} stablecoins from treasury",
                        amount
                    );
                    self.primary_reserve_vault.take(amount)
                } else {
                    self.current_withdrawable = Decimal::ZERO;
                    error!(
                        "[Treasury]: For now, you can only withdraw {} stablecoins from treasury",
                        withdrawable
                    );
                    info!(
                        "[Treasury]: Withdrawed {} stablecoins from treasury",
                        withdrawable
                    );
                    self.primary_reserve_vault.take(withdrawable)
                }
            } else {
                let vault = self.other_vaults.get_mut(&resource_address);
                match vault {
                    Some(mut vault) => {
                        info!(
                            "[Treasury]: Withdrawed {} token address {} from treasury",
                            amount, resource_address
                        );
                        vault.take(amount)
                    }
                    None => {
                        error!("[Treasury]: No such resource in the treasury");
                        Bucket::new(resource_address)
                    }
                }
            }
        }

        /// This method will allow the DAO's collective action to withdraw all one kind of resource from the Treasury.
        /// 
        /// The method will not work with the primary reserve vault.
        /// # Input
        /// - resource_address: the withdraw resource address.
        /// 
        /// # Output
        /// The withdrawed resource bucket.
        /// 
        /// Return empty bucket if the treasury didn't have the resource.
        /// # Access Rule
        /// Can only be called from the DAO's [dao_withdraw()](crate::align_dao::DAO_impl::DAO::dao_withdraw) method
        pub fn withdraw(&mut self, resource_address: ResourceAddress) -> Bucket {
            let vault = self.other_vaults.get_mut(&resource_address);
            match vault {
                Some(mut vault) => {
                    info!(
                        "[Treasury]: Withdrawed all token address {} from treasury",
                        resource_address
                    );
                    vault.take_all()
                }
                None => {
                    error!("[Treasury]: No such resource in the treasury");
                    return Bucket::new(resource_address);
                }
            }
        }

        /// This method will allow the DAO Member/Delegator to do [Rage Withdraw](crate::align_dao)
        /// 
        /// This method will ignore the withdraw threshold and withdraw period
        /// # Input
        /// - amount: the rage withdraw amount.
        /// 
        /// # Output
        /// The withdrawed primary reserve resource bucket
        /// # Access Rule
        /// Can only be called from the DAO's [rage_withdraw()](crate::align_dao::DAO_impl::DAO::rage_withdraw) method
        pub fn rage_withdraw(&mut self, amount: Decimal) -> Bucket {
            self.primary_reserve_vault.take(amount)
        }

        /// This method will allow anyone to swap between DAO share and the primary reserve resource.
        /// # Input
        /// - token: The DAO share or primary reserve resource bucket
        /// 
        /// # Output
        /// The swapped resource Bucket (DAO share or primary reserve resource)
        /// # Access Rule
        /// Can only be called from the DAO's [swap()](crate::align_dao::DAO_impl::DAO::swap) method
        pub fn auto_swap(&mut self, token: Bucket) -> Bucket {
            if token.resource_address() == self.share.resource_address() {
                let fee = token.amount() * self.treasury_policy.swap_fee;

                let dx = token.amount() - fee;

                self.share.put(token);

                let (x, y) = (self.share.amount(), self.primary_reserve_vault.amount());

                let dy = (dx * y) / (x + dx);

                info!("[Treasury]: Paid {} DAO shares as fee and swapped {} DAO shares into {} stablecoins", fee, dx, dy);

                self.primary_reserve_vault.take(dy)
            } else if token.resource_address() == self.primary_reserve_vault.resource_address() {
                let fee = token.amount() * self.treasury_policy.swap_fee;

                let dx = token.amount() - fee;

                self.primary_reserve_vault.put(token);

                let (x, y) = (self.primary_reserve_vault.amount(), self.share.amount());

                let dy = (dx * y) / (x + dx);

                info!("[Treasury]: Paid {} stablecoins as fee and swapped {} stablecoins into {} DAO shares", fee, dx, dy);

                self.share.take(dy)
            } else {
                panic!("Wrong resource!")
            }
        }

        /// This method will allow DAO's collective actions to change the Treasury fee policy.
        /// # Input
        /// - fee: The new fee provided
        /// 
        /// # Access Rule
        /// Can only be called from the DAO's [amend_treasury_policy()](crate::align_dao::DAO_impl::DAO::amend_treasury_policy) method
        pub fn amend_fee_policy(&mut self, fee: Decimal) {
            self.treasury_policy.swap_fee = fee
        }

        /// Read only method to get current DAO share / primary reserve price.
        /// 
        /// # Access Rule
        /// Can only be called from the DAO's [get_price()](crate::align_dao::DAO_impl::DAO::get_price) method
        pub fn get_price(&self) -> Decimal {
            self.share.amount() / self.primary_reserve_vault.amount()
        }

        /// Read only method to get the treasury rage withdraw policy.
        /// 
        /// # Access Rule
        /// Can only be called from the DAO's [get_rage_withdraw()](crate::align_dao::DAO_impl::DAO::get_rage_withdraw) method
        pub fn get_rage_withdraw(&self) -> (Decimal, u64) {
            (
                self.treasury_policy.rage_withdraw_decline_multiply,
                self.treasury_policy.rage_withdraw_time_limit,
            )
        }

        /// Internal method to reset the withdraw period and withdraw threshold on the Treasury.
        /// 
        /// # Access Rule
        /// Can only be called internally
        fn reset_period(&mut self, current: u64) {
            if current >= self.period_end {
                self.period_end = current + self.treasury_policy.withdraw_period;
                self.current_withdrawable =
                    self.primary_reserve_vault.amount() * self.treasury_policy.withdraw_threshold;
            }
        }
    }
}
