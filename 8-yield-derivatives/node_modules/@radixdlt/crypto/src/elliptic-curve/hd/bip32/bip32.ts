import { combine, err, ok, Result } from 'neverthrow'
import { BIP32PathComponent } from './bip32PathComponent'
import { BIP32T, BIP32PathComponentT, Int32 } from './_types'
import { ValidationWitness } from '@radixdlt/util'

const pathSeparator = '/'
const hardener = `'`

const isBIP32 = (something: unknown): something is BIP32T => {
	const inspection = something as BIP32T
	return (
		inspection.pathComponents !== undefined &&
		inspection.toString !== undefined
	)
}

export const unsafeCreate = (pathComponents: BIP32PathComponentT[]): BIP32T => {
	const toString = (): string =>
		'm' +
		pathSeparator +
		pathComponents.map(pc => pc.toString()).join(pathSeparator)

	return {
		pathComponents,
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		equals: (other: any): boolean => {
			if (!isBIP32(other)) return false
			return other.toString() === toString()
		},
		toString,
	}
}

const validateLevels = (
	pathComponents: BIP32PathComponentT[],
): Result<ValidationWitness, Error> =>
	combine(
		pathComponents.map<Result<ValidationWitness, Error>>(
			(component, i, components) =>
				component.level !== (i > 0 ? components[i - 1].level + 1 : 1)
					? err(
							new Error(
								`Expected components with strictly increasing level with an increment of one.`,
							),
					  )
					: ok({ witness: 'component valid' }),
		),
	).andThen(_a => ok({ witness: 'all components valid' }))

const create = (pathComponents: BIP32PathComponentT[]): Result<BIP32T, Error> =>
	validateLevels(pathComponents).map(() => unsafeCreate(pathComponents))

const fromString = (path: string): Result<BIP32T, Error> => {
	let bip32Path = path.trim()
	if (bip32Path === '' || bip32Path === 'm' || bip32Path === pathSeparator) {
		return ok({
			pathComponents: [],
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			equals: (other: any): boolean => {
				if (!isBIP32(other)) return false
				return other.toString() === 'm'
			},
			toString: (): string => 'm',
		})
	}

	if (bip32Path.startsWith('M/') || bip32Path.startsWith('m/')) {
		bip32Path = bip32Path.slice(2)
		if (bip32Path.length === 0) {
			return err(new Error(`Must start with just 'm/' or 'M/'`))
		}
	}
	if (bip32Path.length === 0) {
		return err(new Error('Must not be empty'))
	}

	if (bip32Path.includes('//')) {
		return err(new Error(`Must not contain '//'`))
	}

	const components = bip32Path.split(pathSeparator)
	const pathComponents: BIP32PathComponentT[] = []
	for (const { index, value } of components.map((value, index) => ({
		index,
		value,
	}))) {
		const pathComponentResult = BIP32PathComponent.fromString(
			value,
			index + 1,
		)
		if (pathComponentResult.isErr()) return err(pathComponentResult.error)
		pathComponents.push(pathComponentResult.value)
	}
	return create(pathComponents)
}
const unsafeFromSimpleComponents = (
	pathComponents: Readonly<{
		index: Int32
		isHardened: boolean
	}>[],
): Result<BIP32T, Error> =>
	combine(
		pathComponents.map((e, i) =>
			BIP32PathComponent.create({
				...e,
				level: i,
			}),
		),
	).map(unsafeCreate)

export const BIP32 = {
	create,
	unsafeCreate,
	fromString,
	unsafeFromSimpleComponents,
	hardener,
	pathSeparator,
}
