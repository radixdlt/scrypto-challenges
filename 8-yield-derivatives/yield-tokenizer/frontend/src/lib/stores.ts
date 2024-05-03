import { UserAccount } from "$lib/models/account";
import { persisted } from 'svelte-local-storage-store';
import { get, writable } from "svelte/store";
import { TOKENIZER } from './addresses';
import { TokenizerComponent } from './models/tokenizer';

// Dapp store (persisted in local storage)

export type DappState = {
	accountAddress?: string;
};

const state: DappState = {
	accountAddress: ''
};

export const dapp_state = persisted('dapp_state_prod', state);

export function reset_dapp_store() {
	dapp_state.update(() => state);
}

// Tokenizer Component store

export const tokenizer_store = writable<TokenizerComponent | undefined>();

export async function refresh_tokenizer_store() {

	let tokenizer = new TokenizerComponent(TOKENIZER);

	await tokenizer.load().then(() => {
		console.log("Tokenizer loaded", tokenizer);
		tokenizer_store.set(tokenizer);
	})
}



// Account store

export const account_store = writable<UserAccount | undefined>();

export async function refresh_account_store() {

	let current_account = get(dapp_state).accountAddress;

	if (!current_account) {
		account_store.set(undefined)
		return;
	}

	let account = new UserAccount(current_account);

	await account.load_user_resources().then(() => {
		console.log("Account loaded", account);
		account_store.set(account);
	})

}

// Awaiting for tx completion

export const awaiting_tx_store = writable<boolean>(false);

