import { switchMap } from './switchMap';
export function switchMapTo(innerObservable, resultSelector) {
    return resultSelector ? switchMap(function () { return innerObservable; }, resultSelector) : switchMap(function () { return innerObservable; });
}
//# sourceMappingURL=switchMapTo.js.map