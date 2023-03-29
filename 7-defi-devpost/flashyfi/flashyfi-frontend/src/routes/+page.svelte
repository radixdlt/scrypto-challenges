<script lang="ts">
    import flashyfiRepo from "$lib/flashyfiRepo";
    import type {Account, ResourceAddressString} from "@radixdlt/radix-dapp-toolkit";
    import {FeeType, type FlashyfiAccount, ResourceDetails} from "../lib/types";
    import Paper, {Title} from '@smui/paper';
    import Textfield, {Input} from "@smui/textfield";
    import {slide} from 'svelte/transition';
    import Slider from '@smui/slider';

    import {Icon} from '@smui/common';
    import Button from "@smui/button";
    import BorrowManifestDialog from "./BorrowManifestDialog.svelte";
    import accountManager from "../lib/stores/accountManager";
    import Select, {Option} from '@smui/select';
    import {shortenAddress} from "$lib/utils.js";
    import LoadingIndicator from "$lib/LoadingIndicator.svelte";

    const {connectedAccounts, allFlashyfiAccountsPromise, borrowableResourcesPromise} = accountManager

    type Resource = { label: string, shortAddress?: string }


    async function getBorrowableResources(): Promise<Array<Resource>> {
        const resourceAddresses = await $borrowableResourcesPromise

        let resources = []
        for (const resourceAddress of resourceAddresses) {
            const resource = await flashyfiRepo.getResourceDetails(resourceAddress as ResourceAddressString)
            // In this demo, users can only borrow fungible tokens
            if (resource.fungible) {
                resources.push(resource)
            }
        }

        return resources
    }

    async function getAccountsWithSelectedResource(resource: ResourceDetails | undefined): Promise<Array<FlashyfiAccount> | null> {
        if (!resource) return null
        const accounts = await $allFlashyfiAccountsPromise
        return accounts.filter(account => {
            if (!account.withdrawMethodsAreAccessible) {
                // Guard against low level adversarial behavior or unfortunate circumstances where an account although
                // it has been flashyfied, is not accessible
                return false
            }

            let feeConfigs = resource.fungible ? account.fungibleFeeConfigs : account.nonFungibleFeeConfigs
            for (const feeConfig of feeConfigs) {
                if (feeConfig.enabled
                    && feeConfig.feeType == FeeType.FIXED // This demo only supports fixed fees
                    && feeConfig.resourceAddress === resource.address) {
                    return true
                }
            }
            return false
        })
    }

    function filterResources(resources: Array<ResourceDetails>, searchText: string) {
        if (!searchText) {
            return resources
        }

        const lowerSearchText = searchText.toLowerCase()
        return resources.filter(resource => {
            return resource.address.toLowerCase().includes(lowerSearchText)
                || (resource.symbol && resource.symbol.toLowerCase().includes(lowerSearchText))
                || (resource.name && resource.name.toLowerCase().includes(lowerSearchText))
        })
    }

    async function getBorrowableAmount(resource: ResourceDetails, borrowerAccount: Account): Promise<number> {
        let borrowableAmount = 0
        const accounts: Array<FlashyfiAccount> = (await accountsWithSelectedResourcePromise)!
        for (const account of accounts) {
            // Do not borrow from the account that is the borrower
            if (account.accountAddress === borrowerAccount.address) continue
            const fungibleResource = account.availableFungibleResources.get(resource.address)!
            borrowableAmount += parseFloat(fungibleResource.amount.value)
        }

        return borrowableAmount
    }

    function reset() {
        selectedResource = undefined;
        searchText = "";
        selectedBorrowAmount = 0;
    }

    let selectedResource: ResourceDetails | undefined

    $: accountsWithSelectedResourcePromise = getAccountsWithSelectedResource(selectedResource)

    let searchText = ""


    let searchField
    let searchFieldFocused = false
    let borrowManifestDialogOpen: boolean
    let selectedBorrowerAccount: Account
    let borrowableAmountPromise: Promise<number>;

    $: if (!selectedBorrowerAccount && $connectedAccounts.length > 0) {
        selectedBorrowerAccount = $connectedAccounts[0]
    }
    $: if (selectedResource && selectedBorrowerAccount) {
        borrowableAmountPromise = getBorrowableAmount(selectedResource, selectedBorrowerAccount)
    }
    let selectedBorrowAmount = 0
</script>

