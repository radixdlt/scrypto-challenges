import { rdt } from "$lib";
import { refresh_account_store, refresh_tokenizer_store } from "$lib/stores";


export async function sendTransaction(transactionManifest: string, reload = true) {
    try {


        // awaiting_tx_store.set(true);

        const result = await rdt.walletApi.sendTransaction({
            transactionManifest
        });

        if (result.isErr()) {
            // awaiting_tx_store.set(false);

            throw result.error;
            ///
        }

        // closeModal();

        if (reload) {
            Promise.all([refresh_account_store(), refresh_tokenizer_store()]);
        }

        return result;
    } catch (e) {
        console.log(e);
    }
}