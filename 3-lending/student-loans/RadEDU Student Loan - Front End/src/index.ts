import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
let accountAddress = undefined; // User account address
let packageAddress = undefined; // GumballMachine package address
let componentAddress = undefined; // GumballMachine component address
let resourceAddress = undefined; // GUM resource address
let badgeAddress = undefined; // Badge resource address

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();
  document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function () {
  // Load the wasm
  const response = await fetch('./student_loans.wasm');
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

  // set interest rate
  let collegeMajor = document.getElementById("major").value;
  let interestRate = undefined;

  switch(collegeMajor){
    case "socialservices":
      interestRate = 0.01;
    break;

    case "performingarts":
      interestRate = 0.02;
    break;

    case "earlychildhoodeducation":
      interestRate = 0.03;
    break;

    case "electricalengineering":
      interestRate = 0.04;
    break;

    case "computerscience":
      interestRate = 0.045;
    break;
  }

  // Construct manifest
  const manifest = new ManifestBuilder()
    .callFunction(packageAddress, 'StudentLend', 'instantiate_studentlend', ['ResourceAddress("030000000000000000000000000000000000000000000000000004")','Decimal("' + interestRate + '")'])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    componentAddress = receipt.newComponents[0];
    resourceAddress = receipt.newResources[0];
    document.getElementById('componentAddress').innerText = componentAddress;
  } else {
    document.getElementById('componentAddress').innerText = 'Error: ' + receipt.status;
  }
}


document.getElementById('registerNewUser').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'new_user', [''])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Define badge resource address
  if (receipt.status == 'Success') {
    badgeAddress = receipt.newResources[0];
  }

  // Update UI
  document.getElementById('registerAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

document.getElementById('deposit').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 100, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .createProofFromAccountByAmount(accountAddress, '1', badgeAddress)
    .popFromAuthZone('proof1')
    .callMethod(componentAddress, 'deposit', ['Proof("proof1")','Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('depositAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

document.getElementById('redeem').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, '1', badgeAddress)
    .popFromAuthZone('proof1')
    .callMethod(componentAddress, 'redeem', ['Proof("proof1")','Decimal("1.0")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('redeemAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

document.getElementById('borrow').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, '1', badgeAddress)
    .popFromAuthZone('proof1')
    .callMethod(componentAddress, 'borrow', ['Proof("proof1")','Decimal("1.0")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('borrowAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

document.getElementById('repay').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 100, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .createProofFromAccountByAmount(accountAddress, '1', badgeAddress)
    .popFromAuthZone('proof1')
    .callMethod(componentAddress, 'repay', ['Proof("proof1")','Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('repayAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

document.getElementById('setBorrow').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'set_borrow_interest_rate', ['Decimal("0.0")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('setBorrowAddress').innerText = JSON.stringify(receipt.status, null, 2);
};

/*
document.getElementById('getUser').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'get_user_id', [badgeAddress])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('getUserAddress').innerText = JSON.stringify(receipt, null, 2);
};
*/

/*
document.getElementById('liquidate').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 100, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .callMethod(componentAddress, 'liquidate', [badgeAddress,'Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('liquidateAddress').innerText = JSON.stringify(receipt, null, 2);
};
*/

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
  document.getElementById('userBalance').innerText = userComponent.ownedResources
    //.filter(e => e.resourceAddress == resourceAddress)
    .filter(e => e.resourceAddress == "030000000000000000000000000000000000000000000000000004")
    .map(e => e.amount)[0] || '0';


  document.getElementById('machineBalance').innerText = machineComponent.ownedResources
    .filter(e => e.resourceAddress == "030000000000000000000000000000000000000000000000000004")
    .map(e => e.amount)[0] || '0';

};
