import { RadixDappToolkit, DataRequestBuilder, RadixNetwork } from '@radixdlt/radix-dapp-toolkit'
// You can create a dApp definition in the dev console at https://stokenet-console.radixdlt.com/dapp-metadata 
// then use that account for your dAppId

// Set an environment variable to indicate the current environment
const environment = process.env.NODE_ENV || 'Stokenet'; // Default to 'development' if NODE_ENV is not set
console.log("environment (admin.js): ", environment)
// Define constants based on the environment
let dAppId, networkId, gwUrl;

if (environment === 'production') {
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Mainnet;
  gwUrl = import.meta.env.VITE_GATEWAY_URL;
} else {
  // Default to Stokenet configuration
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Stokenet;
  gwUrl = import.meta.env.VITE_GATEWAY_URL;
}
console.log("gw url (admin.js): ", gwUrl)

// Instantiate DappToolkit
const rdt = RadixDappToolkit({
  dAppDefinitionAddress: dAppId,
  networkId: networkId,
  applicationName: 'Tokenizer',
  applicationVersion: '1.0.0',
});
console.log("dApp Toolkit: ", rdt)

// Global states
let componentAddress = import.meta.env.VITE_COMP_ADDRESS //Scrypto Challenge component address on stokenet
// You receive this badge(your resource address will be different) when you instantiate the component
let admin_badge = import.meta.env.VITE_ADMIN_BADGE
let owner_badge = import.meta.env.VITE_OWNER_BADGE
let lnd_resourceAddress = import.meta.env.VITE_USERDATA_NFT_RESOURCE_ADDRESS // NFT  manager
let lnd_tokenAddress = import.meta.env.VITE_TOKENIZER_TOKEN_ADDRESS // TKN token resource address

let staff_badge = import.meta.env.VITE_STAFF_BADGE_ADDRESS

let xrdAddress = import.meta.env.VITE_XRD //Stokenet XRD resource address

let accountAddress

// ************ Fetch the user's account address (Page Load) ************
// Check if accountAddress is stored in localStorage
const storedAccountAddress = localStorage.getItem('adminAccountAddress');
if (storedAccountAddress) {
  // If stored, update the variable and any relevant UI elements
  accountAddress = storedAccountAddress;
} else {
  rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1))
  // Subscribe to updates to the user's shared wallet data
  rdt.walletApi.walletData$.subscribe((walletData) => {
    accountAddress = walletData.accounts[0].address
    document.getElementById('accountAddress').value = accountAddress
    // Store the accountAddress in localStorage
    localStorage.setItem('adminAccountAddress', accountAddress);
  })
}  

console.log(" wallet accountAddress: ", accountAddress)


// ***** Main function (elementId = divId del button, inputTextId = divId del field di inserimento, method = scrypto method) *****
function createTransactionOnClick(elementId, inputTextId, method) {
  document.getElementById(elementId).onclick = async function () {
    let inputValue = document.getElementById(inputTextId).value;
    const manifest = generateManifest(method, inputValue);
    console.log(`${method} manifest`, manifest);
    const result = await rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });
    if (result.isErr()) {
      console.log(`${method} User Error: `, result.error);
      throw result.error;
    }
  };
}

function generateManifest(method, inputValue) {
  let code;
  switch (method) {
    case 'extend_lending_pool':
      code = ` 
        CALL_METHOD
          Address("${accountAddress}")
          "create_proof_of_amount"    
          Address("${admin_badge}")
          Decimal("1");
        CALL_METHOD
          Address("${componentAddress}")
          "extend_lending_pool"
          Decimal("${inputValue}");
        CALL_METHOD
          Address("${accountAddress}")
          "deposit_batch"
          Expression("ENTIRE_WORKTOP");
        `;
    break;     
    case 'set_reward':
      code = ` 
        CALL_METHOD
          Address("${accountAddress}")
          "create_proof_of_amount"    
          Address("${admin_badge}")
          Decimal("1");
        CALL_METHOD
          Address("${componentAddress}")
          "set_reward"
          Decimal("${inputValue}");
        CALL_METHOD
          Address("${accountAddress}")
          "deposit_batch"
          Expression("ENTIRE_WORKTOP");
       `;
      break;   
      case 'fund_main_pool':
        code = `
          CALL_METHOD
            Address("${accountAddress}")
            "create_proof_of_amount"    
            Address("${admin_badge}")
            Decimal("1");              
          CALL_METHOD
            Address("${accountAddress}")
            "withdraw"    
            Address("${xrdAddress}")
            Decimal("${inputValue}");
          TAKE_ALL_FROM_WORKTOP
            Address("${xrdAddress}")
            Bucket("xrd");
          CALL_METHOD
            Address("${componentAddress}")
            "fund_main_pool"
            Bucket("xrd");      
          CALL_METHOD
            Address("${accountAddress}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");
            `;
        break;                
      case 'add_token':
        code = ` 
          CALL_METHOD
            Address("${accountAddress}")
            "create_proof_of_amount"    
            Address("${admin_badge}")
            Decimal("1");
          CALL_METHOD
            Address("${componentAddress}")
            "add_token"
            Address("${inputValue}");
          CALL_METHOD
            Address("${accountAddress}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");
          `;
        break;           
    default:
      throw new Error(`Unsupported method: ${method}`);
  }

  return code;
}


function generateManifestConfig(method, inputValue1, inputValue2, inputValue3, inputValue4, inputValue5) {
  let code;
  switch (method) {
    case 'config':
      code = ` 
        CALL_METHOD
          Address("${accountAddress}")
          "create_proof_of_amount"    
          Address("${staff_badge}")
          Decimal("1");  
        CALL_METHOD
          Address("${componentAddress}")
          "config"
          Decimal("${inputValue1}")
          Decimal("${inputValue2}")
          Decimal("${inputValue3}")
          Decimal("${inputValue4}")
          Decimal("${inputValue5}")
          ;
        CALL_METHOD
          Address("${accountAddress}")
          "deposit_batch"
          Expression("ENTIRE_WORKTOP");
        `;
    break;   
    default:
      throw new Error(`Unsupported method: ${method}`);
  }

  return code;
}


// ***** Main function (elementId = divId del button, inputTextId = divId del field di inserimento, method = scrypto method) *****
function createTransactionConfigOnClick(elementId, reward,interest,tokenized_epoch_max_lenght,min_loan_limit,max_loan_limit,method) {
  document.getElementById(elementId).onclick = async function () {
    let inputValue1 = document.getElementById(reward).value;
    let inputValue2 = document.getElementById(interest).value;
    let inputValue3 = document.getElementById(tokenized_epoch_max_lenght).value;
    let inputValue4 = document.getElementById(min_loan_limit).value;
    let inputValue5 = document.getElementById(max_loan_limit).value;

    const manifest = generateManifestConfig(method, inputValue1,inputValue2,inputValue3,inputValue4,inputValue5);
    console.log(`${method} manifest`, manifest);
    const result = await rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });
    if (result.isErr()) {
      console.log(`${method} User Error: `, result.error);
      throw result.error;
    }
  };
}


// Usage
createTransactionOnClick('extendLendingPool', 'extendLendingPoolAmount', 'extend_lending_pool');
createTransactionOnClick('setReward', 'reward', 'set_reward');
createTransactionOnClick('fundMainPool', 'numberOfFundedTokens', 'fund_main_pool');
createTransactionOnClick('addToken', 'tokenAddress', 'add_token');


createTransactionConfigOnClick('config', 'reward2','interest2','tokenized_epoch_max_lenght','min_loan_limit','max_loan_limit','config');

