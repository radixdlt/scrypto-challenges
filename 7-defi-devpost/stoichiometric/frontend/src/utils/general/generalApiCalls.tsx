import { radix_api_url, router_address } from "./constants";
import { EntityDetailsRequest, EntityDetailsResponse } from "@radixdlt/babylon-gateway-api-sdk";
import { EntityDetailsResponseComponentDetails, lender, pool, token } from "types";
import { getPoolInformation } from "utils/dex/routerApiCalls";
import { getLenderInformation, getLendersList } from "utils/stablecoin/issuerApiCalls";



async function getToken(address: string): Promise<token> {

    const obj: EntityDetailsRequest = {
        "address": address
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
    const metadata = data["metadata"]["items"];
    return { name: metadata[2]["value"], symb: metadata[3]["value"], address: address, icon_url: metadata[1]["value"] };
}

async function getOwnedTokens(account: string) {

    let ownedTokensList: any[] = [];

    const obj: EntityDetailsRequest = {
        "address": account
    };

    let data;
    await fetch(radix_api_url + `/entity/resources`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);

    if (!data) return undefined;

    // @ts-ignore
    const fungible = data.fungible_resources.items;

    for (var i = 0; i < fungible.length; ++i) {
        ownedTokensList[fungible[i]["address"]] = parseFloat(fungible[i]["amount"]["value"])
    }

    return [ownedTokensList, account];
}

async function getRawPoolsList() {


    const obj: EntityDetailsRequest = {
        "address": router_address
    };

    const response = await fetch(radix_api_url + `/entity/details`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })

    let data = await response.json() as EntityDetailsResponse

    if (data.details?.discriminator == 'component') {
        const component_details = data.details as EntityDetailsResponseComponentDetails
        return component_details.state.data_json[1].map((row: any) => {
            return { token: row[0], pool_address: row[1] }
        });
    }
}

async function getLendersInfos() {
    let raw_lender_list = await getLendersList();

    return Promise.all(raw_lender_list.map(async (raw_lender: { lender: string, token: string }) => {
        return { token: raw_lender.token, lender: await getLenderInformation(raw_lender.lender) }
    }));
}

async function getTokensAndPools() {
    let raw_list = await getRawPoolsList();

    const tokens: token[] = await Promise.all(raw_list.map(async (raw_pool: { token: string, pool_address: string; }) => {
        return getToken(raw_pool.token);
    }));

    const pools: pool[] = await Promise.all(raw_list.map(async (raw_pool: { token: string, pool_address: string; }, index: number) => {
        return getPoolInformation(tokens[index], raw_pool.pool_address);
    }));


    return { tokens, pools }

}
async function getTokensPoolsAndLenders() {

    const [lenders, { tokens, pools }] = await Promise.all([getLendersInfos(), getTokensAndPools()]);

    return { tokens, pools, lenders };
}

export { getToken, getOwnedTokens, getTokensPoolsAndLenders }