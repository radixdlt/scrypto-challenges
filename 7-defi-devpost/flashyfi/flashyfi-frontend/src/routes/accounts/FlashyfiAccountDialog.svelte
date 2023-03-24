<!--suppress JSUnusedAssignment -->
<script lang="ts">
    import Dialog, {Actions, Content, Title} from '@smui/dialog';
    import Button, {Label} from '@smui/button';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import type {Account, ComponentAddressString} from "@radixdlt/radix-dapp-toolkit";
    import SendTransactionButton from "../../lib/SendTransactionButton.svelte";
    import flashyfiRepo from "$lib/flashyfiRepo.js";
    import accountManager from "../../lib/stores/accountManager";
    import type {AccountResources} from "../../lib/flashyfiRepo";
    import FeeConfigPanel from "./FeeConfigPanel.svelte";
    import {type FeeConfig, FeeType} from "$lib/types";
    import type {FlashyfiAccount} from "../../lib/types";
    import messageManager from "$lib/stores/messageManager.js";

    export let account: Account
    export let existingFlashyfiAccount: FlashyfiAccount | null

    export let open = false
    let agreed = false
    let processing = false
    let resourcesPromise: Promise<AccountResources> | null = null

    let fungibleFeeConfigsHaveChanges: boolean
    let nonFungibleFeeConfigsHaveChanges: boolean
    let fungibleFeeConfigsAreValid: boolean
    let nonFungibleFeeConfigsAreValid: boolean

    let fungibleFeeConfigs: Array<FeeConfig>
    let nonFungibleFeeConfigs: Array<FeeConfig>
    $: feeConfigsReady = createFeeConfigs(account.address)
    let canSendTransaction
    $: {
        if (existingFlashyfiAccount == null) {
            canSendTransaction = agreed && fungibleFeeConfigsAreValid && nonFungibleFeeConfigsAreValid
                && (fungibleFeeConfigs.length > 0 || nonFungibleFeeConfigs.length > 0)
        } else {
            canSendTransaction = (fungibleFeeConfigsHaveChanges || nonFungibleFeeConfigsHaveChanges)
                && fungibleFeeConfigsAreValid && nonFungibleFeeConfigsAreValid

        }
    }

    function resetAndClose() {
        open = false
        agreed = false
        processing = false
        resourcesPromise = null
    }

    async function createFeeConfigs(accountAddress: string) {
        fungibleFeeConfigs = existingFlashyfiAccount?.fungibleFeeConfigs?.map(config => Object.assign({}, config)) ?? []
        nonFungibleFeeConfigs = existingFlashyfiAccount?.nonFungibleFeeConfigs?.map(config => Object.assign({}, config)) ?? []

        const resources = await flashyfiRepo.getAllAccountResources(accountAddress as ComponentAddressString)
        // noinspection TypeScriptUnresolvedVariable
        const existingFungibleFeeConfigAddresses = new Set(fungibleFeeConfigs.map(config => config.resourceAddress))
        for (const resource of resources.fungibles) {
            if (existingFungibleFeeConfigAddresses.has(resource.address)) continue
            fungibleFeeConfigs.push({
                enabled: false,
                resourceAddress: resource.address,
                feeType: FeeType.FIXED,
                feeValue: null
            })
        }

        // noinspection TypeScriptUnresolvedVariable
        const existingNonFungibleFeeConfigAddresses = new Set(nonFungibleFeeConfigs.map(config => config.resourceAddress))
        for (const resource of resources.nonFungibles) {
            if (existingNonFungibleFeeConfigAddresses.has(resource.address)) continue
            // Don't allow a user to lend his account badge
            if (resource.address == (await flashyfiRepo.getFlashyfiAddresses()).accountConfigBadgeResource) continue
            nonFungibleFeeConfigs.push({
                enabled: false,
                resourceAddress: resource.address,
                feeType: FeeType.FIXED,
                feeValue: null
            })
        }
    }

    async function createTransactionManifest(accountAddress: string): Promise<string> {
        if (existingFlashyfiAccount == null) {
            return flashyfiRepo.createManifestFlashyfiAccount(
                accountAddress as ComponentAddressString,
                fungibleFeeConfigs,
                nonFungibleFeeConfigs
            )
        } else {
            return flashyfiRepo.createManifestUpdateFlashyfiAccount(
                accountAddress as ComponentAddressString,
                fungibleFeeConfigs,
                nonFungibleFeeConfigs
            )
        }
    }

    async function refreshFlashyfiedAccounts() {
        accountManager.setAllFlashyfiAccounts(flashyfiRepo.getAllFlashyfiedAccounts())
    }
