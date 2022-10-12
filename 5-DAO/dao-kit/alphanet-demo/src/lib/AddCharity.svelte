<script>
    import {ManifestBuilder} from '@radixdlt/alphanet-walletextension-sdk';

    export let sdk;
    export let transactionApi;
    export let appState;
    export let charityId;
    let charityName = "Unicef"
    let selectedOption = null
    let proposalEvaluated = false
    let proposalImplemented = false

    const proposeAddCharity = async () => {
        const doGoodDaoComponent = appState.componentAddressesByBpName["DoGoodDao"]
        const membershipResource = appState.resourceAddressesByName["Membership Badge"]
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .createProofFromAccount(appState.accountAddress, membershipResource)
            .callMethod(doGoodDaoComponent, "propose_implement_charity_change", [`Enum("CreateNewCharity", "${charityName}", ComponentAddress("${appState.accountAddress}"))`])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk.sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }

        // Fetch the receipt from the Gateway SDK
        const receipt = await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        let output = receipt.committed.receipt.output[2]
        appState.addCharityProposalId = output.data_json.fields[0].bytes
    }

    const voteOnProposal = async (optionName) => {
        const votingSystem = appState.componentAddressesByBpName["VotingSystem"]
        const membershipResource = appState.resourceAddressesByName["Membership Badge"]
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .createProofFromAccount(appState.accountAddress, membershipResource)
            .popFromAuthZone("memberBadge")
            .callMethod(votingSystem, "cast_vote", [
                `NonFungibleId("${appState.addCharityProposalId}")`,
                `"${optionName}"`,
                'Enum("NonFungible", Proof("memberBadge"))'
            ])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk.sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }

        // Fetch the receipt from the Gateway SDK
        const receipt = await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        selectedOption = optionName
    }

    const evaluateProposal = async () => {
        const votingSystem = appState.componentAddressesByBpName["VotingSystem"]
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .callMethod(votingSystem, "evaluate_vote", [`NonFungibleId("${appState.addCharityProposalId}")`])
            .build()
            .toString();

        // Send manifest to extension for signing
        const hash = await sdk.sendTransaction(manifest)
            .map((response) => response.transactionHash)

        if (hash.isErr()) {
            throw hash.error
        }

        // Fetch the receipt from the Gateway SDK
        const receipt = await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        proposalEvaluated = true
    }

    const implementProposalAndGetCharities = async () => {
        const doGoodDaoComponent = appState.componentAddressesByBpName["DoGoodDao"]
        const votingSystem = appState.componentAddressesByBpName["VotingSystem"]
        const codeExecutionSystem = appState.componentAddressesByBpName["CodeExecutionSystem"]
        const membershipResource = appState.resourceAddressesByName["Membership Badge"]
        const codeExecutionResource = appState.resourceAddressesByName["Code Execution Token"]
        let manifest = new ManifestBuilder()
            .callMethod(appState.accountAddress, "lock_fee", ['Decimal("100")'])
            .createProofFromAccount(appState.accountAddress, membershipResource)
            .popFromAuthZone("memberBadge")
            .callMethod(votingSystem, "implement_vote", [
                `NonFungibleId("${appState.addCharityProposalId}")`,
                'Proof("memberBadge")'
            ])
            .takeFromWorktop(codeExecutionResource, "codeExecutionToken")
            .callMethod(codeExecutionSystem, "execute_code", ['Bucket("codeExecutionToken")'])
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
        const receipt = await transactionApi.transactionReceiptPost({
            v0CommittedTransactionRequest: {intent_hash: hash.value},
        })
        const charities = receipt.committed.receipt.output[6].data_json
        charityId = charities.elements[0].fields[0].bytes
        proposalImplemented = true
    }
</script>

<div>
    {#if appState.componentAddressesByBpName["DoGoodDao"]}
        <h2>3. Propose adding a new charity</h2>
        <p>At the moment the DAO is "empty", i.e. no charity has been added.<br>
            Go on and propose to add one! This creates a proposal that members (only we atm) must vote on.
        </p>
        <span>Charity name: <input bind:value={charityName} disabled={appState.addCharityProposalId}></span><br>
        <button on:click={proposeAddCharity} disabled={appState.addCharityProposalId}>Propose charity</button>
        <br>
        <span>Proposal ID: {appState.addCharityProposalId}</span>
    {/if}
</div>

<div>
    {#if appState.addCharityProposalId}
        <h2>4. Vote to either reject or approve charity {charityName}</h2>
        <p>The charity has been proposed but not been added yet.<br>
        Before this can happen, the proposal has to be approved. Choose to either approve or reject the proposal.
        </p>
        <button on:click={() => voteOnProposal("approve")} disabled={proposalEvaluated}>Approve {charityName}</button>
        <button on:click={() => voteOnProposal("reject")} disabled={proposalEvaluated}>Reject {charityName}</button>
    {/if}
</div>

<div>
    {#if selectedOption != null }
        <h2>5. Evaluate the proposal, i.e. determine which option wins</h2>
        <p>After we have voted, we eventually have to evaluate the proposal, i.e. we have to check which option,<br>
            "approve" or "reject", has won.
        </p>
        {#if selectedOption === "approve" }
            <button on:click={evaluateProposal} disabled={proposalEvaluated}>Evaluate the proposal</button>
            {#if proposalEvaluated}
                <!-- The proposal will be approved. -->
                <!-- It is just too cumbersome to parse the output for the actual winning option... -->
                <br>
                <span>Proposal approved</span>
            {/if}
        {:else}
            <h3>Sorry you cannot evaluate the proposal right away if you reject it :-(</h3>
            <p>
                Proposals can only be evaluated after the voting deadline epoch has passed (so you would have to wait
                about a week...).
                Exceptions are possible if a soft deadline has been configured and it can safely be determined that
                enough
                "approve" votes have already been collected so that the proposal can always be considered "approved" no
                matter how many reject votes come in.
                <br><br>
                To learn more about this, refer to the in code documentation on enum variant
                VotingDeadline::SoftEpochDeadline.
            </p>
            <h3>To continue with this demo, change your vote to approve!</h3>
        {/if}
    {/if}
</div>

<div>
    {#if proposalEvaluated }
        <h2>6. Implement the proposal and run the associated code</h2>
        <p>Great, the proposal has been approved! Now its time to run the CodeExecution that is associated with the<br>
        winning option. In this case the "approve" option is associated with calling a method on the DAO that<br>
        actually adds the charity to the list of charities the DAO keeps track of.</p>
        <button on:click={implementProposalAndGetCharities} disabled={proposalImplemented}>Implement the proposal
        </button>
        <br>
        <span>Charity ID: {charityId}</span>
    {/if}
</div>