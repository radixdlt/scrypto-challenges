import { IJSONRPCNotification } from "./Request";
interface Arguments {
    readonly method: string;
    readonly params?: readonly unknown[] | object;
}
export declare type RequestArguments = Arguments;
export declare type NotificationArguments = Arguments;
export declare type JSONRPCMessage = RequestArguments | NotificationArguments;
export interface IClient {
    request(args: RequestArguments): Promise<unknown>;
    notify(args: NotificationArguments): Promise<unknown>;
    close(): void;
    onNotification(callback: (data: IJSONRPCNotification) => void): void;
}
export {};
