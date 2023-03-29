import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest,
    NonFungibleDataRequest,
    NonFungibleDataResponse,
    NonFungibleIdsRequest,
    NonFungibleIdsResponse
} from "@radixdlt/babylon-gateway-api-sdk";
import { backend_api_url, issuer_address, loan_address, radix_api_url, token_default } from "../general/constants";
import { amountToLiquidate } from "./stablecoinMaths";

import { decoded, Hexes, lender, loan } from "types";
import { getToken } from "utils/general/generalApiCalls";


async function getLendersList() {
    const obj: EntityDetailsRequest = {
        "address": issuer_address
    };

    let data;
    await fetch(radix_api_url + `/entity/details`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);

    // @ts-ignore
    return data["details"]["state"]["data_json"][1].map(row => {
        return { token: row[0], lender: row[1] }
    });
}

async function getLenderInformation(lender_address: string) {

    if (!lender_address) return undefined;

    const obj: EntityDetailsRequest = {
        "address": lender_address
    };

    let data;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    if (!data) return undefined;

    const loan_to_value = data[1];
    const daily_interest_rate = data[2];
    const liquidation_threshold = data[3];
    const liquidation_penalty = data[4];
    const oracle_address = data[5];

    const current_price = await getOraclePrice(oracle_address);

    return { lender_address: lender_address, loan_to_value: loan_to_value, daily_interest_rate: daily_interest_rate, liquidation_threshold: liquidation_threshold, liquidation_penalty: liquidation_penalty, collateral_price: current_price }
}

async function getLoansOwnedBy(account: string) {

    const obj: EntityNonFungibleIdsRequest = {
        "address": account,
        "resource_address": loan_address
    };

    let data;
    await fetch(radix_api_url + `/entity/non-fungible/ids`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["non_fungible_ids"]["items"])
        .catch(console.error);

    let loan_ids: any[] = [];
    // @ts-ignore
    for (const id of data) {

        const loan_id = id["non_fungible_id"];
        loan_ids.push(loan_id);
    }

    return loan_ids
}

async function getHex(loan_id: string): Promise<Hexes> {

    const obj: NonFungibleDataRequest = {
        "address": loan_address,
        "non_fungible_id": loan_id
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

    return { mutable_hex, immutable_hex, id: loan_id };

}

async function getOraclePrice(oracle_address: string) {

    const obj: EntityDetailsRequest = {
        "address": oracle_address
    };

    let data;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error)

    // @ts-ignore
    return data[0];
}

async function decode_hex(mutable_hex: string, immutable_hex: string): Promise<decoded> {
    const params = new URLSearchParams();
    params.append('mutable_data_hex', mutable_hex);
    params.append('immutable_data_hex', immutable_hex);

    const request = new Request(`${backend_api_url}/loan?${params}`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    });

    const res = await fetch(request);
    return res.json();
}

async function getLoanInformation(mutable_data: string, immutable_data: string, lenders: Map<string, lender>, id: string): Promise<loan> {

    const data = await decode_hex(mutable_data, immutable_data);

    if (!data) return {
        collateral_token: token_default,
        collateral_amount: 0,
        amount_lent: 0,
        loan_date: 0,
        liquidation_price: 0,
        loan_to_value: 0,
        interest_rate: 0,
        amount_to_liquidate: 0,
        id: "-1"
    };

    const lender: lender = lenders[data.collateral_token];

    if (!lender) return {
        collateral_token: token_default,
        collateral_amount: 0,
        amount_lent: 0,
        loan_date: 0,
        liquidation_price: 0,
        loan_to_value: 0,
        interest_rate: 0,
        amount_to_liquidate: 0,
        id: "-1"
    };

    const collateral_price = lender.collateral_price * data.collateral_amount;

    const amount_to_liquidate_promise = amountToLiquidate(data.collateral_amount, collateral_price, data.amount_lent, lender.liquidation_threshold, lender.liquidation_penalty, data.interest_rate, data.loan_time);

    const token_promise = getToken(data.collateral_token);

    const [amount_to_liquidate, token] = await Promise.all([amount_to_liquidate_promise, token_promise])

    let liquidation_price = lender.liquidation_threshold * data.amount_lent / data.collateral_amount

    return {
        collateral_token: token,
        collateral_amount: data.collateral_amount,
        amount_lent: data.amount_lent,
        loan_date: data.loan_time,
        liquidation_price: liquidation_price,
        loan_to_value: data.loan_to_value,
        interest_rate: data.interest_rate,
        amount_to_liquidate: amount_to_liquidate,
        id: id
    };
}

interface PromiseFulfilledResult {
    status: "fulfilled";
    value: any;
}

interface PromiseRejectedResult {
    status: "rejected";
    reason: any;
}

type PromiseSettledResult = PromiseFulfilledResult | PromiseRejectedResult;

async function getAllLoansInformation(loan_ids: string[], lenders: Map<string, lender>) {
    const loans = await Promise.allSettled(loan_ids.map(async id => {
        const hex = await getHex(id)
        return getLoanInformation(hex.mutable_hex, hex.immutable_hex, lenders, hex.id)
    }));
    return loans.filter(x => x.status == "fulfilled").map(x => (x as PromiseFulfilledResult).value);
}

async function getAllCollection(): Promise<string[]> {
    try {
        let cursor: string | null | undefined = '';
        const ids: string[] = [];

        while (cursor !== undefined) {
            const obj: NonFungibleIdsRequest = {
                "address": loan_address,
                "cursor": cursor
            };
            await fetch(radix_api_url + '/non-fungible/ids', {
                method: 'POST',
                body: JSON.stringify(obj),
                headers: { 'Content-Type': 'application/json; charset=UTF-8' },
            })
                .then((response) => response.json())
                .then((data) => {
                    const response = data as NonFungibleIdsResponse;
                    response.non_fungible_ids.items.forEach(item => {
                        ids.push(item.non_fungible_id)
                    });
                    cursor = response.non_fungible_ids.next_cursor;
                })
                .catch(console.error);
        }

        return ids;
    } catch {
        throw new Error("error");
    }
}

export { getLendersList, getLenderInformation, getLoansOwnedBy, getAllLoansInformation, getAllCollection }
