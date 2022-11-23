<script lang="ts">
    import {appState} from "$lib/app_state_store";
    import balance from "$lib/balance_store";
    import Sdk from '@radixdlt/alphanet-walletextension-sdk';
    import Button, {Icon, Label as BLabel} from "@smui/button";
    import Snackbar, {Label as SLabel} from "@smui/snackbar";
    import {shortenAddress} from "$lib/utils.js";

    let connectSucceededSnackbar: Snackbar;
    let connectFailedSnackbar: Snackbar;
    let accountAddressSnackbar: Snackbar;

    const sdk = Sdk();
    const connectWallet = async () => {
        const result = await sdk.request({accountAddresses: {}});
        if (result.isErr()) {
            throw result.error;
        }
        const {accountAddresses} = result.value;
        if (accountAddresses) {
            const accountAddress = accountAddresses[0].address;
            appState.onAccountConnected(accountAddress);
            await balance.onAccountConnected(accountAddress);
            connectSucceededSnackbar.open();
        } else {
            throw Error("Cannot connect to account");
        }
    }

    const connectWalletWithTimeout = async (timeout) => {
        return Promise.race([
            connectWallet(),
            new Promise((fulfill, _) => setTimeout(() => {
                if (!$appState.accountAddress) {
                    connectFailedSnackbar.open();
                }
                fulfill("");
            }, timeout))
        ])
    }
</script>

<div class="wallet-connect">
    <div style="width: 1px;height:1px;" class="connect-succeeded"></div>
    <Snackbar bind:this={connectSucceededSnackbar} class="connect-succeeded">
        <SLabel>Connected to {$appState.accountAddress}</SLabel>
    </Snackbar>

    <Snackbar bind:this={connectFailedSnackbar} class="connect-failed">
        <SLabel>Failed to connect to account. Is the wallet extension installed and active?</SLabel>
    </Snackbar>

    <Snackbar bind:this={accountAddressSnackbar}>
        <SLabel>Connected to {$appState.accountAddress}</SLabel>
    </Snackbar>

    {#if $appState.accountAddress}
        <Button color="secondary" variant="unelevated" on:click={() => accountAddressSnackbar.open()}>
            <Icon class="material-icons">wallet</Icon>
            <BLabel>{shortenAddress($appState.accountAddress, 13, 5)}</BLabel>
        </Button>
    {:else }
        <Button color="primary" variant="raised" on:click={()=>connectWalletWithTimeout(2000)}>
            <Icon class="material-icons">login</Icon>
            <BLabel>Connect Account</BLabel>
        </Button>
    {/if}
</div>

<style lang="scss">
  @use '@material/snackbar/mixins';
  @use '@material/theme/color-palette';
  @use '@material/theme/theme-color';

  * :global {
    .mdc-snackbar.connect-succeeded {
      @include mixins.fill-color(color-palette.$green-500);
      @include mixins.label-ink-color(theme-color.accessible-ink-color(color-palette.$green-500));
    }

    .mdc-snackbar.connect-failed {
      @include mixins.fill-color(color-palette.$red-500);
      @include mixins.label-ink-color(theme-color.accessible-ink-color(color-palette.$red-500));
    }
  }
</style>