</script>

<Dialog bind:open
        scrimClickAction=""
        escapeKeyAction=""
        surface$style="width: 700px; max-width: calc(100vw - 32px); max-height: calc(100vh - 80px - 25px - 32px); margin-top: 80px">
    <Title>
        {#if existingFlashyfiAccount == null}
            Flashyfi account "{account.label}"
        {:else }
            Update account "{account.label}"
        {/if}
    </Title>
    <Content>
        {#if existingFlashyfiAccount == null}
            {#if !agreed}
                <p>
                    By <i>flashyfying</i> your account, you allow other users to borrow tokens from your account via
                    flash loans for a fee that you set.
                </p>

                <p>
                    The borrowed tokens are only lent for the duration of a transaction and never permanently leave your
                    account. This is guaranteed by the Radix network.
                </p>

                <p>
                    Please ensure that you do not configure tokens for lending that are used as badges, and are meant to
                    give only you access to secured applications.
                </p>
            {:else}
                <p>
                    By <i>flashyfying</i> your account, you allow other users to borrow tokens from your account via
                    flash loans for a fee...
                </p>
            {/if}
            <FormField>
                <Checkbox bind:checked={agreed}/>
                <span slot="label">I have read the warning and understand the implications</span>
            </FormField>
        {/if}
        {#if agreed || existingFlashyfiAccount != null}
            <hr>
            {#await feeConfigsReady then _}
                <h3>Borrowable Fungible Tokens:</h3>
                <FeeConfigPanel bind:feeConfigs={fungibleFeeConfigs}
                                feeTypeSelectable={true}
                                bind:isValid={fungibleFeeConfigsAreValid}
                                bind:hasChanges={fungibleFeeConfigsHaveChanges}
                                transactionProcessing={processing}/>
                {#if nonFungibleFeeConfigs.length > 0}
                    <hr>
                    <h3>Borrowable NFTs:</h3>
                    <div class="warning">
                        NFTS are often used as access badges. Only lend out NFTs that do not allow other users to
                        impersonate you when using secured applications!
                    </div>
                {/if}
                <FeeConfigPanel bind:feeConfigs={nonFungibleFeeConfigs}
                                feeTypeSelectable={false}
                                bind:isValid={nonFungibleFeeConfigsAreValid}
                                bind:hasChanges={nonFungibleFeeConfigsHaveChanges}
                                transactionProcessing={processing}/>

                {#if fungibleFeeConfigs.length === 0 && nonFungibleFeeConfigs.length === 0 }
                    <div class="warning">
                        You do not have any tokens or NFTs in your account
                    </div>
                {/if}
            {/await}
        {/if}
    </Content>
    <Actions>
        <Button disabled={processing} on:click={resetAndClose}>
            <Label>Cancel</Label>
        </Button>
        <SendTransactionButton
                text={existingFlashyfiAccount==null ?"Flashyfi account" : "Update account"}
                icon={existingFlashyfiAccount==null ? "bolt" : null}
                buildManifestFn={() =>createTransactionManifest(account.address)}
                onBeforeTransactionSent={()=>processing=true}
                onTransactionSucceeded={_committedResponse => {
                    resetAndClose();
                    // Closing the dialog via the open prop leads to missing scrollbars on the website.
                    // This is fixed by the following line. Note that this is a workaround and the proper way
                    // would probably be to let the button emit the proper close event
                    document.body.style.overflow = "auto";
                    refreshFlashyfiedAccounts()
                    messageManager.showMessage(
                        existingFlashyfiAccount==null ?"Your account has been flashyfied" : "Your account has been updated",
                        "Success"
                    )
                }}
                onTransactionFailed={_statusResponse => {
                    processing=false
                    messageManager.showMessage("Transaction failed", "Error")
                }}
                disabled={!canSendTransaction}
        />
    </Actions>
</Dialog>


<style>
    .warning {
        margin-bottom: 1rem;
    }
</style>