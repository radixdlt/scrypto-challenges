import {getAccountAddress, signTransaction} from 'pte-browser-extension-sdk';
import {Configuration, DefaultApi, ManifestBuilder, SubmitTransactionRequest} from 'pte-sdk';
//import { updateCommaList } from 'typescript';

// set to false to use the real pte (not well tested)
// true means the localpte
const USE_LOCAL_PTE = false;

let PTE_BASE_PATH = undefined;
if (USE_LOCAL_PTE) {
  PTE_BASE_PATH = 'http://localhost:3500';
} else {
  PTE_BASE_PATH = 'https://pte02.radixdlt.com';
}

// wrapper around the API to create "getTransactions" and interact locally instead of remotely
class LocalPTE extends DefaultApi {
  async getTransactions() {
      if (!USE_LOCAL_PTE) {
        if (txhash_to_validate === undefined) {
          return []
        } else {
          return [txhash_to_validate]
        }
      } else {
        const queryParameters: any = {};
        const headerParameters: any = {};
        const response = await this.request({
            path: `/transactions`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, {});
        return response.json();
      }
  }
  constructor(
      config = new Configuration({basePath: PTE_BASE_PATH})) {
    super(config);
  }
}

// Global states
let accountAddress = undefined;    // User account address
let packageAddress = undefined;    // package address
let componentAddress = undefined;  // component address
let txhash_to_validate = undefined;   // txhash that needs MFA validation

// load previous addresses for faster testing
accountAddress = localStorage.getItem('accountAddress');
packageAddress = localStorage.getItem('packageAddress');
componentAddress = localStorage.getItem('componentAddress');
document.getElementById('accountAddress').innerText = accountAddress;
document.getElementById('packageAddress').innerText = packageAddress;
document.getElementById('componentAddress').innerText = componentAddress;

document.getElementById('fetchAccountAddress').onclick = async function() {
  // Retrieve extension user account address
  accountAddress = await getAccountAddress();
  localStorage.setItem('accountAddress', accountAddress);

  document.getElementById('accountAddress').innerText = accountAddress;
};

document.getElementById('publishPackage').onclick = async function() {
  // Load the wasm
  const response = await fetch('./mfa_oracle.wasm');
  const wasm = new Uint8Array(await response.arrayBuffer());

  // Construct manifest
  const manifest =
      new ManifestBuilder().publishPackage(wasm).build().toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  packageAddress = receipt.newPackages[0];
  localStorage.setItem('packageAddress', packageAddress);
  document.getElementById('packageAddress').innerText = packageAddress;
};


document.getElementById('instantiateComponent').onclick =
    async function() {
  // Construct manifest
  
  // This example uses the localhost version used for simplicity.  For real deployments use "new"
  // also using the signer virtual badge though any non-fungible could be used.
  const ECDSA_TOKEN = "030000000000000000000000000000000000000000000000000005";
  const manifest = new ManifestBuilder()
                       .createProofFromAuthZone(ECDSA_TOKEN, "signer_badge")
                       .callFunction(packageAddress, 'MFAOracle', 'new_localhost', ['Proof("signer_badge")'])
                       .build()
                       .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  if (receipt.status == 'Success') {
    componentAddress = receipt.newComponents[0];
    localStorage.setItem('componentAddress', componentAddress);
    document.getElementById('componentAddress').innerText = componentAddress;
  } else {
    document.getElementById('componentAddress').innerText =
        'Error: ' + receipt.status;
  }
}

async function digestMessage(message) {
  const msgUint8 = new TextEncoder().encode(message);                           // encode as (utf-8) Uint8Array
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgUint8);           // hash the message
  const hashArray = Array.from(new Uint8Array(hashBuffer));                     // convert buffer to byte array
  const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join(''); // convert bytes to hex string
  return hashHex;
}

const hex2bytes = function(q) {
    var bytes = [];
    for (var i = 0; i < q.length; i += 2) {
        var byte = parseInt(q.substring(i, i + 2), 16);
        if (byte > 127) {
            byte = -(~byte & 0xFF) - 1;
        }
        bytes.push(byte);
    }
    return bytes;
}
const my_concat = function(...arrays) {
    let myArrays = arrays;

    // Get the total length of all arrays.
    let length = 0;
    myArrays.forEach(item => {
      length += item.length;
    });

    // Create a new array with total length and merge all source arrays.
    let mergedArray = new Uint8Array(length);
    let offset = 0;
    myArrays.forEach(item => {
      mergedArray.set(item, offset);
      offset += item.length;
    });
    return mergedArray;
}

