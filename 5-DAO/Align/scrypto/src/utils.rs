/*!
This module implement two helpful function: [assert_rate](crate::utils::assert_rate) and [expo](crate::utils::expo)
and two helpful struct: [Methods](crate::utils::Methods) and [Method](crate::utils::Method) to support general method call for the DAO's collective actions.
*/

use scrypto::prelude::*;

/// Assert if the rate is in the right range or not
pub fn assert_rate(rate: Decimal) {
    assert!(
        rate <= dec!("100") && rate >= Decimal::ZERO,
        "Wrong rate/fee provided!"
    );
}

/// Helpful exponential math function, same as a^b
pub fn expo(number: Decimal, times: u64) -> Decimal {
    let mut counter = 0;

    let mut result = Decimal::ONE;

    while counter != times {
        counter += 1;
        result = result * number;
    }

    result
}

/// Helpful struct to support general method call.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Method {
    /// Component address that would be called.
    pub component: ComponentAddress,

    /// Method name that would be called.
    pub method: String,

    /// Method input arguments for the call.
    pub args: String,
}

impl Method {
    fn call(&self) {
        let args = hex::decode(&self.args).expect("Cannot decode hex");
        Runtime::call_method::<&str, ()>(self.component, &self.method, args);
    }
}

/// The struct keep track off all the methods that a DAO member will propose and 
/// will be executed when the proposal is accepted.
#[derive(TypeId, Encode, Decode, Describe)]
pub struct Methods(pub Vec<Method>);

impl Methods {
    /// The function to call all the methods that reach consensus.
    pub fn call_all(&self) {
        self.0.iter().for_each(|method| method.call())
    }
}