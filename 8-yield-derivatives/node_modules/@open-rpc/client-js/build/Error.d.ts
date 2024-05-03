export declare const ERR_TIMEOUT = 7777;
export declare const ERR_MISSIING_ID = 7878;
export declare const ERR_UNKNOWN = 7979;
export declare class JSONRPCError extends Error {
    message: string;
    code: number;
    data?: unknown;
    constructor(message: string, code: number, data?: any);
}
export declare const convertJSONToRPCError: (payload: any) => JSONRPCError;
