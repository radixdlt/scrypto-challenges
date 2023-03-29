

import { send_transaction } from "$lib/common"
import { dapp_state } from "$lib/state/dapp"
import { } from "$lib/state/lending_pool_manager"
import { Bucket, Expression, ManifestBuilder, ResourceAddress, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { XRD } from "../../data"
import { update_dapp_state } from "../api/rdt"


export async function get_resources(resource_address: string) {
    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, 50, XRD)
        .takeFromWorktop(XRD, 'xrd')
        .callMethod(data.faucetComponentAddress as ComponentAddressString, 'get_resource', [
            ResourceAddress(resource_address as ResourceAddressString), Bucket('xrd')
        ])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])

    let result = await send_transaction(txManifest)
}