<script lang="ts">
    import Card, {Content as CardContent} from '@smui/card';
    import {onDestroy, onMount} from "svelte";
    import EventsTable from "./EventsTable.svelte";
    import Dialog, {Content, Header, Title} from '@smui/dialog';
    import IconButton from "@smui/icon-button";

    export let user: string;
    export let heading: string;

    let timer: NodeJS.Timer;
    let events: Array<Event> = [];
    let dialogOpen = false;

    onMount(setTimer);
    onDestroy(() => {
        clearInterval(timer);
    });

    function setTimer() {
        clearInterval(timer);
        timer = setInterval(async () => {
            let response = await fetch(`./mocks/tracking_api/${user}/events`);
            events = await response.json();
        }, 1000);
    }
</script>

<div class="card-container">
    <Card variant="outlined">
        <CardContent class="mdc-typography--body2">
            <div class="card-header">
                <h2 class="card-title mdc-typography--headline6" style="margin: 0;">{heading}</h2>
                <IconButton class="material-icons" on:click={()=>{dialogOpen=true}} size="button">open_in_full
                </IconButton>
            </div>
            <br/>
            <EventsTable bind:events/>
        </CardContent>
    </Card>
</div>

<Dialog bind:open={dialogOpen} fullscreen>
    <Header>
        <Title>{heading}</Title>
        <IconButton action="close" class="material-icons">close</IconButton>
    </Header>
    <Content>
        <EventsTable bind:events/>
    </Content>
</Dialog>


<style>
    .card-header {
        display: flex;
        align-items: center;
    }

    .card-title {
        flex-grow: 1;
    }

    .card-container {
        display: inline-block;
        width: 100%;
    }
</style>
