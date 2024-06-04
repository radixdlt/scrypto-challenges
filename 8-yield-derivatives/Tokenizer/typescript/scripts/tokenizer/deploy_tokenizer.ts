import { ResultAsync } from 'neverthrow'
import { deployPackage, loadBinaryFromPath, logger } from '../../helpers'
import { radixEngineClient } from '../../config'

import * as fs from 'fs';

const instantiateTokenizerdApp = (sugarOraclePackage: string, tokenSymbol: string) =>
  radixEngineClient
    .getManifestBuilder()
    .andThen(
      ({ wellKnownAddresses, convertStringManifest, submitTransaction }) =>
        convertStringManifest(`
        CALL_METHOD
            Address("${wellKnownAddresses.accountAddress}")
            "lock_fee"
            Decimal("10")
        ;
        CALL_FUNCTION
            Address("${sugarOraclePackage}")
            "Tokenizer"
            "instantiate"
            Decimal("5")
            "${tokenSymbol}"
            "timebased"     
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Address("resource_tdx_2_1t57ejuayfdyrzn6wvzdw0u9lh5ae3u72c4pcxwmvvuf47q6jzk4xv2")   
            ;
        CALL_METHOD
            Address("${wellKnownAddresses.accountAddress}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP")
        ;
        `)
          .andThen(submitTransaction)
          .andThen(({ txId }) =>
            radixEngineClient.gatewayClient
              .pollTransactionStatus(txId)
              .map(() => txId)
          )
          .andThen((txId) =>
            radixEngineClient.gatewayClient
              .getCommittedDetails(txId)
              .map((res) => {
                const entities = res.createdEntities;
                const entityMap: Record<string, string> = {}; 

                entities.forEach((entity, index) => {
                    entityMap[predefinedKeys[index]] = entity.entity_address;
                });

                writeToPropertyFile(entityMap,"entities.properties");
                return entityMap;
            })
          )
    )

ResultAsync.combine([
  loadBinaryFromPath('/scrypto/target/wasm32-unknown-unknown/release/tokenizer.wasm'),
  loadBinaryFromPath('/scrypto/target/wasm32-unknown-unknown/release/tokenizer.rpd'),
])
  .andThen(([wasmBuffer, rpdBuffer]) =>
    deployPackage({ wasmBuffer, rpdBuffer, lockFee: 200 })
  )
  .andThen((result) => {
    logger.info('Deployed package', result)
    return instantiateTokenizerdApp(result.packageAddress, "TKN")
  })
  .mapErr((error) => {
    logger.error(error)
  })

  // Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
  // Address("resource_tdx_2_1th9fqs7mfkrsgyc2344hz9z5n47r79v7wxuwyj9mq64wjv3ym6d578")  

  // // Example usage:
  const predefinedKeys = [
    "VITE_COMP_ADDRESS",
    "VITE_OWNER_BADGE",
    "VITE_ADMIN_BADGE",
    "VITE_TOKENIZER_TOKEN_ADDRESS",
    "VITE_USERDATA_NFT_RESOURCE_ADDRESS",
    "VITE_PT_RESOURCE_ADDRESS",
    "VITE_STAFF_BADGE"
    ];


  const writeToPropertyFile = (entityMap: Record<string, string>, fileName: string) => {
    const lines: string[] = [];

    for (const key in entityMap) {
        if (entityMap.hasOwnProperty(key)) {
            const line = `${key}=${entityMap[key]}`;
            lines.push(line);
        }
    }

    try {
        fs.writeFileSync(fileName, lines.join('\n'));
        console.log(`Property file written to ${fileName}`);
    } catch (error) {
        console.error(`Error writing property file: ${error}`);
    }
};
