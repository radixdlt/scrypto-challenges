export * from "./_types";
export * from "./action";

import { Receipt } from 'pte-sdk';
import { ActionType, GetAccountAddressSuccess, SignTransactionSuccess, } from "./_types";
import { sendAction, waitForAction } from "./action";

export const signTransaction = async function (manifest: string): Promise<Receipt> {
    sendAction({
        type: ActionType.SignTransaction,
        payload: manifest,
    });
    const response = await waitForAction<SignTransactionSuccess>(
        ActionType.SignTransactionSuccess,
        [ActionType.SignTransactionFailure]
    );
    console.log("Response: " + response);

    return response.payload;
}


export const getAccountAddress = async function (): Promise<string> {
    sendAction({
        type: ActionType.GetAccountAddress,
        payload: "",
    });
    const response = await waitForAction<GetAccountAddressSuccess>(
        ActionType.GetAccountAddressSuccess,
        [ActionType.GetAccountAddressFailure]
    );
    console.log("Response: " + response);

    return response.payload;
}
