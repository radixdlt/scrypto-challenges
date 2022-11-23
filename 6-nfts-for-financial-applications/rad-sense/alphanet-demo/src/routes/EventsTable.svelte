<script lang="ts">
    import DataTable, {Body, Cell, Head, Row} from '@smui/data-table';
    import {shortenAddress} from "$lib/utils.js";
    import {type RadSenseEvent} from "$lib/model";

    export let events: Array<RadSenseEvent> = [];
</script>

<DataTable style="width: 100%;">
    <Head>
        <title></title>
        <Row>
            <Cell>Type</Cell>
            <Cell>ID</Cell>
            <Cell numeric>Timestamp</Cell>
            <Cell>Ad Placement ID</Cell>
            <Cell>Ad ID</Cell>
            <Cell>Ad Slot ID</Cell>
            <Cell>Ad Broker ID</Cell>
        </Row>
    </Head>
    <Body>
    {#each events as {
        id,
        type,
        timestamp,
        adPlacementId,
        adNonFungibleId,
        adSlotNonFungibleId,
        adBrokerNonFungibleId
    }}
        <Row>
            <Cell>{type}</Cell>
            <Cell>{shortenAddress(id, 5, 5)}</Cell>
            <Cell numeric>{timestamp}</Cell>
            <Cell>{shortenAddress(adPlacementId, 5, 5)}</Cell>
            <Cell>{shortenAddress(adNonFungibleId, 5, 5)}</Cell>
            <Cell>{shortenAddress(adSlotNonFungibleId, 5, 5)}</Cell>
            <Cell>{shortenAddress(adBrokerNonFungibleId, 5, 5)}</Cell>
        </Row>
    {/each}
    </Body>
</DataTable>