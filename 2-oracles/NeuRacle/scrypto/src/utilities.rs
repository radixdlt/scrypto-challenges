use scrypto::prelude::*;

pub fn assert_resource(resource1: ResourceAddress, resource2: ResourceAddress, amount: Decimal, require: Decimal) {

    assert!(
        resource1 == resource2,
        "Wrong resource address."
    );

    assert!(
        amount >= require,
        "Not enough resource."
    )

}

pub fn assert_fee(fee: Decimal) {
    assert!(
        (fee >= Decimal::zero()) && (fee <= dec!("100")),
        "Fee must be in the range of 0 to 100"
    );
}