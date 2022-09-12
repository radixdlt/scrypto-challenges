<script lang="ts">
  import { disconnectProgress } from './appstate.ts';

  
  export let max;
</script>


<div class="unselectable">
<h1>Shutting down ...</h1>

<progress {max} value={$disconnectProgress}/>
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
