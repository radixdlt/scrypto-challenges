<script>
// @ts-nocheck

    import GenPanel from "../GenPanel.svelte";
    import {_addXrdToCollateral, _removeXrdFromCollateral} from "../+page.js";

    import {fade} from "svelte/transition";

    let accountAddress = sessionStorage.getItem('accountAddress');
    let amountToDeposit = 0;
    let amountToRemove = 0;

    let status = '';
    let error = '';
    let loading = false;
    let stdout = '';

    const addScrilla = async () => {
        console.log('Adding Scrilla');
    }

    const removeScrilla = async () => {
        console.log("Removing Scrilla");
    }
</script>

<div class="flex flex-col md:flex-row  border justify-center  rounded-lg my-5 bg-gradient-to-b from-transparent to-slate-700" in:fade>
    <div class="flex md:flex-1 flex-col border rounded-lg p-5">
        <div class="flex flex-row items-baseline w-full justify-center">
            <input class="text-xl border-2 my-5 w-full" placeholder="Deposit Scrilla" bind:value={amountToDeposit} type="number" />
            <button class="border max-h-fit px-5 bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2 rounded-lg" on:click={addScrilla}>ADD</button>
        </div>
        <div class="flex items-baseline">
            <input class="text-xl border-2 w-full" type="number" placeholder="Withdraw Scrilla" bind:value={amountToRemove} />
            <p>This will also collect all rewards you have earned so far. </p>
            <button class="border max-h-fit px-5  bg-gradient-to-b from-cyan-800 to-transparent hover:bg-cyan-700 mx-2  rounded-lg" on:click={removeScrilla}>REMOVE</button>
        </div>
    </div>
    <div class="flex flex-1  justify-center">
        <GenPanel />
    </div>
</div>