<!--suppress JSUnusedAssignment -->
<script lang="ts">
    import Select, {Option} from '@smui/select';
    import flashyfiRepo from "$lib/flashyfiRepo.js";
    import Chip, {Set, Text} from '@smui/chips';
    import Tooltip, {Wrapper} from '@smui/tooltip';
    import Textfield from '@smui/textfield';
    import type {FeeConfig} from "$lib/types";
    import {FeeType} from "$lib/types.js";
    import _ from "lodash"
    import Checkbox from "@smui/checkbox";

    export let feeConfigs: Array<FeeConfig>
    export let isValid: boolean
    export let hasChanges: boolean
    export let transactionProcessing: boolean

    let originalFeeConfigs = feeConfigs.map(config => Object.assign({}, config))

    $: {
        hasChanges = !_.isEqual(feeConfigs, originalFeeConfigs)
        // noinspection TypeScriptUnresolvedVariable
        isValid = feeConfigs
            .map(config => (config.feeType != null && config.feeValue != null && config.feeValue >= 0) || !config.enabled)
            .reduce((left, right) => left && right, true)
    }
</script>

<!-- IMPORTANT: Initialize from the initial fee configs so that svelte does not ruin into an infinite bind update
     loop when values are changed -->
<div class="config-grid">
    {#each originalFeeConfigs as feeConfig, index}
        {#await flashyfiRepo.getResourceDetails(feeConfig.resourceAddress) then resourceDetails}
            <div style="display: flex; align-items: center">
                <!--                    <FormField>-->
                <Checkbox bind:checked={feeConfigs[index].enabled} disabled={transactionProcessing}/>
                <Set chips={[resourceDetails.getSymbolOrNameOrShortenedAddress(15)]} let:chip nonInteractive>
                    <Wrapper>
                        <Chip {chip}>
                            <Text>{chip}</Text>
                        </Chip>
                        <Tooltip>{resourceDetails.shortenedAddress}</Tooltip>
                    </Wrapper>
                </Set>
                <!--                    </FormField>-->
            </div>

            <Select bind:value={feeConfigs[index].feeType}
                    label="Fee Type"
                    disabled>
                {#each Object.values(FeeType) as option}
                    <Option value={option}>{option}</Option>
                {/each}
            </Select>
            {#if feeConfigs[index].feeType === FeeType.FIXED}
                <Textfield bind:value={feeConfigs[index].feeValue} label="XRD Amount"
                           type="number" input$min="0" input$step="any" required
                           disabled={!feeConfigs[index].enabled || transactionProcessing}
                />
            {:else }
                <Textfield bind:value={feeConfigs[index].feeValue} label="Percent Fee"
                           type="number" input$min="0" input$step="any" required
                           disabled={!feeConfigs[index].enabled || transactionProcessing}
                />
            {/if}
        {/await}
    {/each}
</div>


<style lang="scss">
  //@use '@material/theme/theme-color';
  //@use '@material/theme/color-palette';

  .config-grid {
    display: grid;
    grid-template-columns: 40% 30% 1fr;
    grid-gap: 1rem;
  }
</style>
