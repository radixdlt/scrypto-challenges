import { decoder } from '../../../data-formats'
import { ok } from 'neverthrow'
import {
	AccountAddress,
	ValidatorAddress,
	ResourceIdentifier,
	ValidatorAddressOrUnsafeInput,
	AddressOrUnsafeInput,
} from '@radixdlt/account'
import { Amount, NetworkId } from '../../../primitives'
import { isObject, isString } from '../../../util'
import { TransactionIdentifier } from '../dto'

export const amountDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? Amount.fromUnsafe(value)
			: undefined,
	)

export const dateDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? ok(new Date(value))
			: undefined,
	)

export const RRIDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? ResourceIdentifier.fromUnsafe(value)
			: undefined,
	)

export const URLDecoder = (...keys: string[]) =>
	decoder((value, key) => {
		if (key !== undefined && keys.includes(key) && isString(value)) {
			try {
				return ok(new URL(value))
			} catch {
				return undefined
			}
		}
		return undefined
	})

export const transactionIdentifierDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? TransactionIdentifier.create(value)
			: undefined,
	)

export const networkDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined &&
		keys.includes(key) &&
		typeof value === 'number' &&
		Object.keys(NetworkId).includes(value.toString())
			? // @ts-ignore
			  ok(NetworkId[value.toString()])
			: undefined,
	)

export const addressDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? AccountAddress.fromUnsafe(value)
			: undefined,
	)

export const validatorAddressDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined && keys.includes(key) && isString(value)
			? ValidatorAddress.fromUnsafe(value)
			: undefined,
	)

export const addressObjectDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined &&
		keys.includes(key) &&
		isObject(value) &&
		value['address']
			? AccountAddress.fromUnsafe(
					value['address'] as AddressOrUnsafeInput,
			  )
			: undefined,
	)

export const validatorAddressObjectDecoder = (...keys: string[]) =>
	decoder((value, key) =>
		key !== undefined &&
		keys.includes(key) &&
		isObject(value) &&
		value['address']
			? ValidatorAddress.fromUnsafe(
					value['address'] as ValidatorAddressOrUnsafeInput,
			  )
			: undefined,
	)

const validatorAddressPattern = /^(r|t|d)v[0-9]?1[023456789ACDEFGHJKLMNPQRSTUVWXYZacdefghjklmnpqrstuvwxyz]{6,69}$/

const accountAddressPattern = /^(r|t|d)dx[0-9]?1[023456789ACDEFGHJKLMNPQRSTUVWXYZacdefghjklmnpqrstuvwxyz]{6,69}$/

export const addressFromUnsafe = (address: string) =>
	validatorAddressPattern.test(address)
		? ValidatorAddress.fromUnsafe(address)
		: accountAddressPattern.test(address)
		? AccountAddress.fromUnsafe(address)
		: undefined

export const addressRegexDecoder = (...keys: string[]) =>
	// @ts-ignore
	decoder((value, key) => {
		const isValidString =
			key !== undefined && keys.includes(key) && isString(value)

		if (!isValidString) {
			return undefined
		}

		return addressFromUnsafe(value)
	})
