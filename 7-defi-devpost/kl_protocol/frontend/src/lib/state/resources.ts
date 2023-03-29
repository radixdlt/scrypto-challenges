
import { stateApi } from "$lib/api/rdt";
import type { ResourceAddressString } from "@radixdlt/radix-dapp-toolkit";
import { get, writable } from "svelte/store";
import { dapp_state } from "./dapp";

export type ResourceMetadata = {
    address: string,
    symbol: string,
    name: string,
    icon: string,
}

let _resources: Record<ResourceAddressString, ResourceMetadata> = {}

export const resources = writable(_resources);

export async function load_faucet_state1() {

    resources.update((_resources) => { return {} })

    let data = get(dapp_state)

    let address = data.faucetComponentAddress

    let result = await stateApi.entityDetails({ entityDetailsRequest: { address } })

    if (result.details === undefined) return;


    let state = ((result.details as any)['state']['data_json']) as any;




    let lookup1 = state[2] as any[];

    console.log(state)

    let tasks = lookup1.map(async (element) => {


        let resource_address = element[0] as ResourceAddressString;

        let prices = parseFloat(element[1])




        let new_resource_metadata = await load_resource_metada(resource_address, prices)

        resources.update(function (_resources) {
            _resources[resource_address] = new_resource_metadata;
            return _resources;
        })



    });

    await Promise.all(tasks)


    console.log(get(resources))
}

export async function load_resource_metada(resource_address: string, price: number = 0): Promise<ResourceMetadata> {

    let resource_metada = get(resources)[resource_address as ResourceAddressString] ?? {
        address: resource_address,
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
