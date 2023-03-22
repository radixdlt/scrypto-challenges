<script lang="ts">
	// import '../app.postcss';

	import { Button, Space } from '@svelteuidev/core';

	import LendingMarket from '$lib/components/LendingMarket.svelte';
	import type { PoolInfo } from '$lib/data';
	import { create_cdp, new_collateral } from '$lib/transactions/users';
</script>

<!-- <LendingMarket market={btc} /> -->

<Space h="xl" />
<!-- <LendingMarket market={eth} /> -->

<Button on:click={() => create_cdp()} />
<Button on:click={() => new_collateral()} />
