interface ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody(elt: any): any;
}
declare const olStyle = "list-style-type:none; padding-left: 0px; margin-top: 0px; margin-bottom: 0px; margin-left: 12px";
declare function getWithToArrayBody(elt: any): any;
declare class VectorHandler implements ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody: typeof getWithToArrayBody;
}
declare class StreamHandler implements ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody: typeof getWithToArrayBody;
}
declare class ListHandler implements ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody: typeof getWithToArrayBody;
}
declare class HashSetHandler implements ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody: typeof getWithToArrayBody;
}
declare class HashMapHandler implements ElementHandler {
    isElement(object: any): boolean;
    getHeader(object: any): any;
    hasBody(elt: any): boolean;
    getBody(elt: any): any;
}
declare const handlers: (VectorHandler | StreamHandler | ListHandler | HashSetHandler | HashMapHandler)[];
declare function getHandler(object: any): ElementHandler | undefined;
declare const formatter: {
    header: (object: any, config: any) => any;
    hasBody: (object: any, config: any) => boolean;
    body: (object: any, config: any) => any;
};
