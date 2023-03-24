import { loan_user_resources } from '$lib/state/account';
import { dapp_state } from '$lib/state/dapp';
import { load_manager_pool_state, } from '$lib/state/lending_pool_manager';
import {
    StateApi,
    StatusApi,
    StreamApi,
    TransactionApi
} from '@radixdlt/babylon-gateway-api-sdk';
import { RadixDappToolkit } from "@radixdlt/radix-dapp-toolkit";
import { get } from 'svelte/store';


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
            dapp_state.set({ ...get(dapp_state), accountAddress: (accounts.length >= 1 ? accounts[0].address : '') })

            update_dapp_state(0)
        });


    },
    {
        networkId: 11,
        onDisconnect: () => {
            dapp_state.set({ ...get(dapp_state), accountAddress: '' })
        },
        onInit: async (data) => {

            console.log(data)

            let accounts = data.accounts

            if (accounts == undefined || accounts.length === 0) {
                dapp_state.set({ ...get(dapp_state), accountAddress: '' })
            } else {
                dapp_state.set({ ...get(dapp_state), accountAddress: (accounts.length >= 1 ? accounts[0].address : '') })
            }

            update_dapp_state(0)
        }
    }
);


export function update_dapp_state(waiting_time = 1000) {
    setTimeout(async () => {
        await load_manager_pool_state()
        await loan_user_resources()
    }, waiting_time);
}
