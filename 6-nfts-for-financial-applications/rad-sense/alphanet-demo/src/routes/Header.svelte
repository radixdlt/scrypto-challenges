<script lang="ts">
    import AccountConnect from './AccountConnect.svelte';
    import AccountBalances from "./AccountBalances.svelte";
    import Banner, { Label } from '@smui/banner';
</script>

<header>
    <div class="account-balances">
        <AccountBalances/>
    </div>
    <div class="account-connect">
        <AccountConnect/>
    </div>
</header>

<style>
    header {
        width: 100%;
        display: flex;
        justify-content: flex-end;
        align-items: center;
    }

    .account-balances {
        margin-right: auto;
    }
</style>
