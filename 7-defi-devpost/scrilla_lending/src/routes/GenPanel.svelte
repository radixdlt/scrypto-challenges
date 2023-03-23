<script>
// @ts-nocheck
    import InfoPanel from "./InfoPanel.svelte";
    import TerminalReceipt from "./TerminalReceipt.svelte";
    import LoadingSpinner from "./LoadingSpinner.svelte";
    import Redeem from "./Redeem.svelte";
    import {_setPrice, _getPrice, _newUser} from "./+page.js"

    let accountAddress = sessionStorage.getItem('accountAddress');

    let newPrice = 0.05;
    let stdoutAddUser = '';
    let statusAddUser = '';
    let errorAddUser = '';
    let loadingAddUser = false;
    let stdoutSetPrice = '';
    let statusSetPrice = '';
    let errorSetPrice = '';
    let loadingSetPrice = false
    let statusGetPrice = '';
    let errorGetPrice = '';
    let loadingGetPrice = false;
    let xrdPrice = sessionStorage.getItem('xrdPrice') ?  sessionStorage.getItem('xrdPrice') : 'Please Get Price w/ Wallet' ;

    const addNewUser = async () => {
        console.log('ADDING NEW USER')
        stdoutAddUser = '';
        statusAddUser = 'Adding New User - Check Your Wallet to Approve Transaction';
        loadingAddUser = true;
        _newUser("component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9",accountAddress)
        .then(receipt => {
            stdoutAddUser = receipt.details.receipt;
            statusAddUser = 'New User Badge Successfully Returned. See Receipt below or in Browser Console for more details';
            loadingAddUser = false;
            setTimeout(() => {
                statusAddUser = '';
            },5000)    
        })
        .catch(err => {
            loadingAddUser = false
            statusAddUser = '';
            errorAddUser = JSON.stringify(err)
            setTimeout(() => {
                errorAddUser = '';
            },5000)
        })
    }

    const setPrice = async () => {
        console.log('SETTING NEW XRD PRICE');
        stdoutSetPrice = '';
        statusSetPrice = 'Setting New XRD Price - Check Your Wallet to Approve Transaction';
        loadingSetPrice = true;
        _setPrice("component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9",newPrice,accountAddress)
        .then(receipt => {
            stdoutSetPrice = receipt.details.receipt;
            statusSetPrice = "New XRD Price Sucessfully Changed. See Receipt below or in Browser Console for more details";
            loadingSetPrice = false;
            sessionStorage.setItem('xrdPrice',newPrice);
            xrdPrice = sessionStorage.getItem('xrdPrice') ?  sessionStorage.getItem('xrdPrice') : 'Please Get Price w/ Wallet';
            setTimeout(() => {
                statusSetPrice = '';
            },5000)
        })
        .catch(err => {
            loadingSetPrice = false;
            statusSetPrice = '';
            errorSetPrice = JSON.stringify(err);
            setTimeout(() => {
                errorSetPrice = '';
            },5000)
        })
    }

    const getPrice = async () => {
        console.log("GETTING CURRENT XRD PRICE FROM COMPONENT")
        statusGetPrice = "Getting Current XRD Price from Component - Check Your Wallet to Approve Transaction";
        loadingGetPrice = true;
        _getPrice("component_tdx_b_1qtr7c72eudfcfpg4mg6g8ezpnz9wslzfce7x6rqkl9tqp27sk9", accountAddress)
        .then(price => {
            xrdPrice = price;
            sessionStorage.setItem('xrdPrice',xrdPrice);
            statusGetPrice = '';
            loadingGetPrice = false;
        })
        .catch(err => {
            loadingGetPrice = false;
            statusGetPrice = '';
            errorGetPrice = JSON.stringify(err);
            setTimeout(() => {
                errorGetPrice = '';
            }, 5000)
        })
    }
</script>

<div class="flex w-full flex-col items-center py-5 border rounded-lg my-5 mx-45">
    <div class="text-lg  p-5 my-1">
        Open up your Browser Developer Console to see even more Output from the dApp
    </div>
    <div class="flex flex-col items-center p-5 ">
        <button on:click={addNewUser} class="px-4 py-1 bg-gradient-to-b from-cyan-800 to-transparent transition-colors ease-in-out hover:bg-cyan-700 border rounded-lg">
            Add New User
        </button>
        <h2 class="font-bold text-cyan-500">{statusAddUser}</h2>
        <h2 class="font-bold text-red-500 ">{errorAddUser}</h2>
        {#if loadingAddUser}
            <LoadingSpinner />
        {/if}
        {#if stdoutAddUser}
            <TerminalReceipt stdout={stdoutAddUser} />
        {/if}
    </div>
    <div class="w-full flex">
        <div class="border px-4 py-2 w-1/2 flex flex-col items-center rounded-lg transition-all ease-in-out duration-700 justify-between break-all">
            <h1>Get XRD Price</h1>
            <h1 class="text-green-500">${xrdPrice}</h1>
            <button class="border bg-gradient-to-b from-cyan-800 px-5 rounded-lg my-2 to-transparent" on:click={getPrice}>
                Get
            </button>
            <h2 class="font-bold text-cyan-500">{statusGetPrice}</h2>
            <h2 class="font-bold text-red-500">{errorGetPrice}</h2>
            {#if loadingGetPrice}
                <LoadingSpinner />
            {/if}
        </div>
        <div class="border  px-4 py-2 w-1/2 flex flex-col items-center rounded-lg transition-all ease-in-out duration-700 break-all">
            <h1>Set XRD Price</h1>
            <input type="number" class="border-2 p-1 text-center w-full" placeholder="Set XRD Price" bind:value={newPrice}/>
            <button class="border bg-gradient-to-b from-cyan-800 px-5 rounded-lg my-2 to-transparent" on:click={setPrice}>
                Set
            </button>
            <h2 class="font-bold text-cyan-500">{statusSetPrice}</h2>
            <h2 class="font-bold text-red-500">{errorSetPrice}</h2>
            {#if loadingSetPrice}
                <LoadingSpinner />
            {/if}
            {#if stdoutSetPrice}
                <TerminalReceipt stdout={stdoutSetPrice} />
            {/if}
        </div>
    </div>
    <Redeem />
    <InfoPanel />
</div>