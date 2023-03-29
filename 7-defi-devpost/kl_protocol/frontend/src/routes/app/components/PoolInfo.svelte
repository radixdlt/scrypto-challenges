<script lang="ts">
	import AssetAmount from '$lib/components/AssetAmount.svelte';
	import type { PoolState } from '$lib/state/lending_pools';
	import { format_number } from '$lib/utils';
	import { Group, Progress, Space } from '@svelteuidev/core';
	import { INTEREST_RATE_TYPE } from '../../../data';

	export let market: PoolState;
	export let lending_verion = false;
	// $: price = market.price;
	// $: interest_rate = market.loan_state_lookup[INTEREST_RATE_TYPE].interest_rate;

	// $: total_liquidity =
	// 	market.available_liquidity + market.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
	// $: pool_share_supply = market.pool_share_supply;
	// $: pool_share_ratio = total_liquidity == 0 ? 1 : pool_share_supply / total_liquidity;

	// $: total_collateral = market.total_collateral; // pool_shares

	// $: total_loan_share = market.loan_state_lookup[INTEREST_RATE_TYPE].total_loan_share;
	// $: total_loans = market.loan_state_lookup[INTEREST_RATE_TYPE].total_loan;
	// $: loan_to_share_ratio = total_loans == 0 ? 1 : total_loan_share / total_loans;

	// $: total_collateral_value = market.total_collateral / pool_share_ratio;
	// $: borrow_limit = market.available_liquidity - total_collateral_value;

	// $: usage = total_liquidity == 0 ? 0 : total_loans / total_liquidity;

	$: sections =
		market.total_liquidity == 0
			? []
			: [
					{
						value: (market.borrow_limit / market.total_liquidity) * 100,
						color: 'green',
						label: `Available ${format_number(
							(market.borrow_limit / market.total_liquidity) * 100
						)} %`
					},
					{
						value: (market.total_collateral_value / market.total_liquidity) * 100,
						color: 'grey',
						label: `Collaterals ${format_number(
							(market.total_collateral_value / market.total_liquidity) * 100
						)} %`
					},
					{
						value: (market.total_loans / market.total_liquidity) * 100,
						color: 'orange',
						label: `Loans ${format_number((market.total_loans / market.total_liquidity) * 100)} %`
					}
			  ];
</script>

<Group spacing="xl">
	<AssetAmount
		size="xs"
		price_size="xs"
		label="Available"
		amount={market.borrow_limit}
		price={market.price}
		short
	/>
	<AssetAmount
		size="xs"
		price_size="xs"
		label="Collaterals"
		amount={market.total_collateral_value}
		price={market.price}
		short
	/>
	<AssetAmount
		size="xs"
		price_size="xs"
		label="Loans"
		amount={market.total_loans}
		price={market.price}
		short
	/>

	<AssetAmount size="xs" price_size="xs" label="Pool usage" amount={market.usage} percent />
	<AssetAmount
		size="xs"
		price_size="xs"
		label="Interest rate"
		amount={lending_verion ? market.interest_rate * market.usage : market.interest_rate}
		percent
	/>
</Group>
<Space h="xl" />
<Progress size="xl" radius="xl" {sections} />
