import {writable} from "svelte/store";
import {StateApi} from "@radixdlt/alphanet-gateway-api-v0-sdk";

const xrdToken = "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9";
const stateApi = new StateApi();

async function getBalances(accountAddress: string): Promise<Balances> {
    const accountState = await stateApi.stateComponentPost({
        v0StateComponentRequest: {component_address: accountAddress}
    });
    const balances = new Map<string, number>();
    for (const vault of accountState.owned_vaults) {
        // @ts-ignore
        balances.set(vault.resource_amount.resource_address, vault.resource_amount.amount_attos / Math.pow(10, 18));
    }
    return new Balances(balances);
}

class Balances {
    balances: Map<string, number>;

    constructor(balances: Map<string, number>) {
        this.balances = balances;
    }

    getXrdBalance(): number {
        return this.balances.get(xrdToken)!;
    }
}

function createBalance() {
    const {subscribe, set} = writable(new Balances(new Map()));
    let accountAddr: string | null = null;

    return {
        subscribe,
        onAccountConnected: async (accountAddress: string) => {
            console.log("Conencted: " + JSON.stringify(accountAddress))
            accountAddr = accountAddress
            set(await getBalances(accountAddress))
        },
        onBalanceChanged: async () => {
            set(await getBalances(accountAddr!))
        },
    };
}


const balance = createBalance();
export default balance;