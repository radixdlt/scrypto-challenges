<script lang="ts">
    import RadSenseRepo from "$lib/RadSenseRepo";
    import {appState} from "$lib/app_state_store";
    import Button, {Label} from '@smui/button';
    import {page} from "$app/stores";
    import UserCard from "./UserCard.svelte";
    import InfoDialog from "./InfoDialog.svelte";

    const radSenseRepo = new RadSenseRepo($page.url.protocol, $page.url.host)

    async function registerAdBroker() {
        const adBrokerId = await radSenseRepo.registerAdBroker($appState.accountAddress, $appState.radSenseComponent);
        appState.onAdBrokerRegistered(adBrokerId);
    }

    let createInvoiceDialogOpen = false;
</script>

<UserCard title="Ad Broker" bind:userId={$appState.adBrokerId}>
    <div slot="actions">
        {#if $appState.adBrokerId}
            {#if $appState.adIds.length > 0 && $appState.adSlotIds.length > 0}
                <Button on:click={() => createInvoiceDialogOpen=true}>
                    <Label>Create Invoice</Label>
                </Button>
            {/if}
        {:else}
            <Button on:click={registerAdBroker} variant="raised">
                <Label>Register Ad Broker</Label>
            </Button>
        {/if}
    </div>


    <div slot="info-dialog-content">
        <p>
            Ad Brokers have the important task of bringing advertisers and ad slot providers together. Their API
            is called whenever an ad slot on a website is about to be rendered and some appropriate ad content must be
            provided. The ad broker will query their operational database and select the most appropriate ad for the
            current slot.
        </p>
        When matching ads and slots, brokers will take into consideration:
        <ul>
            <li>any tags/topics that are associated with potential ads</li>
            <li>any tags/topics that are associated with the ad slot that is currently being rendered</li>
            <li>any information that is available on the visitor that is seeing the ad</li>
            <li>information such as the ads' cost per click and the ads' click through rates</li>
        </ul>
    </div>
</UserCard>

<InfoDialog title="Create Invoice" bind:open={createInvoiceDialogOpen} fullscreen={false}>
    <p>Sorry, creating an invoice is not supported in this demo :-(</p>
    <p>Please see the automated tests to get an idea of how creating and handling an invoice works!</p>
</InfoDialog>

<style>
</style>
