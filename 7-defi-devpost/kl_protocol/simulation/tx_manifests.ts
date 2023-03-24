import { Bucket, NonFungibleId, Proof, ResourceAddressString } from '@radixdlt/wallet-sdk';
import * as _ from './const.js';

import { ManifestBuilderExt } from './lib/manifest_builder_extended.js';
import { get, _run_temp_manifest } from './lib/utils.js';

const _XRD_ = "resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety"

export async function auto_liquidate({ cdp_id }: { cdp_id: string }, name = 'auto_liquidate') {

    const manifest = new ManifestBuilderExt()
        .lockFee(get(_.main_account), 30)
        .createProofFromAccount(get(_.main_account), get(_.lending_admin_badge))
        .callMethod(
            get(_.lending_component),
            "auto_liquidate",
            [
                `NonFungibleLocalId("#${cdp_id}#")`,
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

