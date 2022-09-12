<script>
  import {assert, getDecimal, getResourceAddress, getU64, getComponentAddress, readOfferFromState, enrichOffers, searchOffers} from './utils.js';
  import {config} from './config.js';
  import {toasts} from './stores.js';
  import OfferCard from './OfferCard.svelte'
  import { getAccountAddress } from 'pte-browser-extension-sdk';
  import Spinner from './Spinner.svelte';

  let offers = null;
  let search = "";
  let requestInFlight = 0;
  let nbRessourcesToCheck = 0;
  let nbRessourcesChecked = 0;
  
  async function getMoreInfo(token, side){
    try {
      const responseResource = await fetch(config.ledger + '/resource/' + token.resource_address);
      const resAsJsonResource = await responseResource.json();
      resAsJsonResource.metadata.forEach(v => token[v.name] = v.value);
      if(token.central_exchange != "MiCroqLending"){
        nbRessourcesChecked++;
        return;
      }
      if(!token.component_address){
        nbRessourcesChecked++;
        return;
      }
      const responseComponent = await fetch(config.ledger + '/component/' + token.component_address);
      const resAsJsonComponent = await responseComponent.json();
      if(resAsJsonComponent.blueprint.blueprint_name != "SecurityLending"){
        nbRessourcesChecked++;
        return;
      }
      if(!config.secLendingPackageAddress[resAsJsonComponent.blueprint.package_address]){
        nbRessourcesChecked++;
        return;
      }
      let offer = readOfferFromState(resAsJsonComponent.state);
      offer.address = token.component_address;
      offer.badgeAddress = token.resource_address;
      offer.side = side;
      nbRessourcesChecked++;
      return offer;
    }catch(err){
      console.log("Exception raised while checking offer: " + token.resource_address + " error is: " + err);
      nbRessourcesChecked++;
      return;
    }
  }
  
  async function getOffers(){
    requestInFlight++;
    offers = null;
    nbRessourcesToCheck = 0;
    nbRessourcesChecked = 0;
    try {
      let accountAddress = await getAccountAddress();
      const response = await fetch(config.ledger + '/component/' + accountAddress);
      const resAsJson = await response.json();
      const owned = resAsJson.owned_resources;
      let lenderToken = owned.filter(token => token.name == "lender badge");
      let borrowerToken = owned.filter(token => token.name == "borrower badge");
      nbRessourcesToCheck = lenderToken.length + borrowerToken.length;
      let lenderPromise = lenderToken.map(t => getMoreInfo(t,"lender"));
      let borrowerPromise = borrowerToken.map(t => getMoreInfo(t,"borrower"));
      let ownedOffers = await Promise.all([...lenderPromise, ...borrowerPromise]);
      ownedOffers = ownedOffers.filter(v => v);
      await enrichOffers(ownedOffers);
      offers = ownedOffers;
    }catch(err){
      console.log("Error while retrieving the offer list", err);
      toasts.error("Error while retrieving the offer list");
    }
    requestInFlight--;
  }
  getOffers(); 
</script>


<div class="d-flex flex-column container bg-light">
  {#if !requestInFlight}
    <button type="button" class="btn btn-primary" on:click={getOffers}>
      {offers?"Refresh offers":"Get my offers"}
    </button>
  {:else if nbRessourcesToCheck == 0}
    <Spinner text="Currently refreshing the offers..." />
  {:else}
    <Spinner text="Currently refreshing the offers... {nbRessourcesChecked}/{nbRessourcesToCheck}" />
  {/if}
  {#if offers}
    <div class="input-group rounded">
      <input type="search" class="form-control rounded" placeholder="Search" bind:value={search} />
      <span class="input-group-text border-0">
        <i class="bi bi-search"></i>
      </span>
    </div>
    {#if offers.length == 0}
      There are currently no offer nor as borrower or lender
    {:else}
      {#each searchOffers(offers, search) as offer, i (offer.side + offer.address)}
        <OfferCard offer={offer} type={offer.side} />
      {/each}
    {/if}
  {/if}
</div>