import { err, ok, Result } from 'neverthrow'
import { BIP32PathComponentT, BIP32PathSimpleT, Int32 } from './_types'
import { BIP32 } from './bip32'

export const hardenedIncrement: number = 0x80000000

const assertNotHardened = (
	simplePath: BIP32PathSimpleT,
): Result<Int32, Error> => {
	const { index, isHardened } = simplePath
	if (index >= hardenedIncrement) {
		return err(
			new Error(
				`Incorrect implementation, expected value of index to be less than 'hardenedIncrement' for path components which are hardended. This function will add 'hardenedIncrement' to the value of index passed in, if 'isHardened' flag is set to true. But got value of index: ${index}, 'isHardened': ${
					isHardened ? 'true' : 'false'
				}`,
			),
		)
	}
	return ok(index)
}

const create = (
	input: Readonly<{
		index: Int32
		isHardened: boolean
		level: number
	}>,
): Result<BIP32PathComponentT, Error> => {
	const { isHardened } = input
	return assertNotHardened({ ...input }).map(index => ({
		...input,
		index: isHardened ? index + hardenedIncrement : index,
		value: () => index,
		toString: (): string => `${index}` + (isHardened ? `'` : ''),
	}))
}

export const isBIP32PathSimpleT = (
	something: unknown,
): something is BIP32PathSimpleT => {
	const inspection = something as BIP32PathSimpleT
	return inspection.index !== undefined && inspection.isHardened !== undefined
}

const fromString = (
	componentString: string,
	level: number,
): Result<BIP32PathComponentT, Error> => {
	if (componentString.includes(BIP32.pathSeparator)) {
		return err(new Error('Path component contains separator'))
	}
	let component = componentString
	let isHardened = false
	if (component.endsWith(BIP32.hardener)) {
		isHardened = true
		component = component.replace(BIP32.hardener, '')
	}

	let parsedInt: number = 0
	try {
		parsedInt = parseInt(component, 10)
	} catch (e) {
		return err(new Error('Failed to parse integer'))
	}
	if (!Number.isInteger(parsedInt)) {
		return err(new Error('Found no integer'))
	}

	return BIP32PathComponent.create({
		index: parsedInt,
		isHardened,
		level,
	})
}

export const BIP32PathComponent = {
	create,
	fromString,
}
