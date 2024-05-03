import { CLAIM_NFT } from "$lib/addresses";
import Decimal from "decimal.js";
import { BaseModel } from "./base";


export declare type ClaimNFT = {
    name: string,
    claim_amount: string,
    claim_epoch: number,
}

export class TokenizerComponent extends BaseModel {

    $component_address: string;

    $fungible_resources: Record<string, Decimal> = {};
    $fungible_vaults: Record<string, Decimal> = {};
    $non_fungible_resources: Record<string, number> = {};

    claim_nft_ids: string[] = []
    claim_nft_list: Record<string, ClaimNFT> = {};

    constructor(account_address: string) {
        super();
        this.$component_address = account_address;
    }

    total_claim_amount() {

        let total = this.stateFetcher.dec(0);

        console.log(total)

        this.claim_nft_ids.forEach((claim_nft_id) => {
            let claim_nft = this.claim_nft_list[claim_nft_id]
            console.log(claim_nft.claim_amount.toString())
            total = total.add(claim_nft.claim_amount)
        })

        console.log(total.toString())


        return total

    }

    async load() {

        if (!this.$component_address) return;

        let account_state = await this.stateFetcher.fetchEntityState(this.$component_address) as Record<string, any>;

        this.$fungible_resources = account_state['$fungible_resources']

        this.$fungible_vaults = account_state['$fungible_vaults']
        this.$non_fungible_resources = account_state['$non_fungible_resources']

        this.load_claim_nft_list()



    }

    async load_claim_nft_list() {

        // let yt_res_address = YT

        // Get yt vaults
        const response2 = await this.stateApi.entityNonFungibleResourceVaultPage({
            stateEntityNonFungibleResourceVaultsPageRequest: {
                address: this.$component_address,
                resource_address: CLAIM_NFT
            }
        });

        // get yt ids from each vault
        const tasks = response2?.items.map(async (_item) => {
            const response3 = await this.stateApi.entityNonFungibleIdsPage({
                stateEntityNonFungibleIdsPageRequest: {
                    address: this.$component_address,
                    resource_address: CLAIM_NFT,
                    vault_address: _item.vault_address
                }
            });

            const ids = response3?.items.map((_item) => _item);

            return ids;
        });

        const result = await Promise.all(tasks);

        this.claim_nft_ids = result.flat();

        await this.load_claim_nft_data()
    }


    async load_claim_nft_data() {

        let yt_count = this.claim_nft_ids.length;

        if (yt_count === 0) return [];

        const fetched_list: any[] = []
        // const loaded_ids = new Set<string>();

        while (yt_count > 0) {

            // proceed by batch of 99
            let nb = Math.min(99, yt_count);
            yt_count -= nb;
            const ids_to_load = this.claim_nft_ids.slice(yt_count, yt_count + nb);

            // Load yt data
            let result = await this.stateApi.nonFungibleData({
                stateNonFungibleDataRequest: {
                    resource_address: CLAIM_NFT,
                    non_fungible_ids: ids_to_load
                }
            })
            let raw_yt_data = result?.non_fungible_ids;
            if (!raw_yt_data) return []

            // Process yt data to make them usable
            let tasks = raw_yt_data.map(async (raw_yt_data: any) => {
                let fields = (raw_yt_data.data?.programmatic_json as any).fields as any[]
                return (this.stateFetcher.fetchElementFields(fields) as Promise<ClaimNFT>)
                    .then((yt) => ({ ...yt, $yieldTokenId: raw_yt_data.non_fungible_id }))
            })
            const new_yt_data = await Promise.all(tasks);

            new_yt_data.forEach((yt) => {
                fetched_list.push(yt)
            })

        }

        this.claim_nft_list = fetched_list.reduce((acc, yt) => {
            acc[yt.$yieldTokenId] = yt;
            return acc;
        }, {} as Record<string, ClaimNFT>);
    }



}