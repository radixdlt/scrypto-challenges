use scrypto::prelude::Decimal;
use sqrt::blueprint::{AdminBadge, Blueprint};
use sqrt::method::Arg::DecimalArg;
use sqrt::method::{Arg, Method};
use sqrt::method_args;

pub struct DumbOracleBlueprint {}

impl Blueprint for DumbOracleBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "DumbOracle"
    }

    fn has_admin_badge(&self) -> AdminBadge {
        AdminBadge::None
    }
}

pub enum DumbOracleMethods {
    SetPrice(Decimal),
}

impl Method for DumbOracleMethods {
    fn name(&self) -> &str {
        match self {
            DumbOracleMethods::SetPrice(_) => "set_price",
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            DumbOracleMethods::SetPrice(price) => {
                method_args!(DecimalArg(price.clone()))
            }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        false
    }

    fn custom_manifest_name(&self) -> Option<&str> {
        None
    }
}
