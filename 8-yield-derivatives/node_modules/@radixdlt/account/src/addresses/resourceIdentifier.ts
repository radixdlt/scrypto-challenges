import { err, ok, Result } from 'neverthrow'
import { buffersEquals, msgFromError } from '@radixdlt/util'
import { PublicKeyT, sha256Twice } from '@radixdlt/crypto'
import { HRP, hrpFullSuffixLength, Network } from '@radixdlt/primitives'
import { Bech32, Encoding } from '../bech32'
import { ResourceIdentifierT } from './_types'

const encoding = Encoding.BECH32
const maxLength: number | undefined = undefined // arbitrarily chosen

const versionByteNativeToken = 0x01
const versionByteNonNativeToken = 0x03

const hrpSuffixFromNetwork = (network: Network) => HRP[network].RRI_suffix

const networkFromHRPSuffix = (hrp: string): Result<Network, Error> =>
	hrp === HRP.mainnet.RRI_suffix
		? ok(Network.MAINNET)
		: hrp === HRP.stokenet.RRI_suffix
		? ok(Network.STOKENET)
		: hrp === HRP.localnet.RRI_suffix
		? ok(Network.LOCALNET)
		: hrp === HRP.releasenet.RRI_suffix
		? ok(Network.RELEASENET)
		: hrp === HRP.rcnet.RRI_suffix
		? ok(Network.RCNET)
		: hrp === HRP.milestonenet.RRI_suffix
		? ok(Network.MILESTONENET)
		: hrp === HRP.testnet6.RRI_suffix
		? ok(Network.TESTNET6)
		: hrp === HRP.sandpitnet.RRI_suffix
		? ok(Network.SANDPITNET)
		: err(
				new Error(
					`Failed to parse network from HRP ${hrp} for ValidatorAddress.`,
				),
		  )

const __create = (input: {
	hash: Buffer
	name: string
	network: Network
	toString: () => string
}): ResourceIdentifierT => ({
	...input,
	__witness: 'isRRI',
	equals: (other): boolean => {
		if (!isResourceIdentifier(other)) return false
		const same =
			other.name === input.name &&
			buffersEquals(other.hash, input.hash) &&
			input.network === other.network
		if (same) {
			if (other.toString() !== input.toString()) {
				const errMsg = `ResourceIdentifiers believed to be equal, but return different values when calling toString, (this)'${input.toString()}' vs other: '${other.toString()}'`
				console.error(errMsg)
				throw new Error(errMsg)
			}
		}
		return same
	},
})

const fromBech32String = (
	bechString: string,
): Result<ResourceIdentifierT, Error> => {
	// const hrpSuffix = hrpBetanetSuffix // TODO make dependent on Network!

	const decodingResult = Bech32.decode({ bechString, encoding, maxLength })

	if (!decodingResult.isOk()) {
		const errMsg = `Failed to Bech32 decode RRI, underlying error: ${msgFromError(
			decodingResult.error,
		)}`
		return err(new Error(errMsg))
	}
	const decoded = decodingResult.value
	const hrp = decoded.hrp

	if (
		!Object.keys(HRP).some(network =>
			hrp.endsWith(HRP[network as Network].RRI_suffix),
		)
	) {
		const errMsg = `suffix found for hrp "${hrp}" not supported.`
		return err(new Error(errMsg))
	}

	const nameToValidate = hrp.split('_')[0]
	const hrpSuffix = '_' + hrp.split('_')[1]
	const networkResult = networkFromHRPSuffix(hrpSuffix)

	if (!networkResult.isOk()) {
		const errMsg = `Expected to get network from HRP suffix '${hrpSuffix}', but failed to get it.`
		return err(new Error(errMsg))
	}
	const network = networkResult.value

	const nameValidationResult = validateCharsInName(nameToValidate)

	if (!nameValidationResult.isOk()) {
		return err(nameValidationResult.error)
	}
	const name = nameValidationResult.value

	const processed = decoded.data
	const combinedDataResult = Bech32.convertDataFromBech32(processed)

	if (!combinedDataResult.isOk()) {
		const errMsg = `Failed to convertDataFromBech32 data, underlying error: ${msgFromError(
			combinedDataResult.error,
		)}`
		console.error(errMsg)
		return err(new Error(errMsg))
	}

	const combinedData = combinedDataResult.value

	if (combinedData.length === 0) {
		const errMsg = `The data part of RRI should NEVER be empty, must at least contain 1 version byte ('${versionByteNativeToken}' for native token, or '${versionByteNonNativeToken}' for other tokens)`
		console.error(errMsg)
		return err(new Error(errMsg))
	}

	const versionByte = combinedData[0]

	if (
		!(
			versionByte === versionByteNativeToken ||
			versionByte === versionByteNonNativeToken
		)
	) {
		const errMsg = `The version byte must be either: '${versionByteNativeToken}' for native token, or '${versionByteNonNativeToken}' for other tokens, but got: ${versionByte}, bechString: '${bechString}'`
		console.error(errMsg)
		return err(new Error(errMsg))
	}

	const isNativeToken = versionByte === versionByteNativeToken

	if (isNativeToken) {
		if (combinedData.length > 1) {
			const errMsg = `Expected data to be empty for native token, but got: #${
				combinedData.length - 1 // minus 1 because we substract the 'versionByte'
			} bytes`
			console.error(errMsg)
			return err(new Error(errMsg))
		}
	} else {
		if (combinedData.length <= 1) {
			const errMsg = `Expected data to be non empty for non native token`
			console.error(errMsg)
			return err(new Error(errMsg))
		}
	}

	return ok(
		__create({
			hash: combinedData,
			network,
			name,
			toString: () => bechString,
		}),
	)
}

