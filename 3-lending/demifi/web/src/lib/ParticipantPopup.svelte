<script lang="ts">
  import Icon from 'svelte-icon'
  import imgPencil from '../img/zondicons/edit-pencil.svg?raw';
  import imgCopy from '../img/zondicons/edit-copy.svg?raw';
  import imgClose from '../img/zondicons/close-outline.svg?raw';
  import imgChevUp from '../img/zondicons/cheveron-outline-up.svg?raw';
  import imgChevDown from '../img/zondicons/cheveron-outline-down.svg?raw';
  import imgRevert from '../img/zondicons/reply.svg?raw';
  import imgInfo from '../img/zondicons/information-outline.svg?raw';
  import imgBookmark from '../img/zondicons/bookmark.svg?raw';
  import { userNfid, allParticipants, promiseParticipants } from './login.ts';
  import UserIcon from './UserIcon.svelte';
  import NumericAddress from './NumericAddress.svelte';
  import ParticipantCell from './ParticipantCell.svelte';
  import { determinePfpSeries } from './participant.ts'
  import ChoosePfp from './ChoosePfp.svelte'
  import { editedPfpSeries, editedPfpId, editedUrl,
	   endorseStore, unendorseStore,
	   sponsorStore, unsponsorStore,
	   expectSponsorStore } from './participantstores.ts'
  import flash from './flash.ts';
  import { mainParticipantsFilter, mainParticipantsFilterTitle } from './appstate.ts';

  export let nfid;
  export let participant: Participant;

  let editUrl = $editedUrl != undefined;
  let showRawUrl: bool = false;

  let urlDiv;
  
  async function copyToClipboard(str: string) {
    // Sometimes the browser won't give us a clipboard and
    // then we just show the URL in an editable
    // field so the user can select + ctrl-C himself.
    if (navigator.clipboard === undefined) {
      showRawUrl = !showRawUrl;
    } else {
      await navigator.clipboard.writeText(str);
    }
    flash(urlDiv);
  }

  const urlToClipboard = () => {
    copyToClipboard(participant.url);
  }

  // We do a bit of adaptive grammar in the HTML below, so we prepare
  // some variables we will need for that.

  let theirOrMy: string = nfid === $userNfid ? "My" : "Their";

  // User names that are too long are replaced with the gender neutral
  // "them", for which we use a number of helper variables.
  const LONGEST_NAME_BEFORE_THEY: number = 15;

  function calcNameOrPronoun(name: string, they: string) {
    if (participant.userName === undefined) return they;
    if (participant.userName.length > LONGEST_NAME_BEFORE_THEY) return they;
    return name;
  }
  
  let nameOrThem: string;
  $: nameOrThem = calcNameOrPronoun(participant.userName, 'them');
  
  let nameOrThey: string;
  $: nameOrThey = calcNameOrPronoun(participant.userName, 'they');

  let isOrAre: string;
  $: isOrAre = calcNameOrPronoun('is', 'are');

  let verbS: string;
  $: verbS = calcNameOrPronoun('s', '');

  // We have several click callbacks in the HTML for switching back
  // and forth between preparing to do something on the next commit
  // (an action), and removing our intent to do so. We call the doing
  // of the action simply <action> and we call the removing of the
  // action cancel<Action>>; for example: endorse / cancelEndorse for
  // the endorse action.
  //
  // These intents go into stores with one store for
  // each <action>/cancel<action> pair. For example the $endorseStore
  // for the endorse action: "endorse" adds something to this store
  // and "cancelEndorse" takes it away from the store. When we later
  // commit to the ledger we use these stores to determine what needs
  // to be done. Every entry in the store is a piece of transaction
  // manifest that needs to be submitted.
  //
  // Since in addition to removing an action, actions also can have
  // the opposite action (e.g. unendorse), this can get a bit
  // confusing. Understand that an action has its store
  // ($endorseStore), and the opposite action has a separate store
  // ($unendorseStore).
  //
  // For example, the user can choose to "endorse" another user. This
  // is an action and so the UI has an "endorse" and a "cancelEndorse"
  // that control the contents of the $endorseStore.
  //
  // The user can also choose to unendorse someone he is already
  // endorsing. This is an action that is technically separate from
  // endorsing so the UI has an "unendorse" and a "cancelUnendorse"
  // that control the contents of the $unendorseStore.
  //
  // This complexity arises from the fact that we need to juggle
  // betwen how things actually are on the ledger, how the user has
  // said he WANTS things to be on the ledger, and the fact that there
  // will be a time gap between him announcing his intention to the
  // GUI and those changes actually being committed.
  
  const endorse = () => {
    $endorseStore.add(nfid);
    $endorseStore = $endorseStore;
  }
  const cancelEndorse = () => {
    $endorseStore.delete(nfid);
    $endorseStore = $endorseStore;
  }
  const unendorse = () => {
    $unendorseStore.add(nfid);
    $unendorseStore = $unendorseStore;
  }
  const cancelUnendorse = () => {
    $unendorseStore.delete(nfid);
    $unendorseStore = $unendorseStore;
  }
  const unsponsor = () => {
    $unsponsorStore.add(nfid);
    $unsponsorStore = $unsponsorStore;
  }
  const cancelUnsponsor = () => {
    $unsponsorStore.delete(nfid);
    $unsponsorStore = $unsponsorStore;
  }
  const sponsor = () => {
    $sponsorStore.add(nfid);
    $sponsorStore = $sponsorStore;
  }
  const cancelSponsor = () => {
    $sponsorStore.delete(nfid);
    $sponsorStore = $sponsorStore;
  }
  const expectSponsor = () => {
    $expectSponsorStore = nfid;
  }
  const cancelExpectSponsor = () => {
    $expectSponsorStore = undefined;
  }

