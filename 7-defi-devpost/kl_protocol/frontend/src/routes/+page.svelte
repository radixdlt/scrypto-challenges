<script lang="ts">
	import { get_number_input } from '$lib/common';
	import Modal1 from '$lib/modals/Modal1.svelte';
	import { closeModal, openModal } from 'svelte-modals';

	function handleClick() {
		get_number_input({
			title: '',
			min: 0,
			max: 0,
			onSubmit: console.log
		});
	}
</script>

<button on:click={handleClick}>Open Modal</button>
