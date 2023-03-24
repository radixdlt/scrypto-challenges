<script lang="ts">
	import { Loader, Modal } from '@svelteuidev/core';
	import { closeModal } from 'svelte-modals';

	// provided by <Modals />
	export let isOpen: boolean;

	export let title: string;
	// export let message: string;
</script>

{#if isOpen}
	<Modal centered opened={isOpen} withCloseButton={false} size="xs" {title}>
		<Loader size="lg" />
	</Modal>
{/if}
