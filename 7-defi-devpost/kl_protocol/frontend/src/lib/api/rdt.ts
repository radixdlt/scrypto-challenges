import { load_faucet_state, load_manager_pool_state } from '$lib/state/pool_state';
import {
    StateApi,
    StatusApi,
    StreamApi,
    TransactionApi
} from '@radixdlt/babylon-gateway-api-sdk';
import { RadixDappToolkit } from "@radixdlt/radix-dapp-toolkit";
import { get } from 'svelte/store';
import { dapp_data } from '../data';


export const dAppId = 'account_tdx_b_1pqwzpeqv8mph3u80g5zch24gtpky3wy3demlg6ta6q4qhkdpd8'
export const package_address = 'package_tdx_b_1q99j3wtzz3tlqvxq85l8ql7u85z64eu6gzn9ndjvlass0k8jtr'

export const transactionApi = new TransactionApi();
export const stateApi = new StateApi();
export const statusApi = new StatusApi();
export const streamApi = new StreamApi();

// export const manifestBuilder = new ManifestBuilder();

export const rdt = RadixDappToolkit(
    { dAppDefinitionAddress: dAppId, dAppName: 'SuperLend' },
    (requestData) => {
        // onConnect
        requestData({
            accounts: { quantifier: 'exactly', quantity: 1 }
        }).map(({ data: { accounts } }) => {
            console.log(accounts)
            if (accounts == undefined) return
            dapp_data.set({ ...get(dapp_data), accountAddress: (accounts.length >= 1 ? accounts[0].address : '') })

        });
    },
    {
        networkId: 11,
        onDisconnect: () => {
            dapp_data.set({ ...get(dapp_data), accountAddress: '' })
        },
        onInit: async (data) => {

            console.log(data)

            let accounts = data.accounts

            if (accounts == undefined || accounts.length === 0) {
                dapp_data.set({ ...get(dapp_data), accountAddress: '' })
            } else {
                dapp_data.set({ ...get(dapp_data), accountAddress: (accounts.length >= 1 ? accounts[0].address : '') })

                let state = await stateApi.entityResources({ entityResourcesRequest: { address: accounts[0].address } })

                console.log(state)

                await load_faucet_state()
                await load_manager_pool_state()
            }

        }
    }
);
