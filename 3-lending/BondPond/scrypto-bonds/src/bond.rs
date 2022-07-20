use scrypto::prelude::*;


#[derive(NonFungibleData)]
pub struct IssuerData {
    pub bond_id: u64,
}

blueprint! {

    // Bond Definition
    struct BondToken {
        token_supply: Vault,
        repayment_vault: Vault,
        face_value: Decimal,
        coupon_epoch: u64,
        maturity_epoch: u64,
        coupon_rate: Decimal,
        issue_price: Decimal, 
        issuer_badge: ResourceAddress, // Issuer holds identity NFT
        issue_epoch: u64,
    }

    impl BondToken {

        // Instantiate and issue a new bond, return component containing supply
        // and the issuer_badge that gives issuer privilage to burn the bonds
        pub fn instantiate_bond(bond_id: u64, face_value: Decimal, coupon_epoch: u64, 
            maturity_epoch: u64, coupon_rate: Decimal, issue_price: Decimal, supply: u32) 
            -> (ComponentAddress, Bucket) {
        
            let mut issuer_badge_data = Vec::new();
            issuer_badge_data.push((NonFungibleId::from_u64(bond_id),
                IssuerData { bond_id: bond_id }));

            let issuer_badge: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Bond Issuer Badge")
                .burnable(rule!(deny_all), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .initial_supply(issuer_badge_data);    
            
            // We only allow the issuer to burn the token
            let burn_rule: AccessRule = rule!( 
                require(issuer_badge.resource_address()) 
            );

            // Bond Token Supply Bucket
            let new_bond_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "BondToken")
                .divisibility(DIVISIBILITY_NONE)
                .metadata(
                    "description",
                    "A bond token used to recieve the principle and to be resold",
                )
                .metadata("bond_id", bond_id.to_string())
                .metadata("face_value", face_value.to_string())
                .metadata("coupon_epoch", coupon_epoch.to_string())
                .metadata("maturity_epoch", maturity_epoch.to_string())
                .metadata("coupon_rate", coupon_rate.to_string())
                .burnable(
                    burn_rule.clone(), 
                    MUTABLE(burn_rule.clone())
                )
                .initial_supply(supply);
            
            let issue_epoch:u64 = Runtime::current_epoch();
            
            let component = Self {
                token_supply: Vault::with_bucket(new_bond_bucket),
                repayment_vault: Vault::new(RADIX_TOKEN),
                face_value: face_value,
                coupon_epoch: coupon_epoch,
                maturity_epoch: maturity_epoch,
                coupon_rate: coupon_rate,
                issue_price: issue_price, 
                issuer_badge: issuer_badge.resource_address(),
                issue_epoch: issue_epoch,
            }
            .instantiate()
            .globalize();

            return (component, issuer_badge);
        }
        
        // // Withdraw principal if available, or maturity amount
        // // TODO
        // pub fn withdraw() -> Bucket {

        // }

    }
}