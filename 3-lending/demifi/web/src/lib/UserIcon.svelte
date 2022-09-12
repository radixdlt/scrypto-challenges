<script lang="ts">
  import Icon from 'svelte-icon'
  import imgUser from '../img/zondicons/user.svg?raw';
  import imgReload from '../img/zondicons/reload.svg?raw';
  import { allParticipants, promiseParticipants } from './login.ts';
  import { generatePfpUrl, colourFor } from './participant.ts';

  export let nfid: string;
  export let size: string = "24";
  export let borderradius = "8px";
</script>

<div style:display="grid">
  {#await $promiseParticipants.get(nfid)}
  <div style:grid-column="1" style:grid-row="1" style:z-index="2">
    <Icon data={imgReload}  stroke="black" fill="white" size="{size}px"/>
  </div>
  {:then p}
  <div style:grid-column="1" style:grid-row="1" style:z-index="1">
    {#if nfid !== undefined && p !== undefined && p.pfpId !== undefined}
      <img 
	style:border-radius={borderradius}
	height="{size}" width="{size}"
	alt="User pfp ({p.refId})"
	src={generatePfpUrl(p)}>
      {:else}
	<span >
	  <Icon data={imgUser} fill={colourFor(nfid)} stroke="black" size="{size}px"/>
	</span>
      {/if}
    </div>
  {/await}
</div>
