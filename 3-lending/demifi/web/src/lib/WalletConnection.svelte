<script lang="ts">
  import Icon from 'svelte-icon'
  import imgInfo from '../img/zondicons/information-outline.svg?raw';
  import { fade } from 'svelte/transition';
  import { DefaultApi, ManifestBuilder } from 'pte-sdk';
  import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';
  import ParticipantName from './ParticipantName.svelte'
  import { walletAddress, tourist, participantNftCount, allParticipants } from './login.ts';
  import { gatewayNodeUrl, participantsComponentAddress, participantsNftResourceAddress } from './serviceconfig.ts';
  import { numberToNfid, getHighestParticipantNfid } from './participant.ts';
  import TouristInfo from './TouristInfo.svelte';
  import { pollCatalog } from './appstate.ts';
  import { getContext } from 'svelte';
  const { open } = getContext('simple-modal');

  enum ConnectionStatus {
    NotConnected,
    Connecting,
    Connected,
  }

  let connected: ConnectionStatus = ConnectionStatus.NotConnected;
  let promise = undefined;
  let takingAwhile: bool = false;
  let takingAwhileTimeoutRef;

  async function determineAccountAddress() {
    if ($tourist) return undefined;
    return getAccountAddress();
  }
  
  async function fetchAddress() {
    takingAwhileTimeoutRef = setTimeout(()=>{takingAwhile = true}, 10000);
    
    const response = await Promise.all([
      determineAccountAddress(),
      getHighestParticipantNfid(),
    ]);
    connected = ConnectionStatus.Connected;
    $walletAddress = response[0];
    $participantNftCount = response[1];
    for (let i = 0; i <= $participantNftCount; ++i) {
      const nfid = numberToNfid(i);
      if (!$allParticipants.has(nfid)) {
        $allParticipants.set(nfid, undefined);
      }
    }
    return response;
  }
  function clearTakingAwhile() {
    takingAwhile= false;
    if (takingAwhileTimeoutRef !== undefined) {
      clearTimeout(takingAwhileTimeoutRef);
      takingAwhileTimeoutRef  = undefined;
    }
  }
  const connect = () => {
    clearTakingAwhile();
    $tourist = false;
    connected = ConnectionStatus.Connecting;
    promise = fetchAddress();
  }
  const demoUser = () => {
    clearTakingAwhile();
    $tourist = true;
    connected = ConnectionStatus.Connecting;
    promise = fetchAddress();
  }
  const disconnect = () => {
    $walletAddress = undefined;
    $tourist = false;
    $pollCatalog = false;
    clearTakingAwhile();
    connected = ConnectionStatus.NotConnected;
  }
</script>

{#if connected === ConnectionStatus.NotConnected}
  <button on:click={connect}>
    Connect Wallet
  </button>
  or
  <button on:click={demoUser}>
    Be a Tourist
  </button>
  <span class="unselectable"
	style:cursor="pointer"
	on:click="{e=>open(TouristInfo)}">
    <Icon data={imgInfo} fill="blue" stroke="blue" size="16px"/>
  </span>
<p>Not connected</p>
{:else}
  <button on:click={disconnect} disabled="{connected != ConnectionStatus.Connected}">
    Disconnect
  </button>
  {#await promise}
      <p>Connecting ...</p>
    {#if takingAwhile}
      <p>If this is taking a while you may not have the PTE browser
	extension installed.</p>
      <p>Please refresh and click the information icon for more
        details.</p>
    {/if}
  {:then}
    {#if $tourist}
      <p>Wallet: Demifi Tourism Board guest wallet</p>
    {:else}
      <p>Wallet: {$walletAddress}</p>
    {/if}
    <div style:display="flow"><div>User:</div> <div><ParticipantName /></div></div>
  {:catch error}
    <p>ERROR: {error}</p>
  {/await}
{/if}
