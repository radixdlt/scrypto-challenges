<script lang="ts">
    import Dialog, {Content, Header, Title} from '@smui/dialog';
    import IconButton from "@smui/icon-button";

    export let open = false;
    export let title;

</script>

<Dialog bind:open={open}>
    <Header>
        <span>
            <Title>{title}</Title>
            <IconButton action="close" class="material-icons">close</IconButton>
        </span>
    </Header>
    <Content>
        <slot/>
    </Content>
</Dialog>

<style>
    * :global(.mdc-dialog__title) {
        flex-grow: 1;
    }

    span {
        display: flex;
        align-items: center;
    }
</style>