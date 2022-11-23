import {StateApi} from '@radixdlt/alphanet-gateway-api-v0-sdk';
import {parseAddress} from "$lib/utils";


class AdSlotRepository {
    private stateApi = new StateApi();

    public async getAdSlot(adSlotResource: string, nonFungibleId: string): Promise<AdSlot> {
        const response = await this.stateApi.stateNonFungiblePost({
            v0StateNonFungibleRequest: {
                resource_address: adSlotResource,
                non_fungible_id_hex: nonFungibleId
            }
        });

        // @ts-ignore
        const data = response.non_fungible.non_fungible_data.immutable_data.struct_data.data_json;
        const width = data.fields[0].fields[0].value;
        const height = data.fields[0].fields[1].value;
        const adSlotProviderNonFungibleId = parseAddress(data.fields[2])
        const approvedBrokers = data.fields[3].elements.map(parseAddress)
        return new AdSlot(nonFungibleId, width, height, adSlotProviderNonFungibleId, approvedBrokers);
    }

    public async getResourceAddresses(adSlotResource: string): Promise<ResourceAddresses> {
        // TODO Doesn't work because of a bug in Alphanet
        const response = await this.stateApi.stateResourcePost({
            v0StateResourceRequest: {
                resource_address: adSlotResource
            }
        });
        // @ts-ignore
        const metadata = response.manager.metadata.reduce(function (map, entry) {
            map[entry.key] = entry.value;
            return map;
        }, {});

        return new ResourceAddresses(
            metadata["AdResource"],
            metadata["AdvertiserResource"],
            metadata["AdSlotProviderResource"],
            metadata["AdBrokerResource"],
        );
    }

    public async getAdBroker(adBrokerResource: string, nonFungibleId: string): Promise<AdBroker> {
        const response = await this.stateApi.stateNonFungiblePost({
            v0StateNonFungibleRequest: {
                resource_address: adBrokerResource,
                non_fungible_id_hex: nonFungibleId
            }
        });

        // @ts-ignore
        const data = response.non_fungible.non_fungible_data.immutable_data.struct_data.data_json;
        const brokerFields = data.fields[0].fields[0].fields;
        const brokerApiUrl = brokerFields[0].value;
        const trackingApiUrl = brokerFields[1].value;

        return new AdBroker(nonFungibleId, brokerApiUrl, trackingApiUrl);
    }

    public async getAdSlotProvider(adSlotProviderResource: string, nonFungibleId: string): Promise<AdSlotProvider> {
        const response = await this.stateApi.stateNonFungiblePost({
            v0StateNonFungibleRequest: {
                resource_address: adSlotProviderResource,
                non_fungible_id_hex: nonFungibleId
            }
        });

        // @ts-ignore
        const data = response.non_fungible.non_fungible_data.immutable_data.struct_data.data_json;
        const adSlotProviderFields = data.fields[0].fields[0].fields;
        const trackingApiUrl = adSlotProviderFields[0].value.value;

        return new AdSlotProvider(nonFungibleId, trackingApiUrl);
    }

    public async getAdvertiser(advertiserResource: string, nonFungibleId: string): Promise<Advertiser> {
        const response = await this.stateApi.stateNonFungiblePost({
            v0StateNonFungibleRequest: {
                resource_address: advertiserResource,
                non_fungible_id_hex: nonFungibleId
            }
        });

        // @ts-ignore
        const data = response.non_fungible.non_fungible_data.immutable_data.struct_data.data_json;
        const advertiserFields = data.fields[0].fields[0].fields;
        const trackingApiUrl = advertiserFields[0].value.value;

        return new Advertiser(nonFungibleId, trackingApiUrl);
    }

