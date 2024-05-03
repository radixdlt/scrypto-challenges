import { JSONDecoding } from '@radixdlt/data-formats'
import { err, ok, Result } from 'neverthrow'
import {
	BuildTransactionEndpoint,
	SubmitTransactionEndpoint,
	LookupTransactionEndpoint,
	NetworkIdEndpoint,
	NetworkTransactionDemandEndpoint,
	NetworkTransactionThroughputEndpoint,
	StakePositionsEndpoint,
	FinalizeTransactionEndpoint,
	TokenBalancesEndpoint,
	TokenInfoEndpoint,
	TransactionHistoryEndpoint,
	TransactionStatusEndpoint,
	UnstakePositionsEndpoint,
	ValidatorsEndpoint,
	LookupValidatorEndpoint,
} from './_types'
import { pipe } from 'ramda'
import { Message } from '@radixdlt/crypto'
import {
	addressDecoder,
	amountDecoder,
	dateDecoder,
	networkDecoder,
	RRIDecoder,
	transactionIdentifierDecoder,
	URLDecoder,
	validatorAddressDecoder,
} from '../decoders'

const executedTXDecoders = JSONDecoding.withDecoders(
	amountDecoder('amount', 'fee'),
	dateDecoder('sentAt'),
	addressDecoder('from', 'to'),
	validatorAddressDecoder('validator'),
	transactionIdentifierDecoder('txID'),
	RRIDecoder('rri'),
)

export type RPCRequestFailureResponse = Readonly<{
	failure: string
}>

const isRPCRequestFailureResponse = (
	something: unknown,
): something is RPCRequestFailureResponse => {
	const inspection = something as RPCRequestFailureResponse
	return inspection.failure !== undefined
}

const hasRequiredProps = <T extends Record<string, unknown>>(
	methodName: string,
	obj: T,
	props: string[],
): Result<T, Error[]> => {
	for (const prop of props) {
		if (obj[prop] === undefined) {
			return err([
				Error(
					`Prop validation failed for ${methodName} response. ${prop} was undefined.`,
				),
			])
		}
	}
	return ok(obj)
}

export const handleTransactionHistoryResponse = (
	json: TransactionHistoryEndpoint.Response,
) =>
	executedTXDecoders
		.create<
			TransactionHistoryEndpoint.Response,
			TransactionHistoryEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			ok({
				...decoded,
				transactions: decoded.transactions.map(tx => ({
					...tx,
					message: (() => {
						if (!tx.message) return undefined

						// Check format
						if (!/^(00|01)[0-9a-fA-F]+$/.test(tx.message))
							return '<Failed to interpret message>'

						return Message.isPlaintext(tx.message)
							? Message.plaintextToString(
									Buffer.from(tx.message, 'hex'),
							  )
							: tx.message
					})(),
				})),
			}),
		)

export const handleLookupTXResponse = (
	json: LookupTransactionEndpoint.Response,
) =>
	executedTXDecoders
		.create<
			LookupTransactionEndpoint.Response,
			LookupTransactionEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			ok({
				...decoded,
				message: (() => {
					if (!decoded.message) return undefined

					// Check format
					if (!/^(00|01)[0-9a-fA-F]+$/.test(decoded.message))
						return '<Failed to interpret message>'

					return Message.isPlaintext(decoded.message)
						? Message.plaintextToString(
								Buffer.from(decoded.message, 'hex'),
						  )
						: decoded.message
				})(),
			}),
		)

export const handleNetworkIdResponse = (json: NetworkIdEndpoint.Response) =>
	JSONDecoding.withDecoders(networkDecoder('networkId'))
		.create<
			NetworkIdEndpoint.Response,
			NetworkIdEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			hasRequiredProps('networkId', decoded, ['networkId']),
		)

export const handleTokenBalancesResponse = (
	json: TokenBalancesEndpoint.Response,
): Result<TokenBalancesEndpoint.DecodedResponse, Error[]> =>
	pipe(
		(json: TokenBalancesEndpoint.Response) => ({
			owner: json.owner,
			tokenBalances: json.tokenBalances.map(balance => ({
				tokenIdentifier: balance.rri,
				amount: balance.amount,
			})),
		}),
		JSONDecoding.withDecoders(
			addressDecoder('owner'),
			RRIDecoder('tokenIdentifier'),
			amountDecoder('amount'),
		).create<
			{
				owner: string
				tokenBalances: {
					tokenIdentifier: string
					amount: string
				}[]
			},
			TokenBalancesEndpoint.DecodedResponse
		>(),
	)(json).andThen(decoded =>
		hasRequiredProps('tokenBalances', decoded, ['owner', 'tokenBalances']),
	)

const validatorDecoders = JSONDecoding.withDecoders(
	validatorAddressDecoder('address'),
	addressDecoder('ownerAddress'),
	URLDecoder('infoURL'),
	amountDecoder('totalDelegatedStake', 'ownerDelegation'),
)

