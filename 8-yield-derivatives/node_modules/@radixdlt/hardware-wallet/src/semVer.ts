import { combine, err, ok, Result } from 'neverthrow'
import { SemVerT } from './_types'

const separator = '.'

const create = (
	input: Readonly<{ major: number; minor: number; patch: number }>,
): SemVerT => {
	const { major, minor, patch } = input
	const toString = (): string =>
		[major, minor, patch]
			.map((n: number): string => n.toString())
			.join(separator)

	const equals = (other: SemVerT): boolean =>
		other.major === major && other.minor === minor && other.patch === patch

	return {
		major,
		minor,
		patch,
		equals,
		toString,
	}
}

const fromBuffer = (buf: Buffer): Result<SemVerT, Error> => {
	const expectedByteCount = 3
	if (buf.length !== expectedByteCount) {
		return err(
			new Error(
				`Incorrect length of buffer, expected #${expectedByteCount} bytes, but got: #${buf.length}`,
			),
		)
	}

	const major = buf.readUInt8(0)
	const minor = buf.readUInt8(1)
	const patch = buf.readUInt8(2)

	return ok(create({ major, minor, patch }))
}

const fromString = (versionString: string): Result<SemVerT, Error> => {
	const components = versionString.split(separator)
	const expectedComponentCount = 3
	if (components.length !== expectedComponentCount) {
		return err(
			new Error(
				`Expected semantic version to contain ${expectedComponentCount} components.`,
			),
		)
	}
	const numAtIndex = (index: number): Result<number, Error> => {
		let parsedInt = undefined
		try {
			parsedInt = parseInt(components[index], 10)
		} catch (e) {
			return err(new Error('Failed to parse integer'))
		}
		if (!Number.isInteger(parsedInt)) {
			return err(new Error('Found no integer'))
		}
		return ok(parsedInt)
	}

	return combine([numAtIndex(0), numAtIndex(1), numAtIndex(2)]).map(
		resultList => {
			const major = resultList[0]
			const minor = resultList[1]
			const patch = resultList[2]
			return create({
				major,
				minor,
				patch,
			})
		},
	)
}

export const SemVer = {
	fromBuffer,
	fromString,
	create,
}
