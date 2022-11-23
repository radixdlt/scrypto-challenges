<script lang="ts">

    import Dialog, {Actions, Content, Title} from '@smui/dialog';
    import Button, {Label} from '@smui/button';
    import Textfield from '@smui/textfield';
    import Icon from '@smui/textfield/icon';
    import HelperText from '@smui/textfield/helper-text';
    import Snackbar from "@smui/snackbar";

    export let onConfirm: (imageUrl: string, linkUrl: string, hoverText: string, costPerClick: number) => void;
    export let open;

    let imageUrl: string = "";
    let linkUrl: string = "";
    let hoverText: string = "";
    let costPerClick: number = 0;

    let imageUrlInvalid: boolean;
    let linkUrlInvalid: boolean;
    let hoverTextInvalid: boolean;
    let costPerClickInvalid: boolean;

    let promptSignTxSnackbar: Snackbar;

    $: confirmButtonDisabled = !imageUrl || !linkUrl || !hoverText || !costPerClick
        || imageUrlInvalid || linkUrlInvalid || hoverTextInvalid || costPerClickInvalid;

    const reset = () => {
        imageUrl = "https://api.ociswap.com/icons/128x128/ociswap.png";
        linkUrl = "https://ociswap.com/";
        hoverText = "SWAP THE MEOW-Y WAY!";
        costPerClick = 2
    }
    reset()

    const confirm = () => {
        onConfirm(imageUrl, linkUrl, hoverText, costPerClick);
        promptSignTxSnackbar.open();
        reset()
    }
</script>

<Dialog bind:open>
    <Title>Create New Ad</Title>
    <Content>
        <Textfield style="width: 500px" bind:invalid={imageUrlInvalid} label="Image URL" bind:value={imageUrl}
                   type="url" required>
            <Icon class="material-icons" slot="leadingIcon">image</Icon>
            <HelperText validationMsg slot="helper">Please enter a valid URL.</HelperText>
        </Textfield>
        <Textfield style="width: 500px" bind:invalid={linkUrlInvalid} label="Link URL" bind:value={linkUrl} type="url"
                   required>
            <Icon class="material-icons" slot="leadingIcon">link</Icon>
            <HelperText validationMsg slot="helper">Please enter a valid URL.</HelperText>
        </Textfield>
        <Textfield style="width: 500px" bind:invalid={hoverTextInvalid} label="Hover Text" bind:value={hoverText}
                   type="text" required>
            <Icon class="material-icons" slot="leadingIcon">short_text</Icon>
            <HelperText validationMsg slot="helper">Please enter some text</HelperText>
        </Textfield>
        <Textfield style="width: 500px" bind:invalid={costPerClickInvalid} label="Price Per Click"
                   bind:value={costPerClick} type="number" suffix="XRD" required>
            <Icon class="material-icons" slot="leadingIcon">payments</Icon>
            <HelperText validationMsg slot="helper">Please enter a valid XRD amount</HelperText>
        </Textfield>
    </Content>
    <Actions>
        <Button color="secondary" on:click={reset}>
            <Label>Cancel</Label>
        </Button>
        <Button color="primary" on:click={confirm} disabled={confirmButtonDisabled}>
            <Label>Create Ad</Label>
        </Button>
    </Actions>
</Dialog>

<Snackbar bind:this={promptSignTxSnackbar}>
    <Label>Please sign the transaction to create the ad</Label>
</Snackbar>

