import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
let accountAddress = undefined;
let participantNfid = undefined;
let participantsNftAddress = undefined;
let participantsOwnerNfid = undefined;
let requestorComponentAddress = undefined;
let loanRequestNftAddress = undefined;
let requestorAdminBadgeAddress = undefined;
let requestorConfigBadgeAddress = undefined;
let acceptorComponentAddress = undefined;
let loanNftAddress = undefined;
let loanRequestNfid = undefined;
let loanNfid = undefined;

let demifiPackageAddress = undefined;
let participantsComponentAddress = undefined;
let participantsResourceAddress = undefined;

let RADIX_TOKEN = '030000000000000000000000000000000000000000000000000004';

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();

  document.getElementById('accountAddress').innerText = accountAddress;
};


document.getElementById('publishDemifi').onclick = async function () {
  // Load the wasm
  const response = await fetch('./demifi.wasm');
  const wasm = new Uint8Array(await response.arrayBuffer());

  // Construct manifest
  const manifest = new ManifestBuilder()
    .publishPackage(wasm)
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  demifiPackageAddress = receipt.newPackages[0];
  document.getElementById('demifiPackageAddress').innerText = demifiPackageAddress;
};


document.getElementById('instantiateParticipants').onclick = async function () {
{
}
  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .callFunction(demifiPackageAddress, 'Participants', 'instantiate_participant_catalog', ['None', 'None', 'None'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    let result = receipt.outputs[0].match(/ComponentAddress\(\\"(.*?)\\".*ResourceAddress\(\\"(.*?)\\".*NonFungibleId\(\\"(.*?)\\"/);
      participantsOwnerNfid = result[3];
      document.getElementById('participantsOwnerNfid').innerText = participantsOwnerNfid;
  
    // Update UI
    if (receipt.status == 'Success') {
      participantsComponentAddress = receipt.newComponents[0];
      document.getElementById('participantsComponentAddress').innerText = participantsComponentAddress;
      participantsNftAddress = receipt.newResources[1];
      document.getElementById('participantsNftAddress').innerText = participantsNftAddress;
    } else {
      document.getElementById('participantsComponentAddress').innerText = 'Error: ' + receipt.status;
    }
  }
  {
    const manifest2 = new ManifestBuilder()
      .callMethod(participantsComponentAddress, 'new_participant', ['"Alice"', '"url"', '"ID ref"', 'None'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt2 = await signTransaction(manifest2);
    participantNfid = receipt2.outputs[0].match(/"value":"NonFungibleId\(\\"(.*?)\\"/)[1];

    document.getElementById('participantNfid').innerText = participantNfid;
  }
}


document.getElementById('instantiateRequestor').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .callFunction(demifiPackageAddress, 'LoanRequestor', 'instantiate_requestor', ['ResourceAddress("'+participantsNftAddress+'")', 'None', 'None'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      requestorComponentAddress = receipt.newComponents[0];
      document.getElementById('requestorComponentAddress').innerText = requestorComponentAddress;
      requestorAdminBadgeAddress = receipt.newResources[0];
      document.getElementById('requestorAdminBadgeAddress').innerText = requestorAdminBadgeAddress;
      requestorConfigBadgeAddress = receipt.newResources[1];
      document.getElementById('requestorConfigBadgeAddress').innerText = requestorConfigBadgeAddress;
      loanRequestNftAddress = receipt.newResources[2];
      document.getElementById('loanRequestNftAddress').innerText = loanRequestNftAddress;
    } else {
      document.getElementById('errorReturn').innerText = 'Error: ' + receipt.status;
    }
  }
}


document.getElementById('instantiateAcceptor').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .callFunction(demifiPackageAddress, 'LoanAcceptor', 'instantiate_loan_acceptor', ['ResourceAddress("'+participantsNftAddress+'")', 'ResourceAddress("'+requestorAdminBadgeAddress+'")', 'None', 'Decimal("0")', 'None', 'None'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      acceptorComponentAddress = receipt.newComponents[0];
      document.getElementById('acceptorComponentAddress').innerText = acceptorComponentAddress;
      loanNftAddress = receipt.newResources[0];
      document.getElementById('loanNftAddress').innerText = loanNftAddress;
    } else {
      document.getElementById('errorReturn').innerText = 'Error: ' + receipt.status;
    }
  }
}


document.getElementById('activateAcceptor').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress, requestorConfigBadgeAddress)
      .takeFromWorktopByAmount(1, requestorConfigBadgeAddress, "badge")
      .callMethod(requestorComponentAddress, 'set_loan_acceptor', ['Bucket("badge")', 'ComponentAddress("'+acceptorComponentAddress+'")'])
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      document.getElementById('acceptorActivatedOk').innerText = "Ok";
    } else {
      document.getElementById('acceptorActivatedOk').innerText = 'Error: ' + receipt.status;
    }
  }
}


document.getElementById('requestLoan').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .createProofFromAccountByIds(accountAddress, [ participantNfid ], participantsNftAddress)
      .popFromAuthZone('participant_proof')
      .callMethod(requestorComponentAddress, 'request_loan',
         ['Proof("participant_proof")',
	  'ResourceAddress("'+RADIX_TOKEN+'")',
	  'Decimal("5000")',
	  'Decimal("100")',
	  '1u64',
	  '1u64',
	  '2u64',
	  '2u64',
	  'Decimal("3000")',
	  '"I will go to the moon and back again and sell NFTs of the trip"',
	  '"moon://darkside.org"'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      loanRequestNfid = receipt.outputs[2].match(/"value":"NonFungibleId\(\\"(.*?)\\"/)[1];
      document.getElementById('loanRequestNfid').innerText = loanRequestNfid;
    } else {
      document.getElementById('loanRequestNfid').innerText = 'Error: ' + receipt.status;
    }
  }
}

document.getElementById('pledgeLoan').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress, 5000, RADIX_TOKEN)
      .takeFromWorktopByAmount(5000, RADIX_TOKEN, 'pledge')
      .createProofFromAccountByIds(accountAddress, [ participantNfid ], participantsNftAddress)
      .popFromAuthZone('participant_proof')
      .callMethod(requestorComponentAddress, 'pledge_loan',
         ['Proof("participant_proof")',
	  'NonFungibleId("'+loanRequestNfid+'")',
	  'Bucket("pledge")'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      document.getElementById('pledgeOk').innerText = 'Ok';
    } else {
      document.getElementById('pledgeOk').innerText = 'Error: ' + receipt.status;
    }
  }
}


