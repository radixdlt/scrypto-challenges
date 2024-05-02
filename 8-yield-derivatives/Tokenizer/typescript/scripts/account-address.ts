import { radixEngineClient } from '../config'
import { logger } from '../helpers'

radixEngineClient
  .getAccountAddress()
  .map((address) =>
    logger.debug({
      address: address,
      url: `${radixEngineClient.gatewayClient.networkConfig.dashboardUrl}/account/${address}`,
    })
  )
  .mapErr((err) => logger.error(err))
