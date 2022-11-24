import {json} from "@sveltejs/kit";
import {AppState} from "$lib/model";

let appState = new AppState();

export const GET = async () => {
    return json(appState)
}

export const PUT = async ({request}: any) => {
    appState = await request.json();
    return new Response(null, {status: 204})
}

export const DELETE = async () => {
    appState = new AppState();
    return new Response(null, {status: 204})
}