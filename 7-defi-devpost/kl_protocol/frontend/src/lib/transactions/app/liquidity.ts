

import { send_transaction } from "$lib/common"
import { dapp_state } from "$lib/state/dapp"
import { Bucket, Expression, ManifestBuilder, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { rdt, update_dapp_state } from "../../api/rdt"



export async function add_liquidity(resource_address: string, amount: number = 50) {

    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, amount, resource_address as ResourceAddressString)
        .takeFromWorktop(resource_address as ResourceAddressString, 'resource')

    txManifest.callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'add_liquidity', [
        Bucket("resource")
    ])

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])

    send_transaction(txManifest)
}


export async function remove_liquidity(resource_address: string, amount: number = 50) {

    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, amount, resource_address as ResourceAddressString)
        .takeFromWorktop(resource_address as ResourceAddressString, 'resource')
        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'remove_liquidity', [

            Bucket("resource")
        ])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])

    send_transaction(txManifest)
}