use scrypto::prelude::*;

#[derive(ScryptoCategorize, LegacyDescribe, ScryptoEncode, ScryptoDecode, Clone)]
pub enum ProposedChange {
    /// Changes the vote period of proposals
    ChangeVotePeriod(i64),

    /// Changes the minimum amount of votes that have to be casted to consider a vote valid
    ChangeMinimumVoteThreshold(Decimal),

    /// Grants a stablecoin minting badge
    GrantIssuingRight,

    /// Recalls a stablecoin minting badge
    RemoveIssuingRight(Vec<u8>),

    /// Allows claiming of a certain amount of resource from the dao reserves
    AllowClaim(Vec<(ResourceAddress, Decimal)>),

    /// Adds a new token as possible collateral. Taking this decision will also create a pool for the given token
    AddNewCollateralToken(
        ResourceAddress,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        ComponentAddress,
    ),

    /// Changes the parameters of a given stablecoin lender
    ChangeLenderParameters(ResourceAddress, Decimal, Decimal, Decimal, Decimal),

    /// Changes the oracle of a given stablecoin lender
    ChangeLenderOracle(ResourceAddress, ComponentAddress),

    /// Adds given tokens to the stablecoin issuer reserves
    AddTokensToIssuerReserves(Vec<(ResourceAddress, Decimal)>),
}
