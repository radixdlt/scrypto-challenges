<script lang="ts">
	import AssetAmount from '$lib/components/AssetAmount.svelte';

	// import PoolDetails from './AssetNameIcon.svelte';

	import { accout_ressource_state } from '$lib/state/account';
	import type { PoolState } from '$lib/state/lending_pools';
	import { add_liquidity, remove_liquidity } from '$lib/transactions/app/liquidity';
	import { Button, Card, Container, Divider, Group, Space } from '@svelteuidev/core';

	import { get_number_input } from '$lib/common';
	import { resources } from '$lib/state/resources';
	import AssetNameIcon from './AssetNameIcon.svelte';
	import PoolInfo from './PoolInfo.svelte';
	import { INTEREST_RATE_TYPE } from '../../../data';

	export let market: PoolState;

	$: resource_metadata = $resources[market.pool_resource_address];

	$: account_resource = $accout_ressource_state?.fungible_resources.find(
		(item) => item.address == market.pool_resource_address
	);

	$: account_pool_share_sresource = $accout_ressource_state?.fungible_resources.find(
		(item) => item.address == market.pool_share_resource_address
	);

	//

	$: price = market.price;
	$: liquidity = market.available_liquidity;
	$: interest_rate = market.loan_state_lookup[INTEREST_RATE_TYPE].interest_rate;
	$: usage = liquidity == 0 ? 0 : loans / (liquidity + loans);
	$: collateral = market.total_collateral;
	$: lp_sypply = market.pool_share_supply;

	$: loans = market.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;

	//

	$: in_acount = parseFloat(account_resource?.amount.value ?? '0');
	$: lended = parseFloat(account_pool_share_sresource?.amount.value ?? '0');
</script>

{#if resource_metadata !== undefined}
	<Card shadow="sm" padding="lg" withBorder>
		<Group spacing="xs" noWrap>
			<AssetNameIcon name={resource_metadata.name} icon={resource_metadata.icon} {price} />

			<Space w="xl" />

			<AssetAmount label="You have" amount={in_acount} {price} />

			<Container fluid />

			<AssetAmount label="You lended" amount={lended} {price} />

			<Group noWrap>
				<Button
					variant="subtle"
					color="green"
					on:click={() => {
						get_number_input({
							title: 'Amount to lend',
							min: 0,
							max: in_acount,
							onSubmit: (_input) => add_liquidity(market.pool_resource_address, _input)
						});
					}}>Add</Button
				>
				<Button
					variant="subtle"
					color="violet"
					on:click={() => {
						get_number_input({
							title: 'Amount to remove',
							min: 0,
							max: lended,
							onSubmit: (_input) => remove_liquidity(market.pool_share_resource_address, _input)
						});
					}}>Remove</Button
				>
			</Group>
		</Group>

		<Space h="sm" />
		<Divider />
		<Space h="sm" />
		<PoolInfo {market} lending_verion />
	</Card>
{/if}
