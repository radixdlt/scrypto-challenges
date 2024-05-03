import { combine, ok, Result } from 'neverthrow'
import { isPublicKey, PublicKey, PublicKeyT } from '@radixdlt/crypto'
import { log, msgFromError } from '@radixdlt/util'
import { Bech32, Encoding } from '../bech32'
import { AbstractAddressT, AddressTypeT } from './_types'
import { Network } from '@radixdlt/primitives'

export const isAbstractAddress = (
	something: unknown,
): something is AbstractAddressT => {
	const inspection = something as AbstractAddressT
	return (
		inspection.publicKey !== undefined &&
		isPublicKey(inspection.publicKey) &&
		inspection.equals !== undefined &&
		inspection.toString !== undefined &&
		inspection.addressType !== undefined
	)
}

export type TypeGuard<A extends AbstractAddressT> = (
	something: unknown,
) => something is A

export type NetworkFromHRP = (hrp: string) => Result<Network, Error>
export type HRPFromNetwork = (network: Network) => string
export type FormatDataToBech32Convert = (publicKeyBytes: Buffer) => Buffer

export type ValidateDataAndExtractPubKeyBytes = (
	data: Buffer,
) => Result<Buffer, Error>

const __create = <A extends AbstractAddressT>(
	input: Readonly<{
		hrp: string
		data: Buffer
		addressType: AddressTypeT
		publicKey: PublicKeyT
		network: Network
		typeguard: TypeGuard<A>
		encoding?: Encoding
		maxLength?: number
	}>,
): Result<A, Error> => {
	const {
		hrp,
		data,
		encoding,
		maxLength,
		network,
		publicKey,
		addressType,
		typeguard,
	} = input
	return Bech32.encode({ hrp, data, encoding, maxLength })
		.mapErr(error => {
			const errMsg = `Incorrect implementation, failed to Bech32 encode data, underlying error: ${msgFromError(
				error,
			)}, but expect to always be able to.`
			console.error(errMsg)
			throw new Error(errMsg)
		})
		.map(encoded => {
			const toString = (): string => encoded.toString()

			const equals = (other: AbstractAddressT): boolean => {
				if (!isAbstractAddress(other)) {
					return false
				}
				return (
					other.publicKey.equals(publicKey) &&
					other.network === network &&
					addressType === other.addressType
				)
			}

			const abstract: AbstractAddressT = {
				addressType,
				network,
				publicKey,
				toString,
				equals,
			}

			if (!typeguard(abstract)) {
				const errMsg = `Incorrect implementation, expected to have created an address of type ${addressType.toString()}`
				log.error(errMsg)
				throw new Error(errMsg)
			}

			return abstract
		})
}

const byFormattingPublicKeyDataAndBech32ConvertingIt = <
	A extends AbstractAddressT
>(
	input: Readonly<{
		publicKey: PublicKeyT
		hrpFromNetwork: HRPFromNetwork
		addressType: AddressTypeT
		network: Network
		typeguard: TypeGuard<A>
		formatDataToBech32Convert?: FormatDataToBech32Convert
		encoding?: Encoding
		maxLength?: number
	}>,
): Result<A, Error> => {
	const { publicKey, hrpFromNetwork, network } = input

	const formatDataToBech32Convert =
		input.formatDataToBech32Convert ?? (b => b)

	const publicKeyBytes = publicKey.asData({ compressed: true })
	const bytes = formatDataToBech32Convert(publicKeyBytes)
	const hrp = hrpFromNetwork(network)
	return Bech32.convertDataToBech32(bytes).andThen(data =>
		__create({
			...input,
			hrp,
			data,
			publicKey,
		}),
	)
}

const fromString = <A extends AbstractAddressT>(
	input: Readonly<{
		bechString: string
		addressType: AddressTypeT
		networkFromHRP: NetworkFromHRP
		typeguard: TypeGuard<A>
		validateDataAndExtractPubKeyBytes?: ValidateDataAndExtractPubKeyBytes
		encoding?: Encoding
		maxLength?: number
	}>,
): Result<A, Error> => {
	const { bechString, networkFromHRP } = input

	const validateDataAndExtractPubKeyBytes =
		input.validateDataAndExtractPubKeyBytes ??
		((passthroughData: Buffer) => ok(passthroughData))

	return Bech32.decode(input)
		.andThen(({ hrp, data: bech32Data }) =>
			Bech32.convertDataFromBech32(bech32Data).map(dataFromBech32 => ({
				bech32Data,
				dataFromBech32,
				hrp,
			})),
		)
		.andThen(({ bech32Data, dataFromBech32, hrp }) =>
			validateDataAndExtractPubKeyBytes(dataFromBech32).map(
				publicKeyBytes => ({
					bech32Data,
					publicKeyBytes,
					hrp,
				}),
			),
		)
		.andThen(({ bech32Data, publicKeyBytes, hrp }) =>
			combine([
				networkFromHRP(hrp),
				PublicKey.fromBuffer(publicKeyBytes),
			]).map(resultList => {
				const network = resultList[0]
				const publicKey = resultList[1] as PublicKeyT
				return {
					bech32Data,
					hrp,
					network,
					publicKey,
				}
			}),
		)
		.andThen(({ bech32Data, hrp, network, publicKey }) =>
			__create({
				...input,
				network: network as Network,
				hrp,
				data: bech32Data,
				publicKey,
			}),
		)
		.map(
			(abstractAddress: A): A => {
				// Soundness check
				if (
					abstractAddress.toString().toLowerCase() !==
					bechString.toLowerCase()
				) {
					const errMsg = `Incorrect implementation, AbstractAddress mismatch, passed in: ${bechString.toLowerCase()}, created: ${abstractAddress
						.toString()
						.toLowerCase()}`
					log.error(errMsg)
					throw new Error(errMsg)
				}
				return abstractAddress
			},
		)
}

export const AbstractAddress = {
	byFormattingPublicKeyDataAndBech32ConvertingIt,
	fromString,
}
