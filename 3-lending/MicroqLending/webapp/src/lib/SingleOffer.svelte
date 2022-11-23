<script>
  import {assert, getDecimal, getResourceAddress, getU64, getComponentAddress, readOfferFromState, enrichOffers} from './utils.js';
  import {config} from './config.js';
  import {toasts} from './stores.js';
  import OfferCard from './OfferCard.svelte'
  import { getAccountAddress } from 'pte-browser-extension-sdk';
  import Spinner from './Spinner.svelte';

  export let offerAddress = "";
  let offer = null;
  let requestInFlight = 0;
  
  async function getMoreInfo(){
    const responseComponent = await fetch(config.ledger + '/component/' + offerAddress);
    const resAsJsonComponent = await responseComponent.json();
    console.log(resAsJsonComponent);
    if(resAsJsonComponent.blueprint.blueprint_name != "SecurityLending")
      return;
    if(!config.secLendingPackageAddress[resAsJsonComponent.blueprint.package_address])
      return;
    let offer = readOfferFromState(resAsJsonComponent.state);
    await enrichOffers([offer]);
    offer.address = offerAddress;
    return offer;
  }
  
  async function getOffer(){
    requestInFlight++;
    offer = null;
    try{
      offer = await getMoreInfo();
    }catch(err){
      console.log("Error while retrieving the offer list", err);
      toasts.error("Error while retrieving the offer list");
    }
    requestInFlight--;
  }
  getOffer(); 
</script>


<div class="d-flex flex-column container bg-light">
  {#if !requestInFlight}
    <button type="button" class="btn btn-primary" on:click={getOffer}>Refresh</button>
  {:else}
    <Spinner text="Grab information about the offer" />
  {/if}
  {#if offer}
    <OfferCard offer={offer} />
  {/if}
</div>