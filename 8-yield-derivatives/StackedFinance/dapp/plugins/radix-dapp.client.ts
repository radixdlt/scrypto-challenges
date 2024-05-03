
import { RadixDappToolkit, RadixNetwork } from '@radixdlt/radix-dapp-toolkit'
import data from '../data/validators.json'

const rdt = RadixDappToolkit({
  dAppDefinitionAddress:
    'account_tdx_2_1292eqwuzwcrlfe6hfxylu8hg46zaq94qmes7sz23xyn7e2kstphfue',
  networkId: RadixNetwork.Stokenet,
  //applicationName: 'Radix Web3 dApp',
  //applicationVersion: '1.0.0',
})

const getRDT = async () => {
  /*
  rdt.walletApi.sendTransaction({
    transactionManifest: claimManifest,
  });
  */
 return rdt;
}

const getWallet = async () => {
  console.log(rdt.walletApi)
 return rdt.walletApi.getWalletData();
}

const getValidators = async () => {
  const validators = await rdt.gatewayApi.state.getAllValidators(undefined)
  console.log(validators);
 return data;
}

const getEntityDetails = async (address: string) => {
  const details = await rdt.gatewayApi.state.getEntityDetailsVaultAggregated(address)
 return details;
}

const getLSUBalance = async (address: string) => {
  const account  = await rdt.walletApi.getWalletData().accounts[0].address;
  const result = await rdt.gatewayApi.state.innerClient.entityFungibleResourceVaultPage(
    {
      stateEntityFungibleResourceVaultsPageRequest: {
        address: account,
        // eslint-disable-next-line camelcase
        resource_address: address,
      },
    }
  );
  console.log(result.items.length);
 return (result.items.length>0) ? result.items[0].amount : 0;
}

const getXRDBalance = async () => {
  const account  = await rdt.walletApi.getWalletData().accounts[0].address;
  const result = await rdt.gatewayApi.state.innerClient.entityFungibleResourceVaultPage(
    {
      stateEntityFungibleResourceVaultsPageRequest: {
        address: account,
        // eslint-disable-next-line camelcase
        resource_address: "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
      },
    }
  );
  console.log(result.items.length);
 return (result.items.length>0) ? result.items[0].amount : 0;
}

/*
export default defineNuxtPlugin((nuxtApp) => {inject('rdt', rdt);})
*/

export default defineNuxtPlugin((nuxtApp) => {
  return {
    provide: {
      getRDT: () => getRDT(),
      getWallet: () => getWallet(),
      getValidators: () => getValidators(),
      getEntityDetails: (address:string) => getEntityDetails(address),
      getLSUBalance: (lsuAddress:string) => getLSUBalance(lsuAddress),
      getXRDBalance: () => getXRDBalance()
    }
  }
})