document.getElementById('startLoan').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .withdrawFromAccountByIds(accountAddress, [ loanRequestNfid ], loanRequestNftAddress)
      .takeFromWorktopByIds([ loanRequestNfid ], loanRequestNftAddress, 'loan_request')
      .createProofFromAccountByIds(accountAddress, [ participantNfid ], participantsNftAddress)
      .popFromAuthZone('participant_proof')
      .callMethod(requestorComponentAddress, 'start_loan',
         ['Proof("participant_proof")',
	  'Bucket("loan_request")'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      loanNfid = receipt.outputs[4].match(/"value":"NonFungibleId\(\\"(.*?)\\"/)[1];
      document.getElementById('loanNfid').innerText = loanNfid;
    } else {
      document.getElementById('loanNfid').innerText = 'Error: ' + receipt.status;
    }
  }
}

document.getElementById('payInstallment').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress, 5000, RADIX_TOKEN)
      .takeFromWorktopByAmount(5000, RADIX_TOKEN, 'payment')
      .callMethod(acceptorComponentAddress, 'pay_installment',
         ['NonFungibleId("'+loanNfid+'")',
	  'Bucket("payment")'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

    // Update UI
    if (receipt.status == 'Success') {
      document.getElementById('payInstallmentStatus').innerText = document.getElementById('payInstallmentStatus').innerText + " Ok";
    } else {
      document.getElementById('payInstallmentStatus').innerText = 'Error: ' + receipt.status;
    }
  }
}


document.getElementById('claimRewards').onclick = async function () {

  {
    // Construct manifest
    const manifest = new ManifestBuilder()
      .createProofFromAccountByIds(accountAddress, [ participantNfid ], participantsNftAddress)
      .popFromAuthZone('participant_proof')
      .callMethod(acceptorComponentAddress, 'claim_lender_rewards',
         ['Proof("participant_proof")'])
      .callMethodWithAllResources(accountAddress, 'deposit_batch')
      .build()
      .toString();
  
    // Send manifest to extension for signing
    const receipt = await signTransaction(manifest);

//document.getElementById('errorReturn').innerText = 'Error: ' + JSON.stringify(receipt, null, 2);
    // Update UI
    if (receipt.status == 'Success') {
      document.getElementById('claimRewardsStatus').innerText = 'Ok';
    } else {
      document.getElementById('claimRewardsStatus').innerText = 'Error: ' + receipt.status;
    }
  }
}
