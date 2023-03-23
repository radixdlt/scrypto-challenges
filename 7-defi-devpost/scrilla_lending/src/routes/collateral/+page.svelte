<script>
// @ts-nocheck

    import GenPanel from "../GenPanel.svelte";
    import TerminalReceipt from "../TerminalReceipt.svelte";
    import LoadingSpinner from "../LoadingSpinner.svelte";
    import {_addXrdToCollateral, _removeXrdFromCollateral} from "../+page.js";

    import {fade} from "svelte/transition";

    let accountAddress = sessionStorage.getItem('accountAddress');

    let amountToDeposit = 0;
    let amountToRemove = 0;

    let status = '';
    let error = '';
    let loading = false;
    let stdout = '';

    const addCollateral = async () => {
        console.log('ADDING XRD COLLATERAL TO COMPONENT');
        status = 'Adding XRD Collateral to Component - Check Your Wallet to Approve Transaction'
        loading = true;
        stdout = '';
        _addXrdToCollateral(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5",amountToDeposit,"component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9")
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = 'XRD Collateral Successfully Added to Component. See Receipt below or in your Browser Console for more details';
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

    const removeCollateral = async () => {
        console.log('REMOVING XRD COLLATERAL TO COMPONENT');
        status = 'Removing XRD Collateral from Component - Check Your Wallet to Approve Transaction'
        loading = true;
        stdout = '';
        _removeXrdFromCollateral(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9",amountToRemove)
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = 'XRD Collateral Successfully Removed from Component. See Receipt below or in your Browser Console for more details';
            loading = false;
            setTimeout(() => {
                status = '';
            },5000)
        })
        .catch(err => {
            loading = false;
            status = '';
            error = JSON.stringify(err) + "\n You need to payback your loan before you can take back your Collateral!";
            setTimeout(() => {
                error = '';
            },5000)
        })
    }
</script>

<div class="flex flex-col md:flex-row  border justify-center  rounded-lg my-5 bg-gradient-to-b from-transparent to-slate-700" in:fade>
    <div class="flex md:flex-1 flex-col border rounded-lg p-5">
        <div class="flex flex-row items-baseline w-full justify-center">
            <input class="text-xl border-2 my-5 w-full" placeholder="Add XRD Collateral" bind:value={amountToDeposit} />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={addCollateral}>ADD</button>
        </div>

        <div class="flex items-baseline">
            <input class="text-xl border-2 w-full" placeholder="Remove XRD Collateral" bind:value={amountToRemove} />
            <button class="border max-h-fit px-5  bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2  rounded-lg" on:click={removeCollateral}>REMOVE</button>
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
    <div class="flex flex-1  justify-center">
        <GenPanel />
    </div>
</div>