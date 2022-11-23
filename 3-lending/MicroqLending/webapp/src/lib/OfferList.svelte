<script>
  import {assert, getDecimal, getResourceAddress, getU64, getComponentAddress, enrichOffers, searchOffers} from './utils.js';
  import {config} from './config.js';
  import {toasts} from './stores.js';
  import OfferCard from './OfferCard.svelte'
  import Spinner from './Spinner.svelte';

  let offers = null;
  let search = "";
  let requestInFlight = 0;
  
  async function getOffers(){
    requestInFlight++;
    offers = null;
    try {
      const response = await fetch(config.ledger + '/component/' + config.centralComponentAddress);
      const resAsJson = await response.json();
      assert(resAsJson.blueprint.blueprint_name == "CentralRepository", "The address of the central component is incorrect");
      const state = JSON.parse(resAsJson.state);
      let tmpOffers = state.fields[0].elements.map(element => {
        let values = element.elements;
        return {
          tokenAmount: getDecimal(values[0]),
          tokenAddress: getResourceAddress(values[1]), 
          collatAmount: getDecimal(values[2]),
          costPerHour: getDecimal(values[3]),
          maxBorrowTime: getU64(values[4]),
          address: getComponentAddress(values[5]),
          collatResourceAddress: getResourceAddress(values[6]),
          feeResourceAddress: getResourceAddress(values[7]),
        };
      });
      await enrichOffers(tmpOffers);
      offers = tmpOffers;
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
      {offers?"Refresh offers":"Get open offers"}
    </button>
  {:else}
    <Spinner text="Currently refreshing the offers..." />
  {/if}
  {#if offers}
    <div class="input-group rounded">
      <input type="search" class="form-control rounded" placeholder="Search" bind:value={search} />
      <span class="input-group-text border-0">
        <i class="bi bi-search"></i>
      </span>
    </div>
    {#if offers.length == 0}
      There are currently no offer available
    {:else}
      {#each searchOffers(offers, search) as offer, i (offer.address)}
        <OfferCard offer={offer} type="open" />
      {/each}
    {/if}
  {/if}
</div>