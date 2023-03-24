<script lang="ts">
	import AssetAmount from '$lib/components/AssetAmount.svelte';

	import { accout_ressource_state } from '$lib/state/account';
	import type { PoolState } from '$lib/state/lending_pools';
	import { add_collateral, remove_collateral } from '$lib/transactions/app/collateral';
	import { borrow, repay } from '$lib/transactions/app/loan';
	import { Button, Card, Container, Divider, Group, Space } from '@svelteuidev/core';

	import { get_number_input } from '$lib/common';
	import { resources } from '$lib/state/resources';
	import AssetNameIcon from './AssetNameIcon.svelte';
	import PoolInfo from './PoolInfo.svelte';
	import { INTEREST_RATE_TYPE } from '../../../data';

	export let market: PoolState;
	export let cdp_id: string;

	$: resource_metadata = $resources[market.pool_resource_address];

	$: account_resource = $accout_ressource_state?.fungible_resources.find(
		(item) => item.address == market.pool_resource_address
	);

	$: in_account = parseFloat(account_resource?.amount.value ?? '0');

	// COLLATERAL
	$: collateral_poition =
		$accout_ressource_state?.cdps[cdp_id]?.collaterals[market.pool_resource_address as string];
	$: collateral_position_id = collateral_poition?.position_id ?? 0;
	$: collateral = collateral_poition?.pool_share ?? 0 / market.pool_share_ratio;

	// DEBT
	$: debt_position =
		$accout_ressource_state?.cdps[cdp_id]?.debts[market.pool_resource_address as string];
	$: loan_position_id = debt_position?.position_id ?? 0;
	$: loan = (debt_position?.loan_share ?? 0) / market.loan_to_share_ratio;

	// MAX TO BORROW
</script>

{#if resource_metadata !== undefined}
	<Card shadow="sm" padding="lg" withBorder>
		<Group>
			<AssetNameIcon
				name={resource_metadata.name}
				icon={resource_metadata.icon}
				price={market.price}
			/>

			<!-- <Container fluid /> -->
			<AssetAmount label="You have:" amount={in_account} price={market.price} />
		</Group>

		<Space h="sm" />
		<Divider />
		<Space h="sm" />

		<Group>
			<Group spacing="xs">
				<AssetAmount label="Collateral" amount={collateral} price={market.price} />

				<Group>
					<Button
						variant="subtle"
						size="sm"
						color="green"
						on:click={() => {
							get_number_input({
								title: 'Amount to add',
								min: 0,
								max: in_account,
								onSubmit: (_input) =>
									add_collateral(
										cdp_id,
										market.pool_resource_address,
										_input,
										collateral_position_id
									)
							});
						}}>Add</Button
					>
					<Button
						variant="subtle"
						size="sm"
						color="violet"
						on:click={() => {
							get_number_input({
								title: 'Amount to remove',
								min: 0,
								max: collateral_poition?.pool_share,
								onSubmit: (_input) => remove_collateral(cdp_id, _input, collateral_position_id)
							});
						}}>Remove</Button
					>
				</Group>
			</Group>

			<Container fluid />

			<Group spacing="xs">
				<AssetAmount label="Debt" amount={loan} price={market.price} />

				<Group>
					<Button
						variant="subtle"
						size="sm"
						on:click={() => {
							get_number_input({
								title: 'Amount to borrow',
								min: 0,
								max: market.borrow_limit,
								onSubmit: (_input) =>
									borrow(cdp_id, market.pool_resource_address, _input, loan_position_id)
							});
						}}>Borrow</Button
					>
					<Button
						variant="subtle"
						size="sm"
						color="orange"
						on:click={() => {
							get_number_input({
								title: 'Amount to repay',
								min: 0,
								max: loan,
								onSubmit: (_input) =>
									repay(cdp_id, market.pool_resource_address, _input, loan_position_id)
							});
						}}>Repay</Button
					>
				</Group>
			</Group>
		</Group>

		<Space h="sm" />
		<Divider />
		<Space h="sm" />
		<PoolInfo {market} />
	</Card>
{/if}
