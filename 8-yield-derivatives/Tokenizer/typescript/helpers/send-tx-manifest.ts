import { radixEngineClient } from '../config'
import { logger } from './logger'

export const sendTransactionManifest = (txManifest: string, lock_fee = 100) => {
  return radixEngineClient
    .getManifestBuilder()
    .andThen(({ wellKnownAddresses, convertStringManifest }) => {
      logger.debug(txManifest)
      return convertStringManifest(`
          CALL_METHOD
              Address("${wellKnownAddresses.accountAddress}")
              "lock_fee"
              Decimal("${lock_fee}")
          ;
          
          ${txManifest}
    `)
        .andThen(radixEngineClient.submitTransaction)
        .andThen(({ txId }) =>
          radixEngineClient.gatewayClient
            .pollTransactionStatus(txId)
            .map(() => txId)
        )
    })
}
