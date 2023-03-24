import { stateApi } from "$lib/api/rdt";
import type { ComponentAddressString, ResourceAddressString } from "@radixdlt/radix-dapp-toolkit";
import { get, writable } from "svelte/store";
import { dapp_state } from "./dapp";
import { lending_pools, load_pool_state, type PoolState } from "./lending_pools";
import { load_resource_metada, resources } from "./resources";


export type CollateralPostion = {
    position_id: number,
    resource_address: string,
    pool_share_resource_address: string,
    pool_component_address: string,
    pool_share: number,
}

export type DebtPostion = {
    position_id: number,
    resource_address: string,
    pool_share_resource_address: string,
    loan_share: number,
    interest_type: number,
}

export type CollaterizedDebtPositionData = {
    cdp_id: string,
    delegator_cdp_id: string,
    delegated_cdp_ids: string[],
    collaterals: Record<string, CollateralPostion>,
    debts: Record<string, DebtPostion>,
}

export type PoolManagerState = {
    cdp_resource_address: string, // 1

    component_address_lookup: Record<ResourceAddressString, ComponentAddressString>
    pool_share_resource_address_lookup: Record<ResourceAddressString, ResourceAddressString>
    resource_address_lookup: Record<ResourceAddressString, ResourceAddressString>

    exchange_component_address: string, //3

    debt_position_counter: number,//4
    collateral_position_counter: number,//5
    cdp_position_counter: number,//6

    cdp_lookup: Record<string, CollaterizedDebtPositionData>

    admin_badge_resource_address: string

}

let _pool_manager_state: PoolManagerState = {
    cdp_resource_address: "",
    component_address_lookup: {},
    pool_share_resource_address_lookup: {},
    resource_address_lookup: {},
    exchange_component_address: "",
    debt_position_counter: 0,
    collateral_position_counter: 0,
    cdp_position_counter: 0,
    cdp_lookup: {},
    admin_badge_resource_address: ""
}

export const pool_manager_state = writable(_pool_manager_state);

export async function load_manager_pool_state() {

    pool_manager_state.update(_ => _pool_manager_state)

    let _lending_pools: Record<string, PoolState> = {}

    lending_pools.update(_ => _lending_pools)

    let data = get(dapp_state)

    let address = data.lendingMarketComponentAddress

    if (address == '') return

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;

    let state = ((result.details as any)['state']['data_json']) as any;

    console.log(state)

    let new_pool_manager_state: PoolManagerState = {
        cdp_resource_address: state[1],
        component_address_lookup: {},
        pool_share_resource_address_lookup: {},
        resource_address_lookup: {},
        exchange_component_address: state[3],
        debt_position_counter: state[4],
        collateral_position_counter: state[5],
        cdp_position_counter: state[6],
        admin_badge_resource_address: state[7],
        cdp_lookup: {},
    }

    //
    // COMPONENT ADDRESS LOOKUP

    let lookup1 = state[2][0] as any[];

    let tasks = lookup1.map(async (element) => {
        new_pool_manager_state.component_address_lookup[element[0] as ResourceAddressString] = element[1]

        let new_pool = await load_pool_state(element[1])
        if (new_pool !== undefined) {
            _lending_pools = { ..._lending_pools, [new_pool!.$component_address]: new_pool! }
        }
    });


    //
    // RESOURCE ADDRES LOOKUP

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



    // 
    // CDP DATA

    let lookup4 = state[8] as any[];

    tasks = lookup4.map(async (element) => {

        let cdp_id = element[0]['value'];

        //
        let _delegator_cdp_id = element[1][0]['fields'][0];
        let delegator_cdp_id = _delegator_cdp_id === undefined ? "" : _delegator_cdp_id['value'];

        //
        let delegated_cdp_ids = element[1][1].map((e: { value: any; }) => e.value);

        //
        let collaterals = element[1][2]
        let debts = element[1][3]

        new_pool_manager_state.cdp_lookup[cdp_id] = {
            cdp_id,
            delegator_cdp_id,
            delegated_cdp_ids,
            collaterals: {},
            debts: {},
        }

        collaterals.forEach((element: any[][]) => {

            let c: CollateralPostion = {
                position_id: parseInt(element[1][0]),
                resource_address: element[1][1],
                pool_share_resource_address: element[1][2],
                pool_component_address: element[1][3],
                pool_share: parseFloat(element[1][4])
            }

            new_pool_manager_state.cdp_lookup[cdp_id].collaterals[c.resource_address] = c
        });

        debts.forEach((element: any[][]) => {
            let d: DebtPostion = {
                position_id: parseInt(element[1][0]),
                resource_address: element[1][1],
                pool_share_resource_address: element[1][2],
                loan_share: parseFloat(element[1][3]),
                interest_type: element[1][4],
            }
            new_pool_manager_state.cdp_lookup[cdp_id].debts[d.resource_address] = d
        });



    });

    await Promise.all(tasks)

    console.log(new_pool_manager_state)
    // 
    // 

    pool_manager_state.update(_ => new_pool_manager_state)
    lending_pools.update(_ => _lending_pools)

}

