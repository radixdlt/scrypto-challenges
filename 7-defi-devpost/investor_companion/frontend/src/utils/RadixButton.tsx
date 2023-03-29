import { DappMetadata, RadixDappToolkit, State,RadixDappToolkitConfiguration, RequestData, SendTransaction } from "@radixdlt/radix-dapp-toolkit";

type RadixDappToolkit = (
  dAppMetadata: DappMetadata,
  onConnect?: (requestData: RequestData) => void,
  configuration?: RadixDappToolkitConfiguration
) => {
  requestData: RequestData
  sendTransaction: SendTransaction
  state$: State
  destroy: () => void
}

const rdt = RadixDappToolkit(
  {
    dAppDefinitionAddress:
      "account_tdx_22_1pz7vywgwz4fq6e4v3aeeu8huamq0ctmsmzltay07vzpqm82mp5",
    dAppName: "Investor Companion",
  },
  (requestData) => {
    requestData({
      accounts: { quantifier: "atLeast", quantity: 1 },
    }).map(({ data: { accounts } }) => {
      // set your application state
    });
  },
  {
    networkId: 34,
    onDisconnect: () => {
      // clear your application state
    },
    onInit: ({ accounts }) => {
      // set your initial application state
    },
  }
);

export default rdt;
