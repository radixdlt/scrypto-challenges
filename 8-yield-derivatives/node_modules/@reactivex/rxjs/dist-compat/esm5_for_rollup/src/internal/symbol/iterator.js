export function getSymbolIterator() {
    if (typeof Symbol !== 'function' || !Symbol.iterator) {
        return '@@iterator';
    }
    return Symbol.iterator;
}
export var iterator = getSymbolIterator();
export var $$iterator = iterator;
//# sourceMappingURL=iterator.js.map