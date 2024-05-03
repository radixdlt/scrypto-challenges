/// <reference types="node" />
import { JSONRPCRequestData, IJSONRPCNotificationResponse, IJSONRPCResponse } from "../Request";
import StrictEventEmitter from "strict-event-emitter-types";
import { EventEmitter } from "events";
import { JSONRPCError } from "../Error";
import { TransportRequestManager } from "./TransportRequestManager";
interface ITransportEvents {
    pending: (data: JSONRPCRequestData) => void;
    notification: (data: IJSONRPCNotificationResponse) => void;
    response: (data: IJSONRPCResponse) => void;
    error: (data: JSONRPCError) => void;
}
declare type TransportEventName = keyof ITransportEvents;
export declare type TransportEventChannel = StrictEventEmitter<EventEmitter, ITransportEvents>;
export declare abstract class Transport {
    protected transportRequestManager: TransportRequestManager;
    constructor();
    abstract connect(): Promise<any>;
    abstract close(): void;
    abstract sendData(data: JSONRPCRequestData, timeout?: number | null): Promise<any>;
    subscribe(event: TransportEventName, handler: ITransportEvents[TransportEventName]): void;
    unsubscribe(event?: TransportEventName, handler?: ITransportEvents[TransportEventName]): EventEmitter | undefined;
    protected parseData(data: JSONRPCRequestData): import("../Request").IJSONRPCRequest | import("../Request").IJSONRPCNotification | (import("../Request").IJSONRPCRequest | import("../Request").IJSONRPCNotification)[];
}
export declare type promiseResolve = (r?: {} | PromiseLike<{}> | undefined) => void;
export declare type promiseReject = (r?: any) => void;
export interface IRequestPromise {
    resolve: promiseResolve;
    reject: promiseReject;
}
export declare type NotificationResponse = "notification";
export declare type RequestResponse = "response";
export declare type BadResponse = "error";
export declare type TransportResponse = JSONRPCError | undefined;
interface IHttpTransportResponse {
    type: "http";
    id?: string | number;
    error?: Error;
    payload: string;
}
interface IWSTransportResponse {
    type: "ws";
    payload: string;
}
export declare type TransportResponseData = IHttpTransportResponse | IWSTransportResponse;
export {};
