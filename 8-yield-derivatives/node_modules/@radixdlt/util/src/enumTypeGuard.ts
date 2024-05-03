export const isSomeEnum = <T>(e: T) => (token: unknown): token is T[keyof T] =>
	Object.values(e).includes(token as T[keyof T])
