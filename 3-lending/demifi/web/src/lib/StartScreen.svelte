<script lang="ts">
  import WalletConnection from './WalletConnection.svelte'
  import { walletAddress, userNfid, tourist, allParticipants, promiseParticipants } from './login.ts';
  import { unendorseStore, endorseStore,
	   unsponsorStore, sponsorStore,
	   expectSponsorStore,
	   editedPfpSeries, editedPfpId, editedUrl } from './participantstores.ts';
  import { mainParticipantsFilter, mainParticipantsFilterTitle,
           pollCatalog, pollParticipantData } from './appstate.ts';

  $pollCatalog = false;
  $pollParticipantData = 0;
  $walletAddress = undefined;
  $promiseParticipants = new Map();
  $allParticipants = new Map();
  $userNfid = undefined;
  $unendorseStore.clear();
  $endorseStore.clear();
  $unsponsorStore.clear();
  $sponsorStore.clear();
  // $unexpectSponsorStore = undefined;
  $expectSponsorStore = undefined;
  $editedPfpSeries = undefined;
  $editedPfpId = undefined;
  $editedUrl = undefined;
  $mainParticipantsFilter = undefined;
  $mainParticipantsFilterTitle = undefined;
  $tourist = false;
</script>

<div>
  <img style:border-radius="25px" src="/DeMiFi logo.png" class="logo" alt="DeMiFi Logo" />
</div>
<h1>Decentralized Micro-Finance</h1>
<h2>Find funding for your dream</h2>
<h2>Or fund the dreams of others</h2>
<div style:border="solid black 1px" style:border-radius="25px">
This is a demo dApp running towards a simulated ledger using real Scrypto contracts<br>
<em>by <a href="mailto:scryptonight@proton.me">scryptonight@proton.me</a></em><br>
<em>aka Bent on the Radix telegram groups</em>
</div>
<div class="connect">
  <WalletConnection />
</div>

<style>
  .logo {
    border-radius: 25px;
    height: 20em;
    will-change: filter;
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .connect {
    height: 10em;
    padding: 1.5em;
  }
</style>
