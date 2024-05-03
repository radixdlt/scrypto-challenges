import { combine, err, ok, Result } from 'neverthrow'
import { BIP32 } from '../bip32'
import { BIP32PathComponent, hardenedIncrement } from '../bip32PathComponent'

import { BIP32PathComponentT, BIP32PathSimpleT, BIP32T, Int32 } from '../_types'
import { BIP44T, BIP44ChangeIndex, HDPathRadixT } from './_types'
import { msgFromError } from '@radixdlt/util'

export const RADIX_COIN_TYPE: Int32 = 1022

const bip44Component = (
	input: Readonly<{
		index: Int32
		isHardened: boolean
		level: number
		name: string
	}>,
): BIP32PathComponentT => {
	const componentResult = BIP32PathComponent.create(input)
	if (componentResult.isErr()) {
		throw new Error(
			`Incorrect implementation, should always be able to create BIP44 component. Underlying error: '${msgFromError(
				componentResult.error,
			)}'.`,
		)
	}
	return {
		...componentResult.value,
		name: input.name,
	}
}

export const bip44Purpose = bip44Component({
	index: 44,
	isHardened: true,
	level: 1,
	name: 'purpose',
})

const bip44CoinType = (index: Int32): BIP32PathComponentT =>
	bip44Component({
		index: index,
		isHardened: true,
		level: 2,
		name: 'coin type',
	})

const bip44Account = (index: Int32): BIP32PathComponentT =>
	bip44Component({
		index: index,
		isHardened: true,
		level: 3,
		name: 'account',
	})

const bip44Change = (index: BIP44ChangeIndex): BIP32PathComponentT =>
	bip44Component({
		index: index as Int32,
		isHardened: false,
		level: 4,
		name: 'change',
	})

const create = (
	input: Readonly<{
		coinType?: Int32 // defauts to `1022'` (Radix)
		account?: Int32 // defaults to `0'`
		change?: BIP44ChangeIndex // defaults to `0`
		address: Readonly<{
			index: Int32
			isHardened?: boolean // defaults to true
		}>
	}>,
): BIP44T => {
	const purpose = bip44Purpose
	const coinType = bip44CoinType(input.coinType ?? RADIX_COIN_TYPE)
	const account = bip44Account(input.account ?? 0)
	const change = bip44Change(input.change ?? 0)
	const addressIndex = bip44Component({
		index: input.address.index,
		isHardened: input.address.isHardened ?? true,
		level: 5,
		name: 'address index',
	})
	const pathComponents = [purpose, coinType, account, change, addressIndex]

	const bip32 = BIP32.unsafeCreate(pathComponents)
	return {
		...bip32,
		purpose,
		coinType,
		account,
		change,
		addressIndex,
		pathComponents,
	}
}

const fromComponents = (
	bip32Components: BIP32PathComponentT[],
): Result<HDPathRadixT, Error> =>
	BIP32.create(bip32Components).andThen(bip32 =>
		radixPathFromString(bip32.toString()),
	)

const createRadixPath = (
	input: Readonly<{
		account?: Int32 // defaults to `0'`
		change?: BIP44ChangeIndex // defaults to `0`
		address: Readonly<{
			index: Int32
			isHardened?: boolean // defaults to true
		}>
	}>,
): HDPathRadixT => create(input) as HDPathRadixT

const validateBIP44Component = (
	expected: Readonly<{
		index?: Int32
		isHardened: boolean
		level: number
		name?: string
	}>,
	component: BIP32PathComponentT,
): Result<BIP32PathComponentT, Error> => {
	if (component.level !== expected.level)
		return err(new Error('Wrong level in BIP44 path'))
	if (component.isHardened !== expected.isHardened)
		return err(
			new Error(
				`Wrong hardened value, expected component at level ${
					component.level
				} to${
					expected.isHardened ? '' : ' NOT'
				} be hardened, but it is${component.isHardened ? '' : ' NOT'}.`,
			),
		)
	if (expected.name) {
		if (component.name !== expected.name)
			return err(new Error('Wrong name'))
	}
	if (expected.index) {
		if (component.index !== expected.index) {
			return err(
				new Error(
					`Wrong index, component.index: ${
						component.index
					}, expected.index: ${
						expected.index
					}, whole expected: ${JSON.stringify(
						expected,
						null,
						4,
					)}, component: ${JSON.stringify(component, null, 4)}`,
				),
			)
		}
	}
	return ok(component)
}

const validateBIP44Purpose = validateBIP44Component.bind(null, bip44Purpose)
const validateBIP44CoinType = validateBIP44Component.bind(null, {
	...bip44CoinType(0),
	index: undefined,
})
const validateBIP44Account = validateBIP44Component.bind(null, {
	...bip44Account(0),
	index: undefined,
})
const validateBIP44Change = validateBIP44Component.bind(null, {
	...bip44Change(0),
	index: undefined,
})

const fromString = (path: string): Result<BIP44T, Error> =>
	BIP32.fromString(path).andThen(
		(bip32: BIP32T): Result<BIP44T, Error> => {
			const components = bip32.pathComponents
			if (components.length !== 5)
				return err(
					new Error(
						`We require BIP44 to have five components: purpose / cointype / account / change / address`,
					),
				)

			return combine([
				validateBIP44Purpose({ ...components[0], name: 'purpose' }),
				validateBIP44CoinType({ ...components[1], name: 'coin type' }),
				validateBIP44Account({ ...components[2], name: 'account' }),
				validateBIP44Change({ ...components[3], name: 'change' }),
				ok({ ...components[4], name: 'address index' }) as Result<
					BIP32PathComponentT,
					Error
				>,
			]).map(
				(bip44Components: BIP32PathComponentT[]): BIP44T => ({
					...bip32,
					purpose: bip44Components[0],
					coinType: bip44Components[1],
					account: bip44Components[2],
					change: bip44Components[3],
					addressIndex: bip44Components[4],
					pathComponents: bip44Components,
				}),
			)
		},
	)

const extractValueFromIndex = (
	pathComponent: BIP32PathSimpleT,
): Result<Int32, Error> => {
	const { index, isHardened } = pathComponent
	if (index >= hardenedIncrement && !isHardened)
		return err(
			new Error(
				`Incorrect values passed, index is hardened, but you believed it to not be. Index: ${index}`,
			),
		)
	if (index < hardenedIncrement && isHardened)
		return err(
			new Error(
				'Incorrect values passed, index is not hardened, but you believed it to be. Index: ${index}',
			),
		)
	return ok(isHardened ? index - hardenedIncrement : index)
}

const radixPathFromString = (path: string): Result<HDPathRadixT, Error> =>
	fromString(path).andThen(bip44 =>
		extractValueFromIndex(bip44.coinType).andThen(coinType =>
			coinType === RADIX_COIN_TYPE
				? ok(bip44 as HDPathRadixT)
				: err(
						new Error(
							`Incorrect coin type, expected Radix coin type: ${RADIX_COIN_TYPE}, but got: ${coinType}`,
						),
				  ),
		),
	)

export const BIP44 = {
	create,
	fromString,
}

export const HDPathRadix = {
	create: createRadixPath,
	fromString: radixPathFromString,
	fromComponents,
}
