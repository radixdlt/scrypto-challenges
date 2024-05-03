import {
	RequestManager,
	Client as OpenRPCClient,
	HTTPTransport,
} from '@open-rpc/client-js'
import { Transport, Client } from './_types'
import { isArray, log } from '@radixdlt/util'
import { validate } from 'open-rpc-utils'
import { v4 as uuid } from 'uuid'
const spec = require('@radixdlt/open-rpc-spec')

const validateMethod = validate.bind(null, spec)

const defaultHeaders = ['X-Radixdlt-Method', 'X-Radixdlt-Correlation-Id']

enum Endpoint {
	NETWORK_ID = 'network.get_id',
	TOKEN_BALANCES = 'account.get_balances',
	TRANSACTION_HISTORY = 'account.get_transaction_history',
	STAKES = 'account.get_stake_positions',
	UNSTAKES = 'account.get_unstake_positions',
	TX_STATUS = 'transactions.get_transaction_status',
	NETWORK_TX_THROUGHPUT = 'network.get_throughput',
	NETWORK_TX_DEMAND = 'network.get_demand',
	VALIDATORS = 'validators.get_next_epoch_set',
	LOOKUP_TX = 'transactions.lookup_transaction',
	LOOKUP_VALIDATOR = 'validators.lookup_validator',
	NATIVE_TOKEN = 'tokens.get_native_token',
	TOKEN_INFO = 'tokens.get_info',
	BUILD_TX_FROM_INTENT = 'construction.build_transaction',
	SUBMIT_TX = 'construction.submit_transaction',
	FINALIZE_TX = 'construction.finalize_transaction',
}

enum MethodLocation {
	ARCHIVE = 'archive',
	CONSTRUCTION = 'construction',
}

const MethodEndpoints = {
	[Endpoint.NETWORK_ID]: MethodLocation.ARCHIVE,
	[Endpoint.TOKEN_BALANCES]: MethodLocation.ARCHIVE,
	[Endpoint.TRANSACTION_HISTORY]: MethodLocation.ARCHIVE,
	[Endpoint.STAKES]: MethodLocation.ARCHIVE,
	[Endpoint.UNSTAKES]: MethodLocation.ARCHIVE,
	[Endpoint.TX_STATUS]: MethodLocation.ARCHIVE,
	[Endpoint.NETWORK_TX_THROUGHPUT]: MethodLocation.ARCHIVE,
	[Endpoint.NETWORK_TX_DEMAND]: MethodLocation.ARCHIVE,
	[Endpoint.VALIDATORS]: MethodLocation.ARCHIVE,
	[Endpoint.LOOKUP_TX]: MethodLocation.ARCHIVE,
	[Endpoint.LOOKUP_VALIDATOR]: MethodLocation.ARCHIVE,
	[Endpoint.NATIVE_TOKEN]: MethodLocation.ARCHIVE,
	[Endpoint.TOKEN_INFO]: MethodLocation.ARCHIVE,
	[Endpoint.BUILD_TX_FROM_INTENT]: MethodLocation.CONSTRUCTION,
	[Endpoint.SUBMIT_TX]: MethodLocation.CONSTRUCTION,
	[Endpoint.FINALIZE_TX]: MethodLocation.CONSTRUCTION,
}

const correlationID = uuid()

export type OpenRPCClientCall = (
	endpoint: string,
	params: unknown[] | Record<string, unknown>,
	headers?: Record<string, string>,
) => Promise<unknown>

export const RPCClient: Client<'json-rpc'> = (url: URL) => {
	const call = async (
		method: string,
		params: unknown[] | Record<string, unknown>,
		headers?: Record<string, string>,
	): Promise<unknown> => {
		// @ts-ignore
		const endpoint = `${url.toString()}${MethodEndpoints[method]}`

		const transport = new HTTPTransport(endpoint, {
			headers: {
				[defaultHeaders[0]]: method,
				[defaultHeaders[1]]: correlationID,
				...headers,
			},
		})

		const requestManager = new RequestManager([transport])
		const client = new OpenRPCClient(requestManager)

		const filteredParams = isArray(params)
			? params.filter(item => !!item)
			: params

		log.info(
			`Sending RPC request with method ${method}. ${JSON.stringify(
				filteredParams,
				null,
				2,
			)}`,
		)

		const result = await validateMethod(method, filteredParams)
		result.mapErr(err => {
			// need to disable this until rpc spec is fixed with the latest addresses and RRI's
			//	throw err
		})

		/*
		console.log(
			`calling ${method} at ${endpoint} with: ${JSON.stringify(
				filteredParams,
				null,
				2,
			)}`,
		)*/

		const response:
			| Record<string, unknown>
			| unknown[] = await client.request({
			method: method,
			params: filteredParams,
		})

		log.info(
			`Response from ${method} call: ${JSON.stringify(
				response,
				null,
				2,
			)}`,
		)

		//console.log(`response for ${method} at ${endpoint}`, JSON.stringify(response, null, 2))
		// TODO validate response

		return response
	}

	return {
		type: 'json-rpc',
		call,
	}
}
