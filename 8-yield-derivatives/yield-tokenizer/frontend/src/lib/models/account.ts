import { YT } from "$lib/addresses";
import Decimal from "decimal.js";
import { BaseModel } from "./base";


export declare type YieldTokenData = {
    $yieldTokenId: string,
    underlyingLsuResource: string,
    underlyingAmount: Decimal,
    redemptionValueAtStart: Decimal
}

export class UserAccount extends BaseModel {

    $account_address: string;

    $fungible_resources: Record<string, Decimal> = {};
    $fungible_vaults: Record<string, Decimal> = {};
    $non_fungible_resources: Record<string, number> = {};


    yt_ids!: string[];
    yt_list: Record<string, YieldTokenData> = {};


    constructor(account_address: string) {
        super();
        this.$account_address = account_address;
    }

    async load_user_resources() {

        if (!this.$account_address) return;

        let account_state = await this.stateFetcher.fetchEntityState(this.$account_address) as Record<string, any>;

        this.$fungible_resources = account_state['$fungible_resources']

        this.$fungible_vaults = account_state['$fungible_vaults']
        this.$non_fungible_resources = account_state['$non_fungible_resources']

        await this.load_yt_list();
    }

    async load_yt_list() {

        // let yt_res_address = YT

        // Get yt vaults
        const response2 = await this.stateApi.entityNonFungibleResourceVaultPage({
            stateEntityNonFungibleResourceVaultsPageRequest: {
                address: this.$account_address,
                resource_address: YT
            }
        });

        // get yt ids from each vault
        const tasks = response2?.items.map(async (_item) => {
            const response3 = await this.stateApi.entityNonFungibleIdsPage({
                stateEntityNonFungibleIdsPageRequest: {
                    address: this.$account_address,
                    resource_address: YT,
                    vault_address: _item.vault_address
                }
            });

            const ids = response3?.items.map((_item) => _item);

            return ids;
        });

        const result = await Promise.all(tasks);

        this.yt_ids = result.flat();

        await this.load_yt_data()
    }


    async load_yt_data() {

        let yt_count = this.yt_ids.length;

        if (yt_count === 0) return [];

        const fetched_yt_list: YieldTokenData[] = []
        // const loaded_ids = new Set<string>();

        while (yt_count > 0) {

            // proceed by batch of 99
            let nb = Math.min(99, yt_count);
            yt_count -= nb;
            const ids_to_load = this.yt_ids.slice(yt_count, yt_count + nb);

            // Load yt data
            let result = await this.stateApi.nonFungibleData({
                stateNonFungibleDataRequest: {
                    resource_address: YT,
                    non_fungible_ids: ids_to_load
                }
            })
            let raw_yt_data = result?.non_fungible_ids;
            if (!raw_yt_data) return []

            // Process yt data to make them usable
            let tasks = raw_yt_data.map(async (raw_yt_data: any) => {
                let fields = (raw_yt_data.data?.programmatic_json as any).fields as any[]
                return (this.stateFetcher.fetchElementFields(fields) as Promise<YieldTokenData>)
                    .then((yt) => ({ ...yt, $yieldTokenId: raw_yt_data.non_fungible_id }))
            })
            const new_yt_data = await Promise.all(tasks);

            new_yt_data.forEach((yt) => {
                fetched_yt_list.push(yt)
            })

        }

        this.yt_list = fetched_yt_list.reduce((acc, yt) => {
            acc[yt.$yieldTokenId] = yt;
            return acc;
        }, {} as Record<string, YieldTokenData>);
    }


}