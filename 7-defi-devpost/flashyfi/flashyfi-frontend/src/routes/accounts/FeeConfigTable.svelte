<script lang="ts">
    import DataTable, {Body, Cell, Head, Row} from '@smui/data-table';
    import type {FeeConfig, FlashyfiAccount} from "../../lib/types";
    import FeeConfigTableRow from "./FeeConfigTableRow.svelte";

    export let account: FlashyfiAccount
    export let fungibleFeeConfigs: Array<FeeConfig>
    export let nonFungibleFeeConfigs: Array<FeeConfig>
</script>


<DataTable style="max-width: 100%;">
    <Head>
        <Row class="fee-table-header">
            <Cell>Fungible Token</Cell>
            <Cell numeric>Amount</Cell>
            <Cell>Fee Type</Cell>
            <Cell numeric>Fee</Cell>
            <Cell style="text-align: right">Lending</Cell>
        </Row>
    </Head>
    <Body>
    {#each fungibleFeeConfigs as feeConfig}
        <FeeConfigTableRow {feeConfig} {account}/>
    {:else}
        <Row>
            <Cell colspan="5" style="text-align: center">No fungible tokens configured</Cell>
        </Row>
    {/each}
    <Row style="height: 0.15rem">
        <Cell colspan="4"/>
    </Row>
    <Row class="fee-table-header">
        <Cell>NFT</Cell>
        <Cell numeric>Count</Cell>
        <Cell>Fee Type</Cell>
        <Cell numeric>Fee</Cell>
        <Cell style="text-align: right">Lending</Cell>
    </Row>
    {#each nonFungibleFeeConfigs as feeConfig}
        <FeeConfigTableRow {feeConfig} {account}/>
    {:else}
        <Row>
            <Cell colspan="5" style="text-align: center">No NFTs configured</Cell>
        </Row>
    {/each}
    </Body>
</DataTable>


<style lang="scss">
  :global(.fee-table-header > th, .fee-table-header > td) {
    font-weight: 500;
  }
</style>