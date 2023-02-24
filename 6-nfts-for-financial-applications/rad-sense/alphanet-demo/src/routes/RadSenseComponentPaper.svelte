<script lang="ts">
    import RadSenseRepo from "../lib/RadSenseRepo";
    import Button from '@smui/button';
    import Paper, {Content as PaperContent} from '@smui/paper';
    import LayoutGrid, {Cell} from '@smui/layout-grid';
    import {appState} from "$lib/app_state_store";
    import {shortenAddress} from "$lib/utils.js";
    import AdBrokerCard from "./AdBrokerCard.svelte";
    import AdSlotProviderCard from "./AdSlotProviderCard.svelte";
    import AdvertiserCard from "./AdvertiserCard.svelte";
    import EventsApiCard from "./EventsApiCard.svelte";
    import {page} from "$app/stores";

    const radSenseRepo = new RadSenseRepo($page.url.protocol, $page.url.host)

    async function instantiateRadSenseComponent(e: Event) {
        const result = await radSenseRepo.instantiateRadSense($appState.accountAddress);
        appState.onRadSenseComponentInstantiated(result.radSenseComponent, result.rsa, result.arbitrationDaoAddresses);
    }
</script>

<Paper>
    {#if !$appState.radSenseComponent}
        <PaperContent>
            <Button on:click={instantiateRadSenseComponent} variant="raised">
                Instantiate RadSense component
            </Button>
        </PaperContent>
    {:else}
        <PaperContent>
            <h2 class="mdc-typography--headline6" style="margin: 0;">RadSense component</h2>
            <h3 class="mdc-typography--subtitle2" style="margin: 0 0 10px; color: #888;">
                Address: {shortenAddress($appState.radSenseComponent, 15, 5)}
            </h3>
            <div class="rad-sense-description">
                <p>
                    The RadSense blueprint offers a decentralized approach to advertising, where instead of one big
                    company like google, many smaller companies may provide advertisement services.
                </p>
                <p>
                    Please continue with the demo by registering an ad broker, an ad slot provider and an advertiser.
                    After you have registered the three users, you can create as many ads and ad slots as you like.
                    Remember that you have to sign a transaction for every action you take here, as everything is
                    written to the Radix Alphanet network. Please also note that you have to pay for an ad's budget up
                    front so do not choose high cost per click values for you ads or you will quickly drain your
                    account!
                </p>
                <p>
                    After you have registered at least one ad slot and one ad, you can click the icon next to the slot
                    which will take you to a dummy site that is a stand in for a real website where an ad slot provider
                    has embedded an ad slot. Observe that the ad slot renders whichever ads you have registered. The ad
                    slot requests the ads from a mock ad broker API which is co-hosted with this demo.
                </p>
                <p>
                    Every time an ad is rendered or clicked, an event is sent to the tracking APIs of each of the three
                    users. It is therefore expected that all three tables on the bottom display the same data.</p>
            </div>
            <LayoutGrid>
                <Cell>
                    <AdBrokerCard/>
                </Cell>
                <Cell>
                    <AdSlotProviderCard/>
                </Cell>
                <Cell>
                    <AdvertiserCard/>
                </Cell>
                <Cell>
                    {#if $appState.adBrokerId}
                        <EventsApiCard user="AdBroker" heading="Ad Broker Event API Log"/>
                    {/if}
                </Cell>
                <Cell>
                    {#if $appState.adSlotProviderId}
                        <EventsApiCard user="AdSlotProvider" heading="Ad Slot Provider Event API Log"/>
                    {/if}
                </Cell>
                <Cell>
                    {#if $appState.advertiserId}
                        <EventsApiCard user="Advertiser" heading="Advertiser Event API Log"/>
                    {/if}
                </Cell>
            </LayoutGrid>
        </PaperContent>
    {/if}
</Paper>


<style>
    .rad-sense-description {
        width: 50%
    }

    :global(.mdc-layout-grid) {
        padding: 0;
    }
</style>
