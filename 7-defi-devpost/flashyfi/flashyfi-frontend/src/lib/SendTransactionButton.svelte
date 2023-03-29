<script lang="ts">
    import Button, {Icon, Label} from "@smui/button";
    import radixDappToolkit from "./radixDappToolkit";
    import CircularProgress from "@smui/circular-progress";
    import {TransactionApi} from "@radixdlt/babylon-gateway-api-sdk";

    export let text: string
    export let icon: string | null = null
    export let buildManifestFn: () => Promise<String>
    export let onBeforeTransactionSent: () => void
    export let onTransactionSucceeded: (TransactionCommittedDetailsResponse) => void
    export let onTransactionFailed: (SendTransactionErrorResponse) => void

    export let disabled: boolean = false

    const transactionApi = new TransactionApi();

    async function sendTransaction() {
        transactionRunning = true
        onBeforeTransactionSent()
        const manifest = await buildManifestFn()
        console.log(manifest.toString())

        const sendResponse = await radixDappToolkit.sendTransaction({transactionManifest: manifest, version: 1})
        if (sendResponse.isErr()) {
            transactionRunning = false
            onTransactionFailed(sendResponse.error)
            return
        }

        const intentHash = sendResponse.value.transactionIntentHash
        let statusResponse = await transactionApi.transactionStatus({
            transactionStatusRequest: {
                intent_hash_hex: intentHash
            }
        });
        switch (statusResponse.status) {
            case "committed_failure":
            case "pending":
            case "rejected":
            case "unknown":
                transactionRunning = false
                onTransactionFailed(statusResponse)
                return
        }

        let commitReceipt = await transactionApi.transactionCommittedDetails({
            transactionCommittedDetailsRequest: {
                transaction_identifier: {
                    type: 'intent_hash',
                    value_hex: intentHash
                }
            }
        })
        transactionRunning = false
        onTransactionSucceeded(commitReceipt)
    }

    let transactionRunning = false
</script>

{#if transactionRunning}
    <Button variant="raised" disabled>
        <CircularProgress style="height: 1.5rem; width: 1.5rem" indeterminate/>&nbsp;Processing
    </Button>
{:else}
    <Button variant="raised" on:click={sendTransaction} disabled="{disabled}">
        {#if icon}
            <Icon class="material-icons">{icon}</Icon>
        {/if}
        <Label>{text}</Label>
    </Button>
{/if}
