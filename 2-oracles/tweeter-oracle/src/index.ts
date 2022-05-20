import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
const XRD_ADDRESS = '030000000000000000000000000000000000000000000000000004';
let accountAddress = undefined; // User account address
let packageAddress = undefined; // GumballMachine package address
let tweeterOracleComponentAddress = undefined; //oracle component address 
let airdropComponentAddress = undefined ; // airdrop component address
let componentAddress = undefined; // GumballMachine component address
let tweeterOracleAdminBadgeResourceAddress = undefined; // tweeterOracle admin badge resource address

document.getElementById('fetchAccountAddress').onclick = async function () {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();
  document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function () {
  // Load the wasm
  const response = await fetch('./tweeter_oracle.wasm');
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


document.getElementById('instantiateTweeterOracleComponent').onclick = async function () {
  // Construct manifest
  const manifest = new ManifestBuilder()
    .callFunction(packageAddress, 'TweeterOracle', 'instantiate_tweeter_oracle', [''])
    .callMethodWithAllResources(accountAddress , 'deposit_batch')
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    tweeterOracleComponentAddress = receipt.newComponents[0];
    tweeterOracleAdminBadgeResourceAddress = receipt.newResources[0];
    document.getElementById('tweeterOracleComponentAddress').innerText = tweeterOracleComponentAddress;
  } else {
    document.getElementById('tweeterOracleComponentAddress').innerText = 'Error: ' + receipt.status;
  }
}

document.getElementById('insertFollowers').onclick = async function () {
  // Construct manifest

  let accountToFollow = document.getElementById('accountToFollow').value.trim(); 
  let followers = document.getElementById('followers').value?.split(';')
  
  if(!accountToFollow || accountToFollow?.trim()== ''){
    alert('accountToFollow is mandatory');
    return ;
  }

  
  if(!followers || followers.length == 0 ){
    alert('followers is mandatory');
    return ;
  }

  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, tweeterOracleAdminBadgeResourceAddress)
    .callMethod(tweeterOracleComponentAddress, 'insert_account_followers',[`"${accountToFollow}"`,`HashSet<String>(${followers.map(o => '"' + o.trim() + '"').join(',')})`])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('insertFollowersReceipt').innerText =  JSON.stringify(receipt, null, 2);
  
}

document.getElementById('checkIfFollower').onclick = async function() {
  
  let accountToFollow = document.getElementById('accountToFollowCheck').value.trim();
  let followerToCheck = document.getElementById('followerToCheck').value.trim(); 

  if(!accountToFollow || accountToFollow == ''){
    alert("account to Follow is mandatory");
    return; 
  }

  if(!followerToCheck || followerToCheck == ''){
    alert("follower to Check is mandatory");
    return; 
  }

  const manifest = new ManifestBuilder()
  .callMethod(tweeterOracleComponentAddress, 'is_account_follower',[`"${accountToFollow}"`,`"${followerToCheck}"`])
  .build()
  .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('checkIfFollowerReceipt').innerText =  JSON.stringify(receipt, null, 2);

  document.getElementById('checkIfFollowerResponse').innerText = receipt.outputs[0]; 
}

document.getElementById('insertLikers').onclick = async function () {
  // Construct manifest

  let tweetToLikeId = document.getElementById('tweetToLikeId').value.trim(); 
  let likers = document.getElementById('likers').value?.split(';')
  
  if(!tweetToLikeId || tweetToLikeId?.trim()== ''){
    alert('tweetToLikeId is mandatory');
    return ;
  }

  
  if(!likers || likers.length == 0 ){
    alert('likers is mandatory' );
    return ;
  }

  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, tweeterOracleAdminBadgeResourceAddress)
    .callMethod(tweeterOracleComponentAddress, 'insert_tweets_likers',[`"${tweetToLikeId}"`,`HashSet<String>(${likers.map(o => '"' + o.trim() + '"').join(',')})`])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('insertLikersReceipt').innerText =  JSON.stringify(receipt, null, 2);
  
}

document.getElementById('checkIfLiker').onclick = async function() {
  
  let tweetIdCheck = document.getElementById('tweetIdCheck').value.trim();
  let likerToCheck = document.getElementById('likerToCheck').value.trim(); 

  if(!tweetIdCheck || tweetIdCheck == ''){
    alert("tweetId is mandatory");
    return; 
  }

  if(!likerToCheck || likerToCheck == ''){
    alert("liker to Check is mandatory");
    return; 
  }

  const manifest = new ManifestBuilder()
  .callMethod(tweeterOracleComponentAddress, 'is_tweet_liker',[`"${tweetIdCheck}"`,`"${likerToCheck}"`])
  .build()
  .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('checkIfLikerReceipt').innerText =  JSON.stringify(receipt, null, 2);

  document.getElementById('checkIfLikerResponse').innerText = receipt.outputs[0]; 
}

