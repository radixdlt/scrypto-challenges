import { openApiClient } from '@radixdlt/networking'
import { getAPI } from './open-api/interface'

export const nodeAPI = (url: URL) => ({
	...getAPI(openApiClient(url).call),
})
