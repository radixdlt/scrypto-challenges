import { Observable, SchedulerLike } from 'rxjs';
import { ShareReplayConfig } from 'rxjs/internal-compatibility';
/**
 * @method shareReplay
 * @owner Observable
 */
export declare function shareReplay<T>(this: Observable<T>, config: ShareReplayConfig): Observable<T>;
export declare function shareReplay<T>(this: Observable<T>, bufferSize?: number, windowTime?: number, scheduler?: SchedulerLike): Observable<T>;
