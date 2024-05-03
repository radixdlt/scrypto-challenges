import { async } from '../scheduler/async';
import { map } from './map';
export function timestamp(scheduler) {
    if (scheduler === void 0) { scheduler = async; }
    return map(function (value) { return new Timestamp(value, scheduler.now()); });
}
var Timestamp = (function () {
    function Timestamp(value, timestamp) {
        this.value = value;
        this.timestamp = timestamp;
    }
    return Timestamp;
}());
export { Timestamp };
//# sourceMappingURL=timestamp.js.map