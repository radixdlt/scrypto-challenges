<script lang="ts">
	import {
		Button,
		Card,
		Container,
		Divider,
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

	import { dapp_state, price_changes, reset_update } from '$lib/state/dapp';
	import { lending_pools, type PoolState } from '$lib/state/lending_pools';
	import type { ResourceMetadata } from '$lib/state/resources';
	import { resources } from '$lib/state/resources';
	import { get_pool_state } from '$lib/transactions/admin';
	import type { ResourceAddressString } from '@radixdlt/radix-dapp-toolkit';

	let isBusy = false;

	$: can_instantiate_faucet = $dapp_state.faucetComponentAddress == '';
	$: can_instantiate_pool_manager =
		!can_instantiate_faucet && $dapp_state.lendingMarketComponentAddress == '';
	$: can_create_resources =
		!can_instantiate_faucet && !can_instantiate_pool_manager && Object.keys($resources).length <= 1;
	$: can_create_lending_pools =
		!can_instantiate_faucet &&
		!can_instantiate_pool_manager &&
		!can_create_resources &&
		Object.values($lending_pools).length == 0;

	$: price_has_change = apdate_price_change($price_changes, $lending_pools);

	// $: sorted_lending_pools = $lending_pools.sort((a, b) =>
	// 	a.pool_resource_address > b.pool_resource_address ? -1 : 1
	// );

	function apdate_price_change(
		x: { [x: string]: number },
		y: Record<`resource_${string}`, PoolState>
	): boolean {
		let _p = true;

		for (const property in x) {
			_p = _p && y[property as ResourceAddressString]?.price == x[property];
		}

		return !_p;
	}

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
		dapp_state.set({ ...get(dapp_state), packageAddress: packageAddress });
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
		<AddressText label="Package address" address={$dapp_state.packageAddress} />

		<Group>
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

			<Container fluid />
			<Button variant="subtle" color="red" on:click={() => reset_update()}>Reset the DApp</Button>
		</Group>
	</Card>

	<Space h="xl" />

	<Grid align="stretch">
		<Grid.Col span={6}>
			<ComponentCard
				can_instantiate={can_instantiate_faucet}
				componentName="Faucet Component"
				creationTxHash={$dapp_state.faucetCreationTxHash}
				componentAddress={$dapp_state.faucetComponentAddress}
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
				can_instantiate={can_instantiate_pool_manager}
				componentName="KL Protocol Compoment"
				creationTxHash={$dapp_state.lendingMarketCreationTxHash}
				componentAddress={$dapp_state.lendingMarketComponentAddress}
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
		<Group direction="row">
			<Text>Resources</Text>
			<Space h="md" />
			<Text color="blue">{Object.keys($resources).length}</Text>
			<Container fluid />

			<Button on:click={() => create_resources()} disabled={!can_create_resources}>Create</Button>
		</Group>
	</Card>

	<Space h="xl" />

	<Card shadow="lg" padding="lg">
		<Group direction="row">
			<Text>Lending pools</Text>
			<Space h="md" />
			<!-- <AddressText truncated address={$persited_data.poolsCreationTxHash} /> -->
			<Container fluid />

			{#if Object.values($lending_pools).length > 0}
				<Button variant="outline" color="green" on:click={() => get_pool_state()}
					>Update pool state</Button
				>
			{/if}
			{#if price_has_change}
				<Button on:click={() => change_prices()}>Commit price changes</Button>
			{/if}

			<Button on:click={() => create_lending_pools()} disabled={!can_create_lending_pools}
				>Create</Button
			>
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

		<Divider />

		{#each Object.values($lending_pools) as item}
			<LendingPool market={item} />
			<Space h="xs" />
		{:else}
			No pools
		{/each}
	</Card>

	<Space h="xl" />
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
