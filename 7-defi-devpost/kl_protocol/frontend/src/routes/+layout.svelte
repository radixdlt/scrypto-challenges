<script lang="ts">
	import '$lib/api/rdt';
	import {
		ActionIcon,
		AppShell,
		Button,
		Container,
		Group,
		Header,
		SvelteUIProvider,
		Text
	} from '@svelteuidev/core';
	import { Moon, Sun } from 'radix-icons-svelte';
	import { closeModal, Modals } from 'svelte-modals';

	export let isDark = false;

	function toggleTheme() {
		isDark = !isDark;
	}
</script>

<SvelteUIProvider withGlobalStyles themeObserver={isDark ? 'dark' : 'light'}>
	<AppShell fixed>
		<Header height={75} slot="header" fixed>
			<Group p="md">
				<Text>KL Protocol</Text>
				<Button variant="subtle" href="/app">App</Button>
				<Button variant="subtle" href="/faucet">Faucet</Button>
				<Button variant="subtle" href="/admin">Admin</Button>
				<Container fluid />
				<ActionIcon color="blue" variant="outline" on:click={toggleTheme} size={32}>
					{#if isDark}
						<Sun size={16} />
					{:else}
						<Moon size={16} />
					{/if}
				</ActionIcon>

				<!-- <Switch on:change={toggleTheme} /> -->

				<radix-connect-button />
			</Group>
		</Header>
		<slot />
	</AppShell>
</SvelteUIProvider>

<Modals>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div slot="backdrop" class="backdrop" on:click={closeModal} />
</Modals>
