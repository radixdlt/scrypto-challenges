import { dao, proposal, proposedChange, token, voterCard } from "../../types";
import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest,
    NonFungibleDataRequest, NonFungibleDataResponse
} from "@radixdlt/babylon-gateway-api-sdk";
import { backend_api_url, dao_address, loan_address, radix_api_url, stablecoin_address, voter_card_address } from "../general/constants";
import { getToken } from "../general/generalApiCalls";

async function getDao(): Promise<dao> {
    const obj: EntityDetailsRequest = {
        "address": dao_address
    };

    let data;
    await fetch(radix_api_url + `/entity/details`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    if (!data) return { total_voting_power: 0, vote_period: 0, vote_validity_threshold: 0, proposals: [], reserves: new Map<string, number>() }

    // @ts-ignore
    const proposals_list: any[] = data[10];

    // @ts-ignore
    const locked_stablecoins = parseFloat(data[12]);

    // @ts-ignore
    const total_voting_power = parseFloat(data[14]);

    // @ts-ignore
    const vote_period = parseFloat(data[15]);

    // @ts-ignore
    const vote_validity_threshold = parseFloat(data[16]);

    const proposals = await getProposalsData(proposals_list);

    const reserves = await getReserves(locked_stablecoins);

    return { total_voting_power: total_voting_power, vote_period: vote_period, vote_validity_threshold: vote_validity_threshold, proposals: proposals, reserves: reserves };
}


async function getProposalsData(proposals_list: any[]): Promise<proposal[]> {

    return Promise.all(
        proposals_list.map(async (proposal: string[]) => {
            return getProposalData(proposal);
        })
    );

}

async function getProposalData(proposal: string[]): Promise<proposal> {

    const obj: EntityDetailsRequest = {
        "address": proposal[1]
    };

    let data;
    await fetch(radix_api_url + `/entity/details`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);


    // @ts-ignore
    let vote_end = parseFloat(data[2]);

    // @ts-ignore
    let votes_for = parseFloat(data[3]);

    // @ts-ignore
    let votes_against = parseFloat(data[4]);

    // @ts-ignore
    let votes_threshold = parseFloat(data[5]);


    return { vote_end: vote_end, votes_for: votes_for, votes_against: votes_against, votes_threshold: votes_threshold, proposed_change_type: proposedChange.ChangeVotePeriod, proposed_change_data: [4] };
}

async function getReserves(locked_stablecoins: number): Promise<Map<string, number>> {

    const obj: EntityDetailsRequest = {
        "address": dao_address
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


    // @ts-ignore
    const fungibles = data.fungible_resources.items;

    let reserves = new Map<string, number>();
    fungibles.map(async (fungible: any[]) => {
        const address = fungible["address"];
        let amount = parseFloat(fungible["amount"]["value"]);
        if (address === stablecoin_address) {
            amount = amount - locked_stablecoins;
        }
        reserves[address] = amount;
    });

    return reserves;
}


async function getVoterCard(account: string): Promise<voterCard> {

    const id_obj: EntityNonFungibleIdsRequest = {
        "address": account,
        "resource_address": voter_card_address
    };

    let id_data;
    await fetch(radix_api_url + `/entity/non-fungible/ids`, {
        method: 'POST',
        body: JSON.stringify(id_obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    })
        .then((response) => response.json())
        .then((tmp_data) => id_data = tmp_data["non_fungible_ids"]["items"])
        .catch(console.error);

    // @ts-ignore
    if (!id_data || !id_data.length) return { voting_power: 0, stablecoins_locked: 0, positions_ids_locked: [], proposals_voted: [] };

    const voter_card_id = id_data[0]["non_fungible_id"];

    const nfr_obj: NonFungibleDataRequest = {
        "address": voter_card_address,
        "non_fungible_id": voter_card_id
    };

    const response = await fetch(radix_api_url + '/non-fungible/data', {
        method: 'POST',
        body: JSON.stringify(nfr_obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8' }),
    })

    const tmp_data = await response.json();
    const responseData = tmp_data as NonFungibleDataResponse

    const mutable_hex = responseData.mutable_data_hex;
    const immutable_hex = responseData.immutable_data_hex;

    return await voterCardDataFromHex(mutable_hex, immutable_hex);
}

async function voterCardDataFromHex(mutable_hex: string, immutable_hex: string): Promise<voterCard> {

    const params = new URLSearchParams();
    params.append('mutable_data_hex', mutable_hex);
    params.append('immutable_data_hex', immutable_hex);

    const request = new Request(`${backend_api_url}/voter_cards?${params}`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8', })
    });

    const res = await (await fetch(request)).json();
    return { voting_power: res.votingPower, stablecoins_locked: res.stablecoinsLocked, positions_ids_locked: res.positionsLockedIds, proposals_voted: res.proposalsVoted };
}

export { getDao, getVoterCard }