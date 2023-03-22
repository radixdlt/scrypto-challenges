import { persisted } from "svelte-local-storage-store";
import { XRD } from "./transactions/admin";

export type PoolInfo = {
    //
    resource_address?: string;

    //
    symbol: string;
    name: string;
    icon: string;
    initial_price: number;

    //
    flashloan_fee_rate: number;
    liquidation_threshold: number;
    liquidation_spread: number;
    liquidation_closing_factor: number;
}

let _price_chages: Record<string, number> = {}

export const price_changes = persisted('price_changes', _price_chages)

export const dapp_data = persisted('persited_data', {
    accountAddress: '',
    dAppId: 'account_tdx_b_1pqwzpeqv8mph3u80g5zch24gtpky3wy3demlg6ta6q4qhkdpd8',
    packageAddress: 'package_tdx_b_1qy8fdtykc2m0kuprfqy6cxk88r938rr4ty8zyxlcdfxq9srfny',

    faucetComponentAddress: '',
    faucetAdminBadgeAddress: '',
    faucetCreationTxHash: '',

    lendingMarketCreationTxHash: '',
    lendingMarketComponentAddress: '',
    lendingMarketAdminBadgeAddress: '',
})

export const default_asset_list: PoolInfo[] = [
    {
        resource_address: XRD,
        symbol: 'XRD',
        name: 'Radix',
        icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/11948.png',
        initial_price: 250,
        liquidation_threshold: 0.7,
        liquidation_closing_factor: 0.5,
        liquidation_spread: 0.05,
        flashloan_fee_rate: 0.05
    },
    {
        symbol: 'BTC',
        name: 'Bitcoin',
        icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/1.png',
        initial_price: 21000,
        liquidation_threshold: 0.7,
        liquidation_closing_factor: 0.5,
        liquidation_spread: 0.05,
        flashloan_fee_rate: 0.05
    },
    {
        symbol: 'ETH',
        name: 'Ethereum',
        icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/1027.png',
        initial_price: 1600,
        flashloan_fee_rate: 0.05,
        liquidation_threshold: 0.6,
        liquidation_closing_factor: 0.5,
        liquidation_spread: 0.05,

    },
    {
        symbol: 'USDC',
        name: 'Circle USD',
        initial_price: 1,
        icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/3408.png',
        flashloan_fee_rate: 0.05,
        liquidation_threshold: 0.4,
        liquidation_closing_factor: 0.5,
        liquidation_spread: 0.05,
    },
    {
        symbol: 'USDT',
        name: 'Tether USD',
        icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/825.png',
        initial_price: 1.01,
        liquidation_threshold: 0.0,
        flashloan_fee_rate: 0.05,
        liquidation_closing_factor: 0.5,
        liquidation_spread: 0.05,
    }
]