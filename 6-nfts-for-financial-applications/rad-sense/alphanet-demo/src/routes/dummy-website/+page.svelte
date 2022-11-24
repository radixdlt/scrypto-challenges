<script lang="ts">
    import {page} from "$app/stores";
    import AdSlotComponent from "$lib/AdSlot/AdSlotComponent.svelte";
    import Button from "@smui/button";
    import IconButton, {Icon} from '@smui/icon-button';
    import CircularProgress from '@smui/circular-progress';
    import {onDestroy, onMount} from "svelte";

    let timer: NodeJS.Timer;
    let progress = 0;
    let timerRunning = true;

    onMount(reset);

    onDestroy(() => {
        clearInterval(timer);
    });

    function reset() {
        progress = 0;
        clearInterval(timer);
        timer = setInterval(() => {
            if (timerRunning) {
                progress += 0.01;
            }

            if (progress >= 1) {
                clearInterval(timer);
                location.reload();
            }
        }, 100);
    }
</script>

<div class="ad-slot-container">
    <h1>AdSlot {$page.url.searchParams.get("adSlotId")}</h1>
    <AdSlotComponent adSlotResourceAddress={$page.url.searchParams.get("adSlotResource")}
                     adSlotNonFungibleId={$page.url.searchParams.get("adSlotId")}
    />
    <span class="button-container">
        <Button on:click={()=>location.reload()} elevated>Reload</Button>
        <IconButton class="material-icons" size="button" toggle bind:pressed={timerRunning}>
             <Icon class="material-icons" on>pause</Icon>
             <Icon class="material-icons">play_arrow</Icon>
        </IconButton>
    </span>
    <CircularProgress style="height: 48px; width: 48px;" {progress} {closed}/>
</div>

<style>
    .ad-slot-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .button-container {
        display: flex;
    }
</style>