export const handleValidatorsResponse = (json: ValidatorsEndpoint.Response) =>
	validatorDecoders
		.create<
			ValidatorsEndpoint.Response,
			ValidatorsEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			hasRequiredProps('validators', decoded, ['cursor', 'validators']),
		)

export const handleLookupValidatorResponse = validatorDecoders.create<
	LookupValidatorEndpoint.Response,
	LookupValidatorEndpoint.DecodedResponse
>()

export const handleTokenInfoResponse = (json: TokenInfoEndpoint.Response) =>
	JSONDecoding.withDecoders(
		RRIDecoder('rri'),
		amountDecoder('granularity', 'currentSupply'),
		URLDecoder('tokenInfoURL', 'iconURL'),
	)
		.create<
			TokenInfoEndpoint.Response,
			TokenInfoEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			hasRequiredProps('tokenInfo', decoded, [
				'name',
				'rri',
				'symbol',
				'granularity',
				'isSupplyMutable',
				'currentSupply',
				'tokenInfoURL',
				'iconURL',
			]),
		)

export const handleStakesResponse = JSONDecoding.withDecoders(
	validatorAddressDecoder('validator'),
	amountDecoder('amount'),
).create<
	StakePositionsEndpoint.Response,
	StakePositionsEndpoint.DecodedResponse
>()

export const handleUnstakesResponse = JSONDecoding.withDecoders(
	validatorAddressDecoder('validator'),
	amountDecoder('amount'),
	transactionIdentifierDecoder('withdrawTxID'),
).create<
	UnstakePositionsEndpoint.Response,
	UnstakePositionsEndpoint.DecodedResponse
>()

export const handleTransactionStatusResponse = (
	json: TransactionStatusEndpoint.Response,
): Result<TransactionStatusEndpoint.DecodedResponse, Error[]> =>
	isRPCRequestFailureResponse(json)
		? err([new Error(json.failure)])
		: JSONDecoding.withDecoders(transactionIdentifierDecoder('txID'))
				.create<
					TransactionStatusEndpoint.Response,
					TransactionStatusEndpoint.DecodedResponse
				>()(json)
				.andThen(decoded =>
					hasRequiredProps('transactionStatus', decoded, [
						'txID',
						'status',
					]),
				)

export const handleNetworkxThroughputResponse = (
	json: NetworkTransactionThroughputEndpoint.Response,
) =>
	JSONDecoding.create<
		NetworkTransactionThroughputEndpoint.Response,
		NetworkTransactionThroughputEndpoint.DecodedResponse
	>()(json).andThen(decoded =>
		hasRequiredProps('NetworkTransactionThroughput', decoded, ['tps']),
	)

export const handleNetworkxDemandResponse = (
	json: NetworkTransactionDemandEndpoint.Response,
) =>
	JSONDecoding.create<
		NetworkTransactionDemandEndpoint.Response,
		NetworkTransactionDemandEndpoint.DecodedResponse
	>()(json).andThen(decoded =>
		hasRequiredProps('NetworkTransactionDemand', decoded, ['tps']),
	)

export const handleBuildTransactionResponse = (
	json: BuildTransactionEndpoint.Response,
): Result<BuildTransactionEndpoint.DecodedResponse, Error[]> =>
	JSONDecoding.withDecoders(amountDecoder('fee'))
		.create<
			BuildTransactionEndpoint.Response,
			BuildTransactionEndpoint.DecodedResponse
		>()(json)
		.andThen(decoded =>
			hasRequiredProps('buildTransaction', decoded, [
				'transaction',
				'fee',
			]),
		)

export const handleFinalizeTransactionResponse = (
	json: FinalizeTransactionEndpoint.Response,
): Result<FinalizeTransactionEndpoint.DecodedResponse, Error[]> =>
	isRPCRequestFailureResponse(json)
		? err([new Error(json.failure)])
		: JSONDecoding.withDecoders(transactionIdentifierDecoder('txID'))
				.create<
					FinalizeTransactionEndpoint.Response,
					FinalizeTransactionEndpoint.DecodedResponse
				>()(json)
				.andThen(decoded =>
					hasRequiredProps('finalizeTransaction', decoded, ['txID']),
				)

export const handleSubmitTransactionResponse = (
	json: SubmitTransactionEndpoint.Response,
): Result<SubmitTransactionEndpoint.DecodedResponse, Error[]> =>
	isRPCRequestFailureResponse(json)
		? err([new Error(json.failure)])
		: JSONDecoding.withDecoders(transactionIdentifierDecoder('txID'))
				.create<
					SubmitTransactionEndpoint.Response,
					SubmitTransactionEndpoint.DecodedResponse
				>()(json)
				.andThen(decoded =>
					hasRequiredProps('submitTransaction', decoded, ['txID']),
				)
