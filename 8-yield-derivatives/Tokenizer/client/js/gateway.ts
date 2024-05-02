import { RadixDappToolkit, DataRequestBuilder, RadixNetwork, createLogger, NonFungibleIdType } from '@radixdlt/radix-dapp-toolkit'
import { it } from 'node:test';

const environment = process.env.NODE_ENV || 'Stokenet'; // Default to 'development' if NODE_ENV is not set
console.log("environment (gateway.js): ", environment)
// Define constants based on the environment
let dAppId, networkId, gwUrl: string, dashboardUrl: string;

if (environment == 'production') {
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Mainnet;
} else {
  // Default to Stokenet configuration
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Stokenet;
}
gwUrl = import.meta.env.VITE_GATEWAY_URL;
dashboardUrl = import.meta.env.VITE_DASHBOARD_URL;
console.log("gw url (gateway.js): ", gwUrl)
console.log("dashboard url (gateway.js): ", dashboardUrl)

let component = import.meta.env.VITE_COMP_ADDRESS
console.log("component address (gateway.js): ", component)

// Instantiate DappToolkit
export const rdt = RadixDappToolkit({
  dAppDefinitionAddress: dAppId,
  networkId: networkId,
  logger: createLogger(1),
  applicationName: 'Tokenizer',
  applicationVersion: '1.0.0'
  ,onDisconnect: () => {
    // clear your application state
    localStorage.removeItem('accountAddress')
  }
});

// manage multi tokens
export function getXrdAddress(currency) {
    if (currency === 'XRD') {
        return 'resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc';
    } else if (currency === 'USDC') {
        return 'resource_tdx_2_1th9fqs7mfkrsgyc2344hz9z5n47r79v7wxuwyj9mq64wjv3ym6d578';
    } else if (currency === 'USDT') {
      return 'resource_tdx_2_1th5z7tgaddluc8xg525rvy6klztvmth2tj4hgjpvd78x0mg5854ccu';
  }
    // Return a default value or handle other cases as needed
    return '';
}

let accountAddress: string | null;

  // ************ Fetch the user's account address (Page Load) ************
  rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1))
  
  // Subscribe to updates to the user's shared wallet data
  const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
    accountAddress = walletData && walletData.accounts && walletData.accounts.length>0 ? walletData.accounts[0].address : null
    console.log("accountAddress : ", accountAddress)
    if (accountAddress!=null) {
      
      const element = document?.getElementById('accountAddress') as HTMLInputElement | null;
      if (element) {
          element.value = accountAddress ?? '';
      }

      // Store the accountAddress in localStorage
      localStorage.setItem('accountAddress', accountAddress);
    }
  })
