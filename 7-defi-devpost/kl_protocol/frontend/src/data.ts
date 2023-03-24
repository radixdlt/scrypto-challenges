


// export const TIME_FACTOR = 32
export const INTEREST_RATE_TYPE = 1
export const XRD = 'resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp'
export const DAPP_ID = ''
export const PACKAGE_ADDRESS = ''

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

export const default_asset_list: PoolInfo[] = [
    // {
    //     resource_address: XRD,
    //     symbol: 'XRD',
    //     name: 'Radix',
    //     icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/11948.png',
    //     initial_price: 250,
    //     liquidation_threshold: 0.7,
    //     liquidation_closing_factor: 0.5,
    //     liquidation_spread: 0.05,
    //     flashloan_fee_rate: 0.05
    // },
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