import { COLORS_SANS_BW} from './colors.ts';

import { gatewayNodeUrl, participantsComponentAddress, participantsNftResourceAddress } from './serviceconfig.ts';
import { editedPfpSeries, editedPfpId } from './participantstores.ts';
import { allParticipants, promiseParticipants, userNfid } from './login.ts';
import { get } from 'svelte/store';

export enum PfpSource {
    Scrops,
    Radderflies,
    CerberRads,
    DapperDachshunds,
}

export interface Participant {
    resourceAddress: string;
    nfid: string;
    userName: string;
    url: string;
    idRef: string;
    sponsor: string;
    expectSponsor: string;
    endorsing: Set<string>;

    pfpSeries: PfpSource;
    pfpId: string;
 }

const pfpSourceMap = new Map<string, PfpSource>;
pfpSourceMap.set("scorp", PfpSource.Scrops);
pfpSourceMap.set("scrop", PfpSource.Scrops);
pfpSourceMap.set("radfly", PfpSource.Radderflies);
pfpSourceMap.set("cerber", PfpSource.CerberRads);
pfpSourceMap.set("dachs", PfpSource.DapperDachshunds);

export function createIdrefForPfp(series: PfpSource, id: string): string {
    let seriesString: string = determinePfpSeries(series);
    if (seriesString === undefined) return "";
    return seriesString + " " + id;
}

export function colourFor(nfid: string) {
    return COLORS_SANS_BW[parseInt(nfid, 16) % COLORS_SANS_BW.length];
}

export function determinePfpSeries(seriesId: PfpSource): string {
    switch(seriesId) {
	case PfpSource.Scrops: return "scrop";
	case PfpSource.Radderflies: return "radfly";
	case PfpSource.CerberRads: return "cerber";
	case PfpSource.DapperDachshunds: return "dachs";
    }
    return undefined;
}

export function establishPfp(p: Participant) {
    if (!p.idRef) return;
    let matches = p.idRef.match(/^(.+) ([0-9]+)$/);
    if (!matches) return;

    let seriesId: string = matches[1];
    let pfpSerial: string = matches[2];

    if (pfpSourceMap.has(seriesId)) {
	p.pfpSeries = pfpSourceMap.get(seriesId);
	p.pfpId = pfpSerial;
    }
}

function padZeros(str: string, totlen: number): string {
    if (str.length >= totlen) return str;
    return "0".repeat(totlen-str.length) + str;
}

export function numberToNfid(n: number): string {
    // NonFungibleIds are 16 digits hex
    let nfid = n.toString(16);
    return padZeros(nfid, 16);
}

export function generatePfpUrl(p: Participant): string {
    if (p.pfpSeries === undefined) return undefined;
    switch(p.pfpSeries) {
	case PfpSource.Scrops: {
	    let id = padZeros(p.pfpId, 4);
	    return `https://radstrike.com/scorpions/img/${id}.png`;
	}
	case PfpSource.Radderflies: {
	    return `https://www.radderflies.com/data/images/radderflies/${p.pfpId}.png`;
	}
	case PfpSource.CerberRads: {
	    let id = padZeros(p.pfpId, 4);
	    return `https://cerberrads.com/tokens/${id}.png`;
	}
	case PfpSource.DapperDachshunds: {
	    return `https://dapper-dachshunds.com/images/dachshunds/${p.pfpId}.png`;
	}
    }
    return undefined;
}

function parseNonFungibleId(str: string): string {
    let matches = str.match(/NonFungibleId\("(.*)"\)/);
    return matches[1];
}

export async function loadParticipant(nfid: string): Participant {
    const gatewayNode: string = get(gatewayNodeUrl);
    const nftAddress: string = get(participantsNftResourceAddress);
    const nftResp = await fetch(`${gatewayNode}/non-fungible/${nftAddress}${nfid}`);
    const nftData = await nftResp.json();
    const mutableData = JSON.parse(nftData.mutable_data).fields;
    let participant: Participant = {};
    participant.resourceAddress = nftAddress;
    participant.nfid = nfid;
    participant.userName = mutableData[0].value;
    participant.url = mutableData[1].value;
    participant.idRef = mutableData[2].value;
    if (mutableData[3].value) {
      participant.sponsor = parseNonFungibleId(mutableData[3].value.value);
    }
    //     participant.expectSponsor = mutableData[4].value;
    if (mutableData[4].value) {
      participant.expectSponsor = parseNonFungibleId(mutableData[4].value.value);
    }
    participant.endorsing = new Set();
    for (let i = 0; i < mutableData[5].elements.length; ++i) {
      participant.endorsing.add(parseNonFungibleId(mutableData[5].elements[i].value));
    }
    establishPfp(participant);

    return participant;
}


export function getHighestParticipantNfid(): Promise {
    const gatewayNode: string = get(gatewayNodeUrl);
    const compAddress: string = get(participantsComponentAddress);
    let promise = new Promise(function(resolve, reject) {
	const fetchPromise = fetch(`${gatewayNode}/component/${compAddress}`);
	const jsonPromise = fetchPromise.then(result => result.json())
					.then(result => resolve(JSON.parse(result.state).fields[1].value))
					.catch(err => reject(err));
    });
    return promise;
}


export function equalParticipantData(a: Participant, b: Participant): bool {
    let aSet = a.endorsing;
    let bSet = b.endorsing;
    return a.resourceAddress === b.resourceAddress
	&& a.nfid === b.nfid
	&& a.userName === b.userName
	&& a.url === b.url
	&& a.idRef === b.idRef
	&& a.sponsor === b.sponsor
	&& a.expectSponsor === b.expectSponsor
	&& aSet.size === bSet.size
	&& [...aSet].every((member) => bSet.has(member));
}
