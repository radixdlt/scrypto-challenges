/// **This submission does not need to be judged due to being incomplete***
/// 
/// 
use scrypto::prelude::*;

blueprint! {
    struct TokenTax {
        /// The resource address of token to be taxed
        resource_address: ResourceAddress,
        tax: Decimal,
        tax_vault: Vault,
    }

    impl TokenTax {
        /// Creates a Radiswap component for token pair A/B and returns the component address
        /// along with the initial LP tokens.
        pub fn new_token_tax(resource_address: ResourceAddress, tax: Decimal, tax_vault: Vault) -> (ComponentAddress, Bucket) {
            
            // Check arguments
            assert!(tax >= dec!("0") && fee <= dec!("1"), "Invalid fee decimal");

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "TokenTax Admin")
                .initial_supply(1);

            // Instantiate our Radiswap component
            let component = Self {
                admin_badge: admin_badge.resource_address(),
                admin_vault: Vault::with_bucket(admin_badge),
                resource_address: resource_address.resource_address(),
                tax: tax,
                tax_vault: Vault::new(resource_address),
            }
            component.instantiate();
            component.globalize()

        }

        pub fn take_tax() {
            /// take in bucket of token
            /// assert token address matches token address from component
            /// take tax from bucket, put
            /// pass remainder back to user
            /// present admin badge to allow the portion sent back to user to be used in trade/swap/transfer
        }

        pub fn collect_taxes() {
            /// require xrdao_access_badge to call this method only
            /// takes all collected taxes and puts them in xrd_fees_collected in xrdao component
            taxes = self.tax_vault.empty();
            taxes // returns bucket of all collected fees

        }


    }
}