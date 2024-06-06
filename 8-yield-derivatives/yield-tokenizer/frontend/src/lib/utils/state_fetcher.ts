import type { StateApi } from "@radixdlt/radix-dapp-toolkit";
import Decimal from "decimal.js";

Decimal.set({ precision: 50, rounding: Decimal.ROUND_HALF_EVEN })
export const PreciseDecimal = Decimal.clone({ precision: 50, rounding: Decimal.ROUND_HALF_EVEN })

export declare type FetchOptions = {
    useDecimals: boolean
}

export declare type FetchEnumPlugin = {
    enumName: string,
    parser: (field: any, fetcher: EntityStateFetcher) => Promise<any>
}

const defaultOptions: FetchOptions = { useDecimals: false }

class EntityStateFetcher {
    private stateApi: StateApi;
    private options: FetchOptions;
    private enumPlugins: FetchEnumPlugin[] = [];

    constructor(stateApi: StateApi, options: FetchOptions = defaultOptions, enumPlugins: FetchEnumPlugin[] = []) {
        this.stateApi = stateApi;
        this.options = options;
        this.enumPlugins = enumPlugins;

        this.addEnumPlugin(optionEnumPlugin);
    }

    addEnumPlugin(plugin: FetchEnumPlugin) {
        this.enumPlugins.push(plugin);
    }

    setOptions(options: FetchOptions) {
        this.options = options;
    }

    async fetchEntityState(entityAddress: string): Promise<Record<string, any> | undefined> {
        let result = await this.stateApi.stateEntityDetails({
            stateEntityDetailsRequest: {
                addresses: [entityAddress],
                aggregation_level: 'Vault'
            }
        });

        if (result.items.length === 0) return;

        let entity = result.items[0];

        if (entity === undefined) return undefined;

        let resources: Record<string, number | Decimal> = {};
        let nfts: Record<string, number | Decimal> = {};
        let vaults: Record<string, number | Decimal> = {};

        entity.fungible_resources?.items.forEach((item) => {
            if (resources[item.resource_address] === undefined) resources[item.resource_address] = this.options.useDecimals ? this.dec(0) : 0;

            const _vaults = (item as any)['vaults']['items'] as any[];

            _vaults.forEach((vault: any) => {

                if (this.options.useDecimals) {
                    resources[item.resource_address] = (resources[item.resource_address] as Decimal).plus(this.dec(vault['amount']))
                    vaults[vault['vault_address']] = this.dec(vault['amount'])
                } else {
                    (resources[item.resource_address] as number) += parseFloat(vault['amount'])
                    vaults[vault['vault_address']] = parseFloat(vault['amount'])
                }

            })
        });

        entity.non_fungible_resources?.items.forEach((item) => {
            if (nfts[item.resource_address] === undefined) nfts[item.resource_address] = 0;

            const _vaults = (item as any)['vaults']['items'] as any[];

            _vaults.forEach((vault: any) => {
                (nfts[item.resource_address] as number) += parseInt(vault['total_count'])
            })
        });

        let rawState = (entity.details as any)?.state;

        let values: Record<string, any> = {};

        if (rawState !== undefined && rawState.fields !== undefined) {
            let fields = rawState.fields as any[];
            values = await this.fetchElementFields(fields);
        }

        let nftKeys = Object.keys(nfts);
        nftKeys.forEach((key) => {
            if (nfts[key] === 0) delete nfts[key];
        });

        values['$fungible_resources'] = resources;
        values['$fungible_vaults'] = vaults;
        values['$non_fungible_resources'] = nfts;

        return values;
    }

    async fetchInternalState(entityAddress: string): Promise<Record<string, any> | undefined> {
        let result = await this.stateApi.stateEntityDetails({
            stateEntityDetailsRequest: {
                addresses: [entityAddress],
                aggregation_level: 'Vault'
            }
        });

        if (result.items.length === 0) return;
        let details = result.items[0]?.details as any
        let rawState = details.state;

        if (!rawState) return undefined;

        let fields = rawState.fields as any[];

        let values = await this.fetchElementFields(fields);

        return values;
    }

    dec(input: any): Decimal {
        return new Decimal(input);
    }

    pdec(input: any): Decimal {
        return new PreciseDecimal(input);
    }

    async fetchField(field: any): Promise<any> {
        let value: any = undefined;

        switch (field.kind) {
            case 'String':
            case 'NonFungibleLocalId':
                value = field.value;
                break;

            case 'Decimal':
                value = this.options.useDecimals ? this.dec(field.value) : parseFloat(field.value);
                break;

            case 'PreciseDecimal':
                value = this.options.useDecimals ? this.pdec(field.value) : parseFloat(field.value);
                break;

            case 'Map':

                let temp_map: Record<string, any> = {};

                let map_tasks = field.entries.map(async (entry: any) => {
                    temp_map[entry.key.value] = await this.fetchField(entry.value);
                });

                await Promise.all(map_tasks);

                value = temp_map;
                break;

            case 'Array':

                let temp_array: Record<string, any> = [];

                let array_tasks = field.elements.map(async (entry: any) => {
                    temp_array.push(await this.fetchField(entry));
                });

                await Promise.all(array_tasks);

                value = temp_array;
                break;



            case 'Reference':
                value = field.value;
                break;

            case 'U8':
            case 'U64':
            case 'I64':
                value = parseInt(field.value);
                break;

            case 'Enum':

                const plugin = this.enumPlugins.reverse().find((plugin) => plugin.enumName === field.type_name)
                if (plugin) {
                    value = await plugin.parser(field, this);
                }

                break;

            case 'Tuple':
                value = await this.fetchElementFields(field.fields);
                break;

            case 'Own':
                if (field.type_name == 'KeyValueStore' || field.type_name == 'Vault') {
                    value = field.value;
                } else {
                    value = await this.fetchInternalState(field.value);
                }

                break;

            default:


                value = field
                break;
        }

        return value;
    }

    async fetchElementFields(fields: any[]): Promise<Record<string, any> | any[]> {
        if (fields.length == 0) return {};

        if (fields.length === 1) {
            if (fields[0]?.field_name === undefined) {
                return await this.fetchField(fields[0]);
            } else {
                return { [fields[0]?.field_name]: await this.fetchField(fields[0]) };
            }
        }

        const isArray = fields[0]?.field_name === undefined;

        if (isArray) {
            fields = fields[0].elements as any[];
        }

        if (!isArray) {
            let values: Record<string, any> = {};
            let tasks = fields.map(async (field) => {
                values[field.field_name as string] = await this.fetchField(field);
            });

            await Promise.all(tasks);

            return values;
        } else {
            let values: any[] = [];

            let tasks = fields.map(async (field) => {
                let value = await this.fetchField(field);
                values.push(value);
            });

            await Promise.all(tasks);

            return values;
        }
    }

}

export default EntityStateFetcher;

export const optionEnumPlugin: FetchEnumPlugin = {
    enumName: 'Option',
    parser: async (field, fetcher) => {
        if (field.variant_id === 0 || field.variant_name === 'None') {
            return undefined;
        }

        if (field.fields.length === 1 && field.fields[0].field_name === undefined) {
            return await fetcher.fetchField(field.fields[0]);
        }

        return (field.variant_id === '0' || field.variant_name === 'None') ? undefined : await fetcher.fetchElementFields(field.fields);

    }
}

