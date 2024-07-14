import { GatewayApiClient as BabylonGatewayApiClient } from '@radixdlt/babylon-gateway-api-sdk';
import { ResultAsync } from 'neverthrow';

import { errorIdentity } from '../helpers/error-identity';

export type GatewayApiClient = ReturnType<typeof createGatewayApiClient>;

export const createGatewayApiClient = ({ basePath }: { basePath: string; dAppDefinitionAddress?: string }) => {
  const { transaction, state, status } = BabylonGatewayApiClient.initialize({
    basePath,
    applicationName: 'SRWA Yield Derivatives',
  });

  const getTransactionStatus = (transactionIntentHashHex: string) =>
    ResultAsync.fromPromise(transaction.getStatus(transactionIntentHashHex), errorIdentity);

  const getTransactionDetails = (transactionIntentHashHex: string) =>
    ResultAsync.fromPromise(transaction.getCommittedDetails(transactionIntentHashHex), errorIdentity);

  const getEntityDetails = (address: string) =>
    ResultAsync.fromPromise(state.getEntityDetailsVaultAggregated(address), errorIdentity);

  const getEntitiesDetails = (addresses: string[]) =>
    ResultAsync.fromPromise(state.getEntityDetailsVaultAggregated(addresses), errorIdentity);

  const getEntityNonFungibleIds = ({
    accountAddress,
    nftAddress,
    vaultAddress,
  }: {
    accountAddress: string;
    nftAddress: string;
    vaultAddress: string;
  }) =>
    ResultAsync.fromPromise(
      state.innerClient.entityNonFungibleIdsPage({
        stateEntityNonFungibleIdsPageRequest: {
          address: accountAddress,
          vault_address: vaultAddress,
          resource_address: nftAddress,
        },
      }),
      errorIdentity,
    );

  const getNetworkConfiguration = () => ResultAsync.fromPromise(status.getNetworkConfiguration(), errorIdentity);

  return {
    getTransactionStatus,
    getTransactionDetails,
    getEntityDetails,
    getEntitiesDetails,
    getEntityNonFungibleIds,
    getNetworkConfiguration,
    transactionApi: transaction,
    stateApi: state,
    statusApi: status,
  };
};
