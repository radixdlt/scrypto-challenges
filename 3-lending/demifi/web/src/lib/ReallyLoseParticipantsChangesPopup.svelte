<script lang="ts">
  import { appStarted } from './appstate.ts';
  import Icon from 'svelte-icon'
  import imgWarning from '../img/zondicons/exclamation-outline.svg?raw';

  import { getContext } from 'svelte';
  const { close } = getContext('simple-modal');

  export let callback;
  
  const yesReally = () => {
    close();
    callback();
  }

</script>



<h1><Icon data={imgWarning} fill="red" stroke="red" size="36px"/> You changed stuff</h1>

<p>You have made changes that are not yet committed to the ledger.</p>

<p>If you disconnect now you will lose those changes.</p>

<p>Are you sure this is what you want?</p>

<button on:click="{yesReally}" >Yes. Disconnect me.</button>
