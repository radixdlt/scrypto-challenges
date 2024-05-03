/// <reference types="ws" />
import WS from "isomorphic-ws";
import { Transport } from "./Transport";
import { JSONRPCRequestData } from "../Request";
declare class WebSocketTransport extends Transport {
    connection: WS;
    uri: string;
    constructor(uri: string);
    connect(): Promise<any>;
    sendData(data: JSONRPCRequestData, timeout?: number | null): Promise<any>;
    close(): void;
}
export default WebSocketTransport;
