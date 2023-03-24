import { backend_api_url, position_address, radix_api_url, stable_coin, token_default } from "../general/constants";
import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest,
    NonFungibleDataRequest,
    NonFungibleDataResponse
} from "@radixdlt/babylon-gateway-api-sdk";

import { pool, position, step, token, decoded, Hexes } from "types";


async function getPositionHex(position_id: string): Promise<Hexes> {

    const obj: NonFungibleDataRequest = {
        "address": position_address,
        "non_fungible_id": position_id
    };

    const response = await fetch(radix_api_url + '/non-fungible/data', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8' }),
    })

    if (!response.ok) {
        throw new Error("Gateway error : " + response.statusText);
    }

    const data = await response.json();
    const responseData = data as NonFungibleDataResponse;

    if (responseData.mutable_data_hex === undefined || responseData.immutable_data_hex === undefined) {
        throw new Error("Data hex undefined");
    }

    const mutable_hex = responseData.mutable_data_hex
    const immutable_hex = responseData.immutable_data_hex

    return { mutable_hex, immutable_hex, id: position_id };

}


async function decode_position(mutable_hex: string, immutable_hex: string): Promise<any> {
    const params = new URLSearchParams();
    params.append('mutable_data_hex', mutable_hex);
    params.append('immutable_data_hex', immutable_hex);

    const request = new Request(`${backend_api_url}/position?${params}`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    });

    const res = await fetch(request);
    return res.json();
}


async function getOwnedPositions(account: string, pools: pool[], tokens: token[]): Promise<position[]> {

    const obj: EntityNonFungibleIdsRequest = {
        "address": account,
        "resource_address": position_address
    };

    let data: any;
    await fetch(radix_api_url + `/entity/non-fungible/ids`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["non_fungible_ids"]["items"])
        .catch(console.error);

    if (!data) return [];

    const positions: position[] = [];
    // @ts-ignore
    for (let i = 0; i < data.length; ++i) {

        const nf_id = data[i]["non_fungible_id"];

        const hex = await getPositionHex(nf_id);
        const decodedPosition = await decode_position(hex.mutable_hex, hex.immutable_hex);
        var total_liquidity = 0;
        for (var j: number = 0; j < decodedPosition.step_positions.length; ++j) {
            total_liquidity += decodedPosition.step_positions[j].liquidity
        }

        var token = token_default;
        for (var j = 0; j < tokens.length; ++j) if (tokens[i].address == decodedPosition.other_token) token = tokens[i];

        positions.push({ ...{ nfIdValue: await getNFIDValue(nf_id) }, token: token, id: data[i]["non_fungible_id"], x_fees: 0, y_fees: 0, value_locked: total_liquidity, price_x: 1, liquidity: total_liquidity });

    }

    return positions;
}

async function getNFIDValue(id: string) {

    const obj: NonFungibleDataRequest = {
        "address": position_address,
        "non_fungible_id": id
    }

    let data: any;
    await fetch(radix_api_url + `/non-fungible/data`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);

    return data

}

async function getPoolInformation(token: token, pool_address: string): Promise<pool> {

    const obj: EntityDetailsRequest = {
        "address": pool_address
    }

    let data: any;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    const pool_steps: step[] = await Promise.all(data[6].map((pool_step: string[]) => {
        return getPoolStep(parseFloat(pool_step[0]), pool_step[1]);
    }));
    var steps: step[];

    const current_step = parseFloat(data[1]);
    var flag: boolean = false;
    for (var i = 0; i < pool_steps.length; ++i) {
        flag = flag || pool_steps[i].step_id == current_step;
    }
    if (!flag) steps = pool_steps.concat([{ other_token_amount: 0, stablecoin_amount: 0, step_id: current_step, rate: parseFloat(data[2]) * (1 + parseFloat(data[0]) ** current_step) }])
    else steps = pool_steps;

    return { token: token, rate_step: parseFloat(data[0]), current_step: parseFloat(data[1]), min_rate: parseFloat(data[2]), max_rate: parseFloat(data[3]), steps: steps.sort((a, b) => a.step_id - b.step_id) };
}

async function getPoolStep(step_id: number, step_address: string): Promise<step> {

    const obj: EntityDetailsRequest = {
        "address": step_address
    }

    let data: any;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    return {
        step_id: step_id,
        stablecoin_amount: parseFloat(data[0]),
        other_token_amount: parseFloat(data[1]),
        rate: parseFloat(data[2])
    };
}


export { getPoolInformation, getOwnedPositions }