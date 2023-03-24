import {
  RadixDappToolkit,
  ManifestBuilder,
  Decimal,
  Bucket,
  Bool,
  Expression,
  String,
  ResourceAddress
} from '@radixdlt/radix-dapp-toolkit'
const dAppId = 'account_tdx_22_1prd6gfrqj0avlyxwldgyza09fp7gn4vjmga7clhe9p2qv0qt58'

const rdt = RadixDappToolkit(
  { dAppDefinitionAddress: dAppId, dAppName: 'GumballMachine' },
  (requestData) => {
    requestData({
      accounts: { quantifier: 'atLeast', quantity: 1 },
    }).map(({ data: { accounts } }) => {
      // add accounts to dApp application state
      console.log("account data: ", accounts)
      document.getElementById('accountAddress').innerText = accounts[0].address
      accountAddress = accounts[0].address
    })
  },
  { 
    networkId: 11,
    onDisconnect: () => {

    }
  }
)
console.log("dApp Toolkit: ", rdt)

// There are four classes exported in the Gateway-SDK These serve as a thin wrapper around the gateway API
// API docs are available @ https://betanet-gateway.redoc.ly/
import { TransactionApi, StateApi, StatusApi, StreamApi } from "@radixdlt/babylon-gateway-api-sdk";

// Instantiate Gateway SDK
const transactionApi = new TransactionApi();
const stateApi = new StateApi();
const statusApi = new StatusApi();
const streamApi = new StreamApi();

// Global states
let accountAddress //: string // User account address
let componentAddress //: string  // GumballMachine component address
let resourceAddress //: string // GUM resource address
// You can use this packageAddress to skip the dashboard publishing step package_tdx_b_1qxtzcuyh8jmcp9khn72k0gs4fp8gjqaz9a8jsmcwmh9qhax345
const xrdAddress = "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp"
const cupPkg = 'package_tdx_b_1q8sv4qf9vsflfendt693ulx26xrde0jul5rk30zrsngsca52rc'
const cupComp = 'component_tdx_b_1qffkjlsgsckay2scsc0tyh88c9k7ag83dh9xvdzac93snzxdgf'
const longResource = "resource_tdx_b_1qqpwwzswde5mfy0jehmdvt9rdk762r5tm4en9n62kg7qz47tx3"
const shortResource = "resource_tdx_b_1qqe4vkanfy8upr7lk9qufrf8eqtyqfj245tpk8rnjagqqap0jr"

const prec = 18;
const precmul = 10**18;

let comp_state = new Array(18);
let comp_long_val = undefined
let comp_short_val = undefined

let acc_resources = undefined
let long_amount = 0
let short_amount = 0

let exchange_rate = 0

function val(b, x) {
  if (b) {
    return x / parseFloat(comp_state[7]) * comp_long_val
  } else {
    return x / parseFloat(comp_state[8]) * comp_short_val
  }
}

async function update() {
  comp_state = 
    (await stateApi
      .entityDetails(
        { entityDetailsRequest: { address: cupComp }}))
    .details
    .state
    .data_json
  
  console.log(comp_state)

  comp_long_val = parseFloat(comp_state[4])

  comp_short_val = parseFloat(comp_state[6])

  if (accountAddress) {
    acc_resources = (await stateApi
      // .entityDetails(
      //   { entityDetailsRequest: { address: accountAddress }}))
      .entityResources(
        { entityResourcesRequest: { address: accountAddress }}))
  }
  document.getElementById('exch-feed').textContent = 
    "RAND/USD\n" + exchange_rate.toString();

  document.getElementById('bucket-feed').textContent = 
    comp_long_val.toFixed(2) + " XRD Long\n" + comp_short_val.toFixed(2) + " XRD Short" 

  // console.log("ACC ACC ACC")
  // console.log(accountAddress)
  if (accountAddress) {

    let temp = acc_resources.fungible_resources.items
      .find(x => x.address == longResource)
    long_amount = temp ? parseFloat(temp.amount.value) : 0

    temp = acc_resources.fungible_resources.items
      .find(x => x.address == shortResource)
    short_amount = temp ? parseFloat(temp.amount.value) : 0

  }

  if (long_amount && short_amount) {
    document.getElementById('exposure').textContent = 
      "Long LP: " + long_amount.toFixed(2)
      + "\nShort LP: " + short_amount.toFixed(2)
  } else if (long_amount) {
    document.getElementById('exposure').textContent = 
      "Long LP: " + long_amount.toFixed(2)
  } else if (short_amount) {
    document.getElementById('exposure').textContent = 
      "Short LP: " + short_amount.toFixed(2)
  } else {
    document.getElementById('exposure').textContent = 
      "No Exposure"
  }

  if (accountAddress) {
    document.getElementById('value').textContent = 
      "Long " + val(true, long_amount).toFixed(2) + " XRD \nShort " +
      val(false, short_amount).toFixed(2) + " XRD"
  } else {
    document.getElementById('value').textContent = 
      "0"
  }

}


