use crate::utils::{ADMIN_BADGE_NAME, LOAN_NAME, STABLECOIN_NAME};
use scrypto::prelude::Decimal;
use sqrt::blueprint::{AdminBadge, Blueprint};
use sqrt::method::Arg::{
    ComponentAddressArg, DecimalArg, FungibleBucketArg, NonFungibleBucketArg, NonFungibleLocalId,
    NonFungibleProofArg, ResourceAddressArg, StringArg,
};
use sqrt::method::{Arg, Method};
use sqrt::method_args;

pub struct IssuerBlueprint {}

impl Blueprint for IssuerBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "Issuer"
    }

    fn has_admin_badge(&self) -> AdminBadge {
        AdminBadge::External(ADMIN_BADGE_NAME.to_string())
    }
}

pub enum IssuerMethods {
    NewLender(String, Decimal, Decimal, Decimal, Decimal, String),
    TakeLoan(String, Decimal, Decimal),
    RepayLoans(Decimal, Vec<String>),
    AddCollateral(String, Decimal, String),
    RemoveCollateral(Decimal, String),
    Liquidate(Decimal, String),
    ChangeLenderParameters(String, Decimal, Decimal, Decimal, Decimal),
    ChangeLenderOracle(String),
}

impl Method for IssuerMethods {
    fn name(&self) -> &str {
        match self {
            IssuerMethods::NewLender(_, _, _, _, _, _) => "new_lender",
            IssuerMethods::TakeLoan(_, _, _) => "take_loan",
            IssuerMethods::RepayLoans(_, _) => "repay_loans",
            IssuerMethods::AddCollateral(_, _, _) => "add_collateral",
            IssuerMethods::RemoveCollateral(_, _) => "remove_collateral",
            IssuerMethods::Liquidate(_, _) => "liquidate",
            IssuerMethods::ChangeLenderParameters(_, _, _, _, _) => "change_lender_parameters",
            IssuerMethods::ChangeLenderOracle(_) => "changer_lender_oracle",
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            IssuerMethods::NewLender(
                collateral_token,
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                oracle,
            ) => {
                method_args!(
                    ResourceAddressArg(collateral_token.clone()),
                    DecimalArg(loan_to_value.clone()),
                    DecimalArg(interest_rate.clone()),
                    DecimalArg(liquidation_threshold.clone()),
                    DecimalArg(liquidation_penalty.clone()),
                    ComponentAddressArg(oracle.clone())
                )
            }
            IssuerMethods::TakeLoan(collateral_token, collateral_amount, amount_to_loan) => {
                method_args!(
                    FungibleBucketArg(collateral_token.clone(), collateral_amount.clone()),
                    DecimalArg(amount_to_loan.clone())
                )
            }
            IssuerMethods::RepayLoans(repayment_amount, loan_ids) => {
                method_args!(
                    FungibleBucketArg(STABLECOIN_NAME.to_string(), repayment_amount.clone()),
                    NonFungibleBucketArg(LOAN_NAME.to_string(), loan_ids.clone())
                )
            }
            IssuerMethods::AddCollateral(collateral_token, collateral_amount, loan_id) => {
                method_args!(
                    FungibleBucketArg(collateral_token.clone(), collateral_amount.clone()),
                    NonFungibleProofArg(LOAN_NAME.to_string(), vec![loan_id.clone()])
                )
            }
            IssuerMethods::RemoveCollateral(amount, loan_id) => {
                method_args!(
                    DecimalArg(amount.clone()),
                    NonFungibleProofArg(LOAN_NAME.to_string(), vec![loan_id.clone()])
                )
            }
            IssuerMethods::Liquidate(repayment_amount, loan_id) => {
                let boxed_arg = Box::new(StringArg(loan_id.clone()));
                method_args!(
                    FungibleBucketArg(STABLECOIN_NAME.to_string(), repayment_amount.clone()),
                    NonFungibleLocalId(boxed_arg)
                )
            }
            IssuerMethods::ChangeLenderParameters(
                collateral_token,
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_incentive,
            ) => {
                method_args!(
                    ResourceAddressArg(collateral_token.clone()),
                    DecimalArg(loan_to_value.clone()),
                    DecimalArg(interest_rate.clone()),
                    DecimalArg(liquidation_threshold.clone()),
                    DecimalArg(liquidation_incentive.clone())
                )
            }
            IssuerMethods::ChangeLenderOracle(oracle) => {
                method_args!(ComponentAddressArg(oracle.to_string()))
            }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        match self {
            IssuerMethods::NewLender(_, _, _, _, _, _)
            | IssuerMethods::ChangeLenderParameters(_, _, _, _, _)
            | IssuerMethods::ChangeLenderOracle(_) => true,
            _ => false,
        }
    }

    fn custom_manifest_name(&self) -> Option<&str> {
        None
    }
}
