import { operate } from '../util/lift';
import { OperatorSubscriber } from './OperatorSubscriber';
export function mapTo(value) {
    return operate(function (source, subscriber) {
        source.subscribe(new OperatorSubscriber(subscriber, function () { return subscriber.next(value); }));
    });
}
//# sourceMappingURL=mapTo.js.map