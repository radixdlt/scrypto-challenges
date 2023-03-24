type token = {
    address: string,
    name: string,
    symb: string,
    icon_url: string
}

type step = {
    step_id: number
    stablecoin_amount: number,

    other_token_amount: number,

    rate: number,
}

type pool = {
    token: token,
    rate_step: number,

    current_step: number,

    min_rate: number,

    max_rate: number,

    steps: step[]
}

type position = {
    token: token | null,
    liquidity: number,
    price_x: number,
    value_locked: number | '?',
    x_fees: number | '?',
    y_fees: number | '?',
    nfIdValue: any,
    id: string | null
}

type account = {
    address: string,
    name: string
}

type tokenOwned = number[];

type lender = {
    lender_address: string,

    collateral_token: token,

    collateral_price: number,

    oracle: string,

    loan_to_value: number,

    interest_rate: number,

    liquidation_threshold: number,

    liquidation_penalty: number,

}

type loan = {
    collateral_token: token,

    collateral_amount: number,

    amount_lent: number,

    loan_date: number,

    liquidation_price: number,

    loan_to_value: number,

    interest_rate: number,

    amount_to_liquidate: number,

    id: string
}


interface ComponentState {
    data_json: any; // replace "any" with the actual type of the data
    // define any other properties of the ComponentState object here
}

interface EntityDetailsResponseComponentDetails {
    discriminator: "component";
    package_address: string;
    blueprint_name: string;
    state: ComponentState; // specify the type of the "state" object
    access_rules_chain: object;
}

type decoded = {
    collateral_token: string,
    collateral_amount: number,
    amount_lent: number,
    loan_time: number,
    loan_to_value: number,
    interest_rate: number
}


type Hexes = { mutable_hex: string, immutable_hex: string, id: string };

type dao = {
    total_voting_power: number,

    vote_period: number,

    vote_validity_threshold: number,

    proposals: proposal[],

    reserves: Map<string, number>,


}

export enum proposedChange {
    ChangeVotePeriod,

    ChangeMinimumVoteThreshold,

    GrantIssuingRight,

    AllowClaim,

    AddNewCollateral,

    ChangeLenderParameters,

    ChangeLenderOracle,

    AddTokensToIssuerReserves

}



type proposal = {
    vote_end: number,

    votes_for: number,

    votes_against: number,

    votes_threshold: number,

    proposed_change_type: proposedChange,

    proposed_change_data: any[]

}

type voterCard = {
    voting_power: number,

    stablecoins_locked: number,

    positions_ids_locked: string[],

    proposals_voted: number[]

}


export type { token, pool, step, position, account, tokenOwned, lender, loan, EntityDetailsResponseComponentDetails, decoded, Hexes, dao, proposal, voterCard };