import {error, json} from '@sveltejs/kit';
import {v4 as uuidv4} from 'uuid';


export const POST = async ({request}: any) => {
    const url = new URL(request.url);
    const stateEndpointUrl = url.protocol + "//" + url.host + "/appState";

    const requestBody = await request.json();
    if (!requestBody.adSlotResource) {
        throw error(400, "Missing request body field: adSlotResource");
    }
    if (!requestBody.adSlotNonFungibleId) {
        throw error(400, "Missing request body field: adSlotNonFungibleId");
    }
    if (!requestBody.userTrackingCookie) {
        throw error(400, "Missing request body field: userTrackingCookie");
    }

    const appState = await (await fetch(stateEndpointUrl)).json();
    const adIds = appState.adIds;
    if (adIds.length == 0) {
        return new Response(null, {
            status: 204
        })
    } else {
        const adId = adIds[Math.floor(Math.random() * adIds.length)]
        return json({
            id: uuidv4(),
            adNonFungibleId: adId,
            adSlotNonFungibleId: requestBody.adSlotNonFungibleId,
            userTrackingCookie: requestBody.userTrackingCookie
        }, {
            status: 201
        })
    }
}
