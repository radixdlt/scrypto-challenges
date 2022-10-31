<script lang="ts">
  import Icon from 'svelte-icon'
  import imgStar from '../img/zondicons/star-full.svg?raw';
  import imgHeart from '../img/zondicons/heart.svg?raw';
  import imgBadge from '../img/zondicons/badge.svg?raw';
  import imgFilter from '../img/zondicons/filter.svg?raw';
  import imgClose from '../img/zondicons/close-outline.svg?raw';
  import { fade } from 'svelte/transition';
  import ParticipantCell from './ParticipantCell.svelte';
  import ParticipantPopup from './ParticipantPopup.svelte';
  import type { Participant } from './participant.ts';
  import { allParticipants, promiseParticipants, userNfid } from './login.ts';
  import { getContext } from 'svelte';
  import { participantsDirty, isPfpDirty,
	   unendorseStore, endorseStore,
	   unsponsorStore, sponsorStore,
	   expectSponsorStore,
	   editedPfpSeries, editedPfpId, editedUrl } from './participantstores.ts';
  import { mainParticipantsFilter, mainParticipantsFilterTitle,
           viewportHeight} from './appstate.ts';
  const { open } = getContext('simple-modal');

  let viewportWidth: int;
  let tableBgColor: string = '#f0f0f0';
  let everyoneDiv;

  function determineViewportSize(): void {
    viewportWidth = Math.max(document.documentElement.clientWidth || 0,
			     window.innerWidth || 0);
    $viewportHeight = Math.max(document.documentElement.clientHeight || 0,
			       window.innerHeight || 0);
    if (everyoneDiv !== undefined) {
      let pos = findPos(everyoneDiv);
      if (pos !== undefined) topOfEveryone = pos[1];
    }
  }
  
  window.addEventListener('resize', determineViewportSize);
  determineViewportSize();

  var compareParticipantsByNfid = function (nfidA: string, nfidB: string) {
    let aa: Participant = $allParticipants.get(nfidA);
    let bb: Participant = $allParticipants.get(nfidB);
    if (!aa && !bb) return nfidA.localeCompare(nfidB);
    if (aa && !bb) return -1;
    if (!aa && bb) return 1;
    if (aa.userName === bb.userName) return nfidA.localeCompare(nfidB);
    return aa.userName.localeCompare(bb.userName);
  } 
  
  function calcEveryoneList(participantMap, mainFilter, subfilter: string) {
    if (subfilter) subfilter = subfilter.toLowerCase();
    let list: string[] = Array.from(participantMap.keys()).filter(
      (nfid) =>
      (!mainFilter || mainFilter(nfid))
	&& (!subfilter
	|| subfilter.length === 0
	|| nfid === subfilter
	|| $allParticipants.get(nfid)
	&& $allParticipants.get(nfid).userName.toLowerCase().startsWith(subfilter))
    ).sort(compareParticipantsByNfid);
    return list;
  }

  function calcEndorsingList(participantMap) {
    let list: string[] = Array.from(participantMap.keys()).filter(
      (nfid) =>
      $allParticipants.get($userNfid)
	&& $allParticipants.get($userNfid).endorsing.has(nfid)
    ).sort(compareParticipantsByNfid);
    return list;
  }

  function calcEndorsementsList(participantMap) {
    let list: string[] = Array.from(participantMap.keys()).filter(
      (nfid) =>
      $allParticipants.get(nfid)
	&& $allParticipants.get(nfid).endorsing.has($userNfid)
    ).sort(compareParticipantsByNfid);
    return list;
  }
  
  function calcSponsorshipsList(participantMap) {
    let list: string[] = Array.from(participantMap.keys()).filter(
      (nfid) =>
      $allParticipants.get(nfid)
	&& $allParticipants.get(nfid).sponsor === $userNfid
    ).sort(compareParticipantsByNfid);
    return list;
  }

  let typedEveryoneFilter;
  $: everyoneList = calcEveryoneList($allParticipants, $mainParticipantsFilter, typedEveryoneFilter);
  $: endorsingList = calcEndorsingList($allParticipants);
  $: endorsementsList = calcEndorsementsList($allParticipants);
  $: sponsorshipsList = calcSponsorshipsList($allParticipants);
  $: everyoneListTitle = ($mainParticipantsFilterTitle ? $mainParticipantsFilterTitle : "Everyone") + ((typedEveryoneFilter && typedEveryoneFilter.length > 0) ? " (filtered)":"");

  $: $isPfpDirty = $editedPfpSeries !== undefined && $editedPfpSeries !== '' && $editedPfpSeries !== $allParticipants.get($userNfid).pfpSeries
  || $editedPfpId !== undefined && $editedPfpId !== $allParticipants.get($userNfid).pfpId;
  
  
  $: $participantsDirty = $unendorseStore.size !== 0
  || $endorseStore.size !== 0
  || $unsponsorStore.size !== 0
  || $sponsorStore.size !== 0
