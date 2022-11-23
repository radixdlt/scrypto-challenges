<script lang="ts">
    import balance from "$lib/balance_store";
    import Card, {Content} from "@smui/paper";
    import {fly} from 'svelte/transition';
</script>

<div class="wallet-connect">
    {#if $balance.getXrdBalance() != null}
        <Card>
            <Content>
                <div class="balance-content">
                    Account Balance:&nbsp;<span class="resource-amount">
                    {#key $balance}
                        <span style="display: inline-block" in:fly={{ y: 25 }}>
                        {(Math.round($balance.getXrdBalance() * 10000) / 10000).toFixed(4)}
                        </span>
                    {/key}
                    &nbsp;XRD
                    </span>
                </div>
            </Content>
        </Card>
    {/if}
</div>

<style lang="scss">
  .balance-content {
    display: flex;
    justify-content: flex-end;
  }

  .resource-amount {
    display: inline-flex;
    align-items: center;
    margin-right: 1rem;
    font-weight: bold;
  }
</style>
