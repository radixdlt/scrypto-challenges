import { FindValueOperator } from '../operators/find';
export function findIndex(predicate, thisArg) {
    return function (source) { return source.lift(new FindValueOperator(predicate, source, true, thisArg)); };
}
//# sourceMappingURL=findIndex.js.map