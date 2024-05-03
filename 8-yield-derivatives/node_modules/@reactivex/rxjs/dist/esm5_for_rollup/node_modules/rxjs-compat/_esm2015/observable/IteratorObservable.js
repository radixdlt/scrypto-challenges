import { Observable, from } from 'rxjs';
export class IteratorObservable extends Observable {
    static create(iterable, scheduler) {
        return from(iterable, scheduler);
    }
}
//# sourceMappingURL=IteratorObservable.js.map