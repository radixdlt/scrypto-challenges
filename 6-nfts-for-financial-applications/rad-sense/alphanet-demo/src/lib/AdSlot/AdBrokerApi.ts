export class AdPlacementDto {
    id: string;
    adNonFungibleId: string;
    adSlotNonFungibleId: string;
    userTrackingCookie: string;

    constructor(id: string, adNonFungibleId: string, adSlotNonFungibleId: string, userTrackingCookie: string) {
        this.id = id;
        this.adNonFungibleId = adNonFungibleId;
        this.adSlotNonFungibleId = adSlotNonFungibleId;
        this.userTrackingCookie = userTrackingCookie;
    }
}

export class NoAdFoundError extends Error {

    constructor(message: string) {
        super(message);
    }
}

class AdBrokerApi {
    private readonly apiBaseUrl: string;

    constructor(apiBaseUrl: string) {
        this.apiBaseUrl = apiBaseUrl;
        if (!this.apiBaseUrl.endsWith("/")) {
            this.apiBaseUrl += "/";
        }
    }

    public async placeAd(adSlotResource: string, adSlotNonFungibleId: string, userTrackingCookie: string): Promise<AdPlacementDto> {
        if (!adSlotResource) {
            throw Error("Parameter adSlotResource must be provided");
        }
        if (!adSlotNonFungibleId) {
            throw Error("Parameter adSlotNonFungibleId must be provided");
        }
        if (!userTrackingCookie) {
            throw Error("Parameter userTrackingCookie must be provided");
        }

        const url = new URL("./adPlacements", this.apiBaseUrl);
        const response = await (await fetch(url, {
            method: "POST",
            body: JSON.stringify({
                "adSlotResource": adSlotResource,
                "adSlotNonFungibleId": adSlotNonFungibleId,
                "userTrackingCookie": userTrackingCookie
            })
        }));

        if (response.status == 201) {
            return await response.json();
        } else if (response.status == 204) {
            throw new NoAdFoundError("No suitable ad found for this slot");
        } else {
            throw new Error("Unexpected API Response: " + response.status)
        }
    }
}

export default AdBrokerApi;
