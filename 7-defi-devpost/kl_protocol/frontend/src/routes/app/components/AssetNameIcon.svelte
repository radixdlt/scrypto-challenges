<script lang="ts">
	import { format_number } from '$lib/utils';
	import { Group, Image, Text } from '@svelteuidev/core';

	export let name: string;
	export let price: number;
	export let icon: string;
</script>

<Group spacing="xs">
	<!-- <Avatar src={market.icon} height="32" width="32" alt={market.asset_symbol} /> -->
	{#if icon !== ''}
		<Image width={32} height={32} fit="contain" src={icon} />
	{/if}
	<Text weight={'bold'}>{name}</Text>

	<Text size="md">${format_number(price)}</Text>
</Group>
