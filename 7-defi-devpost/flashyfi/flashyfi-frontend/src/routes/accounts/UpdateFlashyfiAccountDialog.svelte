<script lang="ts">
    import Dialog, {Actions, Content, Title} from '@smui/dialog';
    import Button, {Label} from '@smui/button';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import type {Account, ComponentAddressString} from "@radixdlt/radix-dapp-toolkit";
    import SendTransactionButton from "$lib/SendTransactionButton.svelte";
    import flashyfiRepo from "$lib/flashyfiRepo.js";
    import accountManager from "../../lib/stores/accountManager";
    import type {AccountResources} from "../../lib/flashyfiRepo";
    import FeeConfigPanel from "./FeeConfigPanel.svelte";
    import {type FeeConfig, FeeType} from "$lib/types";

    export let account: Account | null
    // export let flashyfiAccount: FlashyfiAccount | null

    export let open = false
    let agreed = true
    let processing = false
    let resourcesPromise: Promise<AccountResources> | null = null

    let fungiblesFeeConfigHasChanges: boolean
    let fungiblesFeeConfigIsValid: boolean

    let fungibleFeeConfigs: Array<FeeConfig>
    let nonFungibleFeeConfigs: Array<FeeConfig>
    $: feeConfigsReady = account == null
        ? new Promise<void>(() => {
        }) //
        : createFeeConfigs(account.address)

    function resetAndClose() {
        open = false
        agreed = false
        processing = false
        resourcesPromise = null
    }

    async function createFeeConfigs(accountAddress: string) {
        const resources = await flashyfiRepo.getAllAccountResources(accountAddress as ComponentAddressString)
        fungibleFeeConfigs = resources.fungibles.map(resource => {
            return {
                enabled: false,
                resourceAddress: resource.address,
                feeType: FeeType.PERCENTAGE,
                feeValue: 0.1
            }
        })
        nonFungibleFeeConfigs = resources.nonFungibles.map(resource => {
            return {
                enabled: false,
                resourceAddress: resource.address,
                feeType: FeeType.FIXED,
                feeValue: 1
            }
        })
    }


    async function createManifestFlashyfiAccount(accountAddress: string): Promise<string> {
        return flashyfiRepo.createManifestFlashyfiAccount(accountAddress as ComponentAddressString, fungibleFeeConfigs, nonFungibleFeeConfigs)

    }

    async function refreshFlashyfiedAccounts() {
        accountManager.setAllFlashyfiAccounts(flashyfiRepo.getAllFlashyfiedAccounts())
    }
</script>

{#if account}
    <Dialog bind:open scrimClickAction="" escapeKeyAction="">
        <Title>Flashyfi account "{account.label}"</Title>
        <Content id="simple-content">
            {#if !agreed}
                <p>
                    By <i>flashyfying</i> your account, you agree that any user can borrow tokens from your account
                    through flash loans for a fee that you set.
                </p>
                <p>
                    The tokens are only lent for the duration of a transaction and never permanently
                    leave your account. This is guaranteed by the Radix network.
                </p>
                <p>
                    Please make sure not to configure tokens for
                    lending that are used as badges and are meant to give only you access to secured
                    applications!
                </p>
            {:else}
                <p>
                    By <i>flashyfying</i> your account, you agree that any user can borrow tokens from your account
                    through flash loans for a fee ...
                </p>
            {/if}
            <FormField>
                <Checkbox bind:checked={agreed}/>
                <span slot="label">I have read the warning and understand the implications</span>
            </FormField>

            {#if agreed}
                <hr>
                {#await feeConfigsReady then _}
                    <h3>Borrowable Fungible Tokens:</h3>
                    <FeeConfigPanel bind:feeConfigs={fungibleFeeConfigs}
                                    feeTypeSelectable={true}
                                    bind:isValid={fungiblesFeeConfigIsValid}/>
                    <hr>
                    <h3>Borrowable NFTs:</h3>
                    <div class="warning">
                        NFTS are often used as access badges. Only lend out NFTs that do not allow other users to
                        impersonate you when using secured applications!
                    </div>
                    <FeeConfigPanel bind:feeConfigs={nonFungibleFeeConfigs}
                                    feeTypeSelectable={false}
                                    bind:isValid={fungiblesFeeConfigIsValid}/>
                {/await}
                <div class="spacing"></div>
            {/if}
        </Content>
        <Actions>
            <Button disabled={processing} on:click={resetAndClose}>
                <Label>Cancel</Label>
            </Button>
            <SendTransactionButton
                    text="Flashyfi account" icon="bolt"
                    buildManifestFn={() =>createManifestFlashyfiAccount(account.address)}
                    onTransactionSent={()=>processing=true}
                    onTransactionSucceeded={_committedResponse => {resetAndClose(); refreshFlashyfiedAccounts()}}
                    onTransactionFailed={statusResponse => processing=false}
                    disabled={!agreed || !fungiblesFeeConfigIsValid}
            />
        </Actions>
    </Dialog>
{/if}

<style>
    .spacing {
        height: 5rem;
    }

    .warning {
        color: var(--mdc-theme-on-error);
        background-color: var(--mdc-theme-error);
        border: 1px solid var(--mdc-theme-error);
        border-radius: 10px;
        padding: 1rem;
    }
</style>