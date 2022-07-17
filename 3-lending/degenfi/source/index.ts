import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
let accountAddress1 = undefined; // Joe
let accountAddress2 = undefined; // Bob
let accountAddress3 = undefined; // Sally
let accountAddress4 = undefined; // Beth
let accountAddress5 = undefined; // John
let packageAddress = undefined; // GumballMachine package address
let componentAddress = undefined; // GumballMachine component address
let resourceAddress1 = undefined; // GUM resource address
let resourceAddress2 = undefined; // GUM resource address
let resourceAddress3 = undefined; // GUM resource address
let resourceAddress4 = undefined; // GUM resource address
let resourceAddress5 = undefined; // GUM resource address
let addAmount = undefined; // GUM resource address
let badgeAddress = undefined; // Badge resource address



document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();

  document.getElementById('accountAddress').innerText = accountAddress1;
};

document.getElementById('publishPackage').onclick = async function () {
  // Load the wasm
  const response = await fetch('./public/DegenFi.wasm');
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
    .callFunction(packageAddress, 'DegenFi', 'new', [])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    //User Management Component
    componentAddress1 = receipt.newComponents[0];
    //PseudoPriceOracle Component
    componentAddress2 = receipt.newComponents[1];
    //DegenFi Component
    componentAddress3 = receipt.newComponents[2];
    //Admin authority for BasicFlashLoan
    resourceAddress1 = receipt.newResources[0];
    //Promise token for BasicFlashLoan - must be returned to be burned!
    resourceAddress2 = receipt.newResources[1];
    //Admin authority to mint/burn Access Tokens
    resourceAddress3 = receipt.newResources[2];
    //Access Token
    resourceAddress4 = receipt.newResources[3];
    //Degen Admin Badge
    resourceAddress5 = receipt.newResources[4];
    //Degen Token
    resourceAddress6 = receipt.newResources[5];
    //Lending Protocol User Badge
    resourceAddress7 = receipt.newResources[6];
    //Lending Protocol User
    resourceAddress8 = receipt.newResources[7];
    document.getElementById('componentAddress1').innerText = componentAddress1;
    document.getElementById('componentAddress2').innerText = componentAddress2;
    document.getElementById('componentAddress3').innerText = componentAddress3;
    document.getElementById('resourceAddress1').innerText = resourceAddress1;
    document.getElementById('resourceAddress2').innerText = resourceAddress2;
    document.getElementById('resourceAddress3').innerText = resourceAddress3;
    document.getElementById('resourceAddress4').innerText = resourceAddress4;
    document.getElementById('resourceAddress5').innerText = resourceAddress5;
    document.getElementById('resourceAddress6').innerText = resourceAddress6;
    document.getElementById('resourceAddress7').innerText = resourceAddress7;
    document.getElementById('resourceAddress8').innerText = resourceAddress8;
  
  } else {
    document.getElementById('componentAddress').innerText = 'Error: ' + receipt.status;
  }
}

// Create new user
document.getElementById('createNewUser').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress, 'new_user', [''])
    .callMethodWithAllResources(accountAddress1, 'deposit_batch')
    .build()
    .toString();

  }

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Define badge resource address
  if (receipt.status == 'Success') {
    badgeAddress = receipt.newResources[0];
  }

document.getElementById('createxrdpool').onclick = async function () {
  // Construct manifest

  var x = document.getElementById('addamount').value;

  const manifest = new ManifestBuilder()
    .callMethod(componentAddress"${ACC_ADDRESS1}") "create_proof_by_amount" Decimal("1") ResourceAddress("${PROOF}");
    .createProofFromAccountByAmount(accountAddress1, '1', badgeAddress)
    .withdrawFromAccountByAmount(accountAddress, x, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd') // Has "XRD" been defined yet?
    .callMethod(componentAddress, 'new_lending_pool', ['Bucket("xrd")', `ComponentAddress("${accountAddress}")`]) //How are environment variables used?
    .callMethodWithAllResources(accountAddress1, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};


document.getElementById('createusdpool').onclick = async function () {
// Construct manifest
    
  var x = document.getElementById('addamount').value;

  const manifest = new ManifestBuilder()
    .callMethod(componentAddress"${ACC_ADDRESS1}") "create_proof_by_amount" Decimal("1") ResourceAddress("${PROOF}");
    .createProofFromAccountByAmount(accountAddress1, '1', badgeAddress)
    .withdrawFromAccountByAmount(accountAddress, x, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd') // Has "XRD" been defined yet?
    .callMethod(componentAddress, 'new_lending_pool', ['Bucket("xrd")', `ComponentAddress("${accountAddress}")`]) //How are environment variables used?
    .callMethodWithAllResources(accountAddress1, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};

document.getElementById('supplyxrd').onclick = async function () {
  // Construct manifest
          
  var x = document.getElementById('addamount').value;

  const manifest = new ManifestBuilder()
    .callMethod(componentAddress"${ACC_ADDRESS1}") "create_proof_by_amount" Decimal("1") ResourceAddress("${PROOF}");
    .createProofFromAccountByAmount(accountAddress1, '1', badgeAddress)
    .withdrawFromAccountByAmount(accountAddress, x, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd') // Has "XRD" been defined yet?
    .callMethod(componentAddress, 'deposit_supply', ['Bucket("xrd")', `ComponentAddress("${accountAddress}")`]) //How are environment variables used?
    .callMethodWithAllResources(accountAddress1, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};
  

document.getElementById('supplyusd').onclick = async function () {
  // Construct manifest
      
  var x = document.getElementById('addamount').value;
    
  const manifest = new ManifestBuilder()
    .callMethod(componentAddress"${ACC_ADDRESS1}") "create_proof_by_amount" Decimal("1") ResourceAddress("${PROOF}");
    .createProofFromAccountByAmount(accountAddress1, '1', badgeAddress)
    .withdrawFromAccountByAmount(accountAddress, x, '030000000000000000000000000000000000000000000000000004')
    .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd') // Has "XRD" been defined yet?
    .callMethod(componentAddress, 'deposit_supply', ['Bucket("xrd")', `ComponentAddress("${accountAddress}")`]) //How are environment variables used?
    .callMethodWithAllResources(accountAddress1, 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
};
    
// Instantiate RadiSwap


//ADD LIQUIDITY TO THE LENDING POOL

document.getElementById('createxrdpool').onclick = async function () {
  // Construct manifest

var x = document.getElementById('addamount').value;

  const manifest = new ManifestBuilder()
    .callMethod(componentAddress"${ACC_ADDRESS1}") "create_proof_by_amount" Decimal("1") ResourceAddress("${PROOF}");
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