export declare type JSONRPCRequestData = IJSONRPCData | IBatchRequest[];
export interface IJSONRPCData {
    internalID: string | number;
    request: IJSONRPCRequest | IJSONRPCNotification;
}
export interface IBatchRequest {
    resolve: (data: any) => void;
    reject: (data: any) => void;
    request: IJSONRPCData;
}
export interface IJSONRPCRequest {
    jsonrpc: "2.0";
    id: string | number;
    method: string;
    params: any[] | object;
}
export interface IJSONRPCError {
    code: number;
    message: string;
    data: any;
}
export interface IJSONRPCResponse {
    jsonrpc: "2.0";
    id?: string | number;
    result?: any;
    error?: IJSONRPCError;
}
export interface IJSONRPCNotificationResponse {
    jsonrpc: "2.0";
    id?: null | undefined;
    result?: any;
    error?: IJSONRPCError;
}
export interface IJSONRPCNotification {
    jsonrpc: "2.0";
    id?: null | undefined;
    method: string;
    params: any[] | object;
}
interface IRPCRequest {
    method: string;
    params: any[];
    type: "single";
}
interface IBatchRPCRequest {
    type: "batch";
    batch: IJSONRPCRequest[];
}
export declare type Request = IRPCRequest | IBatchRPCRequest;
export declare const isNotification: (data: IJSONRPCData) => boolean;
export declare const getBatchRequests: (data: JSONRPCRequestData) => IJSONRPCData[];
export declare const getNotifications: (data: JSONRPCRequestData) => IJSONRPCData[];
export {};
