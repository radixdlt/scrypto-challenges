import { GatewayApiClient, RadixNetworkConfigById } from '@radixdlt/babylon-gateway-api-sdk';
import { BehaviorSubject } from 'rxjs';

import { DEFAULT_NETWORK_ID } from '../helpers/get-network-id';

export const bootstrapNetwork = (networkId: number) => {
  const gatewayApi = GatewayApiClient.initialize({
    basePath: RadixNetworkConfigById[networkId].gatewayUrl,
    applicationName: 'SRWA Yield Derivatives',
  });
  gatewayApi.status.getNetworkConfiguration().then((response: any) => {
    return xrdAddress.next(response.well_known_addresses.xrd);
  });
};

const xrdAddress = new BehaviorSubject<string | undefined>(undefined);

const getNetworkIdDefault = () => {
  const urlParams = new URLSearchParams(window.location.search);
  return parseInt(urlParams.get('networkId') || localStorage.getItem('networkId') || DEFAULT_NETWORK_ID, 10);
};

export const networkId = new BehaviorSubject<number>(getNetworkIdDefault());
