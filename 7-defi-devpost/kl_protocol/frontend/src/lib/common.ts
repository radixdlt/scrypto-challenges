import { openModal, closeModal } from "svelte-modals";
import { rdt, update_dapp_state } from "./api/rdt";


import Modal1 from '$lib/modals/Modal1.svelte';
import NumberInputModal from '$lib/modals/NumberInputModal.svelte';
import type { ManifestBuilder } from "@radixdlt/radix-dapp-toolkit";


export async function send_transaction(txManifest: ManifestBuilder) {//, callback: Function = async () => undefined) {
    openModal(Modal1, { title: '' });

    let transactionManifest = txManifest.build().toString()

    console.log(transactionManifest)

    let result = await rdt.sendTransaction({
        transactionManifest,
        version: 1
    });

    console.log(result)

    if (result.isErr()) {

        closeModal();


        throw result.error
        ///
    } else {

    }

    closeModal();

    window.location.reload();

    update_dapp_state()

    // return result
}


export async function get_number_input(
    { title, min, max, onSubmit }
        : { title: string; min: number; max: number; onSubmit: (input: number) => any; }
) {
    openModal(NumberInputModal, {
        title,
        min, max,
        onclick: onSubmit,
    });
}