//  const unexpectSponsor = () => {
//    $unexpectSponsorStore = nfid;
//  }
//  const cancelUnexpectSponsor = () => {
//    $unexpectSponsorStore = undefined;
//  }

  const startUrlEdit = () => {
    if (editUrl) return;
    editUrl = true;
    $editedUrl = participant.url;
  }
  const cancelUrlEdit = () => {
    if (!editUrl) return;
    editUrl = false;
    $editedUrl = undefined;
  }
  const revertPfp = () => {
    $editedPfpSeries = participant.pfpSeries;
    $editedPfpId = participant.pfpId;
  }
  const clearPfp = () => {
    $editedPfpSeries = -1;
    $editedPfpId = "";
  }

  const applyEndorsingFilter = () => {
    $mainParticipantsFilter = (candidateNfid) => participant.endorsing.has(candidateNfid);
    $mainParticipantsFilterTitle =
      nfid === $userNfid ? "People I Endorse" :
      "People Endorsed By " + participant.userName;
  }
  
  const applySponsoringFilter = () => {
    $mainParticipantsFilter = (candidateNfid) => $allParticipants.get(candidateNfid).sponsor === nfid;
    $mainParticipantsFilterTitle =
      nfid === $userNfid ? "People I Sponsor" :
      "People Sponsored By " + $allParticipants.get(nfid).userName;
  }

  const applyEndorsersFilter = () => {
    $mainParticipantsFilter = (candidateNfid) => $allParticipants.get(candidateNfid).endorsing.has(nfid);
    $mainParticipantsFilterTitle =
      nfid === $userNfid ? "People I Am Endorsed By" :
      "People Endorsing " + participant.userName;
  }

  const applySolicitorsFilter = () => {
    $mainParticipantsFilter = (candidateNfid) => $allParticipants.get(candidateNfid).expectSponsor === nfid;
    $mainParticipantsFilterTitle = "People Soliciting My Sponsorship";
  }

  if($editedPfpSeries === undefined && nfid === $userNfid) $editedPfpSeries = participant.pfpSeries;
  if($editedPfpId === undefined && nfid === $userNfid) $editedPfpId = participant.pfpId;

  let mySponsorships: number = 0;
  let sponsorshipsUncertain: bool = true;
  $: {
    let count: number = 0;
    let missing: bool = false;
    for (let [_, part] of $allParticipants) {
      if (part === undefined) missing = true;
      else if (part.sponsor === nfid) ++count;
    }
    sponsorshipsUncertain  = missing;
    mySponsorships  = count;
  }

  let myEndorsers: number = 0;
  let endorsersUncertain: bool = true;
  $: {
    let count: number = 0;
    let missing: bool = false;
    for (let [_, part] of $allParticipants) {
      if (part === undefined) missing = true;
      else if (part.endorsing.has(nfid)) ++count;
    }
    endorsersUncertain  = missing;
    myEndorsers  = count;
  }

  let mySolicitors: number = 0;
  let solicitorsUncertain: bool = true;
  $: {
    let count: number = 0;
    let missing: bool = false;
    for (let [_, part] of $allParticipants) {
      if (part === undefined) missing = true;
      else if (part.expectSponsor === nfid) ++count;
    }
    solicitorsUncertain  = missing;
    mySolicitors  = count;
  }
  
