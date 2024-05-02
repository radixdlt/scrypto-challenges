import { RadixDappToolkit, DataRequestBuilder, RadixNetwork, NonFungibleIdType, OneTimeDataRequestBuilder } from '@radixdlt/radix-dapp-toolkit'
import { rdt } from './gateway.ts'; 
import { getXrdAddress } from './gateway.ts';

const environment = process.env.NODE_ENV || 'Stokenet'; // Default to 'development' if NODE_ENV is not set
console.log("environment (index.js): ", environment)

// Define constants based on the environment
let dAppId, networkId, gwUrl;

if (environment == 'production') {
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Mainnet;
} else {
  // Default to Stokenet configuration
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Stokenet;
}
gwUrl = import.meta.env.VITE_GATEWAY_URL;
console.log("gw url (index.js): ", gwUrl)

// Global states
let componentAddress = import.meta.env.VITE_COMP_ADDRESS //Scrypto Challenge component address on stokenet
let admin_badge = import.meta.env.VITE_ADMIN_BADGE
let owner_badge = import.meta.env.VITE_OWNER_BADGE
let lnd_resourceAddress = import.meta.env.VITE_USERDATA_NFT_RESOURCE_ADDRESS // NFT  manager
let lnd_tokenAddress = import.meta.env.VITE_TOKENIZER_TOKEN_ADDRESS // TKN token resource address
let pt_Address = import.meta.env.VITE_PT_RESOURCE_ADDRESS // PT token resource address
let yt_Address = import.meta.env.VITE_YT_RESOURCE_ADDRESS // YT token resource address

let xrdAddress = getXrdAddress(currencySelect.value);

function handleTransactionSuccess(result) {

}

//Utility for cleaning previous red errors
function cleanPreviousErrors() {
  document.getElementById('registerTxResult').textContent = "";
  document.getElementById('unregisterTxResult').textContent = "";
  document.getElementById('lendTxResult').textContent = "";
  document.getElementById('takeBackTxResult').textContent = "";
  document.getElementById('tokenizeTxResult').textContent = "";
  document.getElementById('redeemTxResult').textContent = "";
  document.getElementById('claimedTxResult').textContent = "";
}

// ***** Main function *****
function createTransactionOnClick(elementId, inputTextId, inputTextId2, method, errorField) {
  document.getElementById(elementId).onclick = async function () {
    //clean previous error
    document.getElementById(errorField).textContent = "";
    cleanPreviousErrors()

    let inputValue = document.getElementById(inputTextId).value;
    let inputValue2 = document.getElementById(inputTextId2).value;
    // let accountAddressFrom = document.getElementById('accountAddress').value;
    console.log("epoch length (index.js) on elementId: ", inputValue2, elementId)

    const manifest = generateManifest(method, inputValue, inputValue2);
    console.log(`${method} manifest`, manifest);

    const result = await rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });
    if (result.isErr()) {
      document.getElementById(errorField).textContent = extractErrorMessage(result.error.message);
      document.getElementById(errorField).style.color = "red";
      throw result.error;
    }
    handleTransactionSuccess(result);
  };
}

// ***** Main function on Button Only *****
function createTransactionOnButtonClick(elementId, method, errorField) {
  document.getElementById(elementId).onclick = async function () {

    const manifest = generateManifest(method, '');
    console.log(`${method} manifest`, manifest);

    const result = await rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });
    if (result.isErr()) {
      document.getElementById(errorField).textContent = extractErrorMessage(result.error.message);
      document.getElementById(errorField).style.color = "red";
      throw result.error;
    }
  };
}

