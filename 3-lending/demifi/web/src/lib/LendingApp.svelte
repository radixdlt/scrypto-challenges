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

<div>
  <div class="topPadding" />
  <div class="topbar unselectable"
       style:display="flex"
       style:align-items="baseline" >
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
  <div class="toprightbar unselectable" style="display: flex; align-items: center;">
    {#if $participantsDirty}
      <div transition:fade
	   display="flex"
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
      </div>{
      /if}
      {#if !$tourist}
	<NumericAddress address="{$walletAddress}"/>
      {/if}
    <button style:margin="0 0 0 5px" on:click="{maybeDisconnect}">Disconnect</button></div>
  <div class="apps unselectable" >
    <button>People</button>
    <button disabled="true">Lending</button>
    <button disabled="true">Borrowing</button>
	  <span 
	       style:cursor="pointer"
	       on:click="{showInstructions}">
	    <Icon fill="blue" stroke="blue" data={imgInfo} size="18px"/>
	  </span>
    
    {#if $participantNftCount > $loadedParticipantCount}
      <progress transition:fade max="{$participantNftCount}" value="{$loadedParticipantCount}"/>
    {/if}
  </div>
  <div class="spacer"/>
  <People/>
</div>

<style>
  .spacer {
    width: 100%;
    height: 55px;
  }
  .topPadding {
    position: fixed;
    left: 0;
    top: 0;
    height: 80px;
    width: 100%;
    background-color: white;
  }
  .topbar {
    position: fixed;
    top: 0;
    left: 0;
    margin: 5px;
    background-color: white;
  }
  .toprightbar {
    position: fixed;
    top: 0;
    right: 0;
    margin: 5px;
  }
  .apps {
    position: fixed;
    top: 2em;
    left: 0;
    margin: 5px;
  }
  .unselectable {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
</style>