const get_registration_challenge = async function(rp: string, origin: string, componentAddress: string, registration_index: string): Promise<Uint8Array> {
  // build input to hash
  // componentAddress as byte array
  let ca_bytes_from_str = hex2bytes(componentAddress);
  ca_bytes_from_str.shift(); // drop the starting 0x02 component address identifier since they are not used in the hash calculation on-ledger
  const ca_bytes = Uint8Array.from(ca_bytes_from_str);
  // rp
  const rp_bytes = Uint8Array.from(rp, c => c.charCodeAt(0));
  // origin
  const origin_bytes = Uint8Array.from(origin, c => c.charCodeAt(0));
  // registration_index as le 128 bytes
  const ri_bytes = Uint8Array.from(hex2bytes(registration_index));
  console.log("hash input:", ca_bytes, rp_bytes, origin_bytes, ri_bytes);
  const buffer = my_concat(ca_bytes, rp_bytes, origin_bytes, ri_bytes);
  const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);           // hash the message
  return new Uint8Array(hashBuffer);
}
const get_authentication_challenge = async function(rp: string, origin: string, componentAddress: string, txhash: string): Promise<Uint8Array> {
  // build input to hash
  // componentAddress as byte array
  let ca_bytes_from_str = hex2bytes(componentAddress);
  ca_bytes_from_str.shift(); // drop the starting 0x02 component address identifier since they are not used in the hash calculation on-ledger
  const ca_bytes = Uint8Array.from(ca_bytes_from_str);
  // rp
  const rp_bytes = Uint8Array.from(rp, c => c.charCodeAt(0));
  // origin
  const origin_bytes = Uint8Array.from(origin, c => c.charCodeAt(0));
  // tx hash being authorized
  const txhash_bytes = Uint8Array.from(hex2bytes(txhash));
  console.log("hash input:", ca_bytes, rp_bytes, origin_bytes, txhash_bytes);
  const buffer = my_concat(ca_bytes, rp_bytes, origin_bytes, txhash_bytes);
  const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);           // hash the message
  return new Uint8Array(hashBuffer);
}

document.getElementById('registerMFA').onclick = async function() {
  const user_id = 'user_id';                    // TODO get from UI
  const user_name = 'user_name';                // TODO get from UI
  const user_displayName = 'user_displayName';  // TODO get from UI

  // get the registration_index from the current state of the component
  // TODO read registration index from ledger
  const registration_index = "00000000000000000000000000000000";

  // unlike typical webauthn, we will generate the registration request locally
  // we can do this because the challenge will be derived the same way on ledger (avoiding replays)
  // and we trust the Component
  const rp_name = (new URL(window.origin)).hostname; // this rp should match how the Component was setup
  const origin = window.origin;

  const challenge = await get_registration_challenge(rp_name, origin, componentAddress, registration_index)
  console.log("challenge", challenge);

  // act as the server for webauthn generating the registration request
  //
  const publicKey = {
    // random, cryptographically secure (against replay, does not need to be private), at least 16 bytes
    challenge: challenge,
    // relying party
    rp: {
      name: rp_name
    },
    user: {
      id: strToBytes(user_id),
      name: user_name,
      displayName: user_displayName
    },
    authenticatorSelection: {userVerification: 'preferred'},
    attestation: 'direct',
    pubKeyCredParams: [{
      type: 'public-key',
      alg: -7  // "ES256" IANA COSE Algorithms registry
    }]
  }

  // Send to the WebAuthn "create" API
  const cred: PublicKeyCredential =
      await navigator.credentials.create({publicKey: publicKey as PublicKeyCredentialCreationOptions}) as
      PublicKeyCredential;

  console.log("cred:", cred); // ok to share with anyone (it's going on ledger too!)

  // store the generated id in local storage so we know which one to use for validation.
  // in a more production ready version this might come from the ledger
  // store as base64url since we need it that way later anyway
  const id_base64 = binToStr(cred.rawId);
  const id_base64url = id_base64.replace("+", "-").replace("/", "_");
  localStorage.setItem('rawId', id_base64url);

  const cred_json = cred_to_json(cred);
  const register_response = arg_for_manifest(cred_json);

  // A proof is required to be passed implicitly via the AuthZone to call register
  // In this example, the Component was created with a rule matching the ECDSA_TOKEN virtual badge
  // so nothing additional is needed here.  (User must make sure to sign the transaction before submitting it)

  // Construct manifest
  const manifest = new ManifestBuilder()
                       .callMethod(componentAddress, 'register', [
                         arg_for_manifest(user_id),
                         arg_for_manifest(user_name),
                         arg_for_manifest(user_displayName),
                         register_response
                        ])
                       .build()
                       .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('registerMFA_receipt').innerText =
      JSON.stringify(receipt, null, 2);
};

