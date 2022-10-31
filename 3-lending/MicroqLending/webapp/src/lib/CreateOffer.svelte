<script>
import { useNavigate } from "svelte-navigator";
import { DefaultApi, Configuration } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';
import {config} from './config.js';
import {toasts} from './stores.js';
import {buildNewOffer} from './manifest.js';
import {mapName2Address, checkReceipt} from './utils.js';

let navigator = useNavigate();
let requestInFlight = 0;

let tokenAddress = "";
let tokenAmount = "";
let collatAmount = "";
let costPerHour = "";
let maxBorrowTime = "";
let feeResource = "RADIX";
let collatResource = "RADIX";

async function onCreate(){
  requestInFlight++;
  let offer = {};
  try {
    offer.tokenAddress = tokenAddress;
    offer.tokenAmount = tokenAmount;
    offer.collatAmount = collatAmount;
    offer.costPerHour = costPerHour;
    offer.maxBorrowTime = maxBorrowTime;
    offer.collatResourceAddress = mapName2Address(collatResource);
    offer.feeResourceAddress = mapName2Address(feeResource);
    let accountAddress = await getAccountAddress();
    let manifest = buildNewOffer(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    offer.address = receipt.newComponents[0];
  }catch(ex){
    console.log(ex);
    requestInFlight--;
    return toasts.error("error while interacting with the PTE");
  }
  toasts.success("new offer created !");
  navigator("/offer/" + offer.address);
  requestInFlight--;
}
</script>

<div class="d-flex flex-column container bg-light">
  <div class="card mb-3">
    <div class="card-body">
      <div class="input-group mb-3">
        <span class="input-group-text">Token address</span>
        <input type="text" class="form-control" bind:value={tokenAddress}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Token amount</span>
        <input type="text" class="form-control" bind:value={tokenAmount}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Collateral amount</span>
        <input type="text" class="form-control" bind:value={collatAmount}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Collateral resource</span>
        <input type="text" class="form-control" bind:value={collatResource}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Cost per hour</span>
        <input type="text" class="form-control" bind:value={costPerHour}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Fee resource</span>
        <input type="text" class="form-control" bind:value={feeResource}>
      </div>
      <div class="input-group mb-3">
        <span class="input-group-text">Max borrow time (in hours)</span>
        <input type="text" class="form-control" bind:value={maxBorrowTime}>
      </div>
      
      <div class="btn-group" role="group">
        <button type="button" class="btn btn-success" 
          disabled={requestInFlight}
          on:click={onCreate}>Create</button>
      </div>
    </div>
  </div>
</div>