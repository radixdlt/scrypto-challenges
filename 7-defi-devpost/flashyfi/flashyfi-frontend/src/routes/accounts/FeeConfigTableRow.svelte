<script lang="ts">
    import {Cell, Row} from "@smui/data-table";
    import flashyfiRepo from "$lib/flashyfiRepo.js";
    import type {FeeConfig, FlashyfiAccount} from "../../lib/types";
    import Chip, {Set, Text} from "@smui/chips";
    import Tooltip, {Wrapper} from "@smui/tooltip";
    import {FeeType} from "$lib/types.js";

    export let feeConfig: FeeConfig
    export let account: FlashyfiAccount
</script>
<Row class="{feeConfig.enabled ? 'fee-config-enabled' : 'fee-config-disabled'}">
    {#await flashyfiRepo.getResourceDetails(feeConfig.resourceAddress) then resourceDetails}
        <Cell>
            <Set chips={[resourceDetails.getSymbolOrNameOrShortenedAddress(15)]} let:chip nonInteractive>
                <Wrapper>
                    <Chip {chip}>
                        <Text>{chip}</Text>
                    </Chip>
                    <Tooltip>{resourceDetails.shortenedAddress}</Tooltip>
                </Wrapper>
            </Set>
        </Cell>
        <Cell numeric>
            {#if resourceDetails.fungible}
                {parseFloat(account.availableFungibleResources.get(resourceDetails.address).amount.value).toFixed(2)}
            {:else}
                {account.availableNonFungibleResources.get(resourceDetails.address).amount}
            {/if}
        </Cell>
        <Cell>
            {#if feeConfig.feeType === FeeType.PERCENTAGE}
                Percentage
            {:else if feeConfig.feeType === FeeType.FIXED}
                Fixed
            {:else}
                -
            {/if}
        </Cell>
        <Cell numeric>{feeConfig.feeValue ?? "-"}</Cell>
        <Cell style="text-align: right">
            {#if feeConfig.enabled}
                Enabled
            {:else}
                Disabled
            {/if}
        </Cell>
    {/await}
</Row>

<style lang="scss">
  :global(.fee-config-disabled > *) {
    //background-color:
    color: var(--mdc-theme-text-disabled-on-background);
  }
</style>