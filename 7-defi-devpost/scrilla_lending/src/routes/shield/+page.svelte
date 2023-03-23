<script>
// @ts-nocheck

    import GenPanel from "../GenPanel.svelte";
    import LoadingSpinner from "../LoadingSpinner.svelte";
    import TerminalReceipt from "../TerminalReceipt.svelte";

    import {_addUsdsToShield,_withdrawShieldDepositAndRewards} from "../+page.js"

    import axios from "axios";

    import {fade} from "svelte/transition";
    import {onMount} from "svelte";

    let amountToDeposit = 0;
    let amountToWithdraw = 0;
    let accountAddress = sessionStorage.getItem('accountAddress')

    let status = '';
    let error = '';
    let loading = false;
    let stdout = '';

    // @ts-ignore
    const depositToShield = async () => {
        console.log('DEPOSITING USDS TO SHIELD');
        status = 'Depositing USDS to Shield - Check Your Wallet to Approve Transaction'
        loading = true;
        stdout = '';        
        
        _addUsdsToShield(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5",amountToDeposit,"resource_tdx_b_1qrr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqttv7n7","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9")
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = 'USDS Successfully Added to Shield. See Receipt below or in your Browser Console for more details';
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

    const withdrawFromShield = async () => {
        console.log('WITHDRAWING USDS FROM SHIELD');
        status = 'Withdrawing USDS from Shield - Check Your Wallet to Approve Transaction'
        loading = true;
        stdout = '';
        
        _withdrawShieldDepositAndRewards(accountAddress,"resource_tdx_b_1qpye3s55d9yx44qsqehh5r4fa42f82jttzsy83wr2xhqgt7ms5","component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9")
        .then(receipt => {
            stdout = receipt.details.receipt;
            status = 'USDS Successfully Withdrawn from Shield. See Receipt below or in your Browser Console for more details';
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

    let loans = []

    const getInfo = async () => {
        axios.post('https://betanet.radixdlt.com/entity/details', {
            address: 'component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9',
        })
        .then(function (response) {
            loans = response.data.details.state.data_json[14]
        })
        .catch(function (error) {
            console.log(error);
        });
    }

    onMount(() => {
        getInfo();
    })
</script>

<div class="flex flex-col md:flex-row  border rounded-lg bg-gradient-to-b from-transparent to-slate-700 my-5" in:fade>
    <div class="flex md:flex-1 flex-col border bored-cyan-400 p-5">
        <div class="flex flex-row items-baseline w-full">
            <input class="text-xl border-2 my-5 w-full"  type="number" placeholder="# of USDS to Deposit" bind:value={amountToDeposit} />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={depositToShield}>Deposit</button>
        </div>
        <div class="flex items-baseline">
            <input class="text-xl border-2 w-full" type="number" placeholder="# of USDS to Withdraw" bind:value={amountToWithdraw} />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={withdrawFromShield}>Withdraw</button>
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
    <div class="flex flex-1 justify-center">
        <GenPanel />
    </div>
</div> 
<!-- Start Shield Views -->
<div class="flex flex-col px-4 border py-2 bg-gradient-to-b from-transparent to-slate-800" in:fade>
    <button class="py-1 text-xl  max-w-fit border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg mb-3">Liquidate</button>
    <!-- loan views -->

    <div class="flex flex-col p-5 border rounded-xl self-center w-full">
        <div class="flex flex-row justify-between border py-1 px-3  text-center">
            <h1 class="text-lg font-bold border-r w-full">User #</h1>
            <h1 class="text-lg font-bold border-l border-r w-full">Price to Liquidate</h1>
        </div>
        {#each loans as loan}
            <div class="flex flex-row justify-between border py-1 px-3  text-center">
                <h1 class="text-lg border w-full mx-1">{JSON.stringify(loan[0].value)}</h1>
                <h1 class="text-lg border w-full mx-1">{JSON.stringify(loan[1])}</h1>
            </div>
        {/each}
    </div>
</div>

