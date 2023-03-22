import type { ComponentAddressString, ResourceAddressString } from "@radixdlt/radix-dapp-toolkit"

export type FaucetState = {
    resource_prices: Record<ResourceAddressString, number>
}


export type ResourceMetadata = {
    address: string,
    price: number,
    symbol: string,
    name: string,
    icon: string,
}

export type PoolManagerState = {
    // lending_market_component_badge: Vault, //0

    cdp_resource_address: string, // 1

    // lending_pool_registry: ComponentAddressRegistery,//2

    component_address_lookup: Record<ResourceAddressString, ComponentAddressString>
    pool_share_resource_address_lookup: Record<ResourceAddressString, ResourceAddressString>
    resource_address_lookup: Record<ResourceAddressString, ResourceAddressString>

    exchange_component_address: string, //3

    debt_position_counter: number,//4
    collateral_position_counter: number,//5
    cdp_position_counter: number,//6




}

export type PoolState = {
    // $new_price: number | undefined;
    $component_address: ComponentAddressString;
    // pool_component_badge: Vault, // 0                    

    // liquidity: Vault, // 1                    

    // collaterals: Vault, // 2

    // collected_fees: Vault, // 3

    // insurance_reserve: Vault, // 4 

    pool_resource_address: ResourceAddressString, // 5

    pool_share_resource_address: ResourceAddressString, // 6

    flashloan_term_resource_address: ResourceAddressString, // 7

    interest_factory_address: ComponentAddressString, // 8

    oracle_address: ComponentAddressString, // 9

    loan_state_lookup: InterestTypeState[],//HashMap<u8, InterestTypeState>, // 10

    last_price: number, // 11

    last_price_update: number, // 12 

    flashloan_fee_rate: number, // 13

    liquidation_threshold: number, // 14

    liquidation_spread: number, // 15

    liquidation_close_factor: number, // 16
}

export type InterestTypeState = {
    interest_type: number,
    total_loan: number,
    total_loan_share: number,
    interest_rate: number,
    interest_updated_at: number,
}

