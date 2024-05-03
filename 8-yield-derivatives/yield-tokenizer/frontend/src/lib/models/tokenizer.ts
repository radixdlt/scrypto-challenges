import { VALIDATOR } from "$lib/addresses";
import Decimal from "decimal.js";
import { BaseModel } from "./base";
import { rdt } from "$lib";


export declare type YieldTokenData = {
    $yieldTokenId: string,
    underlyingLsuResource: string,
    underlyingAmount: Decimal,
    redemptionValueAtStart: Decimal
}

export class TokenizerComponent extends BaseModel {

    $component_address: string;

    $fungible_resources: Record<string, Decimal> = {};
    $fungible_vaults: Record<string, Decimal> = {};
    $non_fungible_resources: Record<string, number> = {};

    constructor(account_address: string) {
        super();
        this.$component_address = account_address;
    }

    async load() {

        if (!this.$component_address) return;

        let account_state = await this.stateFetcher.fetchEntityState(this.$component_address) as Record<string, any>;

        this.$fungible_resources = account_state['$fungible_resources']

        this.$fungible_vaults = account_state['$fungible_vaults']
        this.$non_fungible_resources = account_state['$non_fungible_resources']

        // let validator_state = await this.stateFetcher.fetchEntityState(VALIDATOR) as Record<string, any>;

        // let ids = await rdt.gatewayApi.state.getAllNonFungibleIds(this.$component_address);

        // let data = await rdt.gatewayApi.state.getNonFungibleData(this.$component_address, ids);

        // console.log(data)


    }

}