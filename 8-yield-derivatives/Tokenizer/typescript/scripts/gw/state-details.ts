import { logger } from '../../helpers'
import { radixEngineClient } from '../../config'

export const exec = (address: string) => {
  logger.debug(radixEngineClient.gatewayClient.networkConfig)

  radixEngineClient.gatewayClient
    .getState([address])
    .map((res) => logger.debug(res))
    .mapErr((err) => logger.error(JSON.stringify(err, null, 2)))
}

exec(process.argv[2])
