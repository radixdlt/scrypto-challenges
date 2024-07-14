import { RadixNetwork, RadixNetworkConfig } from '@radixdlt/babylon-gateway-api-sdk';

const networkId = RadixNetworkConfig?.['Stokenet']?.networkId;

export const DEFAULT_NETWORK_ID = networkId ? String(networkId) : RadixNetwork.Stokenet.toString();
