<script lang="ts">
	import { format_number } from '$lib/utils';
	import { Group, Text, type SvelteUINumberSize } from '@svelteuidev/core';

	// export let market: PoolState;
	// export let cdp_id: string;

	export let label: string;
	export let amount: number;
	export let price: number = 0;
	export let size: SvelteUINumberSize = 'sm';
	export let price_size: SvelteUINumberSize = 'xs';
	export let percent = false;
	export let short = false;
</script>

<Group position="center" noWrap spacing="xs">
	<Text {size}>{label} :</Text>

	{#if !percent}
		<Text {size} weight={'bold'}>{format_number(amount, short ? 'compact' : undefined)}</Text>
		{#if price != 0}
			<Text size={price_size}
				>(${format_number(price * amount, short ? 'compact' : undefined)})</Text
			>
		{/if}
	{:else}
		<Text {size} weight={'bold'}>{format_number(amount * 100, undefined)} %</Text>
		<!-- else content here -->
	{/if}
</Group>
