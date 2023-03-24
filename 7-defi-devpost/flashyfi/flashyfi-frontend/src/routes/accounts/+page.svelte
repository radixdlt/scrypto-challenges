<!--suppress TypeScriptUnresolvedVariable -->
<script lang="ts">
    import Paper, {Title} from '@smui/paper';
    import {shortenAddress} from "$lib/utils.js";
    import accountManager from "../../lib/stores/accountManager";
    import FlashyfiAccountDialog from "./FlashyfiAccountDialog.svelte";
    import Button, {Icon, Label} from "@smui/button";
    import type {Account} from "@radixdlt/radix-dapp-toolkit";
    import FeeConfigTable from "./FeeConfigTable.svelte";
    import type {FlashyfiAccount} from "../../lib/types";
    import Accordion, {Content as AContent, Header, Panel} from '@smui-extra/accordion';
    import Tooltip, {Wrapper} from '@smui/tooltip';

    let {connectedAccounts, connectedFlashyfiAccountConfigsPromise} = accountManager

    let flashyfiAccountDialogOpen = false
    let selectedAccount: Account | null = null
    let selectedFlashyfiAccount: FlashyfiAccount | null = null

    function flashyfiAccount(account: Account) {
        selectedAccount = account
        selectedFlashyfiAccount = null
        flashyfiAccountDialogOpen = true
    }

    function updateFlashyfiAccount(account: Account, flashyfiAccount: FlashyfiAccount) {
        selectedAccount = account
        selectedFlashyfiAccount = flashyfiAccount
        flashyfiAccountDialogOpen = true
    }
</script>

<div class="content-container">
    <span class="page-heading">Manage Accounts</span>

    {#if $connectedAccounts.length > 0}
        <Accordion style="width: 700px; { flashyfiAccountDialogOpen ? 'opacity:0;':''}">
            {#each $connectedAccounts as account}
                <Panel>
                    <Header>
                        {account.label}
                        <Wrapper slot="description">
                            <span>{shortenAddress(account.address)}</span>
                            <Tooltip>{account.address}</Tooltip>
                        </Wrapper>
                    </Header>
                    <AContent>
                        <div style="display: flex;flex-direction: column; gap: 1rem">
                            {#await $connectedFlashyfiAccountConfigsPromise}
                                Loading config...
                            {:then connectedFlashyfiAccounts}
                                {#if connectedFlashyfiAccounts.has(account.address)}
                                    <div class="info">
                                        This account has been enabled for flash loans. You will earn a fee each time
                                        a user takes out a loan.<br>
                                        <br>
                                        Please refer to the table below to see which tokens and NFTs can be
                                        borrowed.
                                    </div>
                                    <FeeConfigTable
                                            account={connectedFlashyfiAccounts.get(account.address)}
                                            fungibleFeeConfigs={connectedFlashyfiAccounts.get(account.address).fungibleFeeConfigs}
                                            nonFungibleFeeConfigs={connectedFlashyfiAccounts.get(account.address).nonFungibleFeeConfigs}
                                    />
                                    <div style="align-self: end">
                                        <Button variant="raised" style="width: auto"
                                                on:click={()=>updateFlashyfiAccount(account, connectedFlashyfiAccounts.get(account.address))}>
                                            <Label>Configure</Label>
                                            <Icon class="material-icons">settings</Icon>
                                        </Button>
                                    </div>
                                {:else}
                                    <div class="info">
                                        This account has not been enabled for FlashyFi yet.
                                        Enabling FlashyFi on your account allows other users to take out flash loans
                                        from it, earning you a fee for each loan taken out.<br>
                                        <br>
                                        To enable FlashyFi, please click the button below.
                                    </div>

                                    <div style="align-self: end">
                                        <Button variant="raised" on:click={()=>flashyfiAccount(account)}>
                                            <Icon class="material-icons">bolt</Icon>
                                            <Label>Flashyfi Account</Label>
                                        </Button>
                                    </div>
                                {/if}
                            {/await}
                        </div>
                    </AContent>
                </Panel>
            {/each}
        </Accordion>
    {:else}
        <Paper>
            <Title>Please connect your Radix Wallet to manage your accounts</Title>
        </Paper>
    {/if}
</div>
{#if selectedAccount != null}
    <FlashyfiAccountDialog bind:open={flashyfiAccountDialogOpen}
                           account={selectedAccount}
                           existingFlashyfiAccount={selectedFlashyfiAccount}/>
{/if}

<style>
    .info {
        margin-top: 0;
    }
</style>