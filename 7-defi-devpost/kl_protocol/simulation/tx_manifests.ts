import { Bucket, NonFungibleId, Proof, ResourceAddressString } from '@radixdlt/wallet-sdk';
import * as _ from './const.js';

import { ManifestBuilderExt } from './lib/manifest_builder_extended.js';
import { get, _run_temp_manifest } from './lib/utils.js';

const _XRD_ = "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"

export async function auto_liquidate({ delegator_id }: { delegator_id: string }, name = 'auto_liquidate') {

    const manifest = new ManifestBuilderExt()
        .lockFee(get(_.main_account), 30)
        .createProofFromAccount(get(_.main_account), get(_.lending_admin_badge))
        .callMethod(
            get(_.lending_component),
            "auto_liquidate",
            [
                `NonFungibleLocalId("#${delegator_id}#")`,
            ]
        )
        .depositEntireWorktop(get(_.main_account))
        .build()
        .toString()

    return _run_temp_manifest(manifest, name);
}

export async function create_and_send_delegated_cdp({ from, to, delegator_cdp_id: delegator_id }: { from: number; to: number; delegator_cdp_id: string; }, name = 'create_and_send_delegated_cdp') {

    const manifest = new ManifestBuilderExt()
        .lockFee(get(_.account, from), 10)
        .createProofFromAccountByIds(get(_.account, from), `Array<NonFungibleLocalId>(NonFungibleLocalId("#${delegator_id}#"))`, get(_.cdp_resource_address) as ResourceAddressString)
        .createProofFromAuthZoneByIds(`Array<NonFungibleLocalId>(NonFungibleLocalId("#${delegator_id}#"))`, get(_.cdp_resource_address) as ResourceAddressString, "delegator_cdp_proof")
        .callMethod(
            get(_.lending_component),
            "create_delegated_cdp",
            [
                Proof("delegator_cdp_proof"),
            ]
        )
        .takeFromWorktop(get(_.cdp_resource_address) as ResourceAddressString, "delegated_cdp")
        .callMethod(
            get(_.account, to),
            'deposit',
            [
                Bucket("delegated_cdp"),
            ]
        )
        .depositEntireWorktop(get(_.account, from))
        .build()
        .toString()

    return _run_temp_manifest(manifest, name);
}


// export async function create_radiswap_pool(fee: number) {
//     const manifest = new ManifestBuilderExt()
//         .lockFee(ge(_.main_account), 10)
//         .withdrawToBucketByAmount(ge(_.main_account), ge(_.STABLE), 'stable_bucket', 10000)
//         .withdrawToBucketByAmount(ge(_.main_account), _XRD_, 'xrd_bucket', 500)
//         .callFunction(
//             ge(_.pool_manager_package),
//             "Radiswap",
//             "instantiate_pool",
//             [
//                 Bucket('stable_bucket'),
//                 Bucket('xrd_bucket'),
//                 Decimal(1000),
//                 String("LP-STABLE-XRD"),
//                 String("STABLE/XRD Liquidity provider token"),
//                 String(""),
//                 Decimal(0.05),
//             ]
//         )
//         .depositEntireWorktop(ge(_.main_account))
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);
// }

// export async function create_flashloan_pool(fee: number) {
//     const manifest = new ManifestBuilderExt()
//         .lockFee(ge(_.main_account), 10)
//         .callFunction(
//             ge(_.pool_manager_package),
//             "FlashLoanPool",
//             "instantiate_default",
//             [
//                 ResourceAddress(_XRD_),
//                 Decimal(fee),
//             ]
//         )
//         .depositEntireWorktop(ge(_.main_account))
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);
// }

// export async function add_liquidity(user_account: ComponentAddressString, amount: number) {
//     const manifest = new ManifestBuilderExt()
//         .lockFee(user_account, 10)
//         .withdrawToBucketByAmount(user_account, _XRD_, 'xrd_bucket', amount)
//         .callMethod(
//             ge(_.pool_manager_component),
//             "add_liquidity",
//             [
//                 Bucket('xrd_bucket')
//             ]
//         )
//         .depositEntireWorktop(user_account)
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);
// }

// export async function remove_liquidity(user_account: ComponentAddressString) {
//     const manifest = new ManifestBuilderExt()
//         .lockFee(user_account, 10)
//         .withdrawToBucket(user_account, ge(_.pool_manager_lp_token), 'lp_token')
//         .callMethod(
//             ge(_.pool_manager_component),
//             "remove_liquidity",
//             [
//                 Bucket('lp_token')
//             ]
//         )
//         .depositEntireWorktop(user_account)
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);
// }

// export async function simulate_flash_loan(user_account: ComponentAddressString, pool: ComponentAddressString, transient_token: ResourceAddressString, ammount: number, fee: number) {

//     const manifest = new ManifestBuilderExt()
//         .lockFee(user_account, 10)
//         .withdrawFromAccountByAmount(user_account, fee, _XRD_)
//         .callMethod(
//             pool,
//             "take_loan",
//             [Decimal(ammount)]
//         )
//         .takeFromWorktopByAmount(ammount + fee, _XRD_, 'xrd_bucket')
//         .takeFromWorktop(transient_token, 'transient_token')
//         .callMethod(
//             pool,
//             "repay_loan",
//             [Bucket('xrd_bucket'), Bucket('transient_token')]
//         )
//         .depositeFromWorktop(user_account, _XRD_)
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);
// }


// export async function add_pool_to_manager(user_account: ComponentAddressString, pool_index: number, pool_weight: number) {

//     const manifest = new ManifestBuilderExt()
//         .lockFee(user_account, 10)
//         .createProofFromAccount(user_account, ge(pool_manager_admin_badge))

//         .withdrawToBucket(user_account, ge(flashloan_admin_badge, pool_index), 'pool_admin_badge')

//         .callMethod(
//             ge(pool_manager_component),
//             "add_pool",
//             [
//                 ComponentAddress(ge(flashloan_component, pool_index)),
//                 Bucket("pool_admin_badge"),
//                 Decimal(pool_weight)
//             ]
//         ).clearAuthZone()
//         .depositEntireWorktop(user_account)
//         // .depositeFromWorktop(user_account, _XRD_)
//         .build()
//         .toString()

//     return _run_temp_manifest(manifest);

// }