</script>

<div style:display="flex" style:align-items="flex-start">
  <div><UserIcon size="36" nfid="{nfid}"/></div>
  <div style:margin="5px 0 0 5px" style:display="flex" style:flex-direction="column">
    <div align="left" style:font-size="36px">
      <b style:overflow="hidden"
	 style:white-space="nowrap"
	 style:text-overflow="ellipsis">
	{participant.userName}
    </b></div>
    <div align="left"><NumericAddress address="{nfid}"/></div>
  </div>
</div>
<div style:display="grid" style:grid-template-columns="max-content auto" style:align-items="center" >
  {#if nfid === $userNfid || participant.url.length > 0}
    <div align="right" style:margin="5px 0 0 0" style:display="flex" >
      {#if nfid === $userNfid}
	{#if editUrl}
	  <div class="unselectable"
	       style:margin="0 5px 0 0"
	       style:flex-grow="1"
	       style:cursor="pointer"
	       on:click="{cancelUrlEdit}">
	    <Icon data={imgClose} size="12px"/>
	  </div>
	{:else}
	  <div class="unselectable"
	       style:margin="0 5px 0 0"
	       style:flex-grow="1"
	       style:cursor="pointer"
	       on:click="{startUrlEdit}">
	    <Icon data={imgPencil} size="12px"/>
	  </div>
	{/if}
      {/if}
      <div style:flex-grow="{nfid === $userNfid ? 0 : 1}" width="100%"><b>URL:</b></div>
    </div>
    <div align="left" style:display="flex" style:align-items="start" style:margin="5px 0 0 5px">
      {#if editUrl}
	<div style:flex-grow="1" >
	  <!-- svelte-ignore a11y-autofocus -->
	  <input
	    style:min-width="95%"
	    type="text"
	    autofocus
	    placeholder="https://..."
	    bind:value="{$editedUrl}">
	</div>
      {:else if participant.url.length > 0}
	{#if showRawUrl}
	  {participant.url}
	{:else}
	  <a href="{participant.url}">{theirOrMy} promoted link</a>
        {/if}
	<span class="unselectable"
	      style:margin="0 0 0 5px"
	      bind:this="{urlDiv}"
	      style:cursor="pointer"
	      on:click="{urlToClipboard}">
	  <Icon data={imgCopy} size="12px"/>
	</span>
      {/if}
    </div>
  {/if}
  
  {#if nfid === $userNfid}
    <div align="right" style:margin="5px 0 0 0"><b>Profile picture:</b></div>
    <div align="left" style:margin="5px 0 0 5px">
      <span>
	<ChoosePfp bind:selectedPfpSeries={$editedPfpSeries} bind:selectedPfpId={$editedPfpId}/>
      </span>
      {#if participant.pfpSeries !== $editedPfpSeries || participant.pfpId !== $editedPfpId}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{revertPfp}">
	  <Icon data={imgRevert} fill={'black'} stroke="black" size="12px"/>	    
	</span>
      {:else if $editedPfpSeries != undefined}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{clearPfp}">
	  <Icon  data={imgClose} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    </div>
  {/if}

{#if $userNfid !== nfid && $userNfid !== undefined && $allParticipants.get($userNfid).sponsor === undefined}
  <div align="right" style:margin="5px 0 0 0">
    {#if $allParticipants.get($userNfid).expectSponsor === nfid}
      <!--
      {#if $unexpectSponsorStore !== nfid}
	<span class="unselectable" on:click="{unexpectSponsor} style:cursor="pointer"">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable" on:click="{cancelUnexpectSponsor} style:cursor="pointer"">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
      -->
    {:else}
      {#if $expectSponsorStore !== nfid}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{expectSponsor}">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{cancelExpectSponsor}">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    {/if}
  </div>
  <div align="left" style:margin="5px 0 0 5px">
    {#if $allParticipants.get($userNfid).expectSponsor === nfid}
      <!--
	  {#if $unexpectSponsorStore !== nfid}
	    -->
	I am soliciting {nameOrThem} to sponsor me
    <!--
      {:else}
	<b>I will stop soliciting {nameOrThem} to sponsor me</b>
      {/if}
	-->
    {:else}
      {#if $expectSponsorStore !== nfid}
	I am not soliciting {nameOrThem} to sponsor me
      {:else}
	<b>I will solicit {nameOrThem} to
	  {#if $allParticipants.get($userNfid).sponsor}
	    replace my current sponsor
	  {:else}
	    sponsor me
	  {/if}
	</b>
      {/if}
    {/if}
  </div>
  {#if $allParticipants.get($userNfid).expectSponsor !== nfid}
    <div align="right" style:margin="5px 0 0 0">
    </div>		
    <div align="left" style:margin="0 0 0 5px" style:align-content="start" style:font-size="12px">
      <em >
	{#if $expectSponsorStore !== nfid}
	  And you should only do so if you are sure they will accept
	{:else}
	  You can only solicit one sponsorship at a time so any others will be cancelled
	{/if}
      </em>
    </div>
  {/if}	    
{/if}

{#if $userNfid !== nfid}
  {#if participant.sponsor === $userNfid}
    <div align="right" style:margin="5px 0 0 0">
      {#if !$unsponsorStore.has(nfid)}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{unsponsor}">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{cancelUnsponsor}">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    </div>
    <div align="left" style:margin="5px 0 0 5px">
      {#if !$unsponsorStore.has(nfid)}
	I sponsor {nameOrThem}
      {:else}
	<b>I will stop sponsoring {nameOrThem}</b>
      {/if}
    </div>
  {:else if participant.expectSponsor === $userNfid}
    <div align="right" style:margin="5px 0 0 0">
      {#if !$sponsorStore.has(nfid)}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{sponsor}">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{cancelSponsor}">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    </div>
    <div align="left" style:margin="5px 0 0 5px">
      {#if !$sponsorStore.has(nfid)}
	I have been asked to sponsor {nameOrThem}
      {:else}
	<b>I will start sponsoring {nameOrThem}</b>
      {/if}
    </div>
  {/if}
{/if}

{#if $userNfid !== nfid}
  <div align="right" style:margin="5px 0 0 0">
    {#if $allParticipants.get($userNfid).endorsing.has(nfid)}
      {#if !$unendorseStore.has(nfid)}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{unendorse}">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{cancelUnendorse}">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    {:else}
      {#if !$endorseStore.has(nfid)}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{endorse}">
	  <Icon data={imgChevUp} fill={'black'} stroke="black" size="12px"/>
	</span>
      {:else}
	<span class="unselectable"
	      style:cursor="pointer"
	      on:click="{cancelEndorse}">
	  <Icon data={imgChevDown} fill={'black'} stroke="black" size="12px"/>
	</span>
      {/if}
    {/if}
  </div>
  <div align="left" style:margin="5px 0 0 5px">
    {#if $allParticipants.get($userNfid).endorsing.has(nfid)}
      {#if !$unendorseStore.has(nfid)}
	I endorse {nameOrThem}
      {:else}
	<b>I will stop endorsing {nameOrThem}</b>
      {/if}
    {:else}
      {#if !$endorseStore.has(nfid)}
	I am not endorsing {nameOrThem}
      {:else}
	<b>I will start endorsing {nameOrThem}</b>
      {/if}
    {/if}
  </div>
{/if}

<div align="right" style:margin="5px 0 0 0">
  {#if nfid === $userNfid}
    <b>I sponsor:</b>
  {:else}
    <b>Sponsors:</b>
  {/if}
</div>
<div align="left" style:margin="5px 0 0 5px">
  {#if mySponsorships === 0 && !sponsorshipsUncertain}
    Nobody
  {:else}
    {mySponsorships}
  {#if sponsorshipsUncertain}
    or more people
    {:else if mySponsorships === 1}
    person
    {:else}
    people
    {/if}
  {/if}
  <span class="unselectable"
	style:cursor="pointer"
	on:click="{applySponsoringFilter}">
    <Icon data={imgBookmark} fill={'black'} stroke="black" size="12px"/>
  </span>
</div>

<div align="right" style:margin="5px 0 0 0">
  {#if nfid === $userNfid}
    <b>I endorse:</b>
  {:else}
    <b>Endorses:</b>
  {/if}
</div>
<div align="left" style:margin="5px 0 0 5px">
  {#if participant.endorsing.size === 0}
    Nobody
  {:else if participant.endorsing.size === 1}
    {participant.endorsing.size} person
  {:else}
    {participant.endorsing.size} people
  {/if}
  <span class="unselectable"
	style:cursor="pointer"
	on:click="{applyEndorsingFilter}">
    <Icon data={imgBookmark} fill={'black'} stroke="black" size="12px"/>
    </span>
  {#if participant.endorsing.has($userNfid)}
    ({nameOrThey} endorse{verbS} me)
  {/if}
</div>

<div align="right" style:margin="5px 0 0 0">
  {#if nfid === $userNfid}
    <b>I am endorsed by:</b>
  {:else}
    <b>Endorsed by:</b>
  {/if}
</div>
<div align="left" style:margin="5px 0 0 5px">
  {#if myEndorsers === 0 && !endorsersUncertain}
    Nobody
  {:else}
  {myEndorsers}
  {#if endorsersUncertain}
    or more people
  {:else if myEndorsers === 1}
    person
  {:else}
    people
  {/if}
  {/if}
  <span class="unselectable"
	style:cursor="pointer"
	on:click="{applyEndorsersFilter}">
    <Icon data={imgBookmark} fill={'black'} stroke="black" size="12px"/>
  </span>
</div>

{#if nfid === $userNfid}
  <div align="right" style:margin="5px 0 0 0">
    <b>I am being solicited by:</b>
  </div>
<div align="left" style:margin="5px 0 0 5px">
  {mySolicitors}
  {#if solicitorsUncertain}
    or more people
  {:else if mySolicitors === 1}
    person
  {:else}
    people
  {/if}
  {#if !solicitorsUncertain || mySolicitors > 0}
    <span class="unselectable"
	  style:cursor="pointer"
	  on:click="{applySolicitorsFilter}">
      <Icon data={imgBookmark} fill={'black'} stroke="black" size="12px"/>
    </span>
  {/if}
</div>
{/if}

{#if participant.sponsor}
  <div align="right" style:margin="5px 0 0 0">
  {#if nfid === $userNfid}
    <b>I am sponsored by:</b>
  {:else}
    <b>Sponsored by:</b>
  {/if}
  </div>
  <div align="left" style:margin="5px 0 0 5px">
    <ParticipantCell
      selbackground="#f0f0f0"
      unselbackground="#f0f0f0"
      nfid="{participant.sponsor}"/>
  </div>
{/if}

{#if $userNfid == nfid && participant.expectSponsor}
  <div align="right" style:margin="5px 0 0 0"><b>Hoping for sponsor:</b></div>
  <div align="left" style:margin="5px 0 0 5px">
    <ParticipantCell
      selbackground="#f0f0f0"
      unselbackground="#f0f0f0"
      nfid="{participant.expectSponsor}"/>
  </div>
{/if}

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
