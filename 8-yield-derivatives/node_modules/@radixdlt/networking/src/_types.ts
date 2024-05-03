import { ResultAsync } from 'neverthrow'
import { OpenApiClientCall } from './open-api-client'
import { OpenRPCClientCall } from './open-rpc-client'

type TransportType = 'json-rpc' | 'open-api'

export type Call<Methods, Params, Return> = <Methods, Params, Return>(
	method: Methods,
	param: Params,
	headers?: Record<string, string>,
) => ResultAsync<Return, Error>

export type Transport<T extends TransportType> = {
	type: T
	call: T extends 'open-api' ? OpenApiClientCall : OpenRPCClientCall
}

export type Client<T extends TransportType> = (url: URL) => Transport<T>
