<script lang="ts">
	import { lending_pools } from '$lib/state/lending_pools';
	import { Space } from '@svelteuidev/core';
	import { XRD } from '../../data';
	import FaucetPool from './components/FaucetPool.svelte';

	$: lending_pools_withou_xrd = Object.values($lending_pools).filter(
		(pool) => pool.pool_resource_address != XRD
	);
</script>

{#each lending_pools_withou_xrd as item}
	<FaucetPool market={item} />
	<Space h="xs" />
{:else}
	No resource available
{/each}
