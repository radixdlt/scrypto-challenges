import { RadixDappToolkit, DataRequestBuilder, RadixNetwork, NonFungibleIdType, OneTimeDataRequestBuilder } from '@radixdlt/radix-dapp-toolkit'
import { rdt } from './gateway.ts'; 
import { getTokenAddress, fetchComponentConfig } from './gateway.ts';

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
let lnd_resourceAddress = import.meta.env.VITE_USERDATA_NFT_RESOURCE_ADDRESS // NFT  manager
let lnd_tokenAddress = import.meta.env.VITE_TOKENIZER_TOKEN_ADDRESS // TKN token resource address
let pt_Address = import.meta.env.VITE_PT_RESOURCE_ADDRESS // PT token resource address

/**
 * Handle the successful result of a transaction.
 * 
 * @function handleTransactionSuccess
 * @param {Object} result - The result object returned from the transaction.
 * 
 * @example
 * handleTransactionSuccess({ status: 'success', data: ... });
 */
function handleTransactionSuccess(result) {
  //get config parameter of the component
  fetchComponentConfig(componentAddress);
}

/**
 * Utility function for cleaning previous red errors from the result display elements.
 * 
 * @function cleanPreviousErrors
 * 
 * @example
 * cleanPreviousErrors();
 */
function cleanPreviousErrors() {
  document.getElementById('registerTxResult').textContent = "";
  document.getElementById('unregisterTxResult').textContent = "";
  document.getElementById('lendTxResult').textContent = "";
  document.getElementById('takeBackTxResult').textContent = "";
  document.getElementById('tokenizeTxResult').textContent = "";
  document.getElementById('redeemTxResult').textContent = "";
  document.getElementById('claimedTxResult').textContent = "";
  document.getElementById('SwapTxResult').textContent = "";
}

/**
 * Main function that is triggered by an action on the web page and triggers a transaction on the wallet
 * 
 * Functions for account supply/withdraw.
 * 
 * Usage: 
 * createTransactionOnClick(elementId, inputTextId, accountAddressId, method, errorField);
 * 
 * @function createTransactionOnClick
 * @param {string} elementId - The ID of the button element.
 * @param {string} inputTextId - The ID of the input field element.
 * @param {string} accountAddressId - The ID of the account address element.
 * @param {string} method - The scrypto method to call.
 * @param {string} errorField - The ID of the element to display errors.
 * 
 * @example
 * // Supply tokens
 * createTransactionOnClick('lendTokens', 'numberOfTokens', 'accountAddress', 'supply', 'lendTxResult');
 * 
 * @example
 * // Withdraw tokens
 * createTransactionOnClick('takes_back', 'numberOfLndTokens', 'accountAddress', 'takes_back', 'takeBackTxResult');
 */
function createTransactionOnClick(elementId, inputTextId, inputTextId2, method, errorField) {
  document.getElementById(elementId).onclick = async function () {
    //clean previous error
    document.getElementById(errorField).textContent = "";
    cleanPreviousErrors()

    let inputValue = document.getElementById(inputTextId).value;
    let inputValue2 = document.getElementById(inputTextId2).value;
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

/**
 * Functions to allow account sign in / off.
 * 
 * Main function on Button Only
 * 
 * Usage:
 * createTransactionOnButtonClick(elementId, method, errorField);
 * 
 * @function createTransactionOnButtonClick
 * @param {string} elementId - The ID of the button element.
 * @param {string} method - The scrypto method to call.
 * @param {string} errorField - The ID of the element to display errors.
 * 
 * @example
 * // Register a user
 * createTransactionOnButtonClick('register', 'register', 'registerTxResult');
 * 
 * @example
 * // Unregister a user
 * createTransactionOnButtonClick('unregister', 'unregister', 'unregisterTxResult');
 */
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

/**
 * 
 * Generates a transaction manifest for a given method and input values.
 * 
 * @function generateManifest
 * @param {string} method - The scrypto method to call (e.g., 'supply', 'withdraw').
 * @param {string} inputValue - The first input value (e.g., number of tokens).
 * @param {string} inputValue2 - The second input value (e.g., account address).
 * @returns {string} The generated manifest code.
 * 
 * @example
 * const manifest = generateManifest('supply', '100', 'someAccountAddress');
 * console.log(manifest);
 */
function generateManifest(method, inputValue, inputValue2) {
  let code;
  let accountAddressFrom = document.getElementById('accountAddress').value;
  let tokenAddress = getTokenAddress(currencySelect.value);
  console.log(`Working with this token type ${tokenAddress} `);
  switch (method) {
    case 'supply':
      code = `
        CALL_METHOD
          Address("${accountAddressFrom}")
          "withdraw"    
          Address("${tokenAddress}")
          Decimal("${inputValue}");
        TAKE_ALL_FROM_WORKTOP
          Address("${tokenAddress}")
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
          Address("${tokenAddress}");
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
          Address("${tokenAddress}");
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
              Address("${tokenAddress}")
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
                Address("${tokenAddress}")
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
                  Address("${tokenAddress}")
              ;
              CALL_METHOD
                  Address("${accountAddressFrom}")
                  "try_deposit_batch_or_refund"
                  Expression("ENTIRE_WORKTOP")
                  Enum<0u8>()
              ;`;
            break;       
          case 'swap':
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
                Bucket("bucket2")
            ;
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
                Address("${componentAddress}")
                "redeem"
                Bucket("bucket1")
                Bucket("bucket2")
                Address("${tokenAddress}")
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

// Functions to allow account sign in / off
// Usage: createTransactionOnButtonClick (elementId = divId of the button, method = scrypto method, errorField = divId for showing back the error)
createTransactionOnButtonClick('register', 'register', 'registerTxResult');
createTransactionOnButtonClick('unregister', 'unregister', 'unregisterTxResult');

// Functions for account supply/wirthdraw
// Usage: createTransactionOnClick (elementId = divId of the button, inputTextId = divId of the input field, method = scrypto method, errorField = divId for showing back the error)
createTransactionOnClick('lendTokens', 'numberOfTokens', 'accountAddress', 'supply', 'lendTxResult');
createTransactionOnClick('takes_back', 'numberOfLndTokens', 'accountAddress', 'takes_back', 'takeBackTxResult');

// Functions for account tokenize/swap/reedem
// Usage: createTransactionOnClick (elementId = divId of the button, inputTextId = divId of the input field, method = scrypto method, errorField = divId for showing back the error)
createTransactionOnClick('tokenize', 'numberOfTokenizedZero', 'expectedTokenizeLength','tokenize', 'tokenizeTxResult');
createTransactionOnClick('redeem', 'numberOfRedeemedXrdTokens', 'accountAddress', 'redeem', 'redeemTxResult');
createTransactionOnClick('claim', 'numberOfClaimedXrdTokens', 'accountAddress', 'claim', 'claimedTxResult');
createTransactionOnClick('swap', 'numberOfSwapXrdTokens', 'accountAddress', 'swap', 'SwapTxResult');


/**
 * Function for extracting the error message from a string.
 * 
 * @function extractErrorMessage
 * @param {string} inputString - The string containing the error message.
 * @returns {string} - The extracted error message or "No match found".
 * 
 * @example
 * const errorMessage = extractErrorMessage('PanicMessage("Some error message"@...');
 * console.log(errorMessage); // Outputs: Some error message
 */
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
