<script lang="ts">
    import Dialog, {Actions, Content, Title} from '@smui/dialog';
    import Button, {Label} from '@smui/button';
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';
    import Snackbar from "@smui/snackbar";

    export let onConfirm: (width: number, height: number) => void;
    export let open;

    let width: number = 0;
    let height: number = 0;

    let widthInvalid: boolean;
    let heightInvalid: boolean;

    let promptSignTxSnackbar: Snackbar;

    $: confirmButtonDisabled = !width || !height || widthInvalid || heightInvalid || width <= 0 || height <= 0;

    const reset = () => {
        width = 128;
        height = 128;
    }
    reset()

    const confirm = () => {
        onConfirm(width, height);
        promptSignTxSnackbar.open();
        reset()
    }
</script>

<Dialog bind:open>
    <Title>Create New Ad Slot</Title>
    <Content>
        <Textfield bind:invalid={widthInvalid} label="Width" bind:value={width} type="number" suffix="px" required>
            <HelperText validationMsg slot="helper">Please enter a valid width.</HelperText>
        </Textfield>
        <Textfield bind:invalid={heightInvalid} label="Height" bind:value={height} type="number" suffix="px" required>
            <HelperText validationMsg slot="helper">Please enter a valid height.</HelperText>
        </Textfield>
    </Content>
    <Actions>
        <Button color="secondary" on:click={reset}>
            <Label>Cancel</Label>
        </Button>
        <Button color="primary" on:click={confirm} disabled={confirmButtonDisabled}>
            <Label>Create Ad Slot</Label>
        </Button>
    </Actions>
</Dialog>

<Snackbar bind:this={promptSignTxSnackbar}>
    <Label>Please sign the transaction to create the ad slot</Label>
</Snackbar>


