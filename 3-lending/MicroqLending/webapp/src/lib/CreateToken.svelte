<script>
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';
import {buildTransactionNewToken} from './manifest.js';
import {toasts} from './stores.js';
import {checkReceipt} from './utils.js';

let requestInFlight = 0;

let tokenAddress = "";
let tokenAmount = "";
let tokenName = "";
let tokenSymbol = "";

async function onCreate(){
  requestInFlight++;
  let offer = {};
  try {
    let accountAddress = await getAccountAddress();
    let manifest = buildTransactionNewToken(accountAddress, tokenAmount, tokenName, tokenSymbol);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    tokenAddress = receipt.newResources[0];
  }catch(ex){
    console.log(ex);
    requestInFlight--;
    return toasts.error("error while interacting with the PTE");
  }
  toasts.success("new token created !");
  requestInFlight--;
}
</script>

<div class="d-flex flex-column container bg-light">
  <div class="card mb-3">
    <div class="card-body">
      {#if tokenAddress==""}
        <div class="input-group mb-3">
          <span class="input-group-text">Token name</span>
          <input type="text" class="form-control" bind:value={tokenName}>
        </div>
        
        <div class="input-group mb-3">
          <span class="input-group-text">Token amount</span>
          <input type="text" class="form-control" bind:value={tokenAmount}>
        </div>
        
        <div class="input-group mb-3">
          <span class="input-group-text">Token symbol</span>
          <input type="text" class="form-control" bind:value={tokenSymbol}>
        </div>
        
        <div class="btn-group" role="group">
          <button type="button" class="btn btn-success" 
            disabled={requestInFlight}
            on:click={onCreate}>Create</button>
        </div>
      {:else}
        <div class="input-group mb-3">
          <span class="input-group-text">Token address</span>
          <input type="text" class="form-control" bind:value={tokenAddress} readonly>
        </div>
        <p>copy this address, and use it to test the lending component</p>
      {/if}
    </div>
  </div>
</div>