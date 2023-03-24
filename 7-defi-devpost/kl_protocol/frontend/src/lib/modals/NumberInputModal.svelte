<script lang="ts">
	import { Button, Container, Group, Modal, NumberInput, Space } from '@svelteuidev/core';
	import { closeModal } from 'svelte-modals';

	// provided by <Modals />
	export let isOpen: boolean;
	export let title: string;
	export let min: number;
	export let max: number;
	export let step: number = 0.01;
	export let precision: number = 18;
	let input: number;

	export let onclick: Function;
	// export let message: string;
</script>

{#if isOpen}
	<Modal centered opened={isOpen} size="xs" on:close={closeModal} {title}>
		<NumberInput
			description={`From ${min} to ${max}`}
			defaultValue={0}
			bind:value={input}
			{min}
			{max}
			{step}
			{precision}
		/>
		<Space h="md" />

		<Group>
			<Button on:click={() => (input = max)}>Max</Button>
			<Button on:click={() => (input = max / 2)}>half</Button>
			<Container fluid />
			<Button
				color="green"
				on:click={async () => {
					closeModal();
					await onclick(input);
				}}>OK</Button
			>
		</Group>
	</Modal>
{/if}
