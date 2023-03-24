<script lang="ts">
	import LendingComponent from './LendingComponent.svelte';
	import BorrowingComponent from './BorrowingComponent.svelte';

	import { accout_ressource_state } from '$lib/state/account';
	import { Button, Container, Divider, Grid, Space } from '@svelteuidev/core';

	let cdp_id: string; //= $accout_ressource_state?.cdps[0]?.cdp_id;

	let show_lending = false;
</script>

<Container size="lg">
	{#if $accout_ressource_state !== undefined}
		<Grid>
			<Grid.Col span={6}
				><Button
					fullSize
					color="violet"
					variant={!show_lending ? 'light' : 'filled'}
					on:click={() => (show_lending = true)}>LEND</Button
				></Grid.Col
			>
			<Grid.Col span={6}
				><Button
					fullSize
					color="cyan"
					variant={show_lending ? 'light' : 'filled'}
					on:click={() => (show_lending = false)}>BORROW</Button
				></Grid.Col
			>
		</Grid>

		<Divider />

		{#if show_lending}
			<LendingComponent />
		{:else}
			<BorrowingComponent />
		{/if}
	{:else}
		No pool available
	{/if}
</Container>
