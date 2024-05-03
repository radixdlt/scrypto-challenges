import * as req from "../Request";
export declare const generateMockNotificationRequest: (method: string, params: any[]) => req.IJSONRPCNotification;
export declare const generateMockRequest: (id: number, method: string, params: any[]) => req.IJSONRPCRequest;
export declare const generateMockResponse: (id: number, result: any, error?: any) => req.IJSONRPCResponse;
export declare const generateMockNotificationResponse: (result: any, error?: any) => req.IJSONRPCNotificationResponse;
export declare const generateMockErrorResponse: (id: number | undefined, data: any) => req.IJSONRPCResponse;
export declare const generateMockResponseData: (uri: string, data: any) => any;
