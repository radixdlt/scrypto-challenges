import { log } from '@radixdlt/util'
import { err, Result, ok } from 'neverthrow'
import { Observable, throwError, timer } from 'rxjs'
import { mergeMap } from 'rxjs/operators'

export const hasRequiredProps = <T extends Record<string, unknown>>(
	methodName: string,
	obj: T,
	props: string[],
): Result<T, Error[]> => {
	for (const prop of props) {
		if (obj[prop] === undefined) {
			return err([
				Error(
					`Prop validation failed for ${methodName} response. ${prop} was undefined.`,
				),
			])
		}
	}
	return ok(obj)
}

export const retryOnErrorCode = ({
	maxRetryAttempts = 3,
	scalingDuration = 1000,
	errorCodes = [],
}: {
	maxRetryAttempts?: number
	scalingDuration?: number
	errorCodes?: number[]
} = {}) => (attempts: Observable<{ error: { code: number } }>) =>
	attempts.pipe(
		mergeMap(({ error }, i) => {
			const retryAttempt = i + 1
			const foundErrorCode = errorCodes.some(e => e === error.code)
			// if maximum number of retries have been met
			// or response is a error code we don't wish to retry, throw error
			if (retryAttempt > maxRetryAttempts || !foundErrorCode) {
				return throwError(() => error)
			}
			log.debug(
				`Attempt ${retryAttempt}: retrying in ${
					retryAttempt * scalingDuration
				}ms`,
			)
			return timer(retryAttempt * scalingDuration)
		}),
	)
