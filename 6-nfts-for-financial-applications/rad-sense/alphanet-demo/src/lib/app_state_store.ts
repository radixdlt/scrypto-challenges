import {get, writable} from "svelte/store";
import type {DaoSystemAddresses, RadSenseAddresses} from "$lib/model";
import {AppState} from "$lib/model";
import {page} from '$app/stores';

async function putState(stateEndpointUrl:string, appState: AppState): Promise<void> {
    get(page)
    await fetch(stateEndpointUrl, {
        method: "PUT",
        body: JSON.stringify(appState)
    });
}

async function createAppState() {
    const {subscribe, set, update} = writable(new AppState());

    subscribe(async (state: AppState) => {
        console.log("Page: " +  get(page).url);
        let pageUrl = new URL(get(page).url);
        const stateEndpointUrl = pageUrl.protocol+"//"+pageUrl.host+"/appState";
        await putState(stateEndpointUrl, state);
    })

    return {
        subscribe,
        onAccountConnected: (accountAddress: string) => update(state => {
            state.accountAddress = accountAddress;
            return state;
        }),
        onRadSenseComponentInstantiated: (radSenseComponent: string, rsa: RadSenseAddresses, arbitrationDaoAddresses: DaoSystemAddresses) => update(state => {
            state.radSenseComponent = radSenseComponent;
            state.rsa = rsa;
            state.arbitrationDaoAddresses = arbitrationDaoAddresses;
            return state;
        }),
        onAdBrokerRegistered: (adBrokerId: string) => update(state => {
            state.adBrokerId = adBrokerId;
            return state;
        }),
        onAdvertiserRegistered: (advertiserId: string) => update(state => {
            state.advertiserId = advertiserId;
            return state;
        }),
        onAdSlotProviderRegistered: (adSlotProviderId: string) => update(state => {
            state.adSlotProviderId = adSlotProviderId;
            return state;
        }),
        onAdRegistered: (adId: string) => update(state => {
            state.adIds.push(adId);
            return state;
        }),
        onAdSlotRegistered: (adSlotId: string) => update(state => {
            state.adSlotIds.push(adSlotId);
            return state;
        }),
        reset: () => set(new AppState())
    };
}

export const appState = await createAppState();