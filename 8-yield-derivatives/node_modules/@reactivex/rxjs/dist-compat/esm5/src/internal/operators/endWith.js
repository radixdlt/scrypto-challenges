import { concat } from '../observable/concat';
import { of } from '../observable/of';
export function endWith() {
    var array = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        array[_i] = arguments[_i];
    }
    return function (source) { return concat(source, of.apply(void 0, array)); };
}
//# sourceMappingURL=endWith.js.map