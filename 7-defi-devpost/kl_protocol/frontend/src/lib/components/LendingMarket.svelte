<script lang="ts">
	import type { PoolInfo } from '$lib/data';
	import { Button, Card, Container, Group, Space, Text } from '@svelteuidev/core';

	export let market: PoolInfo;
</script>

<Container>
	<Card shadow="sm" padding="lg">
		<Group>
			<!-- ASSET -->
			<Group>
				<!-- <Avatar src={market.icon} height="32" width="32" alt={market.asset_symbol} /> -->

				<Text weight={'bold'}>{market.name}</Text>
			</Group>

			<Space w="xl" />

			<!-- Market -->
			<Group>
				<Text size="md">Market</Text>
				<Text size="md" weight={'bold'}>
					<!-- {numeral(market.available_liquidity).format('0.00a')} -->
					{(market.available_liquidity ?? 0).toLocaleString('en', {
						notation: 'compact',
						compactDisplay: 'short'
					})}
				</Text>
			</Group>

			<Space w="xl" />

			<!-- Borrowed -->
			<Group>
				<Text size="md">Borrowed</Text>

				<Text size="md" weight={'bold'}>
					<!-- {numeral(market.total_loan).format('0.00a')} -->
					{(market.total_loan ?? 0).toLocaleString('en', {
						notation: 'compact',
						compactDisplay: 'short'
					})}
				</Text>
			</Group>

			<Container fluid />

			<Group>
				<Button variant="outline" color="green" size="md" uppercase override={{ width: '130px' }}
					>Lend
				</Button>

				<Button variant="outline" size="md" uppercase override={{ width: '130px' }}>Borrow</Button>
			</Group>
		</Group>
	</Card>
</Container>
