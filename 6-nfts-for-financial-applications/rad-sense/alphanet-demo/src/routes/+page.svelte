<script lang="ts">
    import {appState} from "$lib/app_state_store";
    import Card, {Content} from '@smui/card';
    import Header from "./Header.svelte";
    import RadSenseComponentPaper from "./RadSenseComponentPaper.svelte";
    import {onMount} from "svelte";

    onMount(async () => {
        await appState.reset()
        await fetch("./mocks/tracking_api/ALL/events", {method: "DELETE"});
    })
</script>

<svelte:head>
    <title>RadSense</title>
    <meta content="Svelte demo app" name="description"/>
</svelte:head>

<Header/>

<br>

{#if $appState.accountAddress}
    <RadSenseComponentPaper/>
{:else}
    <section>
        <Card variant="outlined" padded>
            <Content><h1>Please connect an account to proceed with the demo</h1></Content>
        </Card>
    </section>
{/if}


<style>
    section {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        flex: 0.6;
    }

    h1 {
        width: 100%;
    }
</style>
