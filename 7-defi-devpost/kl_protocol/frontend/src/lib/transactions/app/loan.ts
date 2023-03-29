


import { send_transaction } from "$lib/common"
import { dapp_state } from "$lib/state/dapp"
import { pool_manager_state } from "$lib/state/lending_pool_manager"
import { Bucket, Decimal, Expression, ManifestBuilder, Proof, ResourceAddress, U128, U8, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { rdt, update_dapp_state } from "../../api/rdt"

import { create_cdp_proof } from "./cdp"

export async function borrow(cdp_id: string, resource_address: string, amount: number, position_id: number = 0) {
    let data = get(dapp_state)

    let txManifest = create_cdp_proof(cdp_id)

    if (position_id == 0) {
        txManifest.callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'borrow', [
            Proof("cdp_proof"),
            ResourceAddress(resource_address as ResourceAddressString),
            Decimal(amount),
            U8(1)
        ])
    } else {
        txManifest.callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'borrow_more', [
            Proof("cdp_proof"),
            U128(position_id.toString()),
            Decimal(amount),
        ])
    }

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])


    send_transaction(txManifest)
}

export async function repay(cdp_id: string, resource_address: string, amount: number, position_id: number) {


    let data = get(dapp_state)

    let txManifest = create_cdp_proof(cdp_id)

    txManifest
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, amount, resource_address as ResourceAddressString)
        .takeFromWorktop(resource_address as ResourceAddressString, 'resource')
        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'repay', [
            Proof("cdp_proof"),
            Bucket("resource"),
            U128(position_id.toString())
        ])

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])


    send_transaction(txManifest)


}
