import { Result } from 'neverthrow'

export type Decoder = (
	value: unknown,
	key?: string,
) => Result<unknown, Error> | undefined
