<script>
// @ts-nocheck

    import GenPanel from "../GenPanel.svelte";
    import LoadingSpinner from "../LoadingSpinner.svelte";
    import TerminalReceipt from "../TerminalReceipt.svelte";

    import {_borrowUsds,_repayUsds} from "../+page.js"

    import {fade} from "svelte/transition";

    let accountAddress = sessionStorage.getItem('accountAddress')

    let amountToBorrow = 0;
    let amountToRepay = 0;
    
    let status = '';
    let error = '';
    let loading = false;
    let stdout = '';

    const borrowUSDS = async () => {
        console.log('BORROWING USDS FROM COMPONENT');
        status = 'Borrowing USDS from Component - Check Your Wallet to Approve Transaction';
        loading = true;
        stdout = '';

        _borrowUsds(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9", amountToBorrow)
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = "USDS Successfully Borrowed from Component. See Receipt below or in your Browser Console for more details";
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
    
    const repayUSDS = async () => {
        console.log("REPAYING USDS TO COMPONENT");
        status = 'Repaying USDS to Component - Check Your Wallet to Approve Transaction';
        loading = true;
        stdout = '';

        _repayUsds(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5",amountToRepay,"resource_tdx_b_1qrr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqttv7n7","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9")
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = "USDS Successfully Repaid to Component. See Receipt below or in your Browser Console for more details";
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

<div class="flex flex-col md:flex-row  border rounded-lg my-5 bg-gradient-to-b from-transparent to-slate-700" in:fade>
    <div class="flex md:flex-1 flex-col border bored-cyan-400 p-5">
        <div class="flex flex-row items-baseline w-full">
            <input class="text-xl border-2 my-5 w-full" placeholder="Borrow USDS" bind:value={amountToBorrow} />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={borrowUSDS}>BORROW</button>
        </div>
        <div class="flex items-baseline">
            <input class="text-xl border-2 w-full" placeholder="Repay USDS" bind:value={amountToRepay} />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={repayUSDS}>REPAY</button>
        </div>

        <div class="flex flex-col items-center px-3 py-1 border my-2 rounded-lg break-all text-center">
            <h2 class="font-bold text-cyan-500">{status}</h2>
            <h2 class="font-bold text-red-500">{error}</h2>
            {#if loading}
                <LoadingSpinner />
            {/if}
            {#if stdout}
                <TerminalReceipt {stdout} />
            {/if}
        </div>
    </div>
    <div class="flex  flex-1  justify-center">
        <GenPanel />
    </div>
</div>