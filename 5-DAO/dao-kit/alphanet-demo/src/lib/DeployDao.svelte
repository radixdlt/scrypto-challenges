<script>
    import {ManifestBuilder} from '@radixdlt/alphanet-walletextension-sdk';

    export let sdk;
    export let transactionApi;
    export let appState;
    export let updateGlobalAddresses;

    const instantiate_component = async () => {
        let initialMembers = `Vec<Tuple>(Tuple(Struct("${appState.initialMemberFirstName}", "${appState.initialMemberFirstName}"), ComponentAddress("${appState.accountAddress}")))`;
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .callFunction(appState.packageAddress, "DoGoodDao", "instantiate_global", [initialMembers])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk
            .sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) throw hash.error

        // Fetch the receipt from the Gateway SDK
        const receipt = await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        let entityIds = receipt.committed.receipt.state_updates.new_global_entities;
        await updateGlobalAddresses(entityIds);
    }
</script>

<div class="card">
    {#if appState.accountAddress}
        <h2>2. Instantiate a DoGoodDao component</h2>
        <p>We start by instantiating the DAO. We have to specify at least one initial member.<br>The demo is set up such
            that the membership badge is directly deposited into the current account.<br>
            This makes us the one and only member of the DAO. Having multiple members would of course be great but is a
            bit pointless in a one-user demo.<br>
            <br>
            Please note that the package ABI and WASM have already been pre-deployed to the below package address!
        </p>
        <span>Package address: <input value={appState.packageAddress}></span><br>
        <span>Initial member first and last name: <input value={appState.initialMemberFirstName}>  <input
                value={appState.initialMemberLastName}></span><br>
        <button on:click={instantiate_component} disabled={appState.componentAddressesByBpName["DoGoodDao"]!=null}>Instantiate DoGoodDao</button>
        <p>DoGoodDao Component: {appState.componentAddressesByBpName["DoGoodDao"] || ""}</p>
    {/if}
</div>
