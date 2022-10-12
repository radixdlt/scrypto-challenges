<script>
    import Sdk from '@radixdlt/alphanet-walletextension-sdk';
    import {StateApi, TransactionApi} from '@radixdlt/alphanet-gateway-api-v0-sdk'
    import DeployDao from './lib/DeployDao.svelte'
    import AddCharity from './lib/AddCharity.svelte'
    import DonateToCharity from './lib/DonateToCharity.svelte'

    // Initialize the SDK
    const sdk = Sdk()
    const transactionApi = new TransactionApi()
    const stateApi = new StateApi()

    // Global states
    let appState = {
        accountAddress: '',
        packageAddress: 'package_tdx_a_1q9u7wryl4lj677n2sayy02qfyzpfm994vwry07jf4j3seulalw',
        initialMemberFirstName: 'John',
        initialMemberLastName: 'Doe',
        componentAddressesByBpName: {},
        resourceAddressesByName: {},
        addCharityProposalId: '',
        charityId: ''
    }
    let charityId = '';

    const updateGlobalAddresses = async (allEntityIds) => {
        for (const entity_id of allEntityIds) {
            switch (entity_id.entity_type) {
                case "Component":
                    const account_state = await stateApi.stateComponentPost({
                        v0StateComponentRequest: {component_address: entity_id.global_address}
                    })
                    appState.componentAddressesByBpName[account_state.info.blueprint_name] = entity_id.global_address
                    break;
                case "ResourceManager":
                    let resource_data = await stateApi.stateResourcePost({
                        v0StateResourceRequest: {resource_address: entity_id.global_address}
                    });
                    for (const entry of resource_data.manager.metadata) {
                        if (entry["key"] === "name") {
                            appState.resourceAddressesByName[entry["value"]] = entity_id.global_address
                        }
                    }
                    break;
            }
        }
        console.log(appState.componentAddressesByBpName)
        console.log(appState.resourceAddressesByName)
    }

    const fetchAccountAddress = async () => {
        const result = await sdk.request({
            accountAddresses: {},
        })
        if (result.isErr()) {
            throw result.error
        }
        const {accountAddresses} = result.value
        if (!accountAddresses) return
        appState.accountAddress = accountAddresses[0].address
    }
</script>

<main>
    <h1>DoGoodDao</h1>
    <div>
        <p>
            This is a demo for the DoGoodDao component, an example DAO built with the help of dao-kit.<br>
            This DAO has a board of members that researches the most effective charities and then adds their
            donation addresses to the DAO.<br>Everyday users can then make donations towards any of the listed charities
            and the DAO component automatically routes the donations to the resp. charity's account.<br>
            <br>
            The DAO keeps track of each user's donations in a "donor badge" that every donor owns.<br>
        </p>
    </div>


    <div>
        <h2>1. Fetch User Account Address</h2>
        <p>
            <button on:click={fetchAccountAddress}>Get wallet account address</button>
        </p>
        <p>Account Address: {appState.accountAddress}</p>
    </div>

    {#if !appState.accountAddress}
        <h2>Get a wallet account address to enable further interactions</h2>
    {/if}

    <DeployDao {sdk} {transactionApi} {appState} {updateGlobalAddresses}/>
    <AddCharity {sdk} {transactionApi} {appState} bind:charityId={charityId}/>
    <DonateToCharity {sdk} {transactionApi} {appState} {charityId}/>
</main>

<style>

</style>
