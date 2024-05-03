/// <reference types="node" />
import { EventEmitter } from "events";
import { Transport } from "./Transport";
import { JSONRPCRequestData } from "../Request";
declare class EventEmitterTransport extends Transport {
    connection: EventEmitter;
    private reqUri;
    private resUri;
    constructor(destEmitter: EventEmitter, reqUri: string, resUri: string);
    connect(): Promise<any>;
    sendData(data: JSONRPCRequestData, timeout?: number | null): Promise<any>;
    close(): void;
}
export default EventEmitterTransport;
