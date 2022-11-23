const path = require('path');
const https = require("https");
const fs = require("fs");
const cors = require("cors");
const express = require('express');
const ExpressUtil = require('./ExpressUtil.js');
const {init, compile_with_nonce} = require('./pte_manifest_compiler.js');
const crypto = require('node:crypto');
const { ec } = require("elliptic");
const fetch = require("node-fetch");


function readConfigFile(filename){
  console.log("read config file: ", filename);
  const config = JSON.parse(fs.readFileSync(filename));
  let hasError = false;
  if(!config.badgeAddress){
    console.log("missing badgeAddress from config file, this value should be the address of the admin badge returned during the creation of the central component");
    hasError = true;
  }
  if(!config.accountAddress){
    console.log("missing accountAddress from config file, this value should be the address of the admin account which has created the central component and received the admin badge");
    hasError = true;
  }
  if(!config.privateKey){
    console.log("missing privateKey from config file, this value should be the private key of the admin account");
    hasError = true;
  }
  if(!config.ledger){
    console.log("missing ledger from config file, this value should be the ledger address which contain the components to contact");
    hasError = true;
  }
  if(hasError)
    process.exit(1);
  return config;
}


console.log("Server launched with the following arguments: ", process.argv);
const config = readConfigFile("config.json");
let global_nonce = 1;
let activeTimer = {};

async function loadingWebAssembly(){
  let module = fs.promises.readFile("pte_manifest_compiler_bg.wasm");
  return await init(module);
}

function arrayToString(signature){
  return [...new Uint8Array(signature)]
      .map(x => x.toString(16).padStart(2, '0'))
      .join('');
}

async function getTimestamp(){
  const response = await fetch('http://worldtimeapi.org/api/timezone/Europe/Amsterdam');
  return (await response.json())["unixtime"];
}

function buildTransactionUpdateUnixTime(componentAddress, timestamp, badgeAddress) {
  return [
    'CALL_METHOD ComponentAddress("' + config.accountAddress + '") "create_proof" ResourceAddress("' + config.badgeAddress + '");',
    'CALL_METHOD ComponentAddress("' + componentAddress + '") "update_unix_time" ' + timestamp + 'u64;'
  ].join("\n");
}

function buildTransactionInvalidateUnixTime(componentAddress){
  return [
    'CALL_METHOD ComponentAddress("' + config.accountAddress + '") "create_proof" ResourceAddress("' + config.badgeAddress + '");',
    'CALL_METHOD ComponentAddress("' + componentAddress + '") "invalidate_unix_time";'
  ].join("\n");
}

function buildTransactionInstantiate(centralPkgAddr, offerPkgAddr){
  return [
    'CALL_FUNCTION PackageAddress("' + centralPkgAddr + '") "CentralRepository" "instantiate" PackageAddress("' + offerPkgAddr + '");',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + config.accountAddress + '") "deposit_batch";'
  ].join("\n");
}

async function sendTransaction(transaction){
  let nonce = global_nonce++;
  let compiledTransaction = compile_with_nonce(transaction, BigInt(nonce));
  const curve = new ec("p256");
  let keyPair = curve.keyFromPrivate(config.privateKey);
  const publicKey = keyPair.getPublic("hex"); 
  let hasher = crypto.createHash('sha256');
  let hashOfPayload = hasher.update(compiledTransaction).digest('hex');
  const signatureBuf = keyPair.sign(hashOfPayload, { canonical: true });
  let signature = arrayToString([
    ...signatureBuf.r.toArray(),
    ...signatureBuf.s.toArray(),
  ]);
  
  const body = {
    manifest: transaction,
    nonce: {value: nonce},
    signatures: [{public_key: publicKey, signature}]
  };
  const response = await fetch(config.ledger + "/transaction", {
    method: 'post',
    body: JSON.stringify(body),
    headers: {'Content-Type': 'application/json; charset=UTF-8'}
  });
  return await response.json();
}

async function invalidateTimestamp(componentAddress){
  let transaction = buildTransactionInvalidateUnixTime(componentAddress);
  let data = await sendTransaction(transaction);
  delete activeTimer[componentAddress];
  return (data["status"] == "Success");
}

async function main(){
  await loadingWebAssembly();
  
  const port = config.port || "8080";
  const baseUrl = "localhost://" + port;
  console.log("Server will be running on ", baseUrl);
  const app = express();
  let {post} = ExpressUtil(app);
  app.use(express.json());
  app.use(cors());
  
  if(process.argv[2] == "instantiate"){
    let centralPkgAddr = process.argv[3];
    let offerPkgAddr = process.argv[4];
    let transaction = buildTransactionInstantiate(centralPkgAddr, offerPkgAddr);
    let data = await sendTransaction(transaction);
    config.centralComponentAddress = data.new_components[0];
    console.log("central component address", config.centralComponentAddress);
    config.badgeAddress = data.new_resources[2];
    console.log("badge address", config.badgeAddress);
    fs.writeFileSync("config.json", JSON.stringify(config,"",2));
  }

  post('/sendTimeToComponent', async (req) => {
    let componentAddress = req.body.componentAddress;
    if(activeTimer[componentAddress]) 
      clearTimeout(activeTimer[componentAddress]);
    let timestamp = await getTimestamp();
    let transaction = buildTransactionUpdateUnixTime(componentAddress, timestamp);
    activeTimer[componentAddress] = setTimeout(() => invalidateTimestamp(componentAddress), 10 * 60 * 1000);
    let data = await sendTransaction(transaction); 
    if(data["status"] != "Success"){
      console.log(data);
      clearTimeout(activeTimer[componentAddress]);
      if(data.status.startsWith("AuthorizationError"))
        throw new Error("the oracle badge is not the one expected by this component");
      throw new Error("unable to udpate the time for the component in the ledger");
    }
    return {success: true};
  });

  // ***** Launch server *****
  if(config.https){
    const options = {
      key: fs.readFileSync(config.https.key),
      cert: fs.readFileSync(config.https.cert)
    };
    console.log("starting the https server");
    https.createServer(options, app).listen(port);
  }else{
    console.log("starting the http server");
    app.listen(port, () => console.log(`listening on ${port}`));
  }
}

main().then(() => console.log("ok")).catch(err => console.log("error", err));
