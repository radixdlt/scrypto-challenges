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
	import { get_pool_state } from '$lib/transactions/admin';
	import { get } from 'svelte/store';
	import { Reload } from 'radix-icons-svelte';
	import type { PoolState } from '$lib/state/lending_pools';
	import { price_changes } from '$lib/state/dapp';
	import { resources } from '$lib/state/resources';

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
			<Group>
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
				error={market.price === new_price ? '' : 'On-chain price: ' + market.price}
			/><ActionIcon on:click={() => (new_price = market?.price)}><Reload /></ActionIcon>
		</Group>

		<Divider />

		<Group direction="row">
			<AddressText truncated address={market.$component_address ?? ''} />

			<Container fluid />

			<AddressText label="resource: " truncated address={market.pool_resource_address ?? ''} />

			<Container fluid />

			<AddressText
				label="pool share: "
				truncated
				address={market.pool_share_resource_address ?? ''}
			/>
		</Group>
	{/if}
</Card>