//  || $unexpectSponsorStore !== undefined
  || $expectSponsorStore !== undefined
  || $isPfpDirty
  || $editedUrl && $editedUrl !== $allParticipants.get($userNfid).url;

  const showProfile = (nfid) => {
    if ($allParticipants.get(nfid)) {
      open(ParticipantPopup, { nfid: nfid, participant: $allParticipants.get(nfid) });
    }
  }

  const revertToDefaultMainFilter = () => {
    $mainParticipantsFilter = undefined;
    $mainParticipantsFilterTitle = undefined;
  }

  function findPos(obj) {
    var curleft = 0;
    var curtop = 0;
    if (obj && obj.offsetParent) {
      do {
	curleft += obj.offsetLeft;
	curtop += obj.offsetTop;
      } while (obj = obj.offsetParent);
      return [curleft,curtop];
    }
    return undefined;
  }

  let topOfEveryone: number = 0;
  
  $: if (everyoneDiv !== undefined) {
    topOfEveryone = findPos(everyoneDiv)[1];
  }

</script>


<div
     style:display="flex"
     style:flex-direction="row"
     style:align-content="stretch"
     style:justify-content="space-evenly"
     style:margin="0.5em 0.25em 0 0.25em"
     >
  <div class="unselectable inside"
       style:background-color="{tableBgColor}"
       style:flex-grow="1"
       style:flex-basis="50px"
       style:margin="0 0.25em 0 0.25em"
       >
    <div class="tableHeader">People I Endorse</div>
    <div style:overflow-y="auto"
	 style:display="flex"
	 style:flex-direction="column"
	 style:max-height="50vh"
	 style:margin="0 0 1em 0">
      {#each endorsingList as nfid}
	<ParticipantCell {nfid} icon="{imgStar}" fill="yellow" pfpClick="{showProfile}" toggleStore="{unendorseStore}"/>
      {/each}
    </div>
  </div>
  <div class="unselectable inside"
       style:background-color="{tableBgColor}"
       style:flex-grow="1"
       style:flex-basis="50px"
       style:margin="0 0.25em 0 0.25em"
       >
    <div class="tableHeader">People I Sponsor</div>
    <div style:overflow-y="auto"
	 style:display="flex"
	 style:flex-direction="column"
	 style:max-height="40vh"
	 style:margin="0 0 1em 0">
      {#each sponsorshipsList as nfid}
	<ParticipantCell {nfid} icon="{imgBadge}" fill="fuchsia" pfpClick="{showProfile}" toggleStore="{unsponsorStore}"/>
      {/each}
    </div>
  </div>
  <div class="unselectable inside"
       style:background-color="{tableBgColor}"
       style:flex-grow="1"
       style:flex-basis="50px"
       style:margin="0 0.25em 0 0.25em"
       >
    <div class="tableHeader">People Who Endorse Me</div>
    <div style:overflow-y="auto"
	 style:display="flex"
	 style:flex-direction="column"
	 style:max-height="40vh"
	 style:margin="0 0 1em 0">
      {#each endorsementsList as nfid}
	<ParticipantCell {nfid} pfpClick="{showProfile}" icon="{imgHeart}"/>
      {/each}
    </div>
  </div>
</div>

<div 
     style:margin="1em 0.5em 0.5em 0.5em">
  <div class="unselectable below"
       style:background-color="{tableBgColor}"
       >
    <div class="tableHeader"
	 style:display="flex"
	 style:align-items="center"
	 style:justify-content="space-between">
      <div style:margin="0 0 0 10px">
	{everyoneListTitle}
	{#if $mainParticipantsFilterTitle !== undefined}
	  <span
	    style:cursor="pointer"
	    on:click={revertToDefaultMainFilter}>
	    <Icon data={imgClose} />
	  </span>
	{/if}
      </div>
      <div style:display="flex"
	   style:align-items="center"
	   style:margin="0 10px 0 0" >
	{#if typedEveryoneFilter !== undefined && typedEveryoneFilter.length > 0}
	  <div on:click={e=>typedEveryoneFilter = ''}
	    transition:fade
	    style:cursor="pointer"
	    style:margin="0 10px 0 0">
	    <Icon data={imgClose}
		  size="16px"/>
	  </div>
	{/if}
	<div style:margin="0 10px 0 0">
	  <Icon data={imgFilter}
		fill="white"
		stroke="black"
		size="16px"/>
	</div>
	<div>
	  <input bind:value={typedEveryoneFilter}
		 placeholder="Type to filter the list"
		 type="text">
	</div>
      </div>
    </div>
    <div style:overflow-y="auto"
	 style:display="grid"
	 style:grid-template-columns="25% 25% 25% 25%"
	 style:gap="0px"
	 style:grid-auto-rows="min-content"
	 bind:this={everyoneDiv}
     	 style:max-height="calc(100vh - {topOfEveryone}px - 1.5em)"
	 style:margin="0 0 1.5em 0">
      {#each everyoneList as nfid}
	<ParticipantCell {nfid} pfpClick="{showProfile}" showStatusIcons="true" unselbackground="#c0c0c0" />
      {/each}
    </div>
  </div>
</div>

<style>
  .tableHeader {
    font-size: 32px;
    font-weight: bold;
    padding: 10px 0 10px 0;
  }
  .inside {
    border-radius: 25px;
  }
  .below {
    border-radius: 25px;
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