// ***** Utility function *****
function generateManifest(method, inputValue, inputValue2) {
  let code;
  let accountAddressFrom = document.getElementById('accountAddress').value;
  let xrdAddress = getXrdAddress(currencySelect.value);
  console.log(`Working with this token type ${xrdAddress} `);
  switch (method) {
    case 'supply':
      code = `
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${xrdAddress}")
          Decimal("${inputValue}");
        TAKE_ALL_FROM_WORKTOP
          Address("${xrdAddress}")
          Bucket("xrd");
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${lnd_resourceAddress}")
          Decimal("1");
        TAKE_ALL_FROM_WORKTOP
          Address("${lnd_resourceAddress}")
          Bucket("nft");    
        CALL_METHOD
          Address("${componentAddress}")
          "supply"
          Bucket("xrd")
          Bucket("nft")
          Address("${xrdAddress}");
        CALL_METHOD
          Address("${accountAddressFrom}")
          "try_deposit_batch_or_refund"
          Expression("ENTIRE_WORKTOP")
          Enum<0u8>();
          `;
      break;
    case 'register':
        code = ` 
          CALL_METHOD
            Address("${componentAddress}")
            "register";
          CALL_METHOD
            Address("${accountAddressFrom}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");
        `;
        break;      
    case 'unregister':
      code = `
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${lnd_resourceAddress}")
          Decimal("1");
        TAKE_FROM_WORKTOP
          Address("${lnd_resourceAddress}")
          Decimal("1")
          Bucket("nft");      
        CALL_METHOD
          Address("${componentAddress}")
          "unregister"
          Bucket("nft");
        CALL_METHOD
          Address("${accountAddressFrom}")
          "deposit_batch"
          Expression("ENTIRE_WORKTOP");
      `;
      break;      
    case 'takes_back':
      code = `
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${lnd_tokenAddress}")
          Decimal("${inputValue}");
        TAKE_FROM_WORKTOP
          Address("${lnd_tokenAddress}")
          Decimal("${inputValue}")
          Bucket("loan");
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${lnd_resourceAddress}")
          Decimal("1");
        TAKE_FROM_WORKTOP
          Address("${lnd_resourceAddress}")
          Decimal("1")
          Bucket("nft");           
        CALL_METHOD
          Address("${componentAddress}")
          "takes_back"
          Bucket("loan")
          Bucket("nft")
          Address("${xrdAddress}");
        CALL_METHOD
          Address("${accountAddressFrom}")
          "try_deposit_batch_or_refund"
          Expression("ENTIRE_WORKTOP")
          Enum<0u8>();
          `;
      break; 
      case 'tokenize':
        code = `
          CALL_METHOD
              Address("${accountAddressFrom}")
              "withdraw"
              Address("${lnd_tokenAddress}")
              Decimal("${inputValue}")
          ;
          TAKE_FROM_WORKTOP
              Address("${lnd_tokenAddress}")
              Decimal("${inputValue}")
              Bucket("bucket1")
          ;
          CALL_METHOD
              Address("${accountAddressFrom}")
              "withdraw"
              Address("${lnd_resourceAddress}")
              Decimal("1")
          ;
          TAKE_FROM_WORKTOP
              Address("${lnd_resourceAddress}")
              Decimal("1")
              Bucket("bucket2")
          ;
          CALL_METHOD
              Address("${componentAddress}")
              "tokenize_yield"
              Bucket("bucket1")
              Decimal("${inputValue2}")
              Bucket("bucket2")
              Address("${xrdAddress}")
          ;
          CALL_METHOD
              Address("${accountAddressFrom}")
              "try_deposit_batch_or_refund"
              Expression("ENTIRE_WORKTOP")
              Enum<0u8>()
          ;
            `;
        break;          
        case 'redeem':
            code = `
            CALL_METHOD
                Address("${accountAddressFrom}")
                "withdraw"
                Address("${pt_Address}")
                Decimal("${inputValue}")
            ;
            TAKE_FROM_WORKTOP
                Address("${pt_Address}")
                Decimal("${inputValue}")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("${accountAddressFrom}")
                "withdraw"
                Address("${lnd_resourceAddress}")
                Decimal("1")
            ;
            TAKE_FROM_WORKTOP
                Address("${lnd_resourceAddress}")
                Decimal("1")
                Bucket("bucket2")
            ;
            CALL_METHOD
                Address("${componentAddress}")
                "redeem_from_pt"
                Bucket("bucket1")
                Bucket("bucket2")
                Address("${xrdAddress}")
            ;
            CALL_METHOD
                Address("${accountAddressFrom}")
                "try_deposit_batch_or_refund"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;
            `;
          break;    
          case 'claim':
              code = `
              CALL_METHOD
                  Address("${accountAddressFrom}")
                  "withdraw"
                  Address("${lnd_resourceAddress}")
                  Decimal("1")
              ;
              TAKE_FROM_WORKTOP
                  Address("${lnd_resourceAddress}")
                  Decimal("1")
                  Bucket("bucket1")
              ;
              CALL_METHOD
                  Address("${componentAddress}")
                  "claim_yield"
                  Bucket("bucket1")
                  Address("${xrdAddress}")
              ;
              CALL_METHOD
                  Address("${accountAddressFrom}")
                  "try_deposit_batch_or_refund"
                  Expression("ENTIRE_WORKTOP")
                  Enum<0u8>()
              ;`;
            break;                                                               
    // Add more cases as needed
    default:
      throw new Error(`Unsupported method: ${method}`);
  }

  return code;
}


// Usage
// createTransactionOnClick (elementId = divId of the button, inputTextId = divId of the input field, method = scrypto method)
createTransactionOnButtonClick('register', 'register', 'registerTxResult');
createTransactionOnButtonClick('unregister', 'unregister', 'unregisterTxResult');
createTransactionOnClick('lendTokens', 'numberOfTokens', 'accountAddress', 'supply', 'lendTxResult');
createTransactionOnClick('takes_back', 'numberOfLndTokens', 'accountAddress', 'takes_back', 'takeBackTxResult');

//tokenize
createTransactionOnClick('tokenize', 'numberOfTokenizedZero', 'expectedTokenizeLength','tokenize', 'tokenizeTxResult');
createTransactionOnClick('redeem', 'numberOfRedeemedXrdTokens', 'accountAddress', 'redeem', 'redeemTxResult');
createTransactionOnClick('claim', 'numberOfClaimedXrdTokens', 'accountAddress', 'claim', 'claimedTxResult');


function extractErrorMessage(inputString) {
  const panicRegex = /PanicMessage\("([^@]*)@/;
  const resourceRegex = /ResourceError\(([^)]*)/;
  
  const panicMatch = panicRegex.exec(inputString);
  if (panicMatch && panicMatch[1]) {
    return panicMatch[1];
  }
  
  const resourceMatch = resourceRegex.exec(inputString);
  if (resourceMatch && resourceMatch[1]) {
    return resourceMatch[1];
  }
  
  return "No match found";
}
