<script lang="ts">
    import Dialog, {Header, Title, Content, Actions} from '@smui/dialog';
    import IconButton from '@smui/icon-button';
    import Button, {Label} from '@smui/button';
    import {type FlashyfiAccount, ResourceDetails} from "../lib/types";
    import Textfield from "@smui/textfield";
    import {calculateLoan} from "../lib/loanCalculator";
    import type {Account, ComponentAddressString} from "@radixdlt/radix-dapp-toolkit";
    import SendTransactionButton from "$lib/SendTransactionButton.svelte";
    import messageManager from "../lib/stores/messageManager";
    import {FAUCET_DISPENSE_AMOUNT} from "$lib/constants.js";

    export let open
    export let resource: ResourceDetails
    export let amount: number

    export let borrowerAccount: Account
    export let lenderAccounts: Array<FlashyfiAccount>

    export let resetPageFn: () => void

    export const loanDefinition = calculateLoan(resource, amount, lenderAccounts)

    let originalManifest = ""
    $: manifest = originalManifest

    let response = 'Nothing yet.';

    async function createManifest() {
        originalManifest = await loanDefinition.generateManifest(borrowerAccount.address as ComponentAddressString)
    }

    let processing = false
    let dialog: Dialog
</script>

<Dialog
        bind:this={dialog}
        bind:open
        scrimClickAction=""
        escapeKeyAction=""
        fullscreen>
    <Header>
        <Title id="fullscreen-title">Borrow {resource.getLabel()}</Title>
    </Header>
    <Content id="fullscreen-content">
        <div class="container">
            <div style="display: flex; height: 2rem; align-items: center">
            <span>
                An amount of <b>{amount} {resource.getLabel()}</b>
                will be borrowed from <b>{loanDefinition.borrowInstructions.length}</b>
                account
                {#if loanDefinition.borrowInstructions.length > 1}s{/if}
                incurring a <b>fee</b> of <b>{loanDefinition.totalFee} XRD</b>
            </span>
                <div style="margin-left: auto">
                    {#if manifest !== originalManifest}
                        <Button on:click={() => manifest=originalManifest}> Reset</Button>
                    {/if}
                </div>
            </div>
            {#await createManifest()}
                Building manifest...
            {:then _}
                <Textfield
                        textarea
                        style="width: 100%; height: 50vh; background-color: var(--mdc-theme-background);"
                        input$style="white-space: pre; overflow-x: scroll; font-family: monospace"
                        bind:value={manifest}
                        disabled={processing}
                        input$autofocus={false}
                />
            {/await}
            {#if loanDefinition.totalFee > FAUCET_DISPENSE_AMOUNT}
                <div class="warning">
                    WARNING: This transaction cannot succeed because the fee is higher than the "arbitrage" provided by
                    the faucet in the second part of the manifest.<br>
                    Please note that calling the faucet twice will not work.
                </div>
            {/if}
            {#if loanDefinition.borrowInstructions.length > 12}
                <div class="warning">
                    WARNING: You are borrowing from many accounts at the same time. This may result in exceeding the
                    networks fee limit, leading to a failed transaction.
                </div>
            {/if}
        </div>
    </Content>
    <Actions>
        <Button disabled={processing}>
            <Label>Cancel</Label>
        </Button>
        <SendTransactionButton
                text="Submit manifest"
                icon="send"
                buildManifestFn={() =>manifest}
                onBeforeTransactionSent={()=>processing=true}
                onTransactionSucceeded={_committedResponse => {
                    open=false
                    // Closing the dialog via the open prop leads to missing scrollbars on the website.
                    // This is fixed by the following line. Note that this is a workaround and the proper way
                    // would probably be to let the button emit the proper close event
                    document.body.style.overflow = "auto";
                    messageManager.showMessage("Transaction succeeded","Success")
                    resetPageFn()
                }}
                onTransactionFailed={_statusResponse => {
                    processing=false
                    messageManager.showMessage("Transaction failed", "Error")
                }}
        />
    </Actions>
</Dialog>

<style>
    .container {
        display: flex;
        flex-direction: column;
    }
</style>