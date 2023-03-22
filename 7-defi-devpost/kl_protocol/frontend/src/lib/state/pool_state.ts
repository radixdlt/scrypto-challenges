import { stateApi } from "$lib/api/rdt";
import { dapp_data } from "$lib/data";
import type { ComponentAddressString, ResourceAddressString } from "@radixdlt/radix-dapp-toolkit";
import { get, writable } from "svelte/store";
import type { ResourceMetadata, PoolManagerState, FaucetState, PoolState } from "./types";


let _lending_pools: PoolState[] = []
let _resources: Record<ResourceAddressString, ResourceMetadata> = {}

let _pool_manager_state: PoolManagerState = {
    cdp_resource_address: "",
    component_address_lookup: {},
    pool_share_resource_address_lookup: {},
    resource_address_lookup: {},
    exchange_component_address: "",
    debt_position_counter: 0,
    collateral_position_counter: 0,
    cdp_position_counter: 0
}

// let _faucet_state: FaucetState = {
//     resource_prices: {}
// }


// export const faucet_state = writable(_faucet_state);
export const pool_manager_state = writable(_pool_manager_state);
export const lending_pools = writable(_lending_pools);
export const resources = writable(_resources);

export async function load_faucet_state() {

    let data = get(dapp_data)

    let address = data.faucetComponentAddress

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;


    let state = ((result.details as any)['state']['data_json']) as any;

    // console.log(state)

    let new_faucet_state: FaucetState = {
        resource_prices: {}
    }

    let lookup1 = state[2] as any[];

    console.log(state)

    let tasks = lookup1.map(async (element) => {


        let resource_address = element[0] as ResourceAddressString;

        let prices = parseFloat(element[1])


        if (get(resources)[resource_address] === undefined) {

            let new_resource_metadata = await load_resource_metada(resource_address, prices)

            resources.update(function (_resources) {
                _resources[resource_address] = new_resource_metadata;
                return _resources;
            })
        }

    });

    await Promise.all(tasks)


    // faucet_state.update(_ => new_faucet_state);


    // console.log(get(faucet_state))
    console.log(get(resources))
}

export async function load_manager_pool_state() {

    let data = get(dapp_data)

    let address = data.lendingMarketComponentAddress

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;


    let state = ((result.details as any)['state']['data_json']) as any;

    // console.log(state)

    let new_pool_manager_state: PoolManagerState = {
        cdp_resource_address: state[1],
        component_address_lookup: {},
        pool_share_resource_address_lookup: {},
        resource_address_lookup: {},
        exchange_component_address: state[3],
        debt_position_counter: state[4],
        collateral_position_counter: state[5],
        cdp_position_counter: state[6]
    }

    let lookup1 = state[2][0] as any[];

    let tasks = lookup1.map(async (element) => {
        new_pool_manager_state.component_address_lookup[element[0] as ResourceAddressString] = element[1]

        let new_pool = await load_pool_state(element[1])
        if (new_pool !== undefined) {
            lending_pools.update(lending_pools => {
                let p = [...lending_pools, new_pool!]

                // p.sort((a, b) => a.pool_resource_address > b.pool_resource_address ? 1 : -1)

                return p
            })
        }
    });

    await Promise.all(tasks)

    let lookup2 = state[2][1] as any[];

    tasks = lookup2.map(async (element) => {


        let resource_address1 = element[0] as ResourceAddressString;
        let resource_address2 = element[1] as ResourceAddressString;

        new_pool_manager_state.pool_share_resource_address_lookup[resource_address1] = resource_address2

        if (get(resources)[resource_address1] === undefined) {

            let new_resource_metadata = await load_resource_metada(resource_address1)

            resources.update(function (_resources) {
                _resources[resource_address1] = new_resource_metadata;
                return _resources;
            })
        }

        if (get(resources)[resource_address2] === undefined) {

            let new_resource_metadata = await load_resource_metada(resource_address2)

            resources.update(function (_resources) {
                _resources[resource_address2] = new_resource_metadata;
                return _resources;
            })
        }



    });

    await Promise.all(tasks)

    let lookup3 = state[2][2] as any[];
    lookup3.forEach((element) => {
        new_pool_manager_state.resource_address_lookup[element[0] as ResourceAddressString] = element[1]
    });

    pool_manager_state.update(_ => new_pool_manager_state);


    console.log(get(pool_manager_state))
    console.log(get(resources))
    console.log(get(lending_pools))

}

async function load_pool_state(address: string): Promise<PoolState | undefined> {

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;


    let state = ((result.details as any)['state']['data_json']) as any;


    let pool_tate: PoolState = {
        $component_address: address as ComponentAddressString,

        pool_resource_address: state[5],

        pool_share_resource_address: state[6],

        flashloan_term_resource_address: state[7],

        interest_factory_address: state[8],

        oracle_address: state[9],

        loan_state_lookup: [],

        last_price: parseFloat(state[11]),

        last_price_update: parseFloat(state[12]),

        flashloan_fee_rate: parseFloat(state[13]),

        liquidation_threshold: parseFloat(state[14]),

        liquidation_spread: parseFloat(state[15]),

        liquidation_close_factor: parseFloat(state[16]),

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

    // let lookup2 = state[2][1] as any[];

    // lookup2.forEach((element) => {
    //     pool_manager_state.pool_share_resource_address_lookup[element[0] as string] = element[1]
    // });

    // let lookup3 = state[2][2] as any[];
    // lookup2.forEach((element) => {
    //     pool_manager_state.resource_address_lookup[element[0] as string] = element[1]
    // });

    return pool_tate
}

async function load_resource_metada(resource_address: string, price: number = 0): Promise<ResourceMetadata> {

    let resource_metada = get(resources)[resource_address as ResourceAddressString] ?? {
        address: resource_address,
        price: price,
        symbol: "",
        name: "",
        icon: ""
    }

    let metadata_result = await stateApi.entityMetadata({
        entityMetadataRequest: { address: resource_address }
    })

    metadata_result.metadata.items.forEach((data) => {
        if (data.key === "icon") resource_metada.icon = data.value
        if (data.key === "symbol") resource_metada.symbol = data.value
        if (data.key === "name") resource_metada.name = data.value
    })

    return resource_metada

}
