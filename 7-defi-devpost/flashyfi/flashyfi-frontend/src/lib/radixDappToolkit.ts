import {RadixDappToolkit} from "@radixdlt/radix-dapp-toolkit";
import accountManager from "$lib/stores/accountManager"


let rdt = RadixDappToolkit(
    {
        dAppDefinitionAddress:
            'account_tdx_b_1pq0vz7gjtqwyk0snchnwvygl6px7f2p993zgn353efgqqgmx2k',
        dAppName: 'FlashyFi',
    },
    (requestData) => {
        requestData({
            accounts: {quantifier: 'atLeast', quantity: 1},
        }).map(({data: {accounts}}) => {
            accountManager.setConnectedAccounts(accounts)
        })
    },
    {
        networkId: 11,
        onDisconnect: () => {
            accountManager.setConnectedAccounts([])
        },
        onInit: ({accounts}) => {
            accountManager.setConnectedAccounts(accounts ?? [])
        },
    }
)

export default rdt