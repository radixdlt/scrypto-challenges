<script lang="ts">
	import { Space } from '@svelteuidev/core';
	import { lending_pools } from '$lib/state/pool_state';
	import FaucetPool from './components/FaucetPool.svelte';
</script>

{#each $lending_pools as item}
	<FaucetPool market={item} />
	<Space h="xs" />
{:else}
	No resource
{/each}
