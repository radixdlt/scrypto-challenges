<script >
    // @ts-ignore
    import {_instantiateComponent, _newUser} from "./+page.js"
    import GenPanel from "./GenPanel.svelte";

    import {fade} from "svelte/transition";

    let accountName = sessionStorage.getItem('accountName')
    let accountAddress = sessionStorage.getItem('accountAddress')
    // TODO - make sure that session storage activates

    let compAddr1Sess = sessionStorage.getItem("componentAddress1") ? sessionStorage.getItem("componentAddress1") : "component_tdx_b_1qg4g0j6kh93yxuzevht5zwumadccl24q95c277t33cxq6szqxg"; 
    let compAddr2Sess = sessionStorage.getItem("componentAddress2") ? sessionStorage.getItem("componentAddress2") : "component_tdx_b_1q2n5pgg78va7w0v7y6vfmdktwfh4wdfdn878an5x8mlsyyg7ya";
    let compAddr3Sess =  "component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9"

    let componentAddresses = [
        compAddr1Sess,
        compAddr2Sess,
        compAddr3Sess
    ]

    console.log({componentAddresses, accountAddress});
</script>

<div class="border-2 rounded-xl px-4 py-2 flex flex-col items-center text-center" in:fade>
    {#if accountName && accountAddress}
    <div class=" p-5 flex flex-col  w-full sm:w-10/12 rounded-lg">
        <h1 class="font-bold text-lg">All Associated Scrilla Components for this dApp</h1>
        <hr class="my-5"/>
        <!-- <div class="flex flex-col items-center border-2 py-2 px-4 bg-gradient-to-b from-cyan-900 to-transparent transition-colors ease-in-out duration-300 hover:bg-slate-400 rounded-xl m-5">
            <button on:click={_instantiateComponent} >INSTANTIATE NEW COMPONENT</button>
        </div> -->
        <span class="font-light border  cursor-crosshair">
            <p class="font-bold break-words ">
                {componentAddresses[0]}
            </p>
        </span>
        <span class="font-light border  cursor-crosshair">
            <p class="font-bold break-words">
                 {componentAddresses[1]}
            </p>
        </span>
        <span class="font-light border  cursor-crosshair">
            <p class="font-bold break-words">
                 {componentAddresses[2]}
            </p>
        </span>
    </div>
    {:else}
    <div class="flex flex-col text-purple-900 font-bold items-center">
        Please Connect to Radix Wallet
    </div>
    {/if}
    <div class="flex flex-col sm:flex-row border w-full rounded-xl my-5 bg-gradient-to-b from-transparent to-gray-800 items-center justify-center">
        <GenPanel />
    </div>
</div>