<script lang="ts">
	import { LSU, SXRD, TOKENIZER, VALIDATOR, XRD, YT } from '$lib/addresses';
	import {
		callMethod,
		depositBatch,
		takeAllFromWorktop,
		withdrawFungible,
		withdrawNonFungibles
	} from '$lib/manifest';
	import { account_store, tokenizer_store } from '$lib/stores';
	import { dec } from '$lib/utils/dec';
	import { sendTransaction } from '$lib/utils/send_tx';
	import {
		ActionIcon,
		Alert,
		AppShell,
		Badge,
		Button,
		Card,
		Checkbox,
		Container,
		Group,
		Header,
		Loader,
		RadioGroup,
		Space,
		Stack,
		SvelteUIProvider,
		Text,
		TextInput
	} from '@svelteuidev/core';
	import { Cross2, InfoCircled, Moon, Sun } from 'radix-icons-svelte';
	import { Modals, closeModal } from 'svelte-modals';
	import { get } from 'svelte/store';

	export let isDark = false;

	let yt_list: { id: string; selected: boolean }[] = [];

	const lsuOrXrd = [
		{ label: 'XRD', value: XRD },
		{ label: 'LSU', value: LSU }
	];
	let valueOfLsuOrXrd = XRD;

	const claimYieldFor = [
		{ label: 'sXRD', value: 'claim_yield_for_sxrd' },
		{ label: 'LSU', value: 'claim_yield_for_lsu' }
	];
	let valueOfClaimYieldFor = 'claim_yield_for_sxrd';

	let tokenizeAmount = '0';
	let tokenizeBusy = false;

	let mintSXRDAmount = '0';
	let mintSXRDBusy = false;

	let redeemSXRDAmount = '0';
	let redeemSXRDBusy = false;
	let noLSU = false;

	let claimYTBusy = false;

	$: anyYTSelected = yt_list.some((i) => i.selected);

	$: {
		update_yt_list($account_store?.yt_ids ?? []);
	}

	function update_yt_list(yt_ids: string[]) {
		yt_list = yt_ids.map((id) => ({ id, selected: false }));
	}

	function toggleTheme() {
		isDark = !isDark;
	}

	async function callTokenize() {
		console.log('calling tokenize');
		tokenizeBusy = true;

		let tx = '';

		if (valueOfLsuOrXrd === XRD) {
			tx += withdrawFungible(get(account_store)!.$account_address, XRD, dec(tokenizeAmount));
			tx += takeAllFromWorktop(valueOfLsuOrXrd, 'asset');
			tx += callMethod(VALIDATOR, 'stake', ['Bucket("asset")']);
		} else {
			tx += withdrawFungible(get(account_store)!.$account_address, LSU, dec(tokenizeAmount));
		}

		tx += takeAllFromWorktop(LSU, 'lsu');
		tx += callMethod(TOKENIZER, 'tokenize_yield', ['Bucket("lsu")']);
		tx += depositBatch(get(account_store)!.$account_address);

		console.log(tx);

		await sendTransaction(tx);

		tokenizeBusy = false;
	}

	async function mintSXRD() {
		console.log('calling mint sxrd');

		mintSXRDBusy = true;

		let tx = '';

		tx += withdrawFungible(get(account_store)!.$account_address, XRD, dec(mintSXRDAmount));
		tx += takeAllFromWorktop(XRD, 'xrd');
		tx += callMethod(TOKENIZER, 'mint_sxrd', ['Bucket("xrd")']);
		tx += depositBatch(get(account_store)!.$account_address);

		console.log(tx);

		await sendTransaction(tx);

		mintSXRDBusy = false;
	}

	async function redeemSXRD() {
		console.log('calling redeem sxrd');
		redeemSXRDBusy = true;

		let tx = '';

		tx += withdrawFungible(get(account_store)!.$account_address, SXRD, dec(redeemSXRDAmount));
		tx += takeAllFromWorktop(SXRD, 'sxrd');
		tx += callMethod(TOKENIZER, 'redeem_sxrd', ['Bucket("sxrd")', noLSU ? 'true' : 'false']);
		tx += depositBatch(get(account_store)!.$account_address);

		console.log(tx);

		await sendTransaction(tx);

		redeemSXRDBusy = false;
	}

	async function claimYieldToken() {
		console.log('calling claim yield token');

		claimYTBusy = true;

		let ids = yt_list.filter((i) => i.selected).map((i) => i.id);

		let tx = '';

		tx += withdrawNonFungibles(get(account_store)!.$account_address, YT, ids);
		tx += takeAllFromWorktop(YT, 'yield_token');
		tx += callMethod(TOKENIZER, valueOfClaimYieldFor, ['Bucket("yield_token")']);
		tx += depositBatch(get(account_store)!.$account_address);

		console.log(tx);

		await sendTransaction(tx);

		claimYTBusy = false;
	}
