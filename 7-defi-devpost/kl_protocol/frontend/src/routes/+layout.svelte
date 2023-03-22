<script lang="ts">
	// import '../app.postcss';
	import '$lib/api/rdt';

	import { ActionIcon, Button, Container, Group, SvelteUIProvider, Text } from '@svelteuidev/core';
	import { Moon, Sun } from 'radix-icons-svelte';

	import { AppShell, Header } from '@svelteuidev/core';
	import type { Market } from '$lib/models';

	export let isDark = false;
	function toggleTheme() {
		isDark = !isDark;
	}

	let btc: Market = {
		asset_name: 'BTC',
		asset_symbol: 'Bitcoin',
		available_liquidity: 28834,
		total_loan: 7492,
		icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/1.png'
	};

	let eth: Market = {
		asset_name: 'ETH',
		asset_symbol: 'Ethereum',
		available_liquidity: 12343,
		total_loan: 5678,
		icon: 'https://s2.coinmarketcap.com/static/img/coins/64x64/1027.png'
	};
</script>

<SvelteUIProvider withGlobalStyles themeObserver={isDark ? 'dark' : 'light'}>
	<AppShell>
		<Header height={75} slot="header">
			<Group p="md">
				<Text>SuperLend</Text>
				<Button variant="subtle" href="/">Home</Button>
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
