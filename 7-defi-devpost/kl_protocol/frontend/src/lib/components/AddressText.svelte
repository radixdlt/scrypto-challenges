<script lang="ts">
	import { truncate } from '$lib/utils';
	import { clipboard } from '@svelteuidev/composables';
	import { ActionIcon, Container, Group, Text, Tooltip } from '@svelteuidev/core';
	import { Copy } from 'radix-icons-svelte';

	let copied = false;
	let onCopy = () => {
		copied = true;
		setTimeout(function () {
			copied = false;
		}, 1000);
	};

	export let address: string;
	export let label: string = '';
	export let truncated = false;
</script>

<!-- <Button use={[[clipboard, $persited_data.packageAddress]]} color={copied ? 'green' : 'blue'}>
			{copied ? 'copied' : 'Click me to copy text'}
		</Button> -->

{#if address != ''}
	<Group>
		{#if label != ''}
			<Text p={0}>{label}</Text>
		{/if}

		<Text p={0}
			>{truncated ? truncate(address, 12, 6) : address}

			<Tooltip opened={copied} closeDelay={500} label="Copied !">
				<ActionIcon use={[[clipboard, address]]} on:useclipboard={onCopy} p={0}>
					<Copy size={12} />
				</ActionIcon>
			</Tooltip>
		</Text>
	</Group>
{/if}
