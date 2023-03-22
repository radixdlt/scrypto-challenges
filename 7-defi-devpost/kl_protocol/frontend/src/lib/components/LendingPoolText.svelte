<script lang="ts">
	import { Card, Container, Group, Space, Text, Image } from '@svelteuidev/core';
	import type { PoolInfo } from '../models';

	export let name: string;
	export let value: number | undefined;
</script>

<Group>
	<Text size="md">{name}</Text>

	<Text size="md" weight={'bold'}>
		<!-- {numeral(market.total_loan).format('0.00a')} -->
		{(value ?? 0).toLocaleString('en', {
			notation: 'compact',
			compactDisplay: 'short'
		})}
	</Text>
</Group>
