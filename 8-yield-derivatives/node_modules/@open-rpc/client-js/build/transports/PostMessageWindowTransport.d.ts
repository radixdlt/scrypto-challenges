import { Transport } from "./Transport";
import { JSONRPCRequestData } from "../Request";
declare class PostMessageTransport extends Transport {
    uri: string;
    frame: undefined | null | Window;
    postMessageID: string;
    constructor(uri: string);
    createWindow(uri: string): Promise<Window | null>;
    private messageHandler;
    connect(): Promise<any>;
    sendData(data: JSONRPCRequestData, timeout?: number | undefined): Promise<any>;
    close(): void;
}
export default PostMessageTransport;
