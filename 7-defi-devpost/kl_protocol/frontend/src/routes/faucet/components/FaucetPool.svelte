<script lang="ts">
	import AddressText from '$lib/components/AddressText.svelte';
	import type { PoolState } from '$lib/state/lending_pools';
	import { resources } from '$lib/state/resources';
	import { get_resources } from '$lib/transactions/faucet';
	import { Button, Card, Container, Group, Image, Space, Text } from '@svelteuidev/core';

	export let market: PoolState;

	$: resource_metadata = $resources[market.pool_resource_address];
</script>

<Container>
	{#if resource_metadata !== undefined}
		<Card shadow="sm" padding="lg">
			<Group>
				<Group>
					<!-- <Avatar src={market.icon} height="32" width="32" alt={market.asset_symbol} /> -->

					{#if resource_metadata.icon !== ''}
						<Image width={32} height={32} fit="contain" src={resource_metadata.icon} />
					{/if}

					<Text weight={'bold'}>{resource_metadata.name}</Text>

					<Text>({resource_metadata.symbol})</Text>
				</Group>

				<Container fluid />
				<AddressText truncated address={market.pool_resource_address ?? ''} />
				<Button
					variant="outline"
					color="green"
					override={{ width: '200px' }}
					on:click={() => get_resources(market.pool_resource_address)}>GET SOME ASSETS</Button
				>
			</Group>
		</Card>
	{/if}
</Container>
