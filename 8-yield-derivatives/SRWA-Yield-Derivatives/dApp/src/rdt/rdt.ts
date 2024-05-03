import { RadixNetwork, RadixNetworkConfigById } from '@radixdlt/babylon-gateway-api-sdk';
import { DataRequestBuilder, RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
import { BehaviorSubject } from 'rxjs';

import { setAccounts } from '../account/state';
import { addEntities } from '../entity/state';
import { createGatewayApiClient } from '../gateway/gateway-api';
import { createChallenge } from '../helpers/create-challenge';
import { bootstrapNetwork, networkId as networkIdSubject } from '../network/state';
import { useBearStore } from '../store';

const networkId = networkIdSubject.value;

const getDAppDefinitionFromLocalStorage = (): Record<string, string> => {
  try {
    const raw = localStorage.getItem('dAppDefinitionAddress');
    if (!raw) {
      return {
        [RadixNetwork.Stokenet]: 'account_tdx_2_1283kamtx7vt6n26nwaupzflsy2ee0hud9nhy37zqwcruy4vfzs0yts',
      };
    }

    return JSON.parse(raw);
  } catch (error) {
    return {};
  }
};

const getDAppDefinitionAddressDefault = () => getDAppDefinitionFromLocalStorage()[networkId] || '';

export const dAppDefinitionAddress = new BehaviorSubject<string>(getDAppDefinitionAddressDefault());

bootstrapNetwork(networkId);

export const gatewayApi = createGatewayApiClient({
  basePath: RadixNetworkConfigById[networkId].gatewayUrl,
  dAppDefinitionAddress: dAppDefinitionAddress.value,
});

const options = {
  dAppDefinitionAddress: dAppDefinitionAddress.value,
  networkId,
  applicationName: 'SRWA Yield Derivatives',
  useCache: false,
};

export const rdt = RadixDappToolkit(options);

rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));

rdt.walletApi.walletData$.subscribe(async (state: any) => {
  setAccounts(state.accounts);
  useBearStore.setState({ accountAddress: state.accounts[0]?.address });
  useBearStore.setState({ accountAssets: [] });
  if (state.persona) {
    addEntities([
      {
        address: state.persona?.identityAddress,
        type: 'identity',
      },
    ]);
  }
});

rdt.walletApi.provideChallengeGenerator(async () => createChallenge());

rdt.walletApi.setRequestData(
  DataRequestBuilder.config({
    personaData: { fullName: true },
    accounts: { numberOfAccounts: { quantifier: 'atLeast', quantity: 1 } },
  }),
);
