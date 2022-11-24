<script lang="ts">
    import Card, {ActionButtons, ActionIcons, Actions, Content as CardContent,} from '@smui/card';
    import IconButton from '@smui/icon-button';
    import {shortenAddress} from "$lib/utils.js";
    import InfoDialog from "./InfoDialog.svelte";
    import {fade} from "svelte/transition";

    export let title;
    export let userId;

    let infoDialogOpen = false;
</script>

<div class="card-container">
    <Card variant="outlined">
        <CardContent class="mdc-typography--body2">
            <h2 class="mdc-typography--headline6" style="margin: 0;">{title}</h2>

            <h3 class="mdc-typography--subtitle2" style="margin: 0 0 10px; color: #888;">
                ID:
                {#if userId}
                    {#key userId}
                    <span transition:fade>
                        {shortenAddress(userId, 5, 5)}
                    </span>
                    {/key}
                {:else}
                    &lt;Not registered yet&gt;
                {/if}
            </h3>

            {#if userId}
                <div transition:fade>
                    <slot name="user-details" />
                </div>
            {/if}
        </CardContent>

        <Actions>
            <ActionButtons>
                <slot name="actions"/>
            </ActionButtons>
            <ActionIcons>
                <IconButton on:click={() => infoDialogOpen=true} class="material-icons">info
                </IconButton>
            </ActionIcons>
        </Actions>
    </Card>
</div>

<InfoDialog bind:open={infoDialogOpen} title="{title}">
    <slot name="info-dialog-content"/>
</InfoDialog>

<style>
</style>