await update()
console.log(comp_state)
exchange_rate = parseInt(comp_state[12]) // integer only exch 

// ************ Instantiate component and fetch component and resource addresses *************
// document.getElementById('instantiateComponent').onclick = async function () {
//   await update()
//   // let packageAddress = document.getElementById("packageAddress").value;
//   let exch = document.getElementById("exch").value;

//   let manifest = new ManifestBuilder()
//     // .callMethod(accountAddress, "create_proof", [ResourceAddress("resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")])
//     // .callMethod(accountAddress, "lock_fee", [Decimal(10)])
//     .withdrawFromAccountByAmount(
//       accountAddress, 
//       100, 
//       xrdAddress)
//     .takeFromWorktop(
//       xrdAddress,
//       "buck")
//     .callFunction(
//       cupPkg, 
//       "CupPerp", 
//       "instantiate_pair", 
//       [String("RAND/USD"), Decimal(exch), Bucket("buck")])
//     .callMethod(
//       accountAddress, 
//       "deposit_batch", 
//       [Expression("ENTIRE_WORKTOP")])
//     .build()
//     .toString();

//   console.log("Instantiate Manifest: ", manifest)
//   // Send manifest to extension for signing
//   const result = await rdt
//     .sendTransaction({
//       transactionManifest: manifest,
//       version: 1,
//     })

//   if (result.isErr()) throw result.error

//   console.log("Intantiate WalletSDK Result: ", result.value)

//   // ************ Fetch the transaction status from the Gateway API ************
//   let status = await transactionApi.transactionStatus({
//     transactionStatusRequest: {
//       intent_hash_hex: result.value.transactionIntentHash
//     }
//   });
//   console.log('Instantiate TransactionApi transaction/status:', status)

//   // ************* fetch component address from gateway api and set componentAddress variable **************
//   let commitReceipt = await transactionApi.transactionCommittedDetails({
//     transactionCommittedDetailsRequest: {
//       transaction_identifier: {
//         type: 'intent_hash',
//         value_hex: result.value.transactionIntentHash
//       }
//     }
//   })
//   console.log('Instantiate Committed Details Receipt', commitReceipt)

//   // ****** set componentAddress and resourceAddress variables with gateway api commitReciept payload ******
//   // componentAddress = commitReceipt.details.receipt.state_updates.new_global_entities[0].global_address <- long way -- shorter way below ->
//   // componentAddress = commitReceipt.details.referenced_global_entities[0]
//   // document.getElementById('componentAddress').innerText = componentAddress;

//   // resourceAddress = commitReceipt.details.referenced_global_entities[1]
//   // document.getElementById('gumAddress').innerText = resourceAddress;
// }

document.getElementById('exch-feed').textContent = 
  "RAND/USD\n" + exchange_rate

