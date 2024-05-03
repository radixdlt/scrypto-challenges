export function jsonStringifyRecursive(obj) {
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
