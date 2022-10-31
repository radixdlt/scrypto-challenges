<script lang="ts">
  import { appStarted } from './appstate.ts';
  import Icon from 'svelte-icon'
  import imgWarning from '../img/zondicons/exclamation-outline.svg?raw';
  import { unendorseStore, endorseStore,
           unsponsorStore, sponsorStore,
	   expectSponsorStore,
	   editedPfpSeries, editedPfpId, editedUrl } from './participantstores.ts';

  import { getContext } from 'svelte';
  const { close } = getContext('simple-modal');
  
  const yesReally = () => {
    $unendorseStore = new Set<string>();
    $endorseStore = new Set<string>();
    $unsponsorStore = new Set<string>();
    $sponsorStore = new Set<string>();
    //$unexpectSponsorStore = undefined;
    $expectSponsorStore = undefined;
    $editedPfpSeries = undefined;
    $editedPfpId = undefined;
    $editedUrl = undefined;
    close();
  }

</script>



<h1><Icon data={imgWarning} fill="red" stroke="red" size="36px"/> Reset Changes</h1>

<p>You will lose any uncommitted changes you have made.</p>

<p>Are you sure this is what you want?</p>

<button on:click="{yesReally}" >Yes. Get rid of them.</button>
