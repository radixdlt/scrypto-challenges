import { bech32, bech32m, BechLib, Decoded } from 'bech32'
import { log, msgFromError } from '@radixdlt/util'
import { err, ok, Result } from 'neverthrow'
import { Bech32T } from './_types'

export enum Encoding {
	BECH32 = 'bech32',
	BECH32m = 'bech32m',
}

export const defaultEncoding = Encoding.BECH32

const convertDataFromBech32 = (bech32Data: Buffer): Result<Buffer, Error> => {
	try {
		const data = bech32.fromWords(bech32Data)
		return ok(Buffer.from(data))
	} catch (e) {
		const underlyingError = msgFromError(e)
		const errMsg = `Failed to converted bech32 data to Buffer, underlying error: '${underlyingError}'`
		return err(new Error(errMsg))
	}
}

const convertDataToBech32 = (data: Buffer): Result<Buffer, Error> => {
	try {
		const bech32Data = bech32.toWords(data)
		return ok(Buffer.from(bech32Data))
	} catch (e) {
		const underlyingError = msgFromError(e)
		const errMsg = `Failed to converted buffer to bech32 data, underlying error: '${underlyingError}'`
		return err(new Error(errMsg))
	}
}

const __unsafeCreate = (
	input: Readonly<{
		bech32String: string
		hrp: string
		data: Buffer
	}>,
): Bech32T => {
	const toString = (): string => input.bech32String
	const equals = (other: Bech32T): boolean => toString() === other.toString()
	return { hrp: input.hrp, data: input.data, equals, toString }
}

export type Bech32EncodeInput = Readonly<{
	hrp: string
	data: Buffer
	encoding?: Encoding
	maxLength?: number
}>

const encode = (input: Bech32EncodeInput): Result<Bech32T, Error> => {
	const { hrp, data, maxLength } = input
	const encoding = input.encoding ?? defaultEncoding

	const impl: BechLib = encoding === Encoding.BECH32 ? bech32 : bech32m

	try {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-call,@typescript-eslint/no-unsafe-member-access
		const bech32String: string = impl.encode(hrp, data, maxLength)
		return ok(
			__unsafeCreate({
				bech32String: bech32String.toLowerCase(),
				hrp,
				data,
			}),
		)
	} catch (e) {
		const errMsg = msgFromError(e)
		log.error(errMsg)
		return err(new Error(errMsg))
	}
}

export type Bech32DecodeInput = Readonly<{
	bechString: string
	encoding?: Encoding
	maxLength?: number
}>

const decode = (input: Bech32DecodeInput): Result<Bech32T, Error> => {
	const { bechString, maxLength } = input
	const encoding = input.encoding ?? defaultEncoding

	const impl: BechLib = encoding === Encoding.BECH32 ? bech32 : bech32m

	try {
		const decoded: Decoded = impl.decode(bechString, maxLength)
		return ok(
			__unsafeCreate({
				bech32String: bechString,
				hrp: decoded.prefix,
				data: Buffer.from(decoded.words),
			}),
		)
	} catch (e) {
		const errMsg = msgFromError(e)
		log.error(errMsg)
		return err(new Error(errMsg))
	}
}

export const Bech32 = {
	convertDataToBech32,
	convertDataFromBech32,
	decode,
	encode,
}
