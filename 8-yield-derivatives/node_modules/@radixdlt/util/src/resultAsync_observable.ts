import { Observable } from 'rxjs'
import { Result, ResultAsync } from 'neverthrow'

export const toObservable = <T, E = Error>(
	asyncResult: ResultAsync<T, E | E[]>,
): Observable<T> =>
	new Observable(subscriber => {
		void asyncResult.then((res: Result<T, E | E[]>) => {
			res.match(
				(value: T) => {
					subscriber.next(value)
					subscriber.complete()
				},
				(e: E | E[]) => {
					subscriber.error(e)
				},
			)
		})
	})

export const toObservableFromResult = <T, E = Error>(
	result: Result<T, E>,
): Observable<T> =>
	new Observable(subscriber => {
		result.match(
			(value: T) => {
				subscriber.next(value)
				subscriber.complete()
			},
			(e: E) => subscriber.error(e),
		)
	})
