<script lang="ts">
  import imgInfo from '../img/zondicons/information-outline.svg?raw';
  import { fade } from 'svelte/transition';
  import People from './People.svelte';
  import UserIcon from './UserIcon.svelte';
  import NumericAddress from './NumericAddress.svelte';
  import flash from './flash.ts';
  import { walletAddress, userNfid, tourist, allParticipants,
	   promiseParticipants, participantNftCount, loadedParticipantCount } from './login.ts';
  import { appStarted, pollCatalog, commitButton, disconnectProgress } from './appstate.ts';
  import { getContext } from 'svelte';
  import ParticipantPopup from './ParticipantPopup.svelte';
  import ReallyLoseParticipantsChangesPopup from './ReallyLoseParticipantsChangesPopup.svelte';
  import { participantsDirty, isPfpDirty,
	   unendorseStore, endorseStore,
	   unsponsorStore, sponsorStore,
	   expectSponsorStore } from './participantstores.ts';
  import Icon from 'svelte-icon'
  import imgWarning from '../img/zondicons/exclamation-outline.svg?raw';
  import imgReset from '../img/zondicons/refresh.svg?raw';
  import { commitParticipants } from './participantsrtm.ts';
  import ReallyResetChanges from './ReallyResetChanges.svelte';
  import TouristsNeverCommit from './TouristsNeverCommit.svelte';
  import WaitingDialog from './WaitingDialog.svelte';
  import UserInstructions from './UserInstructions.svelte';

  const { open } = getContext('simple-modal');
  const { close } = getContext('simple-modal');

  const reallyDisconnect = async () => {
    let count: number = 0;
    $disconnectProgress = 0;
    $pollCatalog = false;

    // There's something wonky in our Promise handling so we
    // need to wait for all our outstanding fetch participant
    // Promises to complete before we can safely shut down.
    for (let [nfid,promise] of $promiseParticipants) {
      if (promise.waiting()) count += 1;
    }
    if (count > 0) {
      open(WaitingDialog, { max: count },
	   { closeButton: false, closeOnEsc: false, closeOnOuterClick: false} );
      for (let [nfid,promise] of $promiseParticipants) {
        if (promise.waiting()) {
    	  await promise;
	}
	$disconnectProgress = $disconnectProgress + 1;
      }
    }
    // We used to also have to wait a bit in addition but that's
    // hopefully fixed now.
    //    setTimeout(e=>{close(); $appStarted = false;}, 1000);
    close();
    $appStarted = false;
  }
  
  const maybeDisconnect = async () => {
    if (!$participantsDirty) {
      reallyDisconnect();
    } else {
      open(ReallyLoseParticipantsChangesPopup, { callback: reallyDisconnect });
    }
  }

  const openProfile = () => {
    open(ParticipantPopup, { nfid: $userNfid, participant:$allParticipants.get($userNfid) });
  }

  const commitChanges = () => {
    if ($tourist) {
    open(TouristsNeverCommit);
    return;
    }
    // Typescript code has updated a lot of stores and we need to
    // trigger the update events
    commitParticipants((nfid) => {
      $allParticipants = $allParticipants
      $unendorseStore = $unendorseStore;
      $endorseStore = $endorseStore;
      $unsponsorStore = $unsponsorStore;
      $sponsorStore = $sponsorStore;
//      $unexpectSponsorStore = $unexpectSponsorStore;
      $expectSponsorStore = $expectSponsorStore;
    });
  }

  const showInstructions = () => {
    open(UserInstructions);
  }
  
  const resetChanges = () => {
    open(ReallyResetChanges);
  }

</script>

<div 
     style:display="flex"
     style:width="100%"
     style:flex-direction="column"
     style:justify-content="space-between"
     style:align-content="space-between">
  <div class="unselectable"
       style:flex-grow="1"
       style:width="100%"
       style:display="flex"
       style:align-items="baseline"
       style:justify-content="space-between"
       style:align-content="space-between">
    <div style:display="flex"
	 style:align-items="baseline"
	 style:margin="0.5em 0 0 0.5em">
      <div on:click="{openProfile}" style:cursor="pointer"><UserIcon  nfid="{$userNfid}"/></div>
      <div on:click="{openProfile}" style:cursor="pointer" style:margin="0 0 0 5px" style:font-size="32px">
	{#if $allParticipants.get($userNfid)}
	  <b>{$allParticipants.get($userNfid).userName}</b>
	{:else}
	  <b>???</b>
	{/if}
      </div>
      <div style:margin="0 0 0 5px" style:font-size="12px"><NumericAddress address={$userNfid}/> </div>
    </div>
    <div style:display="flex"
	 style:align-items="baseline"
	 style:margin="0 0.5em 0 0">
      {#if $participantsDirty}
	<div transition:fade
	     style:display="flex"
	     style:align-items="center"
	     style:align-content="center"
	     style:margin="0 5px 0 0">
	  <span class="unselectable"
		style:align-self="flex-end"
		style:margin="0 5px 0 0"
		style:cursor="pointer"
		on:click="{resetChanges}" >
	    <Icon data={imgReset} size="12px"/>
	  </span>
	  <button bind:this={$commitButton} >
	    <span ><Icon data={imgWarning} fill="red" stroke="red" size="12px"/></span>
	    <span align="center" on:click="{commitChanges}" >Commit Changes</span>
	  </button>
	</div>
      {/if}
      {#if !$tourist}
	<div><NumericAddress address="{$walletAddress}"/></div>
      {/if}
      <div><button style:margin="0 0 0 5px" on:click="{maybeDisconnect}">Disconnect</button></div>
    </div>
  </div>
  <div class="unselectable"
       style:display="flex"
       style:align-items="baseline"
       style:margin="0 0.5em 0 0.5em">
    <div><button>People</button></div>
    <div><button disabled="true">Lending</button></div>
    <div><button disabled="true">Borrowing</button></div>
    <div
      style:cursor="pointer"
      style:margin="0 0.5em 0 0.5em"
      on:click="{showInstructions}">
      <Icon fill="blue" stroke="blue" data={imgInfo} size="18px"/>
    </div>
    
    {#if $participantNftCount > $loadedParticipantCount}
      <div><progress transition:fade max="{$participantNftCount}" value="{$loadedParticipantCount}"/></div>
    {/if}
  </div>
  <div >
    <People/>
  </div>
</div>

<style>
  .unselectable {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
</style>

