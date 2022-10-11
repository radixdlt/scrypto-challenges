<script>
    import {ManifestBuilder} from '@radixdlt/alphanet-walletextension-sdk';

    export let sdk;
    export let transactionApi;
    export let appState;
    export let charityId;
    let donorRegistered = false
    let donationMade = false

    const registerAsADonor = async () => {
        const doGoodDaoComponent = appState.componentAddressesByBpName["DoGoodDao"]
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .callMethod(doGoodDaoComponent, "register_as_donor", [])
            .callMethod(appState.accountAddress, "deposit_batch", ['Expression("ENTIRE_WORKTOP")'])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk.sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }

        // Fetch the receipt from the Gateway SDK
        await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        donorRegistered = true
    }

    const makeDonation = async () => {
        const doGoodDaoComponent = appState.componentAddressesByBpName["DoGoodDao"]
        const donorBadge = appState.resourceAddressesByName["Donor Badge"]
        const xrdToken = "resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9"
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .createProofFromAccount(appState.accountAddress, donorBadge)
            .popFromAuthZone("donorBadge")
            .withdrawFromAccountByAmount(appState.accountAddress, 1, xrdToken)
            .takeFromWorktop(xrdToken, "donation")
            .callMethod(doGoodDaoComponent, "make_donation", [
                `NonFungibleId("${charityId}")`,
                'Bucket("donation")',
                'Proof("donorBadge")'
            ])
            .callMethod(doGoodDaoComponent, "get_charities", [])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk.sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }

        // Fetch the receipt from the Gateway SDK
        await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        donationMade = true
    }
</script>

<div>
    {#if charityId}
        <h2>7. Register as a donor</h2>
        <p>Now that the charity has been added, we can register ourselves as donor...</p>
        <button on:click={registerAsADonor} disabled={donorRegistered}>Register as a donor</button>
    {/if}

    {#if donorRegistered}
        <h2>8. Make a donation</h2>
        <p>...and make a small donation towards the new charity</p>
        <button on:click={makeDonation}>Donate 1 XRD</button>
    {/if}

    {#if donationMade}
        <h3>Donation made! This concludes the demo. Thank you for your time!</h3>
    {/if}

</div>
