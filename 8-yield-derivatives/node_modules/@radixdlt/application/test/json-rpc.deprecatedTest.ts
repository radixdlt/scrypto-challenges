/**
 * @jest-environment ./packages/application/test/_load-rpc.ts
 */
/*import {
	nodeAPI,
	TransactionIdentifier,
	RawExecutedAction,
	RawToken,
	Token,
	ActionType,
	ExecutedAction,
	ExecutedOtherAction,
	ExecutedTransferTokensAction,
	BuildTransactionEndpoint,
	FinalizeTransactionEndpoint,
	LookupTransactionEndpoint,
	LookupValidatorEndpoint,
	NativeTokenEndpoint,
	NetworkIdEndpoint,
	NetworkTransactionDemandEndpoint,
	NetworkTransactionThroughputEndpoint,
	StakePositionsEndpoint,
	SubmitTransactionEndpoint,
	TokenBalancesEndpoint,
	TokenInfoEndpoint,
	TransactionHistoryEndpoint,
	TransactionStatusEndpoint,
	UnstakePositionsEndpoint,
	ValidatorsEndpoint,
	ApiMethod,
	Message,
} from '../src'
import { Amount, Network } from '@radixdlt/primitives'

import { isArray, isObject } from '@radixdlt/util'
import {
	ContentDescriptorObject,
	MethodObject,
	OpenrpcDocument,
} from '@open-rpc/meta-schema'
import {
	AccountAddress,
	ResourceIdentifier,
	ValidatorAddress,
} from '@radixdlt/account'

const faker = require('json-schema-faker')

let mockClientReturnValue: any

function mockHTTPTransport() {}
function mockRequestManager() {}
function mockClient() {
	return {
		request: async () => mockClientReturnValue,
	}
}

jest.mock('@open-rpc/client-js', () => ({
	Client: mockClient,
	HTTPTransport: mockHTTPTransport,
	RequestManager: mockRequestManager,
}))

const executedActionFromRaw = (action: RawExecutedAction): ExecutedAction => {
	if (action.type === ActionType.TOKEN_TRANSFER) {
		const executed: ExecutedTransferTokensAction = {
			...action,
			// transactionType: TransactionType.OUTGOING,
			from: AccountAddress.fromUnsafe(action.from)._unsafeUnwrap({
				withStackTrace: true,
			}),
			to: AccountAddress.fromUnsafe(action.to)._unsafeUnwrap({
				withStackTrace: true,
			}),
			rri: ResourceIdentifier.fromUnsafe(action.rri)._unsafeUnwrap({
				withStackTrace: true,
			}),
			amount: Amount.fromUnsafe(action.amount)._unsafeUnwrap({
				withStackTrace: true,
			}),
		}
		return executed
	} else if (
		action.type === ActionType.STAKE_TOKENS ||
		action.type === ActionType.UNSTAKE_TOKENS
	) {
		return {
			...action,
			from: AccountAddress.fromUnsafe(action.from)._unsafeUnwrap({
				withStackTrace: true,
			}),
			validator: ValidatorAddress.fromUnsafe(
				action.validator,
			)._unsafeUnwrap({ withStackTrace: true }),
			amount: Amount.fromUnsafe(action.amount)._unsafeUnwrap({
				withStackTrace: true,
			}),
		}
	} else {
		const executed: ExecutedOtherAction = {
			type: ActionType.OTHER,
		}
		return executed
	}
}

// @ts-ignore
const rpcSpec: OpenrpcDocument = global.rpcSpec

const tokenInfoFromResponse = (response: RawToken): Token => ({
	name: response.name,
	rri: ResourceIdentifier.fromUnsafe(response.rri)._unsafeUnwrap({
		withStackTrace: true,
	}),
	symbol: response.symbol,
	description: response.description,
	granularity: Amount.fromUnsafe(response.granularity)._unsafeUnwrap(),
	isSupplyMutable: response.isSupplyMutable,
	currentSupply: Amount.fromUnsafe(response.currentSupply)._unsafeUnwrap(),
	tokenInfoURL: new URL(response.tokenInfoURL),
	iconURL: new URL(response.iconURL),
})

const methodParams = {
	[rpcSpec.methods[0].name]: {},

	[rpcSpec.methods[1].name]: {
		rri: 'xrd_tr1qyf0x76s',
	},

	[rpcSpec.methods[2].name]: {
		address:
			'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	},

	[rpcSpec.methods[3].name]: {
		address:
			'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
		size: 1,
		cursor: 'xyz',
	},

	[rpcSpec.methods[4].name]: {
		address:
			'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	},

	[rpcSpec.methods[5].name]: {
		address:
			'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
	},

	[rpcSpec.methods[6].name]: {
		txID:
			'deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef',
	},

	[rpcSpec.methods[7].name]: {
		txID:
			'deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef',
	},

	[rpcSpec.methods[8].name]: {
		size: 1,
		cursor: 'xyz',
	},

	[rpcSpec.methods[9].name]: {
		validatorAddress:
			'tv1qdqft0u899axwce955fkh9rundr5s2sgvhpp8wzfe3ty0rn0rgqj2x6y86p',
	},

	[rpcSpec.methods[10].name]: {},
	[rpcSpec.methods[11].name]: {},
	[rpcSpec.methods[12].name]: {},

	[rpcSpec.methods[13].name]: {
		actions: [
			{
				amount: '100000000000000000',
				from:
					'brx1qsphund3df3xmycqr9fud8tyvspru95tytezy0ke2pk0gpjukjltjscyn03ah',
				to:
					'brx1qsppypnmrwl95h70cx0zm09lgf8f047r5j9hxqgre92lf53kzq07h0gz9a4hy',
				rri: 'xrd_tr1qyf0x76s',
				type: 'TokenTransfer',
			},
		],
		feePayer:
			'rdx1qsps28kdn4epn0c9ej2rcmwfz5a4jdhq2ez03x7h6jefvr4fnwnrtqqjaj7dt',
		message: 'xyz',
	},

	[rpcSpec.methods[14].name]: {
		transaction: {
			blob: 'xyz',
		},
		signatureDER: 'xyz',
		publicKeyOfSigner: 'xyz',
	},

	[rpcSpec.methods[15].name]: {
		transaction: {
			blob: 'xyz',
		},
		signatureDER: 'xyz',
		publicKeyOfSigner: 'xyz',
		txID:
			'deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef',
	},
}

const expectedDecodedResponses = {
	[rpcSpec.methods[0].name]: (
		response: NativeTokenEndpoint.Response,
	): NativeTokenEndpoint.DecodedResponse => tokenInfoFromResponse(response),

	[rpcSpec.methods[1].name]: (
		response: TokenInfoEndpoint.Response,
	): TokenInfoEndpoint.DecodedResponse => tokenInfoFromResponse(response),

	[rpcSpec.methods[2].name]: (
		response: TokenBalancesEndpoint.Response,
	): TokenBalancesEndpoint.DecodedResponse => ({
		owner: AccountAddress.fromUnsafe(response.owner)._unsafeUnwrap(),
		tokenBalances: [
			{
				tokenIdentifier: ResourceIdentifier.fromUnsafe(
					response.tokenBalances[0].rri,
				)._unsafeUnwrap({ withStackTrace: true }),
				amount: Amount.fromUnsafe(
					response.tokenBalances[0].amount,
				)._unsafeUnwrap(),
			},
		],
	}),

	[rpcSpec.methods[3].name]: (
		response: TransactionHistoryEndpoint.Response,
	): TransactionHistoryEndpoint.DecodedResponse => {
		const txID = TransactionIdentifier.create(
			response.transactions[0].txID,
		)._unsafeUnwrap({ withStackTrace: true })

		return {
			cursor: response.cursor,
			transactions: [
				{
					txID,
					sentAt: new Date(response.transactions[0].sentAt),
					fee: Amount.fromUnsafe(
						response.transactions[0].fee,
					)._unsafeUnwrap({ withStackTrace: true }),
					message: 'Example message',
					actions: response.transactions[0].actions.map(raw =>
						executedActionFromRaw(raw),
					),
				},
			],
		}
	},

	[rpcSpec.methods[4].name]: (
		response: StakePositionsEndpoint.Response,
	): StakePositionsEndpoint.DecodedResponse => [
		{
			validator: ValidatorAddress.fromUnsafe(
				response[0].validator,
			)._unsafeUnwrap({ withStackTrace: true }),
			amount: Amount.fromUnsafe(response[0].amount)._unsafeUnwrap({
				withStackTrace: true,
			}),
		},
	],

	[rpcSpec.methods[5].name]: (
		response: UnstakePositionsEndpoint.Response,
	): UnstakePositionsEndpoint.DecodedResponse => [
		{
			amount: Amount.fromUnsafe(response[0].amount)._unsafeUnwrap({
				withStackTrace: true,
			}),
			validator: ValidatorAddress.fromUnsafe(
				response[0].validator,
			)._unsafeUnwrap({ withStackTrace: true }),
			epochsUntil: response[0].epochsUntil,
			withdrawTxID: TransactionIdentifier.create(
				response[0].withdrawTxID,
			)._unsafeUnwrap({ withStackTrace: true }),
		},
	],

	[rpcSpec.methods[6].name]: (
		response: LookupTransactionEndpoint.Response,
	): LookupTransactionEndpoint.DecodedResponse => {
		const txID = TransactionIdentifier.create(response.txID)._unsafeUnwrap({
			withStackTrace: true,
		})

		return {
			txID,
			sentAt: new Date(response.sentAt),
			fee: Amount.fromUnsafe(response.fee)._unsafeUnwrap({
				withStackTrace: true,
			}),
			message: 'Example message',
			actions: response.actions.map(action =>
				executedActionFromRaw(action),
			),
		}
	},

	[rpcSpec.methods[7].name]: (
		response: TransactionStatusEndpoint.Response,
	): TransactionStatusEndpoint.DecodedResponse => ({
		txID: TransactionIdentifier.create(response.txID)._unsafeUnwrap({
			withStackTrace: true,
		}),
		status: response.status,
	}),

	[rpcSpec.methods[8].name]: (
		response: ValidatorsEndpoint.Response,
	): ValidatorsEndpoint.DecodedResponse => ({
		cursor: response.cursor,
		validators: [
			{
				address: ValidatorAddress.fromUnsafe(
					response.validators[0].address,
				)._unsafeUnwrap({ withStackTrace: true }),
				ownerAddress: AccountAddress.fromUnsafe(
					response.validators[0].ownerAddress,
				)._unsafeUnwrap({ withStackTrace: true }),
				name: response.validators[0].name,
				infoURL: new URL(response.validators[0].infoURL),
				totalDelegatedStake: Amount.fromUnsafe(
					response.validators[0].totalDelegatedStake,
				)._unsafeUnwrap({ withStackTrace: true }),
				validatorFee: response.validators[0].validatorFee,
				uptimePercentage: response.validators[0].uptimePercentage,
				proposalsCompleted: response.validators[0].proposalsCompleted,
				proposalsMissed: response.validators[0].proposalsMissed,
				registered: response.validators[0].registered,
				ownerDelegation: Amount.fromUnsafe(
					response.validators[0].ownerDelegation,
				)._unsafeUnwrap({ withStackTrace: true }),
				isExternalStakeAccepted:
					response.validators[0].isExternalStakeAccepted,
			},
		],
	}),

	[rpcSpec.methods[9].name]: (
		response: LookupValidatorEndpoint.Response,
	): LookupValidatorEndpoint.DecodedResponse => ({
		address: ValidatorAddress.fromUnsafe(response.address)._unsafeUnwrap({
			withStackTrace: true,
		}),
		ownerAddress: AccountAddress.fromUnsafe(
			response.ownerAddress,
		)._unsafeUnwrap({
			withStackTrace: true,
		}),
		name: response.name,
		infoURL: new URL(response.infoURL),
		totalDelegatedStake: Amount.fromUnsafe(
			response.totalDelegatedStake,
		)._unsafeUnwrap({ withStackTrace: true }),
		ownerDelegation: Amount.fromUnsafe(
			response.ownerDelegation,
		)._unsafeUnwrap({ withStackTrace: true }),
		validatorFee: response.validatorFee,
		uptimePercentage: response.uptimePercentage,
		proposalsCompleted: response.proposalsCompleted,
		proposalsMissed: response.proposalsMissed,
		registered: response.registered,
		isExternalStakeAccepted: response.isExternalStakeAccepted,
	}),

	[rpcSpec.methods[10].name]: (
		response: NetworkIdEndpoint.Response,
	): NetworkIdEndpoint.DecodedResponse => ({
		networkId: Network.MAINNET,
	}),

	[rpcSpec.methods[11].name]: (
		response: NetworkTransactionThroughputEndpoint.Response,
	): NetworkTransactionThroughputEndpoint.DecodedResponse => ({
		tps: response.tps,
	}),

	[rpcSpec.methods[12].name]: (
		response: NetworkTransactionDemandEndpoint.Response,
	): NetworkTransactionDemandEndpoint.DecodedResponse => ({
		tps: response.tps,
	}),

	[rpcSpec.methods[13].name]: (
		response: BuildTransactionEndpoint.Response,
	): BuildTransactionEndpoint.DecodedResponse => ({
		transaction: {
			blob: response.transaction.blob,
			hashOfBlobToSign: response.transaction.hashOfBlobToSign,
		},
		fee: Amount.fromUnsafe(response.fee)._unsafeUnwrap({
			withStackTrace: true,
		}),
	}),

	[rpcSpec.methods[14].name]: (
		response: FinalizeTransactionEndpoint.Response,
	): FinalizeTransactionEndpoint.DecodedResponse => ({
		txID: TransactionIdentifier.create(response.txID)._unsafeUnwrap({
			withStackTrace: true,
		}),
		blob: '',
	}),

	[rpcSpec.methods[15].name]: (
		response: SubmitTransactionEndpoint.Response,
	): SubmitTransactionEndpoint.DecodedResponse => ({
		txID: TransactionIdentifier.create(response.txID)._unsafeUnwrap({
			withStackTrace: true,
		}),
	}),
}

const client = nodeAPI(new URL('http://xyz'))

const testRpcMethod = (method: MethodObject, index: number) => {
	it(`should decode ${method.name} response`, async () => {
		const mockedResult = method.examples
			? (method.examples[0] as any).result.value
			: faker.generate((method.result as ContentDescriptorObject).schema)

		mockClientReturnValue = mockedResult

		const expected = expectedDecodedResponses[method.name](mockedResult)

		// @ts-ignore
		const result = await client[method.name](
			// @ts-ignore
			methodParams[method.name],
		)

		if (result.isErr()) {
			throw result.error
		}

		const response = result.value

		const checkEquality = (
			obj1: Record<string, any>,
			obj2: Record<string, any>,
		) => {
			if (obj1.equals) {
				if (!obj2.equals)
					throw Error(`Type mismatch when checking for equality.`)
				expect(obj1.equals(obj2)).toEqual(true)
			} else {
				for (const key in obj1) {
					const value1 = obj1[key]
					const value2 = obj2[key]

					isObject(value1)
						? checkEquality(value1, value2)
						: isArray(value1)
						? value1.forEach((item, i) =>
								checkEquality(item as any, value2[i]),
						  )
						: expect(value1).toEqual(value2)
				}
			}
		}

		checkEquality(expected, response)
	})
}

describe.skip('json-rpc spec', () => {
	rpcSpec.methods
		.filter(method =>
			Object.values(ApiMethod).includes(method.name as ApiMethod),
		)
		.forEach((method, i) => testRpcMethod(method, i))
})
*/
