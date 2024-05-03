import { multicast } from './multicast';
import { refCount } from './refCount';
import { Subject } from '../Subject';
function shareSubjectFactory() {
    return new Subject();
}
export function share() {
    return function (source) { return refCount()(multicast(shareSubjectFactory)(source)); };
}
//# sourceMappingURL=share.js.map