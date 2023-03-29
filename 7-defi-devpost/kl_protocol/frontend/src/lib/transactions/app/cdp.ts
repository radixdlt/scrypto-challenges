


import { rdt, update_dapp_state } from "$lib/api/rdt"
import { send_transaction } from "$lib/common"
import { dapp_state } from "$lib/state/dapp"
import { pool_manager_state } from "$lib/state/lending_pool_manager"
import { Expression, ManifestBuilder, Proof, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"


export async function create_cdp() {
    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'create_cdp', [])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])

    send_transaction(txManifest)
}

export async function create_delegated_cdp(cdp_id: string) {
    let data = get(dapp_state)

    let txManifest = create_cdp_proof(cdp_id)

    txManifest
        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'create_delegated_cdp', [
            Proof('cdp_proof')
        ])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])

    send_transaction(txManifest)
}


export function create_cdp_proof(cdp_id: string, proof_name = 'cdp_proof'): ManifestBuilder {
    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()

    txManifest.createProofFromAccountByIds(
        data.accountAddress as ComponentAddressString,
        `Array<NonFungibleLocalId>(NonFungibleLocalId("${cdp_id}"))`,
        get(pool_manager_state).cdp_resource_address as ResourceAddressString
    )
        .createProofFromAuthZoneByIds(
            `Array<NonFungibleLocalId>(NonFungibleLocalId("${cdp_id}"))`,
            get(pool_manager_state).cdp_resource_address as ResourceAddressString,
            proof_name
        )

    return txManifest

}