import { DefaultApi, ManifestBuilder } from 'pte-sdk';
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk';

// Global states
const XRD_ADDRESS = '030000000000000000000000000000000000000000000000000004';
let accountAddress = undefined; // User account address
let packageAddress = undefined; // TweeterOracle package address
let tweeterOracleComponentAddress = undefined; //TweeterOracle component address 
let airdropComponentAddress = undefined ; // airdrop component address
let airdropComponentAdminBadgeResourceAddress = undefined;
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

document.getElementById('addDatasToUpdate').onclick = async function() {
  
  let accountsToFollow = document.getElementById('accountsToFollow').value?.split(';').filter(o => o.trim()!=""); 
  let tweetsToLike = document.getElementById('tweetsToLike').value?.split(';').filter(o => o.trim()!=""); 
  let tweetsToRetweet = document.getElementById('tweetsToRetweet').value?.split(';').filter(o => o.trim()!=""); 

  if(accountsToFollow.length == 0 && tweetsToLike.length == 0 && tweetsToRetweet.length == 0 ){
    alert("You must provide at least one entry");
    return ;
  }

  var manifestBuilder = new ManifestBuilder(); 
  
  if(accountsToFollow.length > 0 ){
    manifestBuilder = manifestBuilder.callMethod(tweeterOracleComponentAddress, 'add_accounts_to_follows',[`Vec<String>(${accountsToFollow.map(o => '"' + o.trim() + '"').join(',')})`]);
  }

  if(tweetsToLike.length > 0){
    manifestBuilder = manifestBuilder.callMethod(tweeterOracleComponentAddress, 'add_tweets_to_like',[`Vec<String>(${tweetsToLike.map(o => '"' + o.trim() + '"').join(',')})`]);
  }

  if(tweetsToRetweet.length > 0){
    manifestBuilder = manifestBuilder.callMethod(tweeterOracleComponentAddress, 'add_tweets_to_retweet',[`Vec<String>(${tweetsToRetweet.map(o => '"' + o.trim() + '"').join(',')})`]);
  }
  

  const manifest = manifestBuilder
  .build()
  .toString();

  // Send manifest to extension for signing
   const receipt = await signTransaction(manifest);

    // Update UI
  document.getElementById('addDatasToUpdateReceipt').innerText =  JSON.stringify(receipt, null, 2);
}

document.getElementById('getDatasToUpdate').onclick = async function() {
  
  const manifest = new ManifestBuilder()
  .callMethod(tweeterOracleComponentAddress, 'get_datas_to_update',[])
  .build()
  .toString();

   // Send manifest to extension for signing
   const receipt = await signTransaction(manifest);

    // Update UI
  document.getElementById('getDatasToUpdateReceipt').innerText =  JSON.stringify(receipt, null, 2);

  if(receipt.status != "Success")
      return; 

  var infos = receipt.logs[0].split('|');
  const selectIdByKey = { 
                          "ACCOUNTS_TO_FOLLOW":["accountToFollow", "accountToFollowCheck"], 
                          "TWEETS_TO_LIKE":["tweetToLikeId","tweetIdCheck"],
                          "TWEETS_TO_RETWEETS":["tweetToRetweetId","tweetToRetweetIdCheck"]
                        }
  infos.forEach(element => {
     let key = element.split(":")[0];
     let values = element.split(":")[1].split(";").filter(o => o.trim()!= "");

     if(values.length > 0){
       
      selectIdByKey[key].forEach(selectId => {
      
       let select = document.getElementById(selectId);
      
        while(select.options.length > 0){
          select.remove(0); 
        }

        let defaultOption = document.createElement("option"); 
        defaultOption.text = "Select";
        defaultOption.value = "";
        select.add(defaultOption, null);

        for(let i in values) {
              let opt = document.createElement("option"); 
              opt.text = values[i]; 
              opt.value = values[i]; 
              select.add(opt, null);
         }  
     });  
    } 
  });
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
    airdropComponentAddress = receipt.newComponents[0];
    airdropComponentAdminBadgeResourceAddress = receipt.newResources[0];
    document.getElementById('airdropComponentAddress').innerText = airdropComponentAddress;
  } else {
    document.getElementById('airdropComponentAddress').innerText = 'Error: ' + receipt.status;
  }
}

document.getElementById('findAndStoreAirdropRecipients').onclick = async function(){
  
  const manifest = new ManifestBuilder()
  .createProofFromAccountByAmount(accountAddress, 1, airdropComponentAdminBadgeResourceAddress)
  .callMethod(airdropComponentAddress, 'find_and_store_airdrop_recipients',[])
  .build()
  .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('findAndStoreAirdropRecipientsReceipt').innerText =  JSON.stringify(receipt, null, 2);
}
