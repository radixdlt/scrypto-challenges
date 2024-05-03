import { CombineLatestOperator } from '../observable/combineLatest';
export function combineAll(project) {
    return function (source) { return source.lift(new CombineLatestOperator(project)); };
}
//# sourceMappingURL=combineAll.js.map