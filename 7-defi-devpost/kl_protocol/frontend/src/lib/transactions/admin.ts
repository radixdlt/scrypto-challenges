
import { load_faucet_state1 as load_faucet_state, resources } from "$lib/state/resources"
import type { TransactionCommittedDetailsResponse } from "@radixdlt/babylon-gateway-api-sdk"
import { ComponentAddress, Decimal, Expression, ManifestBuilder, ResourceAddress, String, type ComponentAddressString, type PackageAddressString, type ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"
import { get } from "svelte/store"
import { rdt, stateApi, transactionApi, update_dapp_state } from "../api/rdt"
import { default_asset_list, XRD, } from "../../data"
import { dapp_state, price_changes } from "$lib/state/dapp"
import { lending_pools } from "$lib/state/lending_pools"


export async function instantiate_faucet() {
    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .callMethod(data.accountAddress as ComponentAddressString, 'create_proof', [
            ResourceAddress(XRD)
        ])
        .callFunction(data.packageAddress as PackageAddressString, 'Faucet', 'new', [
            // Decimal(TIME_FACTOR)
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

    await load_faucet_receipt(result.value.transactionIntentHash)
}

export async function load_faucet_receipt(transactionIntentHash: string) {
    let receipt = await get_receipt(transactionIntentHash)

    console.log(receipt)

    let faucet_comp = receipt.details.referenced_global_entities[0]
    let faucet_admin_badge = ''

    let addresses = (await get_entity_addresses(receipt, ['faucet_admin_bage']))
    faucet_admin_badge = addresses['faucet_admin_bage']

    dapp_state.set({
        ...get(dapp_state),
        faucetComponentAddress: faucet_comp,
        faucetAdminBadgeAddress: faucet_admin_badge,
        faucetCreationTxHash: transactionIntentHash,
    })

    update_dapp_state()
}

export async function instantiate_lending_market() {
    let data = get(dapp_state)

    let txManifest = new ManifestBuilder()
        .callMethod(data.accountAddress as ComponentAddressString, 'create_proof', [
            ResourceAddress(XRD)
        ])
        .callFunction(data.packageAddress as PackageAddressString, 'LendingPoolManager', 'instantiate', [
            ComponentAddress(data.faucetComponentAddress as ComponentAddressString)
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

    await load_lending_market_receipt(result.value.transactionIntentHash)
}

export async function load_lending_market_receipt(transactionIntentHash: string) {
    let receipt = await get_receipt(transactionIntentHash)

    console.log(receipt)

    let comp = receipt.details.referenced_global_entities[0]

    let addresses = (await get_entity_addresses(receipt, ['lendind_market_admin_badge', 'cdp_resource']))
    let admin_badge = addresses['lendind_market_admin_badge']
    let cdp = addresses['cdp_resource']

    dapp_state.set({
        ...get(dapp_state),
        lendingMarketComponentAddress: comp,
        lendingMarketAdminBadgeAddress: admin_badge,
        lendingMarketCreationTxHash: transactionIntentHash,
    })

    update_dapp_state()
}

export async function create_resources() {

    let data = get(dapp_state)

    let txManifestString = ''
    let txManifest = new ManifestBuilder()

    txManifest
        .createProofFromAccount(data.accountAddress as ComponentAddressString, data.faucetAdminBadgeAddress as ResourceAddressString)

    default_asset_list.forEach((asset_details) => {


        if (asset_details.resource_address === undefined) {
            txManifest.callMethod(data.faucetComponentAddress as ComponentAddressString, 'create_resource', [
                String(asset_details.symbol),
                String(asset_details.name),
                String(asset_details.icon),
                Decimal(asset_details.initial_price)
            ])
        } else {
            txManifest.callMethod(data.faucetComponentAddress as ComponentAddressString, 'update_price', [
                ResourceAddress(asset_details.resource_address as ResourceAddressString),
                Decimal(asset_details.initial_price),
            ])
        }

    })

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])

    txManifestString = txManifest.build().toString()

    console.log(txManifestString)

    let result = await rdt.sendTransaction({
        transactionManifest: txManifestString,
        version: 1
    })

    console.log(result)

    if (result.isErr()) throw result.error


    await load_faucet_state()

    update_dapp_state()

}

export async function create_lending_pools() {
    let data = get(dapp_state)

    default_asset_list

    let txManifest = new ManifestBuilder()
        .createProofFromAccount(data.accountAddress as ComponentAddressString, data.lendingMarketAdminBadgeAddress as ResourceAddressString)

    default_asset_list.forEach((asset_details) => {

        let resource_address = asset_details.resource_address ?? Object.values(get(resources)).find(x => x.symbol === asset_details.symbol)?.address


        console.log(resource_address)

        if (resource_address !== undefined) {
            txManifest.callMethod(data.lendingMarketComponentAddress as ComponentAddressString, 'create_lending_pool', [
                ResourceAddress(resource_address as ResourceAddressString),
                String(`LEND-${asset_details.symbol}`),
                String(`Lended ${asset_details.symbol}`),
                String(`${asset_details.icon}`),
                Decimal(asset_details.flashloan_fee_rate ?? 0.005),
                Decimal(asset_details.liquidation_threshold),
                Decimal(asset_details.liquidation_spread ?? 0.05),
                Decimal(asset_details.liquidation_closing_factor ?? 0.5),
                ComponentAddress(data.faucetComponentAddress as ComponentAddressString),
                ComponentAddress(data.faucetComponentAddress as ComponentAddressString),
                String(asset_details.symbol),
            ])
        }

    })

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])

    let txManifestString = txManifest.build().toString()

    console.log(txManifestString)

    let result = await rdt.sendTransaction({
        transactionManifest: txManifestString,
        version: 1
    })

    // console.log(result)

    if (result.isErr()) throw result.error


    update_dapp_state()


}


//
//
//

export async function change_prices() {
    let data = get(dapp_state)


    let asset_list = get(price_changes)



    let txManifest = new ManifestBuilder()
        .createProofFromAccount(data.accountAddress as ComponentAddressString, data.faucetAdminBadgeAddress as ResourceAddressString)

    Object.keys(asset_list).forEach((key) => {

        let price = asset_list[key]

        txManifest
            .callMethod(data.faucetComponentAddress as ComponentAddressString, 'update_price', [
                ResourceAddress(key as ResourceAddressString),
                Decimal(price),
            ])
    })

    txManifest.callMethod(data.accountAddress as ComponentAddressString, 'deposit_batch', [
        Expression('ENTIRE_WORKTOP')
    ])

    let txManifestString = txManifest.build().toString()

    // console.log(txManifestString)

    let result = await rdt.sendTransaction({
        transactionManifest: txManifestString,
        version: 1
    })


    if (result.isErr()) throw result.error



    update_dapp_state()

}

export async function get_pool_state() {
    let data = get(dapp_state)
    let pools = get(lending_pools)




    let txManifest = new ManifestBuilder()

    txManifest
        .callMethod(data.accountAddress as ComponentAddressString, 'create_proof', [
            ResourceAddress(XRD)
        ])
    Object.values(pools).forEach((pool) => {
        txManifest
            .callMethod(pool.$component_address as ComponentAddressString, 'update_price', [])
        // .callMethod(pool.$component_address as ComponentAddressString, 'update_all_interest', [])

    })

    let transactionManifest = txManifest.build().toString()

    console.log(txManifest)

    let result = await rdt.sendTransaction({
        transactionManifest,
        version: 1
    })

    console.log(result)

    if (result.isErr()) throw result.error

    update_dapp_state()

}

//
//
//


async function get_receipt(tx_intent_hash: string) {

    // fetch commit receipt from gateway api
    let commitReceipt = await transactionApi.transactionCommittedDetails({
        transactionCommittedDetailsRequest: {
            transaction_identifier: {
                type: 'intent_hash',
                value_hex: tx_intent_hash
            }
        }
    })

    return commitReceipt
}

async function get_entity_addresses(receipt: TransactionCommittedDetailsResponse, internal_tags: string[], metadata_key = 'internal_tag') {

    let result: Record<string, string> = {}

    let metadata_request_list = receipt.details.referenced_global_entities.map(async (address) => {

        let metadata_result = await stateApi.entityMetadata({
            entityMetadataRequest: { address }
        })

        metadata_result.metadata.items.forEach((data) => {
            internal_tags.forEach((internal_tag) => {
                if (data.key == metadata_key && data.value == internal_tag) {
                    result[internal_tag] = address
                    return
                }
            })
        })

    })

    await Promise.all(metadata_request_list)

    console.log(result)

    return result
}


// 