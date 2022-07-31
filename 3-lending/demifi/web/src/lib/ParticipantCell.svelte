<script lang="ts">
  import Icon from 'svelte-icon'
  import imgUser from '../img/zondicons/user.svg?raw';
  import imgQuestion from '../img/zondicons/question.svg?raw';
  import imgStar from '../img/zondicons/star-full.svg?raw';
  import imgBadge from '../img/zondicons/badge.svg?raw';
  import imgEducation from '../img/zondicons/education.svg?raw';
  import NumericAddress from './NumericAddress.svelte';
  import { shorten } from './idstrings.ts';
  import type { Participant } from './participant.ts';
  import { generatePfpUrl, colourFor } from './participant.ts';
  import { allParticipants, promiseParticipants } from './login.ts';
  import { endorseStore, unendorseStore,
	   sponsorStore, unsponsorStore,
	   expectSponsorStore } from './participantstores.ts';
  import UserIcon from './UserIcon.svelte';

  export let nfid: string;
  export let icon: Icon = undefined;
  export let toggleStore = undefined;
  export let fill: string = 'black';
  export let selbackground: string = 'lightblue';
  export let unselbackground: string = 'transparent';
  export let pfpClick = undefined;
  export let showStatusIcons: bool = false;

  let selected: bool = false;

  function toggleIcon (nfid: string) {
    if (!toggleStore) return;
    if ($toggleStore.has(nfid)) {
      $toggleStore.delete(nfid);
    } else {
      $toggleStore.add(nfid);
    }
    $toggleStore=$toggleStore;
  }


</script>

<div class="participant"
     style:display="flex"
     style:align-items="center"
     style:border-radius="15px"
     style:background-color="{selected?selbackground:unselbackground}"
     on:mouseover="{e=>selected=true}"
     on:focus="{e=>selected=true}"
     on:mouseout="{e=>selected=false}"
     on:blur="{e=>selected=false}"
     >
  <div style:padding="8px 0 0 8px"
       style:flex-shrink="0"
       style:flex-grow="0"
       style:cursor="{pfpClick?'pointer':'default'}"
       on:click="{e=>{if (pfpClick) pfpClick(nfid)}}"
       >
    {#await $promiseParticipants.get(nfid)}
      <Icon data={imgQuestion} fill={colourFor(nfid)} stroke="black" size="48px"/>
    {:then p}
      <UserIcon nfid={nfid} borderradius="15px" size="48"/>
    {/await}
    </div>
  <div style:width="100%"
       style:padding="5px"
       style:display="flex"
       style:flex-direction="column"
       style:align-items="center">
    <div style:align="left"
	 style:width="100%"
	 style:margin="0"
	 style:padding="0"
	 style:font-size="24px">
      {#await $promiseParticipants.get(nfid)}
	<em>loading…</em>
      {:then p}
	{#if p !== undefined}
	<b>{p.userName.length<=30?p.userName:p.userName.substr(0,30)+'…'}</b>
<!--	<b>{$allParticipants.get(nfid).userName.length<=30?$allParticipants.get(nfid).userName:$allParticipants.get(nfid).userName.substr(0,30)+'…'}</b>-->
        {/if}
      {/await}
    </div>
    <div style:width="100%" 
	 style:display="flex"
	 style:flex-direction="row"
	 style:justify-content="space-between">
      <div style:align="left"
	   style:margin="0"
	   style:padding="0"
	   style:font-size="12px">
	<NumericAddress address="{nfid}"/>
      </div>
      {#if showStatusIcons}
      <div style:align="right" size="100%"
	   style:margin="0 10px 0 0"
	   style:font-size="12px">
	{#if $endorseStore.has(nfid)}
	  <Icon data={imgStar} fill="black" stroke="black" size="12px"/>
	{/if}
	{#if $unendorseStore.has(nfid)}
	  <Icon data={imgStar} fill="red" stroke="red" size="12px"/>
	{/if}
	{#if $sponsorStore.has(nfid)}
	  <Icon data={imgBadge} fill="black" stroke="black" size="12px"/>
	{/if}
	{#if $unsponsorStore.has(nfid)}
	  <Icon data={imgBadge} fill="red" stroke="red" size="12px"/>
	{/if}
	{#if $expectSponsorStore === nfid}
	  <Icon data={imgEducation} fill="black" stroke="black" size="12px"/>
	{/if}
	<!--
	{#if $unexpectSponsorStore === nfid}
	  <Icon data={imgEducation} fill="red" stroke="red" size="12px"/>
	{/if}
	-->
      </div>
      {/if}
    </div>
  </div>
  {#if icon}
    <div class="tooltip"
	 style:padding="0 10px 0 0"
	 style:flex-shrink="0"
	 style:flex-grow="0"
	 style:cursor="{toggleStore?'pointer':'default'}"
	 on:click="{e=>toggleIcon(nfid)}"
	 >
      <Icon data={icon}
	    fill="{(toggleStore && $toggleStore.has(nfid)) ? 'transparent' : fill}"
	    stroke="{(toggleStore && $toggleStore.has(nfid)) ? 'gray' : 'black'}"
	    size="24px"
	    />
      {#if (toggleStore && $toggleStore.has(nfid))}
	<span class="tooltiptext">Will be removed on next commit</span>
      {/if}
    </div>
  {/if}
</div>


<style>
  .participant {
    padding: 0px;
    margin: 5px;
    text-align:left;
  }

  .tooltip {
    position: relative;
    display: inline-block;
  }

  .tooltip .tooltiptext {
    visibility: hidden;
    width: 120px;
    background-color: black;
    color: #fff;
    text-align: center;
    padding: 5px 0;
    border-radius: 6px;

    opacity: 0;
    transition: opacity 1s;
    
    position: absolute;
    top: 5px;
    right: 105%;
    z-index: 1000;
  }

  .tooltip:hover .tooltiptext {
    visibility: visible;
    opacity: 0.5;
  }
</style>
