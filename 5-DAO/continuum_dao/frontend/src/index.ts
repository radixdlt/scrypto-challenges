import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import { StateApi, TransactionApi } from '@radixdlt/alphanet-gateway-api-v0-sdk'

// Initialize the SDK
const sdk = Sdk()
const transactionApi = new TransactionApi()
const stateApi = new StateApi()

// Global states
let accountAddress: string // User account address
let componentAddress: string  // GumballMachine component address
let resourceAddress: string // GUM resource address

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  const result = await sdk.request({
    accountAddresses: {},
  })

  if (result.isErr()) {
    throw result.error
  }

  const { accountAddresses } = result.value
  if (!accountAddresses) return

  document.getElementById('accountAddress').innerText = accountAddresses[0].address
  accountAddress = accountAddresses[0].address
}

document.getElementById('instantiateUselessBoxButton').onclick = async function () {
  let packageAddress = document.getElementById("packageAddress").value;
  
  let manifest = new ManifestBuilder()
    .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
    .callFunction(packageAddress, "GumballMachine", "instantiate_gumball_machine", ['Decimal("10")'])
    .build()
    .toString();

  // Send manifest to extension for signing
  const hash = await sdk
    .sendTransaction(manifest)
    .map((response) => response.transactionHash)

  if (hash.isErr()) throw hash.error

  // Fetch the receipt from the Gateway SDK
  const receipt = await transactionApi.transactionReceiptPost({
    v0CommittedTransactionRequest: { intent_hash: hash.value },
  })

  componentAddress = receipt.committed.receipt.state_updates.new_global_entities[1].global_address
  document.getElementById('componentAddress').innerText = componentAddress;
  
  resourceAddress = receipt.committed.receipt.state_updates.new_global_entities[0].global_address
  document.getElementById('gumAddress').innerText = resourceAddress;
}

// document.getElementById('buyGumball').onclick = async function () {

//   let manifest = new ManifestBuilder()
//     .callMethod(accountAddress, "lock_fee", ['Decimal("100")'])
//     .withdrawFromAccountByAmount(accountAddress, 10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9")
//     .takeFromWorktopByAmount(10, "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9", "bucket1")
//     .callMethod(componentAddress, "buy_gumball", ['Bucket("bucket1")'])
//     .callMethod(accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
//     .build()
//     .toString();

//   // Send manifest to extension for signing
//   const hash = await sdk
//     .sendTransaction(manifest)
//     .map((response) => response.transactionHash)

//   if (hash.isErr()) throw hash.error

//   // Fetch the receipt from the Gateway SDK
//   const receipt = await transactionApi.transactionReceiptPost({
//     v0CommittedTransactionRequest: { intent_hash: hash.value },
//   })

//   // Show the receipt on the DOM
//   document.getElementById('receipt').innerText = JSON.stringify(receipt.committed.receipt, null, 2);
// };

document.getElementById('checkBalance').onclick = async function () {

  // Fetch the state of the account component
  const account_state = await stateApi.stateComponentPost({
    v0StateComponentRequest: { component_address: accountAddress }
  })

  let account_gum_vault = account_state.owned_vaults.find(vault => vault.resource_amount.resource_address == `${resourceAddress}`)

  // Fetch the state of the machine component
  const machine_state = await stateApi.stateComponentPost({
    v0StateComponentRequest: { component_address: componentAddress }
  })

  let machine_gum_vault = machine_state.owned_vaults.find(vault => vault.resource_amount.resource_address == `${resourceAddress}`)

  // Update the DOM
  document.getElementById("userBalance").innerText = account_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
  document.getElementById("machineBalance").innerText = machine_gum_vault.resource_amount.amount_attos / Math.pow(10,18)
};