import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
let accountAddress = undefined; // User account address
let packageAddress = undefined; // GumballMachine package address
let componentAddress = undefined; // GumballMachine component address
let resourceAddress1 = undefined; // GUM resource address
let resourceAddress2 = undefined; // GUM resource address
let resourceAddress3 = undefined; // GUM resource address
let resourceAddress4 = undefined; // GUM resource address
let resourceAddress5 = undefined; // GUM resource address
let addAmount = undefined; // GUM resource address



document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();

  document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function () {
  // Load the wasm
  const response = await fetch('./credit_lender.wasm');
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
    .callFunction(packageAddress, 'CreditLender', 'new', [])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    //Component
    componentAddress = receipt.newComponents[0];
    //Mining Badge
    resourceAddress1 = receipt.newResources[0];
    //Lender Receipt
    resourceAddress2 = receipt.newResources[1];
    //Borrower Receipt
    resourceAddress3 = receipt.newResources[2];
    //Loan NFT
    resourceAddress4 = receipt.newResources[3];
    //Credit Report NFT
    resourceAddress5 = receipt.newResources[4];
    document.getElementById('componentAddress').innerText = componentAddress;
    document.getElementById('resourceAddress1').innerText = resourceAddress1;
    document.getElementById('resourceAddress2').innerText = resourceAddress2;
    document.getElementById('resourceAddress3').innerText = resourceAddress3;
    document.getElementById('resourceAddress4').innerText = resourceAddress4;
    document.getElementById('resourceAddress5').innerText = resourceAddress5;
  
  } else {
    document.getElementById('componentAddress').innerText = 'Error: ' + receipt.status;
  }
}


//ADD LIQUIDITY TO THE LENDING POOL

document.getElementById('addliquidity').onclick = async function () {
  // Construct manifest

var x = document.getElementById('addamount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, x, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .callMethod(componentAddress, 'add_funds', ['Bucket("xrd")', `ComponentAddress("${accountAddress}")`])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};

//REMOVE LIQUIDITY TO THE LENDING POOL

document.getElementById('removeliquidity').onclick = async function () {
  // Construct manifest

var y = document.getElementById('removeamount').value;

console.log(y);

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress2)
    .takeFromWorktop(resourceAddress2, 'lenderNFT')
    .callMethod(componentAddress, 'remove_funds', ['Bucket("lenderNFT")', `Decimal("${y}")`])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};

//OPEN A LOAN

document.getElementById('createLoan').onclick = async function () {
  // Construct manifest

var loanAmount = document.getElementById('loanamount').value;
var collateralAmount = document.getElementById('collateralamount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, collateralAmount, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .callMethod(componentAddress, 'new_loan', [ `Decimal("${loanAmount}")`, 'Bucket("xrd")', `ComponentAddress("${accountAddress}")` ])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt2').innerText = JSON.stringify(receipt, null, 2);
};

//CLOSE LOAN

document.getElementById('payloan').onclick = async function () {
  // Construct manifest

var payAmount = document.getElementById('payamount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, payAmount, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'borrowerNFT')
    .callMethod(componentAddress, 'close_loan', ['Bucket("borrowerNFT")', 'Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt3').innerText = JSON.stringify(receipt, null, 2);
};

//ADD COLLATERAL 
document.getElementById('addCollateral').onclick = async function () {
  // Construct manifest

var addCollateralAmount = document.getElementById('addCollateralAmount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, addCollateralAmount, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'borrowerNFT')
    .callMethod(componentAddress, 'add_collateral', ['Bucket("borrowerNFT")', 'Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt4').innerText = JSON.stringify(receipt, null, 2);
};

//REMOVE COLLATERAL
document.getElementById('removeCollateral').onclick = async function () {
  // Construct manifest

var RemoveCollateralAmount = document.getElementById('RemoveCollateralAmount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'borrowerNFT')
    .callMethod(componentAddress, 'remove_collateral', ['Bucket("borrowerNFT")', `Decimal("${RemoveCollateralAmount}")`])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt4').innerText = JSON.stringify(receipt, null, 2);
};

//BORROW MORE

document.getElementById('borrowMore').onclick = async function () {
  // Construct manifest

var borrowAmount = document.getElementById('borrowAmount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'borrowerNFT')
    .callMethod(componentAddress, 'remove_collateral', ['Bucket("borrowerNFT")', `Decimal("${borrowAmount}")`])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt4').innerText = JSON.stringify(receipt, null, 2);
};

//PAY MORE

document.getElementById('payMore').onclick = async function () {
  // Construct manifest

var payAmount = document.getElementById('payAmount').value;

  const manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(accountAddress, payAmount, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
    .withdrawFromAccountByAmount(accountAddress, 1, resourceAddress3)
    .takeFromWorktop(resourceAddress3, 'borrowerNFT')
    .callMethod(componentAddress, 'pay_loan', ['Bucket("borrowerNFT")', 'Bucket("xrd")'])
    .callMethodWithAllResources(accountAddress, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt4').innerText = JSON.stringify(receipt, null, 2);
};



document.getElementById('checkBalance').onclick = async function () {
  // Retrieve component info from PTE service
  const api = new DefaultApi();
  const userComponent = await api.getComponent({
    address: accountAddress
  });
  
  // Update UI
  document.getElementById('xrdBalance').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == '030000000000000000000000000000000000000000000000000004')
    .map(e => e.amount)[0] || '0';

  document.getElementById('lenderReceipt').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress2)
    .map(e => e.amount)[0] || '0';

  document.getElementById('lenderNFT').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress2)
    .map(e => e.nonFungibleIds)[0] || '0';

  document.getElementById('borrowerReceipt').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress3)
    .map(e => e.amount)[0] || '0';

  document.getElementById('borrowerNFT').innerText = userComponent.ownedResources
    .filter(e => e.resourceAddress == resourceAddress3)
    .map(e => e.nonFungibleIds)[0] || '0';

  
};