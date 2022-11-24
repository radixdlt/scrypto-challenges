<script lang="ts">
    import AdSlotRepository, {AdBroker, AdPlacement, AdSlot, ResourceAddresses,} from "./AdSlotRepository";
    import {v4 as uuidv4} from 'uuid';
    import AdBrokerApi, {AdPlacementDto} from "./AdBrokerApi";

    export let adSlotResourceAddress: string;
    export let adSlotNonFungibleId: string;

    // Some imagined tracking cookie that identifies the user
    const dummyUserTrackingCookie = "4711";

    const adSlotRepo = new AdSlotRepository();
    const adSlot = adSlotRepo.getAdSlot(adSlotResourceAddress, adSlotNonFungibleId);
    const adSlotAddresses = adSlotRepo.getResourceAddresses(adSlotResourceAddress);
    const adBroker = Promise.all([adSlot, adSlotAddresses]).then((values) => {
        const [adSlot, adSlotAddresses]: [AdSlot, ResourceAddresses] = values;
        let brokerIds: Array<string> = adSlot.approved_broker_user_ids;
        const adBrokerId: string = brokerIds[Math.floor(Math.random() * brokerIds.length)];
        return adSlotRepo.getAdBroker(adSlotAddresses.ad_broker_resource, adBrokerId);
    })

    const createAdPlacement = async (adSlot: AdSlot, adBroker: AdBroker) => {
        const addresses = await adSlotAddresses;
        const adBrokerApi = new AdBrokerApi(adBroker.broker_api_url);
        const adPlacementDto: AdPlacementDto = await adBrokerApi.placeAd(adSlotResourceAddress, adSlot.nonFungibleId, dummyUserTrackingCookie);
        const ad = await adSlotRepo.getAd(addresses.ad_resource, adPlacementDto.adNonFungibleId);
        const promisedAdvertiser = adSlotRepo.getAdvertiser(addresses.advertiser_resource, ad.advertiserNonFungibleId);
        const promisedAdSlotProvider = adSlotRepo.getAdSlotProvider(addresses.ad_slot_provider_resource, adSlot.adSlotProviderNonFungibleId);
        const [advertiser, adSlotProvider] = await Promise.all([promisedAdvertiser, promisedAdSlotProvider]);

        const adPlacement = new AdPlacement(adPlacementDto.id, adSlot, ad, adBroker, advertiser, adSlotProvider);
        // noinspection ES6MissingAwait
        sendEventToTrackingApis("AdPlaced", adPlacement);

        return adPlacement;
    }

    const onAdClicked = async (adPlacement: AdPlacement) => {
        // noinspection ES6MissingAwait
        sendEventToTrackingApis("AdClicked", adPlacement)
        open(adPlacement.ad.linkUrl, "_blank")
    }

    /**
     * Sends out an event of the given type, containing the information of the given AdPlacement to the tracking APIs
     * of the advertiser, the ad slot provider and the ad broker so that all parties are informed.
     *
     * Requests happen in parallel.
     */
    const sendEventToTrackingApis = async (eventType: string, adPlacement: AdPlacement) => {
        const event = {
            id: uuidv4(),
            type: eventType,
            timestamp: Date.now(),
            adPlacementId: adPlacement.id,
            adNonFungibleId: adPlacement.ad.nonFungibleId,
            adSlotNonFungibleId: adPlacement.adSlot.nonFungibleId,
            adBrokerNonFungibleId: adPlacement.adBroker.nonFungibleId
        };

        const trackingApiUrls = [
            adPlacement.adBroker.tracking_api_url,
            adPlacement.advertiser.tracking_api_url,
            adPlacement.adSlotProvider.tracking_api_url
        ];

        const responses = trackingApiUrls.map((trackingApiUrl) => {
            if (!trackingApiUrl.endsWith("/")) {
                trackingApiUrl += "/";
            }

            return fetch(new URL("./events", trackingApiUrl), {
                method: "POST",
                body: JSON.stringify(event)
            })
        });

        return Promise.all(responses);
    }
</script>

<div class="ad-slot">

    {#await Promise.all([adSlot, adBroker]) then [adSlot, adBroker] }
        {#await createAdPlacement(adSlot, adBroker) then adPlacement}
            <img
                    on:click={() => onAdClicked(adPlacement)}
                    title="{adPlacement.ad.hoverText}"
                    src={adPlacement.ad.imageUrl}
                    alt={adPlacement.ad.hoverText}
                    style="width: {adSlot.width}px; height: {adSlot.height}px"
                    class="ad-img"/>
        {:catch error}
            <span>{error}</span>
        {/await}
    {/await}
</div>


<style>
    .ad-slot {
        border: 3px black dashed;
        padding: 0.5rem;
    }

    .ad-img {
        object-fit: cover;
    }

</style>