document.getElementById('target-swap-xrd').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
    (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time
  
  let target = document.getElementById('target-exposure-xrd').value || 10

  let long_xrd = val(true, long_amount)
  let short_xrd = val(false, short_amount)
  
  let wdLong = 0
  let wdShort = 0
  let dpLong = 0
  let dpShort = 0
  console.log(target)
  if (target > long_xrd) {
    dpLong = target - long_xrd
    wdShort = short_xrd
  } else if (target == long_xrd) {
    wdShort = short_xrd
  } else if (long_xrd > target && target > 0) {
    wdLong = long_xrd - target
    wdShort = short_xrd
  } else if (target == 0) {
    wdLong = long_xrd
    wdShort = short_xrd
  } else if (target < 0 && target > -short_xrd) {
    wdLong = long_xrd
    wdShort = short_xrd + target
  } else if (target == -short_xrd) {
    wdLong = long_xrd
  } else {
    wdLong = long_xrd
    dpShort = -short_xrd - target 
  }

  let manifest = new ManifestBuilder()

  if (dpLong) {
    manifest = manifest
      .withdrawFromAccountByAmount(
        accountAddress,
        dpLong,
        xrdAddress)
      .takeFromWorktop(
        xrdAddress,
        "buck1")
      .callMethod(
        cupComp,
        "deposit",
        [Bool(true), Bucket("buck1")])
  }
  if (dpShort) {
    manifest = manifest
      .withdrawFromAccountByAmount(
        accountAddress,
        dpShort,
        xrdAddress)
      .takeFromWorktop(
        xrdAddress,
        "buck2")
      .callMethod(
        cupComp,
        "deposit",
        [Bool(false), Bucket("buck2")])
  }
  if (wdLong) {
    manifest = manifest 
      .withdrawFromAccountByAmount(
        accountAddress,
        wdLong,
        longResource)
      .takeFromWorktop(
        longResource,
        "buck3")
      .callMethod(
        cupComp,
        "withdraw",
        [Bool(true), Bucket("buck3")])
  }
  if (wdShort) {
    manifest = manifest 
    .withdrawFromAccountByAmount(
      accountAddress,
      wdShort,
      shortResource)
    .takeFromWorktop(
      shortResource,
      "buck4")
    .callMethod(
      cupComp,
      "withdraw",
      [Bool(false), Bucket("buck4")])
  }
  manifest = manifest
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
    console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

document.getElementById('target-swap-long').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
    (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time

  let move = document.getElementById('target-exposure-long').value

  let manifest 
  if (move >= 0) {
    manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      move, 
      xrdAddress)
    .takeFromWorktop(
      xrdAddress,
      "buck")
    .callMethod(
      cupComp,
      "deposit",
      [Bool(true), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  } else {
    let amount = parseFloat(comp_state[7]) / comp_long_val * -move

    manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      amount, 
      longResource)
    .takeFromWorktop(
      longResource,
      "buck")
    .callMethod(
      cupComp,
      "withdraw",
      [Bool(true), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  }

  console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

document.getElementById('target-swap-short').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
  (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time

  let move = document.getElementById('target-exposure-short').value

  let manifest 
  if (move >= 0) {
    manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      move, 
      xrdAddress)
    .takeFromWorktop(
      xrdAddress,
      "buck")
    .callMethod(
      cupComp,
      "deposit",
      [Bool(false), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  } else {
    let amount = parseFloat(comp_state[8]) / comp_short_val * -move

    manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      amount, 
      shortResource)
    .takeFromWorktop(
      shortResource,
      "buck")
    .callMethod(
      cupComp,
      "withdraw",
      [Bool(false), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  }

  console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

document.getElementById('long-deposit').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
    (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time

  let manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      10, 
      xrdAddress)
    .takeFromWorktop(
      xrdAddress,
      "buck")
    .callMethod(
      cupComp,
      "deposit",
      [Bool(true), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();

  console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

document.getElementById('short-deposit').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
    (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time

  let manifest = new ManifestBuilder()
    .withdrawFromAccountByAmount(
      accountAddress, 
      10, 
      xrdAddress)
    .takeFromWorktopByAmount(
      10,
      xrdAddress,
      "buck")
    .callMethod(
      cupComp,
      "deposit",
      [Bool(false), Bucket("buck")]
      )
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)]
      )
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  
  console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

document.getElementById('pass-time').onclick = async function () {
  await update()
  if (!accountAddress) {
    alert("connect first")
    return
  }

  let mov = Math.floor(Math.random() * 10) * 
    (Math.random() <= 0.45 ? -1 : 1 ); // stocks go up most of the time

  let manifest = new ManifestBuilder()
    .callMethod(
      cupComp,
      "set_oracle",
      [Decimal(exchange_rate+mov)])
    .callMethod(
      accountAddress, 
      "deposit_batch", 
      [Expression("ENTIRE_WORKTOP")])
    .build()
    .toString();
  
  console.log("Instantiate Manifest: ", manifest)

  const result = await rdt
    .sendTransaction({
      transactionManifest: manifest,
      version: 1,
    })

  if (result.isErr()) throw result.error

  console.log("Intantiate WalletSDK Result: ", result.value)

  // ************ Fetch the transaction status from the Gateway API ************
  let status = await transactionApi.transactionStatus({
    transactionStatusRequest: {
      intent_hash_hex: result.value.transactionIntentHash
    }
  });
  console.log('Instantiate TransactionApi transaction/status:', status)

  exchange_rate += mov;
  await update()
}

  // let manifest = new ManifestBuilder()
  //   .withdrawFromAccountByAmount(accountAddress, 10, "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp")
  //   .takeFromWorktopByAmount(10, "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp", "xrd_bucket")
  //   .callMethod(componentAddress, "buy_gumball", [Bucket("xrd_bucket")])
  //   .callMethod(accountAddress, "deposit_batch", [Expression("ENTIRE_WORKTOP")])
  //   .build()
  //   .toString();

  // console.log('buy_gumball manifest: ', manifest)

  // // Send manifest to extension for signing
  // const result = await rdt
  //   .sendTransaction({
  //     transactionManifest: manifest,
  //     version: 1,
  //   })

  // if (result.isErr()) throw result.error

  // console.log("Buy Gumball getMethods Result: ", result)

  // // Fetch the transaction status from the Gateway SDK
  // let status = await transactionApi.transactionStatus({
  //   transactionStatusRequest: {
  //     intent_hash_hex: result.value.transactionIntentHash
  //   }
  // });
  // console.log('Buy Gumball TransactionAPI transaction/status: ', status)

  // // fetch commit reciept from gateway api 
  // let commitReceipt = await transactionApi.transactionCommittedDetails({
  //   transactionCommittedDetailsRequest: {
  //     transaction_identifier: {
  //       type: 'intent_hash',
  //       value_hex: result.value.transactionIntentHash
  //     }
  //   }
  // })
  // console.log('Buy Gumball Committed Details Receipt', commitReceipt)

  // // Show the receipt on the DOM
  // document.getElementById('receipt').innerText = JSON.stringify(commitReceipt.details.receipt, null, 2);
// };