const validateCharsInName = (name: string): Result<string, Error> => {
	const regexLowerAlphaNumerics = new RegExp('^[a-z0-9]+$')
	if (!regexLowerAlphaNumerics.test(name)) {
		const errMsg = `Illegal characters found in name`
		// console.error(errMsg)
		return err(new Error(errMsg))
	}
	return ok(name)
}

const withNameRawDataAndVersionByte = (
	input: Readonly<{
		hash: Buffer
		network: Network
		versionByte: number
		name: string
	}>,
): Result<ResourceIdentifierT, Error> => {
	const { versionByte, hash, network } = input
	const hrpSuffix = hrpSuffixFromNetwork(network)

	return validateCharsInName(input.name).andThen(name => {
		const hrp = `${name}${hrpSuffix}`

		const combinedData = Buffer.concat([Buffer.from([versionByte]), hash])

		return Bech32.convertDataToBech32(combinedData)
			.andThen(processed =>
				Bech32.encode({
					data: processed,
					hrp,
					encoding,
					maxLength,
				}),
			)
			.map(bech32 =>
				__create({
					hash,
					network,
					name,
					toString: () => bech32.toString(),
				}),
			)
	})
}

const systemRRIForNetwork = (
	input: Readonly<{
		name: string
		network: Network
	}>,
): Result<ResourceIdentifierT, Error> =>
	withNameRawDataAndVersionByte({
		...input,
		versionByte: versionByteNativeToken,
		hash: Buffer.alloc(0),
	})

const hashByteCount = 26

const pkToHash = (
	input: Readonly<{
		name: string
		publicKey: PublicKeyT
	}>,
): Buffer => {
	const { name, publicKey } = input
	const nameBytes = Buffer.from(name, 'utf8')
	const pubKeyBytes = publicKey.asData({ compressed: true })
	const dataToHash = Buffer.concat([pubKeyBytes, nameBytes])
	const hash = sha256Twice(dataToHash)
	return hash.slice(-hashByteCount) // last bytes
}

const fromPublicKeyAndNameAndNetwork = (
	input: Readonly<{
		publicKey: PublicKeyT
		name: string
		network: Network
	}>,
): Result<ResourceIdentifierT, Error> =>
	withNameRawDataAndVersionByte({
		...input,
		versionByte: versionByteNonNativeToken,
		hash: pkToHash(input),
	})

const fromBuffer = (buffer: Buffer): Result<ResourceIdentifierT, Error> => {
	if (buffer.length === 1 && buffer[0] === 0x01) {
		return systemRRIForNetwork({
			name: 'xrd',
			network: Network.MAINNET, // Yikes!
		})
	}
	return err(
		new Error(
			'Failed to create non XRD RRI because we do not have access to the HRP.',
		),
	)
}

export const isResourceIdentifier = (
	something: ResourceIdentifierT | unknown,
): something is ResourceIdentifierT => {
	const inspection = something as ResourceIdentifierT
	return (
		// inspection.hash !== undefined &&
		inspection.__witness !== undefined &&
		inspection.__witness === 'isRRI' &&
		inspection.name !== undefined &&
		inspection.toString !== undefined &&
		inspection.equals !== undefined
	)
}

export type ResourceIdentifierUnsafeInput = string | Buffer

export const isResourceIdentifierUnsafeInput = (
	something: unknown,
): something is ResourceIdentifierUnsafeInput =>
	typeof something === 'string' || Buffer.isBuffer(something)

export type ResourceIdentifierOrUnsafeInput =
	| ResourceIdentifierT
	| ResourceIdentifierUnsafeInput

export const isResourceIdentifierOrUnsafeInput = (
	something: unknown,
): something is ResourceIdentifierOrUnsafeInput =>
	isResourceIdentifier(something) ||
	isResourceIdentifierUnsafeInput(something)

const fromUnsafe = (
	input: ResourceIdentifierOrUnsafeInput,
): Result<ResourceIdentifierT, Error> =>
	isResourceIdentifier(input)
		? ok(input)
		: typeof input === 'string'
		? fromBech32String(input)
		: fromBuffer(input)

export const ResourceIdentifier = {
	systemRRIForNetwork,
	fromPublicKeyAndNameAndNetwork,
	fromUnsafe,
}
