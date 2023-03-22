import { RadixDappToolkit } from "@radixdlt/radix-dapp-toolkit";

const rdt = RadixDappToolkit(
  {
    dAppDefinitionAddress:
      "account_tdx_22_1pz7vywgwz4fq6e4v3aeeu8huamq0ctmsmzltay07vzpqm82mp5",
    dAppName: "Name of your dApp",
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
