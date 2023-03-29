<script lang="ts">
	import { lending_pools } from '$lib/state/lending_pools';
	import { Space } from '@svelteuidev/core';
	import LendingPoolComponent from './components/LendingPoolComponent.svelte';

	let cdp_id: string; //= $accout_ressource_state?.cdps[0]?.cdp_id;

	$: pool_lists = Object.values($lending_pools).sort((a, b) => (a.price < b.price ? 1 : -1));
</script>

{#each Object.values(pool_lists) as item}
	<LendingPoolComponent market={item} />
	<Space h="xl" />
{:else}
	No pool available
{/each}
