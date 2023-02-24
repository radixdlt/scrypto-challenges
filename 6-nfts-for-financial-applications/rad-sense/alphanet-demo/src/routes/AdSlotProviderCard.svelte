<script lang="ts">
    import RadSenseRepo from "$lib/RadSenseRepo";
    import {appState} from "$lib/app_state_store";
    import {Icon} from '@smui/icon-button';
    import Button, {Label} from '@smui/button';
    import {shortenAddress} from "$lib/utils.js";
    import CreateAdSlotDialog from "./CreateAdSlotDialog.svelte";
    import {page} from "$app/stores";
    import UserCard from "./UserCard.svelte";

    const radSenseRepo = new RadSenseRepo($page.url.protocol, $page.url.host)

    async function registerAdSlotProvider() {
        const adSlotProviderId = await radSenseRepo.registerAdSlotProvider($appState.accountAddress, $appState.radSenseComponent);
        appState.onAdSlotProviderRegistered(adSlotProviderId);
    }

    async function registerAdSlot(width: number, height: number) {
        const adSlotId = await radSenseRepo.registerAdSlot($appState.accountAddress, $appState.radSenseComponent,
            $appState.rsa, $appState.adSlotProviderId, $appState.adBrokerId, width, height);
        appState.onAdSlotRegistered(adSlotId);
    }

    let createAdSlotDialogOpen = false;
</script>

<UserCard title="Ad Slot Provider" bind:userId={$appState.adSlotProviderId}>
    <div slot="user-details">
        Ad Slots:
        <ul>
            {#each $appState.adSlotIds as adSlotId}
                <li>
                    <div style="display: flex; align-items: center;">
                        Slot {shortenAddress(adSlotId, 5, 5)}
                        <a href="/dummy-website?adSlotResource={$appState.rsa.ad_slot_resource}&adSlotId={adSlotId}&adBrokerResourceHack={$appState.rsa.ad_broker_resource}&advertiserResourceHack={$appState.rsa.advertiser_resource}&adSlotProviderResourceHack={$appState.rsa.ad_slot_provider_resource}&adResourceHack={$appState.rsa.ad_resource}"
                           target="_blank" rel="noreferrer">
                            <Icon class="material-icons open-add-slot">open_in_new</Icon>
                        </a>
                    </div>
                </li>
            {/each}
        </ul>
    </div>

    <div slot="actions">
        {#if $appState.adSlotProviderId}
            <Button on:click={() => createAdSlotDialogOpen=true} disabled={!$appState.adBrokerId}>
                <Label>Register Ad Slot</Label>
            </Button>
        {:else}
            <Button on:click={registerAdSlotProvider} variant="raised">
                <Label>Register Ad Slot Provider</Label>
            </Button>
        {/if}
    </div>

    <div slot="info-dialog-content">
        <p>Ad slot providers offer their website (and their traffic) to advertisers and allow ad brokers to choose which
            ads should be placed on their website. Ad slot providers must register an ad slot with the RadSense
            component, supplying information such as the topic of the website and size constraints of the slot on the
            website.
        </p>
        <p>
            They must then embed a pre-made web component on their website for rendering the ad slot. This component
            must be initialized with the "on-chain" address of the ad slot (which is represented by an NFR).
        </p>
    </div>
</UserCard>

<CreateAdSlotDialog bind:open={createAdSlotDialogOpen} onConfirm={(width, height) => { registerAdSlot(width,height) }}/>

<style lang="scss">
  @use '@material/theme/color-palette';
  @use '@material/theme/theme-color';

  * :global {
    .open-add-slot {
      color: var(--mdc-text-button-label-text-color, var(--mdc-theme-primary, #ff3e00));
    }
  }
</style>
