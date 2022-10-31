<script> 
  import {getAccountAddress, signTransaction} from 'pte-browser-extension-sdk';
  import {config} from './config.js';
  import {buildAcceptOffer, buildCancelOffer, buildSeizeCollateral, buildSettleOffer, buildReturnAssets} from './manifest.js';
  import {radixResourceAddress, displayToken, pad2, timeToHuman, checkReceipt} from './utils.js';
  import {toasts} from './stores.js';
  
  export let offer;
  export let type; // open, borrower, lender, or empty

  let requestInFlight = 0;
  
  let timeLeft = null;
  let timeleftHuman = "retrieving...";
  async function updateTime(){
    const response = await fetch('https://worldtimeapi.org/api/timezone/Europe/Amsterdam');
    let unixtime = (await response.json())["unixtime"];
    timeLeft = Math.max(0, offer.unixBorrowStart + offer.maxBorrowTime*3600000 - unixtime)/1000;
    timeleftHuman = timeToHuman(timeLeft);
  };
  if(type != "open")
    updateTime();

  function changeState(state){
    offer.state = state;
    offer = offer;
  }

  // updates the timestamp in a secLending component via a call to the oracle
  async function updateTimestamp(componentAddress){
    // Send request to backend 
    console.log("Send 'sendTimeToComponent' request to backend ");
    const url = new URL(config.backendAddress + "/sendTimeToComponent");
    let res = await fetch(url, {
      method: "POST",
      headers: {'Content-Type': 'application/json'}, 
      body: JSON.stringify({componentAddress})});
    let json = await res.json();
    if(json.error)
      throw new Error(json.error);
    return true; 
  }

  // The borrower accept an offer
  async function acceptOffer(){
    await updateTimestamp(offer.address);
    let accountAddress = await getAccountAddress();
    let manifest = buildAcceptOffer(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    type = "borrower";
    changeState("StateRenting");
  }

  // The lender cancel an offer which has not been accepted yet
  async function cancelOffer(){
    let accountAddress = await getAccountAddress();
    let manifest = buildCancelOffer(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    changeState("StateCancelled");
  }

  // The lender seize collateral when the time is out
  async function seizeCollateral(){
    await updateTimestamp(offer.address);
    let accountAddress = await getAccountAddress();
    let manifest = buildSeizeCollateral(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    changeState("StateDefaulted");
  }
  
  // The lender settles the contract after the loan has be repayed by the borrower
  async function settleOffer(){ 
    let accountAddress = await getAccountAddress();
    let manifest = buildSettleOffer(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    changeState("StateSettled");
  }
  
  // The borrower return the asset and get back his collateral
  async function returnAsset(){
    await updateTimestamp(offer.address);
    let accountAddress = await getAccountAddress();
    let manifest = buildReturnAssets(accountAddress, offer);
    console.log("Manifest", manifest);
    let receipt = await signTransaction(manifest);
    checkReceipt(receipt);
    changeState("StateReturned");
  }
  
  
  function protect(fct){
    return async function(){
      requestInFlight++;
      try {
        await fct();
        toasts.success("successful operation");
      }catch(ex){
        console.log(ex);
        toasts.error(ex);
      }
      requestInFlight--;
    }
  }
</script>

<div class="card">
  <div class="card-header">
    Offer on {offer.tokenAddress}
  </div>
  <div class="card-body">
    {#if offer.state}
      <div>Offer state: {offer.state}</div>
    {/if}
    {#if offer.state == "StateRenting"}
      <div>Time left: {timeleftHuman}</div>
    {/if}
    <div>Lending offer: {offer.tokenAmount} tokens of {displayToken(offer.tokenAddress, offer.tokenMetaInfo)}</div>
    <div>Collateral: {offer.collatAmount} {displayToken(offer.collatResourceAddress, offer.collatResourceMetaInfo)}</div>
    <div>Lending cost: {offer.costPerHour} per hour of {displayToken(offer.feeResourceAddress, offer.feeResourceMetaInfo)}</div>
    <div>Max lending time: {offer.maxBorrowTime} hours</div>
    <div class="text-muted">Lending component address: {offer.address}</div>
  </div>
  <div class="card-footer">
    {#if type == "open" || (!type && offer.state == "StateWaitForRenter")}
      <button type="button" class="btn btn-primary"
        disabled={requestInFlight} on:click={protect(acceptOffer)}>
          Accept offer
      </button>
    {:else if type == "lender"}
      {#if offer.state == "StateWaitForRenter"}
        <button type="button" class="btn btn-primary"
          disabled={requestInFlight} on:click={protect(cancelOffer)}>
            Cancel offer
        </button>
      {:else if offer.state == "StateRenting" && timeLeft !== null && timeLeft < 0}
        <button type="button" class="btn btn-primary"
          disabled={requestInFlight} on:click={protect(seizeCollateral)}>
            Seize Collateral
        </button>
      {:else if offer.state == "StateReturned"}
        <button type="button" class="btn btn-primary"
          disabled={requestInFlight} on:click={protect(settleOffer)}>
            Settle offer
        </button>
      {/if}
    {:else if type == "borrower"}
      {#if offer.state == "StateRenting"}
        <button type="button" class="btn btn-primary"
          disabled={requestInFlight} on:click={protect(returnAsset)}>
            Return asset
        </button>
      {/if}
    {/if}
  </div>
</div>