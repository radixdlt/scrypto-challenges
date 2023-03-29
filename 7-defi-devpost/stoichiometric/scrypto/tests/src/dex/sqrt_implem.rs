use crate::utils::ADMIN_BADGE_NAME;
use scrypto::prelude::Decimal;
use sqrt::blueprint::{AdminBadge, Blueprint};
use sqrt::method::Arg::{
    DecimalArg, FungibleBucketArg, NonFungibleBucketArg, NonFungibleProofArg, ResourceAddressArg,
    U16,
};
use sqrt::method::{Arg, Method};
use sqrt::method_args;

pub struct RouterBlueprint {}

impl Blueprint for RouterBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "Router"
    }

    fn has_admin_badge(&self) -> AdminBadge {
        AdminBadge::External(ADMIN_BADGE_NAME.to_string())
    }
}

pub enum RouterMethods {
    CreatePool(String, Decimal, Decimal, Decimal),
    RemoveLiquidityAtStep(String, String, u16),
    RemoveLiquidityAtSteps(String, String, u16, u16),
    RemoveLiquidityAtRate(String, String, Decimal),
    RemoveAllLiquidity(String, Vec<String>),
    ClaimFees(String, Vec<String>),
    Swap(String, Decimal, String),
    ClaimProtocolFees,
}

impl Method for RouterMethods {
    fn name(&self) -> &str {
        match self {
            RouterMethods::CreatePool(_, _, _, _) => "create_pool",
            RouterMethods::RemoveLiquidityAtStep(_, _, _) => "remove_liquidity_at_step",
            RouterMethods::RemoveLiquidityAtSteps(_, _, _, _) => "remove_liquidity_at_steps",
            RouterMethods::RemoveLiquidityAtRate(_, _, _) => "remove_liquidity_at_rate",
            RouterMethods::RemoveAllLiquidity(_, _) => "remove_all_liquidity",
            RouterMethods::ClaimFees(_, _) => "claim_fees",
            RouterMethods::Swap(_, _, _) => "swap",
            RouterMethods::ClaimProtocolFees => "claim_protocol_fees",
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            RouterMethods::CreatePool(token, initial_rate, min_rate, max_rate) => {
                method_args!(
                    ResourceAddressArg(token.clone()),
                    DecimalArg(initial_rate.clone()),
                    DecimalArg(min_rate.clone()),
                    DecimalArg(max_rate.clone())
                )
            }
            RouterMethods::RemoveLiquidityAtStep(position, position_id, step) => {
                method_args!(
                    NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                    U16(step.clone())
                )
            }
            RouterMethods::RemoveLiquidityAtSteps(position, position_id, start_step, stop_step) => {
                method_args!(
                    NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                    U16(start_step.clone()),
                    U16(stop_step.clone())
                )
            }
            RouterMethods::RemoveLiquidityAtRate(position, position_id, rate) => {
                method_args!(
                    NonFungibleProofArg(position.clone(), vec![position_id.clone()]),
                    DecimalArg(rate.clone())
                )
            }
            RouterMethods::RemoveAllLiquidity(position, position_ids) => {
                method_args!(NonFungibleBucketArg(position.clone(), position_ids.clone()))
            }
            RouterMethods::ClaimFees(position, position_ids) => {
                method_args!(NonFungibleProofArg(position.clone(), position_ids.clone()))
            }
            RouterMethods::Swap(token_input, amount_input, token_output) => {
                method_args!(
                    FungibleBucketArg(token_input.clone(), amount_input.clone()),
                    ResourceAddressArg(token_output.clone())
                )
            }
            RouterMethods::ClaimProtocolFees => {
                method_args!()
            }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        match self {
            RouterMethods::CreatePool(_, _, _, _) | RouterMethods::ClaimProtocolFees => true,
            _ => false,
        }
    }

    fn custom_manifest_name(&self) -> Option<&str> {
        None
    }
}
