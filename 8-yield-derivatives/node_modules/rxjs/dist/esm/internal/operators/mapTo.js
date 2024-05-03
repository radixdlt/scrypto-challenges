import { operate } from '../util/lift';
import { OperatorSubscriber } from './OperatorSubscriber';
export function mapTo(value) {
    return operate((source, subscriber) => {
        source.subscribe(new OperatorSubscriber(subscriber, () => subscriber.next(value)));
    });
}
//# sourceMappingURL=mapTo.js.map