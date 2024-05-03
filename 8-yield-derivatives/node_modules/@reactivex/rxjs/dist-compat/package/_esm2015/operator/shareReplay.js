import { shareReplay as higherOrder } from 'rxjs/operators';
export function shareReplay(configOrBufferSize, windowTime, scheduler) {
    if (configOrBufferSize && typeof configOrBufferSize === 'object') {
        return higherOrder(configOrBufferSize)(this);
    }
    return higherOrder(configOrBufferSize, windowTime, scheduler)(this);
}
//# sourceMappingURL=shareReplay.js.map