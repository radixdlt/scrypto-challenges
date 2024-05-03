import { ProgrammaticScryptoSborValue } from '@radixdlt/babylon-gateway-api-sdk';
import { StateEntityDetailsResponseItem } from '@radixdlt/babylon-gateway-api-sdk/dist/generated/models/StateEntityDetailsResponseItem';
import { SendTransaction } from '@radixdlt/radix-dapp-toolkit';
import { toast } from 'react-toastify';

import { findUserBadgeAddress, setInitialAssets, updateAssetsFields } from './helpers';
// import {AssetsItemModel} from '../app/shared/models/index.model';
import { stateApi } from '../helpers';
import { getTransactionStatus } from '../helpers/get-transaction-status';
import { depositAndCreateUserManifest, depositAssetManifest, withdrawAssetManifest } from '../manifests';
import { handleErrorManifest } from '../manifests/helpers';
import { useBearStore } from '../store';

const fetchUserBadge = async (accountAddress: string) => {
  const data = await stateApi.entityNonFungiblesPage({
    stateEntityNonFungiblesPageRequest: {
      address: accountAddress,
    },
  });

  useBearStore.setState({ userBadge: data.items.length > 0 ? await findUserBadgeAddress(data) : undefined });
};

const depositAndCreateUser = async (
  sendTx: SendTransaction,
  tokenAmount: number,
  address: string,
  tokenAddress: string,
  componentAddress: string,
) => {
  const result = await sendTx(depositAndCreateUserManifest(tokenAmount, address, tokenAddress, componentAddress));

  handleErrorManifest(result);

  if (result.value) {
    const status = await getTransactionStatus(result.value.transactionIntentHash);

    if (status.status === 'CommittedSuccess') {
      toast.success('Transaction successful.');
    }

    return {
      result,
    };
  }
};

const depositAsset = async (
  sendTx: SendTransaction,
  tokenAmount: number,
  userBadge: string | null,
  address: string,
  tokenAddress: string,
  componentAddress: string,
) => {
  const result = await sendTx(depositAssetManifest(tokenAmount, userBadge, address, tokenAddress, componentAddress));

  handleErrorManifest(result);

  if (result.value) {
    const status = await getTransactionStatus(result.value.transactionIntentHash);

    if (status.status === 'CommittedSuccess') {
      toast.success('Transaction successful.');
    }

    return {
      result,
    };
  }
};

const withdrawAsset = async (
  sendTx: SendTransaction,
  userBadge: string | undefined,
  address: string,
  tokenAddress: string,
  componentAddress: string,
) => {
  const result = await sendTx(withdrawAssetManifest(userBadge, address, tokenAddress, componentAddress));

  handleErrorManifest(result);

  if (result.value) {
    const status = await getTransactionStatus(result.value.transactionIntentHash);

    if (status.status === 'CommittedSuccess') {
      toast.success('Transaction successful.');
    }

    return {
      result,
    };
  }
};

const fetchAssets = async (address: string, addressType: string) => {
  const fungiblesData = await stateApi.entityFungiblesPage({
    stateEntityFungiblesPageRequest: {
      address: address,
    },
  });

  const resourceAddresses = fungiblesData.items.map((item) => item.resource_address);

  if (resourceAddresses.length === 0) {
    if (addressType === 'account') {
      useBearStore.setState({ accountAssets: [] });
    }
    return [];
  }

  const entityData: StateEntityDetailsResponseItem = { items: [] };

  const chunkSize: number = 20;
  const addressChunks: string[][] = [];
  for (let i = 0; i < resourceAddresses.length; i += chunkSize) {
    addressChunks.push(resourceAddresses.slice(i, i + chunkSize));
  }

  for (const chunk of addressChunks) {
    const data = await stateApi.stateEntityDetails({
      stateEntityDetailsRequest: {
        addresses: chunk,
      },
    });
    entityData.items.push(...data.items);
  }

  if (addressType === 'component') {
    useBearStore.setState({ yieldAssets: setInitialAssets(entityData, fungiblesData) });
  } else {
    const accountAssets = setInitialAssets(entityData, fungiblesData);
    const yieldAssets = useBearStore.getState().yieldAssets;
    const filteredAccountAssets = accountAssets.filter((item) =>
      yieldAssets.some((subItem) => subItem.address === item.address),
    );

    useBearStore.setState({ accountAssets: filteredAccountAssets });
  }
};

