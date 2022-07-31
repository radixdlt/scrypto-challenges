<script lang="ts">
  import Icon from 'svelte-icon'
  import imgCopy from '../img/zondicons/edit-copy.svg?raw';
  import flash from './flash.ts';
  import { shorten } from './idstrings.ts';

  export let address;
  let showFullAddress: bool = false;
  let addressDiv;

  async function copyToClipboard(str: string) {
    // Sometimes the browser won't give us a clipboard and
    // then we just show the full address in an editable
    // field so the user can select + ctrl-C himself.
    if (navigator.clipboard === undefined) {
      showFullAddress = !showFullAddress;
    } else {
      await navigator.clipboard.writeText(str);
      flash(addressDiv);
    }
  }

  const addressToClipboard = () => {
    copyToClipboard(address);
  }
</script>


{#if showFullAddress}
<div class="selectable">
     {address}
  <span
     style:cursor="pointer"
     on:click="{addressToClipboard}">
      <Icon data={imgCopy} size="10px"/>
      </span>
      </div>
{:else}
<div class="unselectable"
     bind:this="{addressDiv}"
     style:cursor="pointer"
     on:click="{addressToClipboard}">
  {shorten(address)}
      <Icon data={imgCopy} size="10px"/>
</div>
{/if}

<style>
  .selectable {
    -webkit-user-select: text;
    -khtml-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;
    user-select: text;
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