document.getElementById('insertRetweeters').onclick = async function () {
  // Construct manifest

  let tweetToRetweetId = document.getElementById('tweetToRetweetId').value.trim(); 
  let retweeters = document.getElementById('retweeters').value?.split(';');
  
  if(!tweetToRetweetId || tweetToRetweetId?.trim()== ''){
    alert('tweetToRetweetId is mandatory');
    return ;
  }

  
  if(!retweeters || retweeters.length == 0 ){
    alert('retweeters is mandatory');
    return ;
  }

  const manifest = new ManifestBuilder()
    .createProofFromAccountByAmount(accountAddress, 1, tweeterOracleAdminBadgeResourceAddress)
    .callMethod(tweeterOracleComponentAddress, 'insert_tweets_retweeters',[`"${tweetToRetweetId}"`,`HashSet<String>(${retweeters.map(o => '"' + o.trim() + '"').join(',')})`])
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('insertRetweetersReceipt').innerText =  JSON.stringify(receipt, null, 2);
  
}

document.getElementById('checkIfRetweeter').onclick = async function() {
  
  let tweetToRetweetIdCheck = document.getElementById('tweetToRetweetIdCheck').value.trim();
  let retweeterToCheck = document.getElementById('retweeterToCheck').value.trim(); 

  if(!tweetToRetweetIdCheck || tweetToRetweetIdCheck == ''){
    alert("tweetToRetweetId is mandatory");
    return; 
  }

  if(!retweeterToCheck || retweeterToCheck == ''){
    alert("Retweeter to Check is mandatory");
    return; 
  }

  const manifest = new ManifestBuilder()
  .callMethod(tweeterOracleComponentAddress, 'is_tweet_retweeter',[`"${tweetToRetweetIdCheck}"`,`"${retweeterToCheck}"`])
  .build()
  .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('checkIfRetweeterReceipt').innerText =  JSON.stringify(receipt, null, 2);

  document.getElementById('checkIfRetweeterResponse').innerText = receipt.outputs[0]; 
}

document.getElementById('instantiateAirdropComponent').onclick = async function () {
  // Construct manifest

  let accountsToFollow = document.getElementById('accountsToFollow').value?.split(';');
  let tweetsToLike = document.getElementById('tweetsToLike').value?.split(';');
  let tweetsToRetweet = document.getElementById('tweetsToRetweet').value?.split(';'); 

  if((!accountsToFollow || accountsToFollow.length == 0) 
    && (!tweetsToLike || tweetsToLike.length == 0) 
    && (!tweetsToRetweet || tweetsToRetweet.length == 0) )
  {
     alert("You must specify at least one of the following values  : accounts to follow or tweets to like or tweets to retweet");
     return; 
  }
  
  const manifest = new ManifestBuilder()
    .callFunction(packageAddress, 'AirdropWithTweeterOracle', 'new',[`ResourceAddress("${XRD_ADDRESS}")`,
                                                                    `Vec<String>(${accountsToFollow.map(o => '"' + o.trim() + '"').join(',')})`,
                                                                    `Vec<String>(${tweetsToLike.map(o => '"' + o.trim() + '"').join(',')})`,
                                                                    `Vec<String>(${tweetsToRetweet.map(o => '"' + o.trim() + '"').join(',')})`,
                                                                     `ComponentAddress("${tweeterOracleComponentAddress}")`])
    .callMethodWithAllResources(accountAddress , 'deposit_batch')                                                             
    .build()
    .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    tweeterOracleComponentAddress = receipt.newComponents[0];
    tweeterOracleAdminBadgeResourceAddress = receipt.newResources[0];
    document.getElementById('airdropComponentAddress').innerText = tweeterOracleComponentAddress;
  } else {
    document.getElementById('airdropComponentAddress').innerText = 'Error: ' + receipt.status;
  }
}


// document.getElementById('buyGumball').onclick = async function () {
//   // Construct manifest
//   const manifest = new ManifestBuilder()
//     .withdrawFromAccountByAmount(accountAddress, 1, '030000000000000000000000000000000000000000000000000004')
//     .takeFromWorktop('030000000000000000000000000000000000000000000000000004', 'xrd')
//     .callMethod(componentAddress, 'buy_gumball', ['Bucket("xrd")'])
//     .callMethodWithAllResources(accountAddress, 'deposit_batch')
//     .build()
//     .toString();

//   // Send manifest to extension for signing
//   const receipt = await signTransaction(manifest);

//   // Update UI
//   document.getElementById('receipt').innerText = JSON.stringify(receipt, null, 2);
// };

// document.getElementById('checkBalance').onclick = async function () {
//   // Retrieve component info from PTE service
//   const api = new DefaultApi();
//   const userComponent = await api.getComponent({
//     address: accountAddress
//   });
//   const machineComponent = await api.getComponent({
//     address: componentAddress
//   });

//   // Update UI
//   document.getElementById('userBalance').innerText = userComponent.ownedResources
//     .filter(e => e.resourceAddress == tweeterOracleAdminBadgeResourceAddress)
//     .map(e => e.amount)[0] || '0';
//   document.getElementById('machineBalance').innerText = machineComponent.ownedResources
//     .filter(e => e.resourceAddress == tweeterOracleAdminBadgeResourceAddress).map(e => e.amount)[0];
// };