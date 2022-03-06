/*
// taken from scrypto_statictypes

/// Proxy for an Account taking the place of the removed scrypto::core::Account API
struct Account {
    component: Address,
}
impl Account {
    fn address(&self) -> Address {
        self.component
    }
}

// Deposit
//

pub trait Deposit {
    fn deposit(&self, bucket: Bucket);
}
impl Deposit for Account {
    fn deposit(&self, bucket: Bucket) {
        let args = vec![scrypto_encode(&bucket)];
        let rtn = call_method(self.address(), "deposit", args);
        scrypto_unwrap(scrypto_decode(&rtn))
    }
}

// Withdraw
//

pub trait Withdraw {
    fn withdraw<A: Into<ResourceDef>>(&self, amount: Decimal, resource_def: A) -> Bucket;
}
impl Withdraw for Account {
    fn withdraw<A: Into<ResourceDef>>(&self, amount: Decimal, resource_def: A, auth: BucketRef) -> Bucket {
        let args = vec![
            scrypto_encode(&amount),
            scrypto_encode(&resource_def.into()),
            scrypto_encode(&auth),
        ];
        let rtn = call_method(self.address(), "withdraw", args);
        scrypto_unwrap(scrypto_decode(&rtn))
    }
}
*/