    public async getAd(adResource: string, nonFungibleId: string): Promise<Ad> {
        const response = await this.stateApi.stateNonFungiblePost({
            v0StateNonFungibleRequest: {
                resource_address: adResource,
                non_fungible_id_hex: nonFungibleId
            }
        });

        // @ts-ignore
        const data = response.non_fungible.non_fungible_data.immutable_data.struct_data.data_json;
        const imageUrl = data.fields[0].fields[0].value;
        const linkUrl = data.fields[1].value;
        const hoverText = data.fields[2].value;
        const advertiserNonFungibleId = parseAddress(data.fields[6]);

        return new Ad(nonFungibleId, imageUrl, linkUrl, hoverText, advertiserNonFungibleId);
    }
}

export class AdSlot {
    readonly nonFungibleId: string;
    readonly width: string;
    readonly height: string;
    readonly adSlotProviderNonFungibleId: string;
    readonly approved_broker_user_ids: Array<string>;

    constructor(nonFungibleId: string, width: string, height: string, adSlotProviderNonFungibleId: string, approved_broker_user_ids: Array<string>) {
        this.nonFungibleId = nonFungibleId;
        this.width = width;
        this.height = height;
        this.adSlotProviderNonFungibleId = adSlotProviderNonFungibleId;
        this.approved_broker_user_ids = approved_broker_user_ids;
    }
}

export class AdBroker {
    readonly nonFungibleId: string;
    readonly broker_api_url: string;
    readonly tracking_api_url: string;

    constructor(nonFungibleId: string, broker_api_url: string, tracking_api_url: string) {
        this.nonFungibleId = nonFungibleId;
        this.broker_api_url = broker_api_url;
        this.tracking_api_url = tracking_api_url;
    }
}

export class Advertiser {
    readonly nonFungibleId: string;
    readonly tracking_api_url: string;

    constructor(nonFungibleId: string, tracking_api_url: string) {
        this.nonFungibleId = nonFungibleId;
        this.tracking_api_url = tracking_api_url;
    }
}

export class AdSlotProvider {
    readonly nonFungibleId: string;
    readonly tracking_api_url: string;

    constructor(nonFungibleId: string, tracking_api_url: string) {
        this.nonFungibleId = nonFungibleId;
        this.tracking_api_url = tracking_api_url;
    }
}

export class Ad {
    readonly nonFungibleId: string;
    readonly imageUrl: string;
    readonly linkUrl: string;
    readonly hoverText: string;
    readonly advertiserNonFungibleId: string;

    constructor(nonFungibleId: string, imageUrl: string, linkUrl: string, hoverText: string, advertiserNonFungibleId: string) {
        this.nonFungibleId = nonFungibleId;
        this.imageUrl = imageUrl;
        this.linkUrl = linkUrl;
        this.hoverText = hoverText;
        this.advertiserNonFungibleId = advertiserNonFungibleId;
    }
}

export class AdPlacement {
    readonly id: string;
    readonly adSlot: AdSlot;
    readonly ad: Ad;
    readonly adBroker: AdBroker;
    readonly advertiser: Advertiser;
    readonly adSlotProvider: AdSlotProvider;

    constructor(id: string, adSlot: AdSlot, ad: Ad, adBroker: AdBroker, advertiser: Advertiser, adSlotProvider: AdSlotProvider) {
        this.id = id;
        this.adSlot = adSlot;
        this.ad = ad;
        this.adBroker = adBroker;
        this.advertiser = advertiser;
        this.adSlotProvider = adSlotProvider;
    }
}

export class ResourceAddresses {
    ad_resource: string;
    advertiser_resource: string;
    ad_slot_provider_resource: string;
    ad_broker_resource: string;

    constructor(ad_resource: string, advertiser_resource: string, ad_slot_provider_resource: string, ad_broker_resource: string) {
        this.ad_resource = ad_resource;
        this.advertiser_resource = advertiser_resource;
        this.ad_slot_provider_resource = ad_slot_provider_resource;
        this.ad_broker_resource = ad_broker_resource;
    }
}

export default AdSlotRepository;