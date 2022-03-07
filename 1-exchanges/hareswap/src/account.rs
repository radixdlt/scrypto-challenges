use scrypto::prelude::*;

use super::requirement::*;

// mostly copied from assets/Account but authentication guard changed to use BucketRequirement.  Interface is the same

blueprint! {
    struct CustodialAccount {
        auth_requirement: BucketRequirement,
        vaults: LazyMap<Address, Vault>,
    }

    impl CustodialAccount {
        pub fn new(auth_requirement: BucketRequirement) -> Component {
            CustodialAccount {
                auth_requirement,
                vaults: LazyMap::new(),
            }
            .instantiate()
        }

        pub fn new_easy(auth_requirement_resource: Address) -> Component {
            let requirement = BucketRequirement {
                resource: auth_requirement_resource.into(),
                contents: BucketContents::Fungible(Decimal::one())
            };
            CustodialAccount::new(requirement)
        }

        pub fn with_bucket(auth_requirement: BucketRequirement, bucket: Bucket) -> Component {
            let vaults = LazyMap::new();
            vaults.insert(bucket.resource_address(), Vault::with_bucket(bucket));

            CustodialAccount { auth_requirement, vaults }.instantiate()
        }

        /// Deposit a batch of buckets into this account
        pub fn deposit_batch(&mut self, buckets: Vec<Bucket>) {
            for bucket in buckets {
                self.deposit(bucket);
            }
        }

        /// Deposits resource into this account.
        pub fn deposit(&mut self, bucket: Bucket) {
            let address = bucket.resource_address();
            match self.vaults.get(&address) {
                Some(mut v) => {
                    v.put(bucket);
                }
                None => {
                    let v = Vault::with_bucket(bucket);
                    self.vaults.insert(address, v);
                }
            }
        }

        /// Withdraws resource from this account.
        pub fn withdraw(
            &mut self,
            amount: Decimal,
            resource_address: Address,
            account_auth: BucketRef,
        ) -> Bucket {
            assert_eq!(self.auth_requirement.check_at_least_ref(&account_auth), true, "CustodialAccount::withdraw: account_auth requirement not met");

            let vault = self.vaults.get(&resource_address);
            match vault {
                Some(mut vault) => vault.take(amount),
                None => {
                    panic!("Insufficient balance");
                }
            }
        }

        /// Withdraws resource from this account.
        pub fn withdraw_with_auth(
            &mut self,
            amount: Decimal,
            resource_address: Address,
            auth: BucketRef,
            account_auth: BucketRef,
        ) -> Bucket {
            assert_eq!(self.auth_requirement.check_at_least_ref(&account_auth), true, "CustodialAccount::withdraw_with_auth: account_auth requirement not met");

            let vault = self.vaults.get(&resource_address);
            match vault {
                Some(mut vault) => vault.take_with_auth(amount, auth),
                None => {
                    panic!("Insufficient balance");
                }
            }
        }

        /// Withdraws non-fungibles from this account.
        pub fn withdraw_non_fungibles(
            &mut self,
            keys: BTreeSet<NonFungibleKey>,
            resource_address: Address,
            account_auth: BucketRef,
        ) -> Bucket {
            assert_eq!(self.auth_requirement.check_at_least_ref(&account_auth), true, "CustodialAccount::withdraw_non_fungibles: account_auth requirement not met");

            let vault = self.vaults.get(&resource_address);
            match vault {
                Some(vault) => {
                    let mut bucket = Bucket::new(resource_address);
                    for key in keys {
                        bucket.put(vault.take_non_fungible(&key));
                    }
                    bucket
                }
                None => {
                    panic!("Insufficient balance");
                }
            }
        }

        /// Withdraws non-fungibles from this account.
        pub fn withdraw_non_fungibles_with_auth(
            &mut self,
            keys: BTreeSet<NonFungibleKey>,
            resource_address: Address,
            auth: BucketRef,
            account_auth: BucketRef,
        ) -> Bucket {
            assert_eq!(self.auth_requirement.check_at_least_ref(&account_auth), true, "CustodialAccount::withdraw_non_fungibles_with_auth: account_auth requirement not met");

            let vault = self.vaults.get(&resource_address);
            match vault {
                Some(vault) => {
                    let mut bucket = Bucket::new(resource_address);
                    for key in keys {
                        bucket.put(vault.take_non_fungible_with_auth(&key, auth.clone()));
                    }
                    bucket
                }
                None => {
                    panic!("Insufficient balance")
                }
            }
        }
    }
}
