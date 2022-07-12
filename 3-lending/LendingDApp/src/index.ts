import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
let accountAddress = undefined; // User account address
let packageAddress = undefined; // GumballMachine package address
let componentAddress = undefined; // GumballMachine component address
let resourceAddress = undefined; // Badge resource address
let resourceAddress1 = undefined; // Lending NFT resource address
let resourceAddress2 = undefined; // Borrowing NFT resource address
let resourceAddress3 = undefined; // LND Token  resource address
let xrdAddress = '030000000000000000000000000000000000000000000000000004'; // XRD Token  resource address

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();

  document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function () {
  // Load the wasm
  const response = await fetch('./lending_dapp.wasm');
  const wasm = new Uint8Array(await response.arrayBuffer());

  // Construct manifest
  const manifest = new ManifestBuilder()
    .publishPackage(wasm)
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  packageAddress = receipt.newPackages[0];
  document.getElementById('packageAddress').innerText = packageAddress;
};


document.getElementById('instantiateComponent').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 1000,xrdAddress)
    .takeFromWorktop(xrdAddress, 'StartValue')  
    .callFunction(packageAddress, 'LendingApp', 'instantiate_pool',['Bucket("StartValue")', 'Decimal("1000")', 'Decimal("10")','Decimal("7")'])    
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    componentAddress = receipt.newComponents[0];
    resourceAddress = receipt.newResources[0];
    resourceAddress1 = receipt.newResources[1];
    resourceAddress2 = receipt.newResources[2];
    resourceAddress3 = receipt.newResources[3];
    document.getElementById('componentAddress').innerText = componentAddress;
    document.getElementById('resourceAddress').innerText = resourceAddress;
    document.getElementById('resourceAddress1').innerText = resourceAddress1;
    document.getElementById('resourceAddress2').innerText = resourceAddress2;
    document.getElementById('resourceAddress3').innerText = resourceAddress3;
  } else {
    document.getElementById('componentAddress').innerText = 'Error: ' + receipt.status;
  }
}

document.getElementById('register').onclick = async function () {

  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'register', [])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('result').innerText = JSON.stringify(receipt, null, 2);
};

document.getElementById('register_borrower').onclick = async function () {

  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'register_borrower', [])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('resultBorrower').innerText = JSON.stringify(receipt, null, 2);
};

document.getElementById('lendMoney').onclick = async function () {

  let value_xrd = document.getElementById('number_to_lend').value;
  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, resourceAddress1)
    .popFromAuthZone('proof1')    
    .withdrawFromAccountByAmount(accountAddress, value_xrd, xrdAddress)
    .takeFromWorktop(xrdAddress, 'xrd')
    .callMethod(componentAddress, 'lend_money', (['Bucket("xrd") Proof("proof1")']))
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};


document.getElementById('getMoney').onclick = async function () {

  //.callMethod(validator!, 'add_stake', ['Bucket("bucket") Bucket("bucket1")'])
  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, resourceAddress1)
    .popFromAuthZone('proof1')    
    .withdrawFromAccount(accountAddress, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'lnd')
    .callMethod(componentAddress, 'take_money_back', (['Bucket("lnd") Proof("proof1")']))
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('getMoneyResult').innerText = JSON.stringify(receipt, null, 2);
};


document.getElementById('borrowMoney').onclick = async function () {
 
  //let value_xrd = document.getElementById('number_to_borrow').value;
  //alert(" ok " + value_xrd);

  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, resourceAddress2)
    .popFromAuthZone('proof1')  
    .callMethod(componentAddress, 'borrow_money', (['Decimal("100") Proof("proof1")']))
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('borrowMoneyResult').innerText = JSON.stringify(receipt, null, 2);
};

document.getElementById('repayMoney').onclick = async function () {

  //let value_xrd = document.getElementById('number_to_borrow').value;
  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, resourceAddress2)
    .popFromAuthZone('proof1')  
    .withdrawFromAccountByAmount(accountAddress, 110, xrdAddress)
    .takeFromWorktop(xrdAddress, 'xrd')
    .callMethod(componentAddress, 'repay_money', (['Bucket("xrd") Proof("proof1")']))
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('repayMoneyResult').innerText = JSON.stringify(receipt, null, 2);
};

  document.getElementById('checkBalance').onclick = async function () {
  // Retrieve component info from PTE service
  const api = new DefaultApi();
  const userComponent = await api.getComponent({
    address: accountAddress
  });
  const machineComponent = await api.getComponent({
    address: componentAddress
  });

  // Update UI
  document.getElementById('userBalanceXRD').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == xrdAddress)
    .map(e => e.amount)[0] || '0';
  document.getElementById('userBalanceLND').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress3)
    .map(e => e.amount)[0] || '0';    
    
  document.getElementById('machineBalanceXRD').innerText = machineComponent.ownedResources
    .filter(e => e.resourceAddress == xrdAddress).map(e => e.amount)[0];
  document.getElementById('machineBalanceLND').innerText = machineComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress3).map(e => e.amount)[0];
    
};