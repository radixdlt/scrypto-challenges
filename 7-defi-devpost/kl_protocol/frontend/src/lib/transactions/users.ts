


import { pool_manager_state } from "$lib/state/pool_state"
import { Bucket, Expression, ManifestBuilder, Proof, ResourceAddress, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { rdt } from "../api/rdt"
import { dapp_data } from "../data"

const XRD = 'resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp'

export async function create_cdp() {
    let data = get(dapp_data)

    let txManifest = new ManifestBuilder()

        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'create_cdp', [])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])
        .build()
        .toString()

    console.log(txManifest)

    let result = await rdt.sendTransaction({
        transactionManifest: txManifest,
        version: 1
    })

    console.log(result)

    if (result.isErr()) throw result.error
}

export async function new_collateral() {
    let data = get(dapp_data)

    let txManifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, 50, XRD)
        .takeFromWorktop(XRD, 'xrd')
        .createProofFromAccountByIds(data.accountAddress as ComponentAddressString, `Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))`, get(pool_manager_state).cdp_resource_address as ResourceAddressString)
        .createProofFromAuthZoneByIds(`Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))`, get(pool_manager_state).cdp_resource_address as ResourceAddressString, "delegator_cdp_proof")
        .callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'new_collateral', [
            Proof("delegator_cdp_proof"),
            Bucket("xrd")
        ])
        .callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
            Expression('ENTIRE_WORKTOP')
        ])
        .build()
        .toString()

    console.log(txManifest)

    let result = await rdt.sendTransaction({
        transactionManifest: txManifest,
        version: 1
    })

    console.log(result)

    if (result.isErr()) throw result.error
}

