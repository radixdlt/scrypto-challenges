<script>
// @ts-nocheck

    import LoadingSpinner from "./LoadingSpinner.svelte";
    import TerminalReceipt from "./TerminalReceipt.svelte";

    import {_redeemUsds} from "./+page.js"

    let accountAddress = sessionStorage.getItem('accountAddress')

    let status = '';
    let error = '';
    let loading = false;
    let stdout = '';

    let amountToRedeem;

    const redeemUsds = async () => {
        console.log('REDEEMING USDS FROM COMPONENT');
        stdout = '';
        status = 'Redeeming USDS from Component - Check Your Wallet to Approve Transaction';
        loading = true;
        _redeemUsds(accountAddress,amountToRedeem,"resource_tdx_b_1qrr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqttv7n7","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9")
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = "Redeem USDS Successful. See Receipt below or in Browser Console for more details";
            loading = false;
            setTimeout(() => {
                status = '';
            },5000)
        })
        .catch(err => {
            loading = false;
            status = '';
            error = JSON.stringify(err);
            setTimeout(() => {
                error = '';
            },5000)
        })
    }
</script>

<div class="flex flex-col items-center border rounded-lg px-4 py-1 w-full">
    Redeem USDS for XRD
    <div class=" px-4 py-2 w-11/12 flex flex-col items-center rounded-lg transition-all ease-in-out duration-700 break-all">
        <input type="number" class="border-2 p-1 text-center w-full" placeholder="Amount USDS to Redeem" bind:value={amountToRedeem}/>
        <button class="border bg-gradient-to-b from-cyan-800 px-5 rounded-lg my-2 to-transparent" on:click={redeemUsds}>
            Redeem
        </button>
        <h2 class="font-bold text-cyan-500">{status}</h2>
        <h2 class="font-bold text-red-500">{error}</h2>
        {#if loading}
            <LoadingSpinner />
        {/if}
        {#if stdout}
            <TerminalReceipt stdout={stdout} />
        {/if}
    </div>
</div>
