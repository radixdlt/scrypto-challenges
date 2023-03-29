import { stateApi } from "$lib/api/rdt";
import type { ComponentAddressString, ResourceAddressString } from "@radixdlt/radix-dapp-toolkit";
import { writable } from "svelte/store";
import { INTEREST_RATE_TYPE } from "../../data";



function get_pool_sumary(pool: PoolState): PoolStateSummary {

    let interest_rate = pool.loan_state_lookup[INTEREST_RATE_TYPE].interest_rate;
    let total_liquidity = pool.available_liquidity + pool.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
    let total_loans = pool.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
    let total_loan_share = pool.loan_state_lookup[INTEREST_RATE_TYPE].total_loan_share;
    let loan_to_share_ratio = total_loans == 0 ? 1 : total_loan_share / total_loans;
    let pool_share_ratio = total_liquidity == 0 ? 1 : pool.pool_share_supply / total_liquidity;
    let total_collateral_value = pool.total_collateral / pool_share_ratio;
    let borrow_limit = pool.available_liquidity - total_collateral_value;
    let usage = total_liquidity == 0 ? 0 : total_loans / total_liquidity;

    let pool_sumary: PoolStateSummary = {
        price: pool.price,
        interest_rate,
        total_liquidity,

        pool_share_ratio,

        total_loan_share,
        total_loans,
        loan_to_share_ratio,
        total_collateral_value,
        borrow_limit,
        usage,
    }


    return pool_sumary
}

// $: price = pool.last_price;
// $: interest_rate = ;

// $: total_liquidity =

//     $: pool_share_supply = ;
// $: pool_share_ratio = total_liquidity == 0 ? 1 : pool_share_supply / total_liquidity;

// $: total_collateral = pool.last_collateral_amount; // pool_shares

// $: total_loan_share = pool.loan_state_lookup[INTEREST_RATE_TYPE].total_loan_share;
// $: total_loans = pool.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
// $: loan_to_share_ratio = total_loans == 0 ? 1 : total_loan_share / total_loans;

// $: total_collateral_value = pool.last_collateral_amount / pool_share_ratio;
// $: borrow_limit = pool.last_available_liquidity - total_collateral_value;

// $: usage = total_liquidity == 0 ? 0 : total_loans / total_liquidity;

export type PoolStateSummary = {

}


export type PoolState = {
    $component_address: ComponentAddressString;

    pool_resource_address: ResourceAddressString, // 5

    pool_share_resource_address: ResourceAddressString, // 6

    flashloan_term_resource_address: ResourceAddressString, // 7

    interest_factory_address: ComponentAddressString, // 8

    oracle_address: ComponentAddressString, // 9

    loan_state_lookup: InterestTypeState[],//HashMap<u8, InterestTypeState>, // 10

    price: number, // 11

    last_price_update: number, // 12 

    flashloan_fee_rate: number, // 13

    liquidation_threshold: number, // 14

    liquidation_spread: number, // 15

    liquidation_close_factor: number, // 16

    available_liquidity: number,

    pool_share_supply: number,

    total_collateral: number,

    // last_total_loans_amount: number,

    //////


    interest_rate: number,
    total_liquidity: number,

    pool_share_ratio: number,

    total_loan_share: number,
    total_loans: number,
    loan_to_share_ratio: number,
    total_collateral_value: number,
    borrow_limit: number,
    usage: number,
}

export type InterestTypeState = {
    interest_type: number,
    total_loan: number,
    total_loan_share: number,
    interest_rate: number,
    interest_updated_at: number,
}

let _lending_pools: Record<string, PoolState> = {}

export const lending_pools = writable(_lending_pools);

export async function load_pool_state(address: string): Promise<PoolState | undefined> {

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;

    let state = ((result.details as any)['state']['data_json']) as any;

    console.log(state)

    let pool_tate: PoolState = {
        $component_address: address as ComponentAddressString,

        pool_resource_address: state[5],

        pool_share_resource_address: state[6],

        flashloan_term_resource_address: state[7],

        interest_factory_address: state[8],

        oracle_address: state[9],

        loan_state_lookup: [],

        price: parseFloat(state[11]),

        last_price_update: parseFloat(state[12]),

        flashloan_fee_rate: parseFloat(state[13]),

        liquidation_threshold: parseFloat(state[14]),

        liquidation_spread: parseFloat(state[15]),

        liquidation_close_factor: parseFloat(state[16]),

        available_liquidity: parseFloat(state[17]),

        pool_share_supply: parseFloat(state[18]),

        total_collateral: parseFloat(state[19]),

        // last_total_loans_amount: parseFloat(state[20]),

        ///////

        interest_rate: 0,
        total_liquidity: 0,
        pool_share_ratio: 0,
        total_loan_share: 0,
        total_loans: 0,
        loan_to_share_ratio: 0,
        total_collateral_value: 0,
        borrow_limit: 0,
        usage: 0
    }


    let lookup1 = state[10] as any[];

    lookup1.forEach((element) => {
        pool_tate.loan_state_lookup.push({
            interest_type: element[1][0],
            total_loan: parseFloat(element[1][1]),
            total_loan_share: parseFloat(element[1][2]),
            interest_rate: parseFloat(element[1][3]),
            interest_updated_at: parseFloat(element[1][4]),
        })

    });


    pool_tate.interest_rate = pool_tate.loan_state_lookup[INTEREST_RATE_TYPE].interest_rate;
    pool_tate.total_liquidity = pool_tate.available_liquidity + pool_tate.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
    pool_tate.total_loans = pool_tate.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
    pool_tate.total_loan_share = pool_tate.loan_state_lookup[INTEREST_RATE_TYPE].total_loan_share;
    pool_tate.loan_to_share_ratio = pool_tate.total_loans == 0 ? 1 : pool_tate.total_loan_share / pool_tate.total_loans;
    pool_tate.pool_share_ratio = pool_tate.total_liquidity == 0 ? 1 : pool_tate.pool_share_supply / pool_tate.total_liquidity;
    pool_tate.total_collateral_value = pool_tate.total_collateral / pool_tate.pool_share_ratio;
    pool_tate.borrow_limit = pool_tate.available_liquidity - pool_tate.total_collateral_value;
    pool_tate.usage = pool_tate.total_liquidity == 0 ? 0 : pool_tate.total_loans / pool_tate.total_liquidity;

    console.log(pool_tate)

    return pool_tate
}
