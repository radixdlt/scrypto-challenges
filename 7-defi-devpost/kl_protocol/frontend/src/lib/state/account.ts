import { stateApi } from "$lib/api/rdt";
import type { FungibleResourcesCollectionItem } from "@radixdlt/babylon-gateway-api-sdk";
import { get, writable } from "svelte/store";
import { dapp_state } from "./dapp";
import { pool_manager_state, type CollaterizedDebtPositionData } from "./lending_pool_manager";



type AcountState = {
    fungible_resources: FungibleResourcesCollectionItem[],
    // cdp?: CollaterizedDebtPositionData,
    cdps: Record<string, CollaterizedDebtPositionData>
}

export const accout_ressource_state = writable<AcountState>();

export async function loan_user_resources() {

    let state = await stateApi.entityResources({ entityResourcesRequest: { address: get(dapp_state).accountAddress } })

    let cdp_resource_address = get(pool_manager_state).cdp_resource_address;

    let new_state: AcountState = {
        fungible_resources: state.fungible_resources.items,
        cdps: {}
    }


    //

    let cdp_data = await stateApi.entityNonFungibleIds({
        entityNonFungibleIdsRequest: {
            address: get(dapp_state).accountAddress,
            resource_address: cdp_resource_address
        }
    })


    cdp_data.non_fungible_ids.items.forEach(_item => {


        new_state.cdps[_item.non_fungible_id] = get(pool_manager_state).cdp_lookup[_item.non_fungible_id]

    })

    // new_state.cdp = Object.values(new_state.cdps)[0]


    accout_ressource_state.update(_ => {
        return new_state
    })

}