const fetchComponentAssets = async (componentAddress: string) => {
  if (!componentAddress) {
    return;
  }

  const accountAssets = useBearStore.getState().accountAssets;
  const yieldAssets = useBearStore.getState().yieldAssets;

  const data = await stateApi.stateEntityDetails({
    stateEntityDetailsRequest: {
      addresses: [componentAddress],
    },
  });

  // @ts-expect-error: Property 'state' does not exist on type 'StateEntityDetailsResponseItemDetails'.
  const fields = data?.items[0]?.details?.state?.fields;

  const principalTokensSymbolsField = fields.find((field) => field.field_name === 'principal_tokens_symbols');
  const addressesPrincipalTokensSymbolsField =
    principalTokensSymbolsField?.entries.map((entry) => entry.key.value) || [];
  const filteredAssets = yieldAssets.filter((resource) =>
    addressesPrincipalTokensSymbolsField.includes(resource.address),
  );

  // COMPONENT ASSETS
  const updatedPrincipalTotalBalances = updateAssetsFields(filteredAssets, fields, 'total_balances');
  const updatedPrincipalYieldRates = updateAssetsFields(updatedPrincipalTotalBalances, fields, 'yield_rates');
  useBearStore.setState({ componentAssets: updatedPrincipalYieldRates });

  // ACCOUNT ASSETS
  const userBadge = useBearStore.getState().userBadge;
  const usersField = fields.find((field: ProgrammaticScryptoSborValue) => field.field_name === 'users');
  const connectedUser = usersField.entries.find((user: StateEntityDetailsResponseItem) => user.key.value === userBadge);

  if (connectedUser) {
    const principalTokensDepositBalancesField = connectedUser.value.fields.find(
      (field) => field.field_name === 'deposit_balances',
    );

    const mergedArray = accountAssets.map((item) => {
      const additionalFieldsItem = principalTokensDepositBalancesField.entries.find(
        (field) => field.key.value === item.address,
      );
      if (additionalFieldsItem) {
        const fields = additionalFieldsItem.value.fields;
        const principalBalance = fields.find((field) => field.field_name === 'principal_balance');
        const yieldBalance = fields.find((field) => field.field_name === 'yield_balance');
        const depositedAt = fields.find((field) => field.field_name === 'deposited_at');
        return {
          ...item,
          principal_balance: Number(principalBalance?.value) || 0,
          yield_balance: Number(yieldBalance?.value) || 0,
          deposited_at: Number(depositedAt?.value) || 0,
        };
      } else {
        return item;
      }
    });

    useBearStore.setState({ accountAssets: mergedArray });
  } else {
    useBearStore.setState({ accountAssets: accountAssets });
  }

  // YIELD ASSETS
  const yieldTokensField = fields.find((field) => field.field_name === 'yield_tokens');
  const addressesYieldTokensField = yieldTokensField?.entries.map((entry) => entry.value.value) || [];
  const filteredYieldAssets = yieldAssets.filter((resource) => addressesYieldTokensField.includes(resource.address));
  const updatedYieldTotalBalances = updateAssetsFields(filteredYieldAssets, fields, 'total_balances');

  useBearStore.setState({ yieldAssets: updatedYieldTotalBalances });
};

const featuresService = {
  fetchUserBadge,
  depositAndCreateUser,
  depositAsset,
  withdrawAsset,
  fetchAssets,
  fetchComponentAssets,
};

export default featuresService;