// this is super ugly and not robust at all but shows the idea
const parse_oracle_state_tx_list = function(state_str: string): Array<string> {
  // find the HashSet<String>(...)
  const regexp = /HashSet<String>\((.*?)\)/;
  const match = state_str.match(regexp);
  console.log("parsed_state", match);
  const result = match[1].split(",").map(x => x.trim().slice(1,-1)); // split by comma and shop the quotes
  console.log("result", result);
  return result;
}

// this is in place of a real event protocol, but also filters based on existing custom Component state
const poll_for_mfa = async function(api: LocalPTE, componentAddress: string) {
  const last_tx_hash_to_validate = txhash_to_validate; // if this no longer needs validation by the end, then resend the now valid tx
  txhash_to_validate = undefined;
  document.getElementById("mfa_needed_div").style.display = "none";
  const oracleComponent = await api.getComponent({address: componentAddress});
  const previously_authenticated_transaction_list = parse_oracle_state_tx_list(oracleComponent.state);
  const all_transaction_list = await api.getTransactions();
  const new_transactions_list = all_transaction_list.slice(-10); // for example, just look at the latest 10, from last to first
  let reversed_transactions_list = Array.from(new_transactions_list);
  reversed_transactions_list.reverse();
  const search_tx_list = reversed_transactions_list.filter(x => !previously_authenticated_transaction_list.includes(x as string) ) as Array<string>;
  console.log("new_transactions_list:", new_transactions_list);
  console.log("reversed_transactions_list:", reversed_transactions_list);
  console.log("search_tx_list:", search_tx_list);
  for (const txhash of search_tx_list) {
    console.log("checking tx:", txhash);
    const r = await api.getReceipt({ hash: txhash });
    if (r.status != "SUCCESS") { // find only failed txs
        console.log("found failed tx: ", r);
      // check the first instruction is a "check" on the componentAddress expected.
      if (r.logs.length > 0) {
        const first_output = r.logs[0];
        if (first_output.indexOf("[ERROR] Panicked at 'MFA Needed for Transaction: " + txhash) != -1) {
          // now check component that was called (would love a better way than parsing the manifest...)
          const tx = await api.getTransaction({ hash: txhash });
          const m = tx.manifest;
          console.log("found failed tx with log emitted. tx: ", tx);
          const needle = 'CALL_METHOD ComponentAddress("' + componentAddress + '") "check" ;';
          if (m.startsWith(needle)) {
            // matched component
            console.log("MATCH")
            txhash_to_validate = txhash;
            document.getElementById('mfa_needed').innerText = "MFA needed for tx: " + txhash + "\n\n" + tx.manifest;
            document.getElementById("mfa_needed_div").style.borderColor = "thick solid #FF0000";
            document.getElementById("mfa_needed_div").style.display = "inline";
            break; // keep only the latest one
          }
        }
      }
    }
  }
  if (txhash_to_validate === undefined) { // none found so resend the last one if we have one because it must now be ready
    if (last_tx_hash_to_validate !== undefined) {
      console.log("Resubmitting transaction with MFA check:", last_tx_hash_to_validate);
      // resubmit the transaction, should pass now
      const tx = await api.getTransaction({hash: last_tx_hash_to_validate});
      const submit_tx: SubmitTransactionRequest = { transaction: tx } ;
      const receipt = await api.submitTransaction(submit_tx);
      // Update UI
      document.getElementById('receipt').innerText =
          JSON.stringify(receipt, null, 2);
        }
  }
  console.log("done");
  //console.log(transaction_list)
}

