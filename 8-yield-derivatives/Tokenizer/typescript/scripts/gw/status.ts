import { logger } from '../../helpers'
import { radixEngineClient } from '../../config'

export const exec = () => {
  radixEngineClient.gatewayClient
    .getStatus()
    .map((res) => logger.debug(res))
    .mapErr((err) => logger.error(JSON.stringify(err, null, 2)))
}

exec()
