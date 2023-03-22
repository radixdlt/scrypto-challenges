<script lang="ts">
	import { dapp_data, price_changes } from '$lib/data';
	// import type { PoolInfo } from '$lib/models';
	import {
		Button,
		Card,
		Container,
		Grid,
		Group,
		Loader,
		Modal,
		Space,
		Text,
		TextInput
	} from '@svelteuidev/core';
	import { field, form } from 'svelte-forms';
	import { required } from 'svelte-forms/validators';
	import { get } from 'svelte/store';

	import AddressText from '$lib/components/AddressText.svelte';
	import {
		change_prices,
		create_lending_pools,
		create_resources,
		instantiate_faucet,
		instantiate_lending_market,
		load_faucet_receipt,
		load_lending_market_receipt
	} from '$lib/transactions/admin';
	import ComponentCard from './components/ComponentCard.svelte';
	import LendingPool from './components/LendingPool.svelte';
	import { lending_pools, resources } from '$lib/state/pool_state';
	import type { PoolState, ResourceMetadata } from '$lib/state/types';
	import type { ResourceAddressString } from '@radixdlt/radix-dapp-toolkit';

	let isBusy = false;

	// $: sorted_lending_pools = $lending_pools.sort((a, b) =>
	// 	a.pool_resource_address > b.pool_resource_address ? -1 : 1
	// );

	function apdate_price_change(
		x: { [x: string]: number },
		y: Record<`resource_${string}`, ResourceMetadata>
	): boolean {
		let _p = true;

		for (const property in x) {
			_p = _p && y[property as ResourceAddressString]?.price == x[property];
		}

		return !_p;
	}

	$: price_has_change = apdate_price_change($price_changes, $resources);

	async function launch_action(action: (param: any) => Promise<void>, param: any) {
		try {
			isBusy = true;
			await action(param);
		} catch (error) {
			//
		} finally {
			isBusy = false;
		}
	}

	async function launch_instaciate_faucet() {
		try {
			isBusy = true;
			await instantiate_faucet();
		} catch (error) {
			//
		} finally {
			isBusy = false;
		}
	}

	async function launch_instantiate_lending_market() {
		try {
			isBusy = true;
			await instantiate_lending_market();
		} catch (error) {
			//
		} finally {
			isBusy = false;
		}
	}

	async function save_packge_adress(packageAddress: string) {
		dapp_data.set({ ...get(dapp_data), packageAddress: packageAddress });
	}

	// Form

	const address = field('address', '', [required()]);
	const myForm = form(address);

	let formData: {
		label: string;
		placeholder: string;
		formAction: (arg0: string) => Promise<void>;
	} = { label: '', placeholder: '', formAction: async (arg0) => {} };

	let isModalOpened = false;

	let submit_form = async () => {
		console.log($address.value);

		await formData.formAction($address.value);

		closeModal();
		myForm.reset();
	};

	function openForm(newFormData: typeof formData) {
		myForm.validate();

		isModalOpened = true;

		formData = newFormData;
	}

	function closeModal() {
		isModalOpened = false;
		myForm.reset();
	}

	///

	let copied = false;
</script>

<Container>
	<Card shadow="lg" padding="lg">
		<!-- <Group direction="column" align="start" grow> -->
		<Group>
			<AddressText label="Package address" address={$dapp_data.packageAddress} />

			<Button
				variant="subtle"
				on:click={() =>
					openForm({
						label: 'Package address',
						placeholder: 'Enter package address received Radix Dashboard',
						formAction: save_packge_adress
					})}
			>
				New Package Address</Button
			>
		</Group>
	</Card>

	<Space h="xl" />

	<Grid align="stretch">
		<Grid.Col span={6}>
			<ComponentCard
				componentName="Faucet Component"
				creationTxHash={$dapp_data.faucetCreationTxHash}
				componentAddress={$dapp_data.faucetComponentAddress}
				launch_instaciate={launch_instaciate_faucet}
				load_receipt={() =>
					openForm({
						label: 'Faucet Component tx hash',
						placeholder: 'Faucet component instatiation tx hash',
						formAction: (txhash) => launch_action(load_faucet_receipt, txhash)
					})}
			/>
		</Grid.Col>
		<Grid.Col span={6}>
			<ComponentCard
				componentName="KL Protocol Compoment"
				creationTxHash={$dapp_data.lendingMarketCreationTxHash}
				componentAddress={$dapp_data.lendingMarketComponentAddress}
				launch_instaciate={launch_instantiate_lending_market}
				load_receipt={() =>
					openForm({
						label: 'Lending Component tx hash',
						placeholder: 'Lending component instatiation tx hash',
						formAction: (txhash) => launch_action(load_lending_market_receipt, txhash)
					})}
			/>
		</Grid.Col>
	</Grid>

	<Group p={8} grow>
		{#if isBusy}
			<Loader />
		{/if}
	</Group>

	<Card shadow="lg" padding="lg">
		<Text>Resources</Text>
		<Space h="md" />
		<Group direction="row">
			<!-- <AddressText truncated address={$persited_data.assetsCreationTxHash} /> -->
			<Container fluid />
			<Button on:click={() => create_resources()}>Create</Button>
			<!-- <Button
				on:click={() =>
					openForm({
						label: 'Resources creation tx hash',
						placeholder: 'Resources creation tx hash',
						formAction: (txhash) => launch_action(load_resource_creation_receipt, txhash)
					})}
			>
				Load tx hash
			</Button> -->
		</Group>
	</Card>

	<Space h="xl" />

	<Card shadow="lg" padding="lg">
		<Text>Lending pools</Text>
		<Space h="md" />
		<Group direction="row">
			<!-- <AddressText truncated address={$persited_data.poolsCreationTxHash} /> -->
			<Container fluid />
			<Button on:click={() => create_lending_pools()}>Create</Button>
			<!-- <Button
				on:click={() =>
					openForm({
						label: 'Pools creation creation tx hash',
						placeholder: 'Pools creation tx hash',
						formAction: (txhash) => launch_action(load_pool_creation_receipt, txhash)
					})}
			>
				Load tx hash
			</Button> -->
		</Group>
	</Card>

	<Space h="xl" />

	<Group direction="row" px="xl">
		<Text>Lending Pools</Text>
		<Space h={36} />
		<Container fluid />

		{#if price_has_change}
			<Button on:click={() => change_prices()}>Commit price changes</Button>
		{/if}
	</Group>
	<Space h="xl" />

	{#each $lending_pools as item}
		<LendingPool market={item} />
		<Space h="xs" />
	{:else}
		No resource
	{/each}
</Container>

<Modal opened={isModalOpened} on:close={closeModal} target={'body'}>
	<TextInput
		placeholder={formData.placeholder}
		label={formData.label}
		bind:value={$address.value}
	/>

	<Space h="md" />

	<Group direction="row">
		<Container fluid />

		<Button color="green" disabled={!$myForm.valid} on:click={submit_form}>OK</Button>
		<!-- <Button color="orange" on:click={closeModal}>Cancel</Button> -->
	</Group>
</Modal>
