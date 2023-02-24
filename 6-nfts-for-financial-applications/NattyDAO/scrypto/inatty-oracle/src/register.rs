use scrypto::prelude::*;

// This component is used to register members of the DAO
// It tracks an iNaturalist user name and associates it with a radix wallet.

// TODO: how to prevent someone from registering a username that isn't theirs?
// Perhaps make usernames require .xrd suffix?

blueprint! {

    struct Register {
        user_names: Vec<String>,
        admin_badge: Vault,
        user_name_to_rdx_wallet: Map<String, Address>,
    }

    impl Register {

        pub fn instantiate_register() -> ComponentAddress {

            let admin_badge_bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin Badge")
                .initial_supply(1);

            // Instantiate a new Register component
            Self {
                user_names: Vec::new(),
                admin_badge: Vault::with_bucket(admin_badge_bucket),
            }
            .instantiate()
            .globalize()
        }

        pub fn register_user(&mut self, user_name: String, rdx_wallet: Address) -> bool {
            // Check whether this user name has already been registered
            // If not, register the user name and return true
            if self.user_names.contains(&user_name) {
                return false;
            } else {
                self.user_names.push(user_name.clone());
                self.user_name_to_rdx_wallet.insert(user_name, rdx_wallet);
                return true;
            }
        }
    }

}