document.getElementById('validateMFA').onclick =
    async function() {

  if (!txhash_to_validate) {
    await refresh_ledger_state(componentAddress);
    const api = new LocalPTE();
    await poll_for_mfa(api, componentAddress);
    if (!txhash_to_validate) {
      alert("No MFA validation required. (even after refresh)");
      return;
    }
  }
  // this id must match the real one in the browser
  // pull from local storage
  // in a more production ready version this might come from the ledger
  const user_id_b64url = localStorage.getItem('rawId');
  // urlsafe b64 decode
  const id_raw_str = atob(user_id_b64url.replace("-", "+").replace("_","/"));
  const id_raw = Uint8Array.from(id_raw_str, c => c.charCodeAt(0));

  // these should match device registration and component instantiation
  const rp = (new URL(window.origin)).hostname; // this rp should match how the Component was setup
  const origin = window.origin; // this should match how the device was registered (and how the component was setup)

  const challenge = await get_authentication_challenge(rp, origin, componentAddress, txhash_to_validate);

  console.log("user_id:", user_id_b64url);
  console.log("challenge:", challenge);

  const publicKey = {
    rpId: rp,
    challenge: challenge,
    allowCredentials: [{id: id_raw, type: 'public-key'}],
    authenticatorSelection: {userVerification: 'preferred'},
    userVerification: 'preferred', // this extra line avoids a warning
  };

  const cred = await navigator.credentials.get({publicKey: publicKey as PublicKeyCredentialRequestOptions}) as PublicKeyCredential;

  const cred_json = cred_to_json(cred);
  const response = arg_for_manifest(cred_json);

  // Construct manifest
  const manifest =
      new ManifestBuilder()
          .callMethod(componentAddress, 'authorize_transaction', [
            arg_for_manifest(user_id_b64url),
            response,
            arg_for_manifest(txhash_to_validate),
          ])
          .build()
          .toString();

  // Send manifest to extension for signing/sending
  const receipt = await signTransaction(manifest);

  // Update UI
  document.getElementById('validateMFA_receipt').innerText =
      JSON.stringify(receipt, null, 2);
}


    document.getElementById('checkMFA')
        .onclick =
        async function() {
  // Construct manifest
  const manifest =
      new ManifestBuilder()
          .callMethod(componentAddress, 'check', []) // insert this call anywhere in the manifest to give it MFA support (this otherwise empty manifest is fine for the demo)
          .build()
          .toString();

  // Send manifest to extension for signing
  const receipt = await signTransaction(manifest);

  if (receipt.status != "SUCCESS") {
    txhash_to_validate = receipt.transactionHash;   // txhash that needs MFA validation
  }
  // Update UI
  document.getElementById('receipt').innerText =
      JSON.stringify(receipt, null, 2);
};

document.getElementById('refresh_oracle_status').onclick = async function() {
  refresh_ledger_state(componentAddress);
  const api = new LocalPTE();
  poll_for_mfa(api, componentAddress);
};

document.getElementById('select_component').onclick = async function() {
  componentAddress = prompt("Enter Component Address");
  localStorage.setItem('componentAddress', componentAddress);
  document.getElementById('componentAddress').innerText = componentAddress;
  packageAddress = "";
  localStorage.setItem('packageAddress', packageAddress);
  document.getElementById('packageAddress').innerText = packageAddress;
}

const refresh_ledger_state = async function(componentAddress: string) {
  const api = new LocalPTE();
  const mfa_component = await api.getComponent({address: componentAddress});

  // show rp id / origin (maybe check against this origin and warn)
  document.getElementById('oracle_status').innerText = mfa_component.state;

  // things to make the UI nicer... TODO
  // update list of RPs/Devices (credentials)
  // update list of validated transactions

  // if validated_transactions changes length resend stored transaction

}

/////////////////////////////
// utils
/////////////////////////////


////////
// from https://github.com/dvas0004/web-auth/blob/master/src/webauthn.js
//
// easy way to go from string to ByteArray
const enc = new TextEncoder();
function strToBytes(str) {
    return enc.encode(str);
}

// another function to go from string to ByteArray, but we first encode the
// string as base64 - note the use of the atob() function
function strToBin(str) {
  return Uint8Array.from(atob(str), c => c.charCodeAt(0));
}

// function to encode raw binary to string, which is subsequently
// encoded to base64 - note the use of the btoa() function
function binToStr(bin) {
  return btoa(new Uint8Array(bin).reduce(
      (s, byte) => s + String.fromCharCode(byte), ''));
}
////////

// this issue get the attributes of the entire response for stringify
// https://long2know.com/2021/05/typescript-inheritance-and-json-stringify/
function toJSON() {
    const jsonObj = {};
    let obj = this;
    do {
        // Walk the proto chain to set all properties
        Object.getOwnPropertyNames(obj)
            .filter(
                key =>
                    key[0] !== '_' && key !== 'constructor' && key !== 'toJSON')
            .map(key => {
            let desc: PropertyDescriptor =
                Object.getOwnPropertyDescriptor(obj, key);
            if (desc) {
                // jsonObj[key] = typeof desc.get === 'function' ? desc.get() :
                // this[key];
                try {
                const v = this[key];
                if (!(typeof v == 'function')) {
                    jsonObj[key] = this[key];
                }
                } catch (error) {
                console.error(`Error calling getter ${key}`, error);
                }
            }
            });
    } while (obj = Object.getPrototypeOf(obj));
    return jsonObj;
}

function arg_for_manifest(obj) {
  return JSON.stringify(obj);
}

function cred_to_json(cred: PublicKeyCredential): string {
    const response_for_JSON = toJSON.call(cred.response);
    const obj = {'id': cred.id, 'response': response_for_JSON};
    ArrayBuffer.prototype['toJSON'] = function(this) {
        return binToStr(this)
    };
    return JSON.stringify(obj)
}