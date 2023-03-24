<script lang="ts">
    import RadSenseRepo from "$lib/RadSenseRepo";
    import {appState} from "$lib/app_state_store";
    import Button, {Label} from '@smui/button';
    import {shortenAddress} from "$lib/utils.js";
    import CreateAdDialog from "./CreateAdDialog.svelte";
    import {page} from "$app/stores";
    import UserCard from "./UserCard.svelte";
    import snarkdown from 'snarkdown'

    const radSenseRepo = new RadSenseRepo($page.url.protocol, $page.url.host)

    async function registerAdvertiser() {
        const advertiserId = await radSenseRepo.registerAdvertiser($appState.accountAddress, $appState.radSenseComponent);
        appState.onAdvertiserRegistered(advertiserId);
    }

    async function registerAd(imageUrl: string, linkUrl: string, hoverText: string, costPerClick: number) {
        const adId = await radSenseRepo.registerAd($appState.accountAddress, $appState.radSenseComponent, $appState.rsa,
            $appState.advertiserId, $appState.adBrokerId, imageUrl, linkUrl, hoverText, costPerClick);
        appState.onAdRegistered(adId);
    }

    let createAdDialogOpen = false;
    let advertiserDescription = `

    `
</script>

<UserCard title="Advertiser" bind:userId={$appState.advertiserId}>
    <div slot="user-details">
        Advertisements:
        <ul>
            {#each $appState.adIds as adId}
                <li>
                    <div style="display: flex; align-items: center;">
                        <span>Ad {shortenAddress(adId, 5, 5)}</span>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
    <div slot="actions">
        {#if $appState.advertiserId}
            <Button on:click={() => {createAdDialogOpen=true}} disabled={!$appState.adBrokerId}>
                <Label>Register Ad</Label>
            </Button>
        {:else}
            <Button on:click={registerAdvertiser} variant="raised">
                <Label>Register Advertiser</Label>
            </Button>
        {/if}
    </div>
    <div slot="info-dialog-content">
        <p>
            Advertisers are users with a product or service to advertise. They have the marketing materials (i.e.
            images, videos)
            but lack advertising space and traffic.
        </p>
        <p>
            Advertisers may register ads with the RadSense component, specifying a media item for their ad, a landing
            page and the
            cost per click (CPC). Whenever one of their ad is clicked they have to pay this amount to the ad slot provider
            who hosted
            their ad.
        </p>
    </div>
</UserCard>

<CreateAdDialog bind:open={createAdDialogOpen}
                onConfirm={(imageUrl, linkUrl,hoverText, costPerClick) => {
                    registerAd(imageUrl,linkUrl,hoverText,costPerClick)
                }}/>

<style>
    * :global(.open-add-slot) {
        color: #000;
    }
</style>
