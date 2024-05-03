import { ZipOperator } from '../observable/zip';
export function zipAll(project) {
    return function (source) { return source.lift(new ZipOperator(project)); };
}
//# sourceMappingURL=zipAll.js.map