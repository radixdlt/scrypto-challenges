declare class WebSocket {
    private callbacks;
    private url;
    constructor(url: string, props: any);
    addEventListener(eventName: string, callback: any): void;
    removeEventListener(eventName: string, callback: any): void;
    send(data: any, callback: (err?: Error) => void): void;
    close(): void;
}
export default WebSocket;
