"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.jsonStringifyRecursive = void 0;
function jsonStringifyRecursive(obj) {
    const cache = new Set();
    return JSON.stringify(obj, (key, value) => {
        if (typeof value === "object" && value !== null) {
            if (cache.has(value)) {
                return "[Circular]";
            }
            cache.add(value);
        }
        return value;
    });
}
exports.jsonStringifyRecursive = jsonStringifyRecursive;
