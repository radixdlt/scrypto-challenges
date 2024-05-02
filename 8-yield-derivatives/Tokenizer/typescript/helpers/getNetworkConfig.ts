import { RadixNetworkConfig } from '@radixdlt/radix-dapp-toolkit'

export const getNetworkConfig = (
  networkName: keyof typeof RadixNetworkConfig
) => {
  const network = RadixNetworkConfig[networkName]
  if (!network) throw new Error(`Invalid network: ${networkName}`)
  return network
}
