use crate::utils::{PROPOSAL_RECEIPT, VOTER_CARD_NAME};
use scrypto::prelude::Decimal;
use sqrt::blueprint::{AdminBadge, Blueprint};
use sqrt::method::Arg::{
    ComponentAddressArg, DecimalArg, FungibleBucketArg, NonFungibleBucketArg, ResourceAddressArg,
    StringArg, VecArg, I64,
};
use sqrt::method::{Arg, Method};
use sqrt::{enum_arg, method_args, tuple_arg};

pub struct DaoBlueprint {}

impl Blueprint for DaoBlueprint {
    fn instantiation_name(&self) -> &str {
        "new"
    }

    fn name(&self) -> &str {
        "Dao"
    }

    fn has_admin_badge(&self) -> AdminBadge {
        AdminBadge::None
    }
}

pub enum DaoMethods {
    Unlock(String),
    Gift(String, Decimal),
    MakeChangeVotePeriodProposal(i64),
    MakeMinimumVoteThresholdProposal(Decimal),
    MakeGrantIssuingRightProposal,
    MakeAllowClaimProposal(Vec<(String, Decimal)>),
    MakeAddNewCollateralToken(
        String,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        String,
    ),
    MakeChangeLenderParameters(String, Decimal, Decimal, Decimal, Decimal),
    MakeChangeLenderOracle(String, String),
    MakeAddTokensToIssuerReserves(Vec<(String, Decimal)>),
    ExecuteProposal(String),
    ClaimDexProtocolFees,
}

impl Method for DaoMethods {
    fn name(&self) -> &str {
        match self {
            DaoMethods::Unlock(_) => "unlock",
            DaoMethods::Gift(_, _) => "put_in_reserves",
            DaoMethods::MakeChangeVotePeriodProposal(_)
            | DaoMethods::MakeGrantIssuingRightProposal
            | DaoMethods::MakeMinimumVoteThresholdProposal(_)
            | DaoMethods::MakeAllowClaimProposal(_)
            | DaoMethods::MakeAddNewCollateralToken(_, _, _, _, _, _, _, _, _)
            | DaoMethods::MakeChangeLenderParameters(_, _, _, _, _)
            | DaoMethods::MakeChangeLenderOracle(_, _)
            | DaoMethods::MakeAddTokensToIssuerReserves(_) => "make_proposal",
            DaoMethods::ExecuteProposal(_) => "execute_proposal",
            DaoMethods::ClaimDexProtocolFees => "claim_dex_protocol_fees",
        }
    }

    fn args(&self) -> Option<Vec<Arg>> {
        match self {
            DaoMethods::Unlock(voter_card_id) => {
                method_args!(NonFungibleBucketArg(
                    VOTER_CARD_NAME.to_string(),
                    vec![voter_card_id.clone()]
                ))
            }
            DaoMethods::Gift(token, amount) => {
                method_args!(FungibleBucketArg(token.clone(), amount.clone()))
            }

            DaoMethods::MakeChangeVotePeriodProposal(vote_period) => {
                method_args!(enum_arg!(0, I64(vote_period.clone())))
            }
            DaoMethods::MakeMinimumVoteThresholdProposal(vot_threshold) => {
                method_args!(enum_arg!(1, DecimalArg(vot_threshold.clone())))
            }
            DaoMethods::MakeGrantIssuingRightProposal => {
                method_args!(enum_arg!(2))
            }
            DaoMethods::MakeAllowClaimProposal(resources) => {
                let mut vec_arg = vec![];
                for (resource, amount) in resources {
                    vec_arg.push(tuple_arg!(
                        ResourceAddressArg(resource.clone()),
                        DecimalArg(amount.clone())
                    ))
                }
                method_args!(enum_arg!(4, VecArg(vec_arg)))
            }
            DaoMethods::MakeAddNewCollateralToken(
                token,
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
                initial_rate,
                minimum_rate,
                maximum_rate,
                oracle,
            ) => {
                method_args!(enum_arg!(
                    5,
                    ResourceAddressArg(token.clone()),
                    DecimalArg(loan_to_value.clone()),
                    DecimalArg(interest_rate.clone()),
                    DecimalArg(liquidation_threshold.clone()),
                    DecimalArg(liquidation_penalty.clone()),
                    DecimalArg(initial_rate.clone()),
                    DecimalArg(minimum_rate.clone()),
                    DecimalArg(maximum_rate.clone()),
                    ComponentAddressArg(oracle.clone())
                ))
            }
            DaoMethods::MakeChangeLenderParameters(
                lender,
                loan_to_value,
                interest_rate,
                liquidation_threshold,
                liquidation_penalty,
            ) => {
                method_args!(enum_arg!(
                    6,
                    ResourceAddressArg(lender.clone()),
                    DecimalArg(loan_to_value.clone()),
                    DecimalArg(interest_rate.clone()),
                    DecimalArg(liquidation_threshold.clone()),
                    DecimalArg(liquidation_penalty.clone())
                ))
            }
            DaoMethods::MakeChangeLenderOracle(lender, oracle) => {
                method_args!(enum_arg!(
                    7,
                    ResourceAddressArg(lender.clone()),
                    ComponentAddressArg(oracle.clone())
                ))
            }
            DaoMethods::MakeAddTokensToIssuerReserves(resources) => {
                let mut vec_arg = vec![];
                for (resource, amount) in resources {
                    vec_arg.push(tuple_arg!(
                        StringArg(resource.clone()),
                        DecimalArg(amount.clone())
                    ))
                }
                method_args!(enum_arg!(8, VecArg(vec_arg)))
            }
            DaoMethods::ExecuteProposal(proposal_receipt_id) => {
                method_args!(NonFungibleBucketArg(
                    PROPOSAL_RECEIPT.to_string(),
                    vec![proposal_receipt_id.clone()]
                ))
            }
            DaoMethods::ClaimDexProtocolFees => {
                method_args!()
            }
        }
    }

    fn needs_admin_badge(&self) -> bool {
        false
    }

    fn custom_manifest_name(&self) -> Option<&str> {
        match self {
            DaoMethods::Unlock(_) => None,
            DaoMethods::Gift(_, _) => None,
            DaoMethods::MakeChangeVotePeriodProposal(_) => Some("make_change_vote_period_proposal"),
            DaoMethods::MakeMinimumVoteThresholdProposal(_) => {
                Some("make_minimum_vote_threshold_proposal")
            }
            DaoMethods::MakeGrantIssuingRightProposal => Some("make_grant_issuing_right_proposal"),
            DaoMethods::MakeAllowClaimProposal(_) => Some("make_allow_claim_proposal"),
            DaoMethods::MakeAddNewCollateralToken(_, _, _, _, _, _, _, _, _) => {
                Some("make_add_new_collateral_proposal")
            }
            DaoMethods::MakeChangeLenderParameters(_, _, _, _, _) => {
                Some("make_change_lender_parameters_proposal")
            }
            DaoMethods::MakeChangeLenderOracle(_, _) => Some("make_change_lender_oracle_proposal"),
            DaoMethods::MakeAddTokensToIssuerReserves(_) => {
                Some("make_add_tokens_to_issuer_reserves_proposal")
            }
            DaoMethods::ExecuteProposal(_) => None,
            DaoMethods::ClaimDexProtocolFees => None,
        }
    }
}
