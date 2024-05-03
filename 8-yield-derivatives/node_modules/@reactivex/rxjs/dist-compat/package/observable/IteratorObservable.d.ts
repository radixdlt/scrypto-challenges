import { Observable, SchedulerLike } from 'rxjs';
export declare class IteratorObservable<T> extends Observable<T> {
    static create<T>(iterable: Iterable<T>, scheduler?: SchedulerLike): Observable<T>;
}
