<script>
  import { Router, Route, Link } from "svelte-navigator";
  import { toasts } from './lib/stores.js';
  import { onDestroy } from 'svelte';
  import NavBtn from './lib/NavBtn.svelte';
  import Header from './lib/Header.svelte';
  import OfferList from './lib/OfferList.svelte';
  import MyOfferList from './lib/MyOfferList.svelte';
  import SingleOffer from './lib/SingleOffer.svelte';
  import CreateOffer from './lib/CreateOffer.svelte';
  import CreateToken from './lib/CreateToken.svelte';
  
  let currentToasts = [];
  
  let unsubscribeToasts = toasts.subscribe(value => {
    currentToasts = value;
  });
  onDestroy(() => {
    unsubscribeToasts();
  });
</script>

<div class="position-fixed top-0 start-50 translate-middle-x" style="z-index: 1020">
  <div class="toast-container position-sticky top-0 start-50 translate-middle-x">
    {#each currentToasts as toast (toast.id)}
      <div class="toast show border-{toast.style}" role="alert" aria-live="assertive" aria-atomic="true">
        <div class="toast-header">
          <strong class="me-auto">{toast.title}</strong>
          <button type="button" class="btn-close" on:click={() => toasts.remove(toast.id)} aria-label="Close">
          </button>
        </div>
        <div class="toast-body text-{toast.style}">
          {toast.message}
        </div>
      </div>
    {/each}
  </div>
</div>

<Router primary={false}>
  <Header />
  <div class="d-flex flex-row container bg-light">
    <NavBtn text="Browse available offers" dest="/offerList" />
    <NavBtn text="Consult my offers" dest="/myOfferList" />
    <NavBtn text="Create new token" dest="/createToken" />
    <NavBtn text="Create new lending offer" dest="/createOffer" />
  </div>
  <main>
    <Route path="offer/*offerAddress" let:params>
      <SingleOffer offerAddress={params.offerAddress} />
    </Route>
    <Route path="offerList">
      <OfferList />
    </Route>
    <Route path="myOfferList">
      <MyOfferList />
    </Route>
    <Route path="createOffer">
      <CreateOffer />
    </Route>
    <Route path="createToken">
      <CreateToken />
    </Route>
    <Route path="">
    </Route>
  </main>
</Router>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  
  .toast-container {
    z-index: 999;
  }
  
  body {
    min-height: 100vh;
    min-height: -webkit-fill-available;
  }

  html {
    height: -webkit-fill-available;
  }

  main {
    display: flex;
    flex-wrap: nowrap;
    height: 100vh;
    height: -webkit-fill-available;
    overflow-x: auto;
  }
</style>
