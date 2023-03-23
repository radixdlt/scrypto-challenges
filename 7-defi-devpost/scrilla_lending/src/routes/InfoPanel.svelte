<script>
    import axios from "axios";

    import {onMount} from "svelte";

    // Make a request for main Scrilla Component Entity Details;

    $: info = ''
    const getInfo = async () => {
        axios.post('https://betanet.radixdlt.com/entity/details', {
            address: 'component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9',
        })
        .then(function (response) {
            // console.log('**** SCRILLA COMPONENT INFO ****')
            // console.log(response.data.details.state.data_json);
            info = response.data.details.state.data_json
        })
        .catch(function (error) {
            console.log(error);
        });
    }

    onMount(() => {
        getInfo();
    })

</script>

<div class="flex flex-col items-center px-5 py-2   w-11/12  border-2 rounded-lg">
    <h1 class="font-extrabold text-lg my-1">
        SCRILLA COMPONENT INFO AND USERS
    </h1>
    <p class="font-light text-green-500 border p-2 flex w-full break-all rounded-lg justify-center flex-col max-h-28 overflow-auto">
        {#each info as i,index}
            <div class="px-3 py-1 border rounded-lg bg-gradient-to-r from-black via-transparent to-slate-900">
                {index}"::"
                {JSON.stringify(i[0])}::{JSON.stringify(i[1])}
            </div>
        {/each}
    </p>
</div>