</script>

<SvelteUIProvider withGlobalStyles themeObserver={isDark ? 'dark' : 'light'}>
	<AppShell fixed>
		<Header height={75} slot="header" fixed>
			<Group p="md">
				<Text>
					Welcome to <b>Yield Tokenizer</b>
				</Text>
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

		<Container>
			<Card shadow="lg" padding="lg">
				<Group>
					<Text>Yield Tokenizer</Text>
					<Container fluid />
					<Text>Available XRD:</Text>
					<Text weight={'semibold'}>{$tokenizer_store?.$fungible_resources[XRD] ?? dec(0)}</Text>
					<Text>LSU in the pool:</Text>
					<Text weight={'semibold'}>{$tokenizer_store?.$fungible_resources[LSU] ?? dec(0)}</Text>
				</Group>
			</Card>
		</Container>

		<Space h="xl"></Space>

		{#if $account_store == undefined}
			<Container>
				<Badge color="red" size="sm" radius="sm" variant="filled">No account connected</Badge>
			</Container>
		{/if}

		<Space h="xl"></Space>

		<!-- Tokenize Yield -->

		<Container>
			<Card shadow="lg" padding="lg">
				<Stack>
					<Text weight={'bold'}>Tokenize Yield</Text>
					<Alert>
						Tokenize you LSU and get sXRD (backed 1:1 with XRD) and a Yield Token (accruing same the
						network emission as provided LSU)
					</Alert>
					<Group>
						<Text>I have :</Text>
						<RadioGroup items={lsuOrXrd} bind:value={valueOfLsuOrXrd} />
						<Text>Available :</Text>
						<Text>{$account_store?.$fungible_resources[valueOfLsuOrXrd] ?? 0}</Text>
						<Button
							variant="subtle"
							size="xs"
							compact
							on:click={() => {
								console.log($account_store?.$fungible_resources[valueOfLsuOrXrd]);
								tokenizeAmount = (
									$account_store?.$fungible_resources[valueOfLsuOrXrd] ?? dec(0)
								).toString();
							}}>Use Max</Button
						>
						{#if valueOfLsuOrXrd == LSU}
							<Button
								variant="subtle"
								size="xs"
								compact
								color="violet"
								href="https://stokenet-dashboard.radixdlt.com/network-staking/validator_tdx_2_1s0j35ansmur5q8kxem4edr23j2leutupveqc9g8kuuj29wc7uvmd8z/unstake"
								target="_blank">Get LSU by staking your XRD</Button
							>
						{/if}
					</Group>

					<Group>
						<TextInput bind:value={tokenizeAmount}>
							<svelte:fragment slot="rightSection">
								<ActionIcon on:click={() => (tokenizeAmount = '')}>
									<Cross2 size={16} />
								</ActionIcon>
							</svelte:fragment>
						</TextInput>
						<Button
							disabled={dec(tokenizeAmount).eq(dec(0))}
							on:click={tokenizeBusy ? null : callTokenize}
						>
							Tokenize Yield

							{#if tokenizeBusy}
								<Space w="md" />
								<Loader size="xs" variant="circle" color="white" />
							{/if}
						</Button>
						{#if valueOfLsuOrXrd == XRD}
							<Alert icon={InfoCircled} color="blue" size="xs">XRD will be staked to get LSU</Alert>
						{/if}
					</Group>
				</Stack>
			</Card>
		</Container>

		<Space h="xl"></Space>

		<!--Mint sXRD -->

		<Container>
			<Card shadow="lg" padding="lg">
				<Stack>
					<Text weight={'bold'}>Mint sXRD with XRD</Text>

					<Alert>
						Mint sXRD with 1:1 XRD and use them in DeFI (Providing liquidity, Take advantage of
						arbitrage opportunities ...)
					</Alert>

					<Group>
						<Text>Available XRD:</Text>
						<Text>{$account_store?.$fungible_resources[XRD] ?? 0}</Text>
						<Button
							variant="subtle"
							size="xs"
							compact
							on:click={() => {
								mintSXRDAmount = ($account_store?.$fungible_resources[XRD] ?? dec(0))
									.sub(dec(10))
									.toString();
							}}>Use Max</Button
						>
					</Group>

					<Group>
						<TextInput bind:value={mintSXRDAmount}>
							<svelte:fragment slot="rightSection">
								<ActionIcon on:click={() => (mintSXRDAmount = '')}>
									<Cross2 size={16} />
								</ActionIcon>
							</svelte:fragment>
						</TextInput>
						<Button
							disabled={dec(mintSXRDAmount).eq(dec(0))}
							on:click={mintSXRDBusy ? null : mintSXRD}
						>
							Mint sXRD
							{#if mintSXRDBusy}
								<Space w="md" />
								<Loader size="xs" variant="circle" color="white" />
							{/if}
						</Button>
					</Group>
				</Stack>
			</Card>
		</Container>

		<Space h="xl"></Space>

		<!--Redeem sXRD-->

		<Container>
			<Card shadow="lg" padding="lg">
				<Stack>
					<Text weight={'bold'}>Redeem sXRD</Text>

					<Alert>
						Redeem your sXRD for 1:1 XRD. If the pool lakes of XRD you can get LSU with equal
						redemption value, But you will pay a 3% penalty
					</Alert>

					<Group>
						<Text>Available sXRD:</Text>
						<Text>{$account_store?.$fungible_resources[SXRD] ?? 0}</Text>
						<Button
							variant="subtle"
							size="xs"
							compact
							on:click={() => {
								redeemSXRDAmount = ($account_store?.$fungible_resources[SXRD] ?? dec(0)).toString();
							}}>Use Max</Button
						>
					</Group>

					<Checkbox
						label="Redeem only for XRD (Tx will fail if there is not enough XRD)"
						bind:checked={noLSU}
					/>

					<Group>
						<TextInput bind:value={redeemSXRDAmount}>
							<svelte:fragment slot="rightSection">
								<ActionIcon on:click={() => (redeemSXRDAmount = '')}>
									<Cross2 size={16} />
								</ActionIcon>
							</svelte:fragment>
						</TextInput>
						<Button
							disabled={dec(redeemSXRDAmount).eq(dec(0))}
							on:click={redeemSXRDBusy ? null : redeemSXRD}
						>
							Redeem sXRD
							{#if redeemSXRDBusy}
								<Space w="md" />
								<Loader size="xs" variant="circle" color="white" />
							{/if}
						</Button>
					</Group>
				</Stack>
			</Card>
		</Container>

		<Space h="xl"></Space>

		<!-- Claim Yield sXRD for XRD-->

		<Container>
			<Card shadow="lg" padding="lg">
				<Stack>
					<Text weight={'bold'}>Claim Yield Token</Text>

					<Alert>
						Claim your yielded network emission. You can get sXRD that can be used or redeemed or
						you can get LSU with equal redemption value with no penalty
					</Alert>

					<Group>
						<Text>Available YT:</Text>
						<Button
							variant="subtle"
							size="xs"
							compact
							on:click={() => {
								yt_list = yt_list.map((i) => ({ ...i, selected: true }));
							}}>Use all</Button
						>
					</Group>

					<Stack>
						{#each yt_list as item, index}
							<Checkbox bind:checked={yt_list[index].selected} label={item.id} />
						{/each}
					</Stack>

					<Group>
						<Text>Claim YT for:</Text>
						<RadioGroup items={claimYieldFor} bind:value={valueOfClaimYieldFor} />

						<Button disabled={!anyYTSelected} on:click={claimYTBusy ? null : claimYieldToken}>
							Claim YT
							{#if claimYTBusy}
								<Space w="md" />
								<Loader size="xs" variant="circle" color="white" />
							{/if}
						</Button>
					</Group>
				</Stack>
			</Card>
		</Container>
	</AppShell>
</SvelteUIProvider>

<Modals>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div slot="backdrop" class="backdrop" on:click={closeModal} />
</Modals>