<div class="content-container">
    <span class="page-heading">Lightning Fast Liquidity</span>

    {#if $connectedAccounts.length > 0}
        {#await getBorrowableResources()}
            <LoadingIndicator text="Loading App Data" size="128px"/>
        {:then resources}
            {#if !selectedResource}
                <div out:slide|local={{duration: 150}}>
                    <Paper class="solo-paper search" elevation={6}>
                        <Input
                                bind:this={searchField}
                                bind:value={searchText}
                                on:focus={() => {searchFieldFocused=true; searchText=""}}
                                on:blur={() => searchFieldFocused=false}
                                placeholder="Select a resource to borrow"
                                class="solo-input"
                        />
                    </Paper>
                </div>
            {/if}
            {#if searchFieldFocused}
                <div in:slide|local="{{duration:100}}">
                    <Paper class="solo-paper results" elevation={6} style="margin-top: 0.1rem;">
                        <div class="selectable-resource-list">
                            {#each filterResources(resources, searchText) as matchingResource}
                                <div class="selectable-resource" on:mousedown|preventDefault={() => {
                                selectedResource = matchingResource
                                searchText = matchingResource.getLabel()
                                searchField.blur()
                            }}>
                                    {matchingResource.getLabel()}
                                </div>
                            {:else}
                                <div class="no-selectable-resource" on:mousedown|preventDefault={() => {
                                selectedResource= undefined
                                searchField.blur()
                            }}>
                                    No resources available
                                </div>
                            {/each}
                        </div>
                    </Paper>
                </div>
            {/if}

            {#if selectedResource && !searchFieldFocused}
                <div in:slide|local={{duration: 150, delay: 150}}>
                    <Paper class="solo-paper resource" elevation={6} style="margin-top: 0.1rem">

                        <div style="width: 100%">
                            <h3>Borrow {selectedResource.getLabel(50)}</h3>
                            <div style="display: flex; gap: 1rem">
                                <Select
                                        label="Select your active account"
                                        style="width: 50%"
                                        key={(account) => account?.address ?? ""}
                                        bind:value={selectedBorrowerAccount}
                                        on:click={()=> selectedBorrowAmount=0 }
                                >
                                    {#each $connectedAccounts as account (account.address)}
                                        <Option value={account}>{account.label} ({shortenAddress(account.address)})
                                        </Option>
                                    {/each}
                                </Select>
                                {#await borrowableAmountPromise then borrowableAmount}
                                    <Textfield bind:value={selectedBorrowAmount}
                                               type="number"
                                               label="Amount to borrow"
                                               style="width: 50%"
                                               input$min={0} input$max={borrowableAmount} input$step="any"/>
                                {/await}
                            </div>
                            {#await borrowableAmountPromise then borrowableAmount}
                                <div style="display: flex;align-items: center;">
                                    {#if borrowableAmount > 0}
                                        <Slider bind:value={selectedBorrowAmount}
                                                min={0} max={borrowableAmount} step={0.001}
                                                style="flex-grow: 1; margin: 0 1rem 0 0"/>
                                        {borrowableAmount}
                                    {:else}
                                        <div class="warning" style="width: 100%; margin-bottom: 1rem">
                                            This resource is not available in any other accounts
                                        </div>
                                    {/if}
                                </div>
                                <div style="width: 100%; display: flex; justify-content: end; gap: 1rem">
                                    <Button variant="unelevated"
                                            color="secondary"
                                            on:click={reset}>
                                        Back
                                    </Button>

                                    <Button variant="unelevated"
                                            disabled={selectedBorrowAmount===0 || selectedBorrowAmount > borrowableAmount}
                                            on:click={() => { borrowManifestDialogOpen=true }}>
                                        <Icon class="material-icons">code</Icon>
                                        Create manifest
                                    </Button>
                                </div>
                            {/await}
                        </div>
                    </Paper>
                </div>
            {/if}
        {/await}
    {:else}
        <Paper>
            <Title>Please connect your Radix Wallet to start borrowing tokens</Title>
        </Paper>
    {/if}
</div>

{#if borrowManifestDialogOpen}
    {#await accountsWithSelectedResourcePromise then accountsWithResource}
        <BorrowManifestDialog
                bind:open={borrowManifestDialogOpen}
                resource={selectedResource}
                amount={selectedBorrowAmount}
                lenderAccounts={accountsWithResource.filter(account=> account.accountAddress!==selectedBorrowerAccount.address) }
                borrowerAccount={selectedBorrowerAccount}
                resetPageFn={reset}
        />
    {/await}
{/if}

<!--suppress CssUnusedSymbol -->
<style>
    * :global(.solo-paper) {
        display: flex;
        align-items: center;
        flex-grow: 1;
        width: 650px;
    }

    :global(.solo-paper.search) {
        height: 48px;
        margin: 0 1rem;
        padding: 0 1rem;
    }

    * :global(.solo-paper.search > *) {
        display: inline-block;
        margin: 0 12px;
    }

    * :global(.solo-input) {
        flex-grow: 1;
        color: var(--mdc-theme-on-surface, #000);
    }

    * :global(.solo-input::placeholder) {
        color: var(--mdc-theme-on-surface, #000);
        opacity: 0.6;
    }

    :global(.solo-paper.results) {
        margin: 0 1rem;
        padding: 0 1rem;
        overflow-y: auto;
    }

    :global(.selectable-resource-list) {
        width: calc(100% + 2rem);
        margin: 0 -1rem;
    }

    :global(.selectable-resource, .no-selectable-resource) {
        width: calc(100% - 4rem);
        padding: 0.5rem 2rem 0.5rem;
        display: flex;
    }

    :global(.selectable-resource:hover, .no-selectable-resource:hover) {
        color: var(--mdc-theme-primary);
        background-color: rgba(42, 127, 191, 0.2);
        cursor: pointer;
    }

    :global(.solo-paper.resource) {
        margin: 0 0;
        padding: 1rem 1rem;
        /*min-height: 100px;*/
    }

    :global(.solo-paper.manifest) {
        margin: 0 0;
        padding: 1rem 1rem;
        min-height: 100px;
    }
</style>

