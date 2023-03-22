

import { Bucket, Expression, ManifestBuilder, ResourceAddress, type ComponentAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { rdt } from "../api/rdt"
import { dapp_data } from "../data"

const XRD = 'resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp'

export async function get_resources(resource_address: string) {
    let data = get(dapp_data)

    let txManifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(data.accountAddress as ComponentAddressString, 50, XRD)
        .takeFromWorktop(XRD, 'xrd')
        .callMethod(data.faucetComponentAddress as ComponentAddressString, 'get_resource', [
            ResourceAddress(resource_address as ResourceAddressString), Bucket('xrd')
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