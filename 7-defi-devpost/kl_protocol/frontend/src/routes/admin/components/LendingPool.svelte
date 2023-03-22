<script lang="ts">
	import {
		ActionIcon,
		Button,
		Card,
		Container,
		Divider,
		Group,
		Image,
		NumberInput,
		Text
	} from '@svelteuidev/core';

	import AddressText from '$lib/components/AddressText.svelte';
	import { resources } from '$lib/state/pool_state';
	import type { PoolState } from '$lib/state/types';
	import { get_pool_state } from '$lib/transactions/admin';
	import { get } from 'svelte/store';
	import { price_changes } from '$lib/data';
	import { Reload } from 'radix-icons-svelte';

	let timer: number | undefined;
	const debounce = (v: any) => {
		clearTimeout(timer);
		timer = setTimeout(() => {}, 700);
	};

	export let market: PoolState;

	// $: debounce(new_price);

	$: {
		price_changes.update((_p) => {
			if (new_price === undefined) return _p;
			_p[market.pool_resource_address] = new_price;
			return _p;
		});
	}

	$: resource_metadata = $resources[market.pool_resource_address];

	let new_price: number | undefined = get(price_changes)[market.pool_resource_address]; //?? resource_metadata?.price;
</script>

<Card shadow="sm" padding="lg">
	{#if resource_metadata !== undefined}
		<Group>
			<!-- ASSET -->
			<Group>
				<!-- <Avatar src={market.icon} height="32" width="32" alt={market.asset_symbol} /> -->
				{#if resource_metadata.icon !== ''}
					<Image width={32} height={32} fit="contain" src={resource_metadata.icon} />
				{/if}
				<Text weight={'bold'}>{resource_metadata.name}</Text>

				<Text>({resource_metadata.symbol})</Text>
			</Group>

			<Container fluid />

			<Text size="md">Price</Text>
			<NumberInput
				bind:value={new_price}
				precision={2}
				step={0.5}
				error={resource_metadata.price === new_price
					? ''
					: 'On-chain price: ' + resource_metadata.price}
			/>

			<ActionIcon on:click={() => (new_price = resource_metadata?.price)}><Reload /></ActionIcon>
		</Group>

		<Divider />

		<Group direction="row">
			<AddressText truncated address={market.$component_address ?? ''} />
			<Container fluid />

			<Button
				variant="outline"
				color="green"
				on:click={() => get_pool_state(market.$component_address ?? '')}>Get pool state</Button
			>
		</Group>
	{/if}
</Card>
