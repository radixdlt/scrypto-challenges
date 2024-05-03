import { Transport } from "./Transport";
import { JSONRPCRequestData } from "../Request";
declare type CredentialsOption = "omit" | "same-origin" | "include";
interface HTTPTransportOptions {
    credentials?: CredentialsOption;
    headers?: Record<string, string>;
}
declare class HTTPTransport extends Transport {
    uri: string;
    private readonly credentials?;
    private readonly headers;
    constructor(uri: string, options?: HTTPTransportOptions);
    connect(): Promise<any>;
    sendData(data: JSONRPCRequestData, timeout?: number | null): Promise<any>;
    close(): void;
    private onlyNotifications;
    private static setupHeaders;
}
export default HTTPTransport;
export { HTTPTransport, HTTPTransportOptions, CredentialsOption };
