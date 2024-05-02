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
let component = import.meta.env.VITE_COMP_ADDRESS
console.log("gw url (gateway.js): ", gwUrl)
console.log("dashboard url (gateway.js): ", dashboardUrl)
console.log("component address (gateway.js): ", component)

/**
 * Instantiate Radix Dapp Toolkit (RDT).
 * 
 */
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

// Global states
let componentAddress = import.meta.env.VITE_COMP_ADDRESS //Component address on stokenet

/**
 * Manage multi tokens by returning the token address based on the currency.
 */
export function getTokenAddress(currency) {
    if (currency === 'XRD') {
        return 'resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc';
    } else if (currency === 'USDC') {
        return 'resource_tdx_2_1t57ejuayfdyrzn6wvzdw0u9lh5ae3u72c4pcxwmvvuf47q6jzk4xv2';
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

  interface Hashmap {
    [key: string]: any;
  }    
  const hashmap: Hashmap = fetchComponentConfig(componentAddress)  
  //get config parameter of the component
  console.log("Hashmap:", hashmap);  

})



// *********** Fetch Component Config (/state/entity/details) (Gateway) ***********
interface Hashmap {
  [key: string]: any;
}    
export async function fetchComponentConfig(_componentAddress: any): Promise<Hashmap>  {
  // Define the data to be sent in the POST request.
  const requestData = generatePayload("ComponentConfig", "", "Global");
  const hashmap: Hashmap = {};
  // Make an HTTP POST request to the gateway
  fetch(gwUrl+'/state/entity/details', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: requestData,
  })
  .then(response => response.json()) // Assuming the response is JSON data.
  .then(data => { 
    const json = data.items ? data.items[0] : null;
    
    const currentEpoch = data.ledger_state.epoch;

    const rewardValue = getReward(json);
    const extrarewardValue = getExtraReward(json);

    const currentRewardConfig = document.getElementById("currentReward");
    const currentExtraRewardConfig = document.getElementById("currentExtraReward");

    if (currentRewardConfig) currentRewardConfig.textContent = rewardValue + '%' ?? '';
    if (currentExtraRewardConfig) currentExtraRewardConfig.textContent = extrarewardValue + '%' ?? '';

  })
  .catch(error => {
      console.error('Error fetching data:', error);
  });
  return hashmap;
}



// ************ Utility Function (Gateway) *****************
function generatePayload(method: string, _address: string, resource_address: string) {
  let code;
  console.log("generatePayload for method:", method);
  switch (method) {
    case 'ComponentConfig':
      console.log("generatePayload for method:", method);
      code = `{
        "addresses": [
          "${componentAddress}"
        ],
        "aggregation_level": "Global",
        "opt_ins": {
          "ancestor_identities": true,
          "component_royalty_vault_balance": true,
          "package_royalty_vault_balance": true,
          "non_fungible_include_nfids": true,
          "explicit_metadata": [
            "name",
            "description"
          ]
        }
      }`;
    break;   
    // Add more cases as needed
    default:
      throw new Error(`Unsupported method: ${method}`);
  }
  return code;
}

// ************ Utility Function (Gateway) *****************
function getReward(data: { details: { state: { fields: any[]; }; }; }) {
  const rewardField = data.details.state.fields.find((field: { field_name: string; }) => field.field_name === "reward");
  return rewardField ? rewardField.value : null;
}

function getExtraReward(data: { details: { state: { fields: any[]; }; }; }) {
  const rewardField = data.details.state.fields.find((field: { field_name: string; }) => field.field_name === "extra_reward");
  return rewardField ? rewardField.value : null;
}