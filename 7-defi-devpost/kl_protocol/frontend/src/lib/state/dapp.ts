import { persisted } from "svelte-local-storage-store";
import { DAPP_ID, PACKAGE_ADDRESS } from "../../data";

export type DappState = {
    accountAddress: string;
    dAppId: string;
    packageAddress: string;
    faucetComponentAddress: string;
    faucetAdminBadgeAddress: string;
    faucetCreationTxHash: string;
    lendingMarketCreationTxHash: string;
    lendingMarketComponentAddress: string;
    lendingMarketAdminBadgeAddress: string;
}



let _price_chages: Record<string, number> = {}

let _dap_state: DappState = {
    dAppId: DAPP_ID,
    packageAddress: PACKAGE_ADDRESS,
    accountAddress: '',

    faucetComponentAddress: '',
    faucetAdminBadgeAddress: '',
    faucetCreationTxHash: '',

    lendingMarketCreationTxHash: '',
    lendingMarketComponentAddress: '',
    lendingMarketAdminBadgeAddress: '',
}

export const price_changes = persisted('price_changes', _price_chages)

export const dapp_state = persisted('persited_data', _dap_state)


export function reset_update() {
    price_changes.update(_ => _price_chages)
    dapp_state.update(_ => _dap_state)

}