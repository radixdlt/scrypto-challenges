// place files you want to import through the `$lib` alias in this folder.
import EntityStateFetcher from '$lib//utils/state_fetcher';
import { dapp_state, refresh_account_store, refresh_tokenizer_store } from '$lib/stores';
import {
    DataRequestBuilder,
    RadixDappToolkit,
    type RadixDappToolkitOptions,
    type WalletDataState
} from '@radixdlt/radix-dapp-toolkit';
import { get } from 'svelte/store';

const options: RadixDappToolkitOptions = {
    dAppDefinitionAddress: 'account_tdx_2_128zgmqg2lclwdnm5pt4kwl9uv0j8m2cwnhpxk9ddfzjm69cadeujr7',
    networkId: 2,
    useCache: true
};

export const rdt = RadixDappToolkit(options);
export const state_fetcher = new EntityStateFetcher(rdt.gatewayApi.state.innerClient);

rdt.walletApi.setRequestData(DataRequestBuilder.accounts().exactly(1));
rdt.walletApi.walletData$.subscribe(async (data: WalletDataState) => {

    console.log("RDT initialized");

    let current_account = (data?.accounts ?? [])[0]?.address ?? '';

    dapp_state.set({
        ...get(dapp_state),
        accountAddress: current_account,
    });

    if (current_account !== '') {
        await refresh_account_store();
    }

    await refresh_tokenizer_store();
});
