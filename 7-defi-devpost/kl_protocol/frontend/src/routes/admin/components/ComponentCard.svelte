<script lang="ts">
	import AddressText from '$lib/components/AddressText.svelte';
	import { truncate } from '$lib/utils';
	import { Button, Card, Container, Divider, Group, Space, Text } from '@svelteuidev/core';

	export let can_instantiate: boolean;
	export let componentName: string;
	export let creationTxHash: string;
	export let componentAddress: string;
	export let launch_instaciate: () => void;
	export let load_receipt: () => void;
</script>

<Card shadow="lg" padding="lg">
	<Text>
		{componentName}
	</Text>
	<Space h={30} />

	<Group>
		<Group direction="column">
			<AddressText truncated address={creationTxHash} />
			<AddressText truncated address={componentAddress} />
		</Group>
		<Container fluid />
	</Group>
	<Divider />
	<Group direction="row">
		<Container fluid />
		<Button on:click={launch_instaciate} disabled={!can_instantiate}>Instatiate</Button>
		<Button on:click={load_receipt}>Load tx hash</Button>
	</Group>
</Card>
