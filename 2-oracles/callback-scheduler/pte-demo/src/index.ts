import {DefaultApi, ManifestBuilder, Receipt} from 'pte-sdk';
import {getAccountAddress, signTransaction} from 'pte-browser-extension-sdk';

// Global states
let xrdAddress = '030000000000000000000000000000000000000000000000000004';
let accountAddress = undefined; // User account address
let packageAddress = undefined; // CallbackScheduler package address
let schedulerComponentAddress = undefined; // CallbackScheduler component address
let schedulerComponentAdminBadgeAddress = undefined; // CallbackScheduler AdminBadge resource address
let callbackAdminHandleAddress = undefined; // CallbackAdminHandle resource address
let callbackHandleAddress = undefined; // CallbackHandle resource address
let dummyComponentAddress = undefined; // DummyComponent component address
let dummyComponentAdminBadgeAddress = undefined; // DummyComponent AdminBadge address
let demoTokensAddress = undefined; // Address of the demo token that is created by the DummyComponent
let callbackId = undefined; // The ID of a Callback NFR and its associated handles


function assertTransactionSuccessful(receipt: Receipt) {
    const receiptJson = JSON.stringify(receipt, null, 2);
    console.log(receiptJson)
    if (receipt.status != 'Success') {
        alert(`Transaction failed:\n\n ${receiptJson}`);
        throw "Transaction failed";
    }
}

async function getTokenBalance() {
    const api = new DefaultApi();
    const dummyComponent = await api.getComponent({
        address: dummyComponentAddress
    });
    return dummyComponent.ownedResources
        .filter(r => r.resourceAddress == demoTokensAddress)
        .map(r => r.amount)[0] || '0';
}

document.getElementById('fetchAccountAddress').onclick = async function () {
    // Retrieve extension user account address
    accountAddress = await getAccountAddress();

    document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function () {
    // Load the wasm
    const response = await fetch('./callback_scheduler.wasm');
    const wasm = new Uint8Array(await response.arrayBuffer());

    // Construct manifest
    const manifest = new ManifestBuilder()
        .publishPackage(wasm)
        .build()
        .toString();

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    // Update UI
    packageAddress = receipt.newPackages[0];
    document.getElementById('packageAddress').innerText = packageAddress;
};


document.getElementById('instantiateSchedulerComponent').onclick = async function () {
    // Construct manifest
    const manifest = new ManifestBuilder()
        .callFunction(
            packageAddress,
            'CallbackScheduler',
            'instantiate_callback_scheduler',
            ['Decimal("10")'])
        .callMethodWithAllResources(accountAddress, 'deposit_batch')
        .build()
        .toString();

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    // Update UI
    schedulerComponentAddress = receipt.newComponents[0];
    schedulerComponentAdminBadgeAddress = receipt.newResources[0];
    callbackAdminHandleAddress = receipt.newResources[3];
    callbackHandleAddress = receipt.newResources[4];
    document.getElementById('schedulerComponentAddress').innerText = schedulerComponentAdminBadgeAddress;
}

document.getElementById('instantiateDummyComponent').onclick = async function () {
    // Construct manifest
    const manifest = new ManifestBuilder()
        .callFunction(
            packageAddress,
            'DummyComponent',
            'instantiate_demo_component',
            [`ComponentAddress("${schedulerComponentAddress}")`])
        .callMethodWithAllResources(accountAddress, 'deposit_batch')
        .build()
        .toString();
    console.log(manifest)

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    // Update UI
    dummyComponentAddress = receipt.newComponents[0];
    dummyComponentAdminBadgeAddress = receipt.newResources[0];
    demoTokensAddress = receipt.newResources[2];
    document.getElementById('dummyComponentAddress').innerText = schedulerComponentAdminBadgeAddress;
    document.getElementById('tokenBalance1').innerText = await getTokenBalance();
}

document.getElementById('scheduleCallback').onclick = async function () {
    // Construct manifest
    const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, dummyComponentAdminBadgeAddress)
        .withdrawFromAccountByAmount(accountAddress, 10, xrdAddress)
        .takeFromWorktop(xrdAddress, 'fee')
        .callMethod(schedulerComponentAddress, 'schedule_callback', [
            `Struct(Enum("AtDateTime", "2022-10-01T12:30:55+00", 120u8), ComponentAddress("${dummyComponentAddress}"), "burn_tokens", Vec<Vec>(Bytes("a1100000000000a0dec5adc9353600000000000000")), None)`,
            'Bucket("fee")'])
        .takeFromWorktop(callbackHandleAddress, "callback_handle")
        .callMethod(dummyComponentAddress, "deposit_callback_handle", ['Bucket("callback_handle")'])
        .callMethodWithAllResources(accountAddress, 'deposit_batch')
        .build()
        .toString();

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    for (const log of receipt.logs) {
        console.log("Found log: " + log)
        const matches = /Minted CallbackAdminHandle: #(.*),.*/.exec(log);
        if (matches) {
            console.log("Regex matches: " + matches[1]);
            callbackId = matches[1];
            break;
        } else {
            console.log("Regex doesnt match")
        }
    }

    // Update UI
    document.getElementById('callbackId').innerText = callbackId;
}

document.getElementById('checkTokenBalance2').onclick = async function () {
    document.getElementById('tokenBalance2').innerText = '';
    document.getElementById('tokenBalance2').innerText = await getTokenBalance();
}

document.getElementById('getNewCallbackAdminHandles').onclick = async function () {
    // Construct manifest
    const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, schedulerComponentAdminBadgeAddress)
        .withdrawFromAccountByAmount(accountAddress, 10, xrdAddress)
        .takeFromWorktop(xrdAddress, 'fee')
        .callMethod(schedulerComponentAddress, 'get_new_callback_admin_handles', [])
        .callMethodWithAllResources(accountAddress, 'deposit_batch')
        .build()
        .toString();

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    // Update UI
    document.getElementById('callbackAdminHandleRetrieved').innerText = "CallbackAdminHandle retrieved";
}

document.getElementById('executeCallback').onclick = async function () {
    // Construct manifest
    const manifest = new ManifestBuilder()
        .withdrawFromAccountByIds(accountAddress, [callbackId], callbackAdminHandleAddress)
        .takeFromWorktopByIds([callbackId], callbackAdminHandleAddress, "adminHandle")
        .callMethod(schedulerComponentAddress, 'execute_callback', ['Bucket("adminHandle")'])
        .build()
        .toString();

    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);
    assertTransactionSuccessful(receipt);

    // Update UI
    document.getElementById('callbackExecuted').innerText = "Callback executed";
}

document.getElementById('checkTokenBalance3').onclick = async function () {
    document.getElementById('tokenBalance3').innerText = '';
    document.getElementById('tokenBalance3').innerText = await getTokenBalance();
}
