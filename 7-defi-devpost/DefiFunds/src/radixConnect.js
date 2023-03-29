import {
  RadixDappToolkit,
  ManifestBuilder,
  Decimal,
  Bucket,
  Expression,
  ResourceAddress,
} from "@radixdlt/radix-dapp-toolkit";

// Configure the connect button
export let accountAddress;
const rdt = RadixDappToolkit(
  {
    dAppDefinitionAddress:
      "account_tdx_b_1pplsymjqavhw82vkw69h6zkj5r2gzrh47lvdd0s8h0jseu8sqt",
    dAppName: "defifunds",
  },
  (requestData) => {
    requestData({
      accounts: { quantifier: "atLeast", quantity: 1 },
    }).map(({ data: { accounts } }) => {
      // add accounts to dApp application state
      console.log("account data: ", accounts);
      document.getElementById("accountName").innerText = accounts[0].label;
      document.getElementById("accountAddress").innerText = accounts[0].address;
      accountAddress = accounts[0].address;
    });
  },
  { networkId: 11 }
);
console.log("dApp Toolkit: ", rdt);

// There are four classes exported in the Gateway-SDK These serve as a thin wrapper around the gateway API
// API docs are available @ https://betanet-gateway.redoc.ly/
import {
  TransactionApi,
  StateApi,
  StatusApi,
  StreamApi,
} from "@radixdlt/babylon-gateway-api-sdk";

// Instantiate Gateway SDK
const transactionApi = new TransactionApi();
const stateApi = new StateApi();
const statusApi = new StatusApi();
const streamApi = new StreamApi();

// ************ Send Manifest*************
export async function sendManifest(manifest) {
  // Send manifest to extension for signing
  const result = await rdt.sendTransaction({
    transactionManifest: manifest,
    version: 1,
  });
  if (result.isErr()) throw result.error;
  console.log("Result: ", result.value);

  // Fetch the transaction status from the Gateway API
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash,
    },
  });
  console.log(" TransactionApi transaction/status:", status);

  // fetch component address from gateway api and set componentAddress variable
  let commitReceipt = await transactionApi.transactionCommittedDetails({
    transactionCommittedDetailsRequest: {
      transaction_identifier: {
        type: "intent_hash",
        value_hex: result.value.transactionIntentHash,
      },
    },
  });
  console.log("Committed Details Receipt", commitReceipt);

  return { status, commitReceipt };
}

// ************ Show Recript*************
export function showReceipt(commitReceipt, fieldId) {
  document.getElementById(fieldId).innerText = JSON.stringify(
    commitReceipt.details.receipt,
    null,
    2
  );
}
