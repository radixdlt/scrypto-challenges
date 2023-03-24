<script lang="ts">
	import AssetAmount from '$lib/components/AssetAmount.svelte';
	import { accout_ressource_state } from '$lib/state/account';
	import { lending_pools } from '$lib/state/lending_pools';
	import {
		pool_manager_state,
		type CollaterizedDebtPositionData
	} from '$lib/state/lending_pool_manager';
	import { create_cdp, create_delegated_cdp } from '$lib/transactions/app/cdp';
	import { format_number } from '$lib/utils';
	import type { ResourceAddressString } from '@radixdlt/radix-dapp-toolkit';
	import {
		Alert,
		Badge,
		Button,
		Card,
		Container,
		Group,
		NativeSelect,
		Space,
		Text
	} from '@svelteuidev/core';
	import { InfoCircled } from 'radix-icons-svelte';
	import { onMount } from 'svelte';
	import BorrowingComponent from './components/BorrowingPoolComponent.svelte';
	let picked_cdp_id = '';
	$: cdp_id =
		picked_cdp_id == '' //
			? Object.values($accout_ressource_state?.cdps)[0]?.cdp_id
			: picked_cdp_id;

	let health_factor: number;

	let total_debt_value: number;
	let total_collateral_value: number;
	let total_solvency_value: number;

	function get_delegator_list(cdp_id: string): string[] {
		let main_cdp = $accout_ressource_state.cdps[cdp_id];

		if (main_cdp === undefined) return [''];
		if (main_cdp.delegated_cdp_ids.length > 0) {
			return [main_cdp.cdp_id, ...main_cdp.delegated_cdp_ids];
		} else if (main_cdp.delegator_cdp_id != '') {
			return get_delegator_list(main_cdp.delegator_cdp_id);
		} else {
			return [cdp_id];
		}
	}

	$: pool_lists = Object.values($lending_pools).sort((a, b) => (a.price < b.price ? 1 : -1));

	$: current_cdp = $accout_ressource_state?.cdps[cdp_id];

	$: {
		// let main_cdp = $accout_ressource_state.cdps[cdp_id];

		let list = get_delegator_list(cdp_id);

		total_debt_value = 0;
		total_collateral_value = 0;
		total_solvency_value = 0;

		let debt_values: Record<string, any> = {};
		let collateral_values: Record<string, any> = {};
		let solvency_values: Record<string, any> = {};

		list.forEach((_cdp_id) => {
			debt_values[_cdp_id] = 0;
			collateral_values[_cdp_id] = 0;
			solvency_values[_cdp_id] = 0;
		});

		list.forEach((_cdp_id) => {
			let _cdp = $accout_ressource_state.cdps[_cdp_id];
			Object.values(_cdp?.collaterals ?? {}).forEach((element) => {
				let collateral_pool =
					$lending_pools[
						$pool_manager_state.component_address_lookup[
							element.resource_address as ResourceAddressString
						]
					];

				let collateral_value =
					(element?.pool_share * collateral_pool?.price * collateral_pool?.liquidation_threshold) /
					collateral_pool.pool_share_ratio;

				let solvency_value =
					element.pool_share *
					collateral_pool.price *
					collateral_pool.pool_share_ratio *
					(1 - collateral_pool.liquidation_spread) *
					1;

				total_collateral_value += collateral_value;
				total_solvency_value += solvency_value;

				collateral_values[_cdp_id] += collateral_value;
				solvency_values[_cdp_id] += solvency_value;
			});

			Object.values(_cdp?.debts ?? {}).forEach((element) => {
				let debt_pool =
					$lending_pools[
						$pool_manager_state.component_address_lookup[
							element.resource_address as ResourceAddressString
						]
					];

				total_debt_value += (element.loan_share * debt_pool.price) / debt_pool.loan_to_share_ratio;

				debt_values[_cdp_id] += total_debt_value;
			});
		});

		health_factor = total_debt_value == 0 ? Infinity : total_collateral_value / total_debt_value;
	}
</script>

{#if Object.keys($accout_ressource_state.cdps).length == 0}
	<Card>
		<Group position="center">
			<Text
				>No Collaterized Debt Position (CDP) CDP found in your account. You need a CDP NFT to use
				the borrowing feature</Text
			>
			<Button variant="subtle" on:click={() => create_cdp()}>Create a CDP NFT</Button>
		</Group>
	</Card>
{:else}
	<Group>
		<NativeSelect
			required
			bind:value={picked_cdp_id}
			data={Object.values($accout_ressource_state.cdps).map((item) => item.cdp_id)}
			placeholder="Pick a CDP NFT"
		/>

		<AssetAmount label="Health factor" amount={health_factor} price={0} />

		<!-- {#if health_factor > 2}
			<Badge color="green" radius="md" variant="outline"><Text>Healthy</Text></Badge>
		{:else if health_factor > 1.5}
			<Badge color="yellow" radius="md" variant="outline"><Text>Caution</Text></Badge>
		{:else} -->
		<!-- <Badge color="orrange" radius="md" variant="outline"><Text>Warning</Text></Badge> -->
		<!-- {/if} -->

		<AssetAmount
			label="Borrow power"
			amount={total_collateral_value - total_debt_value}
			price={1}
		/>

		<Container fluid />

		{#if cdp_id !== ''}
			<Button on:click={() => create_delegated_cdp(cdp_id)}>Create a delegated CDP NFT</Button>
		{/if}

		<Button on:click={() => create_cdp()}>Create a CDP NFT</Button>
	</Group>
	<Space h="xl" />
	{#if current_cdp.delegator_cdp_id != ''}
		<Alert icon={InfoCircled} title="Delegated CDP" color="violet">
			<Text>
				CDP {cdp_id} is a delegated. The heal factor is from the Delagator CDP :{current_cdp.delegator_cdp_id}.
			</Text>
		</Alert>
	{/if}
	<Space h="xl" />
	{#each Object.values(pool_lists) as item}
		<BorrowingComponent market={item} {cdp_id} />
		<Space h="xl" />
	{:else}
		<Card>
			<Group grow position="center"><Text>No pool available</Text></Group>
		</Card>
	{/each}
{/if}
