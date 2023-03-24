import React, { useState, useEffect } from 'react';

import {
  RadixDappToolkit,
  ManifestBuilder,
  Decimal,
  Bucket,
  Expression,
  ResourceAddress,
  Array,
  Tuple,
  ComponentAddress,
} from '@radixdlt/radix-dapp-toolkit';

// There are four classes exported in the Gateway-SDK These serve as a thin wrapper around the gateway API
// API docs are available @ https://betanet-gateway.redoc.ly/
import { TransactionApi, StateApi, StatusApi, StreamApi } from '@radixdlt/babylon-gateway-api-sdk';

// const StateContext = createContext();
export default function test() {
  // const [accountAddress, setAccountAddress] = useState();
  const accountAddress = 'account_tdx_b_1pqmg9ewm4cmczvykk483k76y50l0d8f7urmec36kj8jq529jsn';
  const [componentAddress, setComponentAddress] = useState();
  setComponentAddress('component_tdx_b_1qxnw7xu6fvfxy280uceq9shqe3a4t8jn3khxqjd4tm2smk8xa0');
  const [balance, setBalance] = useState();
  const [accountName, setAccountName] = useState();

  // Instantiate Gateway SDK
  const transactionApi = new TransactionApi();
  const stateApi = new StateApi();
  const statusApi = new StatusApi();
  const streamApi = new StreamApi();

  // Instantiate Radix Dapp Toolkit
  const xrdAddress = 'resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp';
  const alkyneFi_package = 'package_tdx_b_1qxnw7xu6fvfxy280uceq9shqe3a4t8jn3khxqjd4tm2smk8xa0';
  const dAppId = 'account_tdx_b_1ppglnkmukh36l2dfw3uygvgjf2jsfypl885u9840md7swrvpmj';

  let rdt = RadixDappToolkit;
  useEffect(() => {
    rdt = RadixDappToolkit(
      { dAppDefinitionAddress: dAppId, dAppName: 'AlkyneFi' },
      (requestData) => {
        requestData({
          accounts: { quantifier: 'atLeast', quantity: 1 },
        }).map(({ data: { accounts } }) => {
          // add accounts to dApp application state
          console.log('account data: ', accounts);
          setAccountName(accounts[0].label);
          // setAccountAddress(accounts[0].address);
        });
      },
      { networkId: 11 }
    );

    // const subscription = rdt?.state$.subscribe((state) => {
    //   console.log('state: ', state);
    //   // state?.accounts ?? [];
    //   setAccountAddress(state.accounts[0].address);
    //   console.log(state.accounts[0].address);
    //   console.log(accountAddress);
    // });

    console.log(rdt.requestData.accounts);
  }, []);

  console.log('dApp Toolkit: ', rdt);

  const instantiate = async () => {
    // ************ Create the manifest for the transaction ************
    console.log(accountAddress);
    let manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress, 1000, xrdAddress)
      .takeFromWorktopByAmount(1000, xrdAddress, 'xrd_bucket')
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(xrdAddress)])
      .callFunction(alkyneFi_package, '', 'instantiate_tradex', [
        Bucket('xrd_bucket'),
        '',
        Decimal('1'),
        xrdAddress,
        // Map<ResourceAddress(xrdAddress), Map<ResourceAddress(xrdAddress), ComponentAddress("")>>,
        Map < ResourceAddress,
        Map >
          (ResourceAddress(xrdAddress),
          Map < ResourceAddress,
          ComponentAddress > (ResourceAddress(xrdAddress), ComponentAddress(''))),
      ])
      .build()
      .toString();
    console.log('Instantiate Manifest: ', manifest);

    // Submit transaction
    let commitReceipt = await submitTransaction(manifest);

    console.log(commitReceipt);
    // componentAddress = commitReceipt.details.referenced_global_entities[0];
    // resourceAddress = commitReceipt.details.referenced_global_entities[1];
  };

  const create_and_fund_wallet = async (amount) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, 'lock_fee', ['Decimal("10")'])
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(xrdAddress)])
      .withdrawFromAccountByAmount(accountAddress, Number(amount), xrdAddress)
      .takeFromWorktopByAmount(Number(amount), xrdAddress, 'xrd_bucket')
      .callMethod(componentAddress, 'create_and_fund_wallet', [Bucket('xrd_bucket')])
      .callMethod(accountAddress, 'deposit_batch', [Expression('ENTIRE_WORKTOP')])
      .build()
      .toString();

    // Submit transaction
    let commitReceipt = await submitTransaction(manifest);

    // Show the receipt on the DOM
    console.log(JSON.stringify(commitReceipt.details.receipt, null, 2));
  };

  const fund_existing_wallet = async (amount, owner_badge) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, 'lock_fee', ['Decimal("10")'])
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(xrdAddress)])
      .withdrawFromAccountByAmount(accountAddress, Number(amount), xrdAddress)
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(owner_badge)])
      .popFromAuthZone('owner_badge')
      .takeFromWorktopByAmount(Number(amount), xrdAddress, 'xrd_bucket')
      .callMethod(componentAddress, 'create_and_fund_wallet', [Bucket('xrd_bucket'), Proof('owner_badge')])
      .build()
      .toString();

    // Submit transaction
    let commitReceipt = await submitTransaction(manifest);

    console.log(JSON.stringify(commitReceipt.details.receipt, null, 2));
  };

  const trade = async (pool_address, amount, resource_address, owner_badge) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, 'lock_fee', ['Decimal("10")'])
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(xrdAddress)])
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(owner_badge)])
      .popFromAuthZone('owner_badge')
      .callMethod(componentAddress, 'trade', [ComponentAddress(pool_address), Decimal(amount), ResourceAddress(resource_address), Proof('owner_badge')])
      .build()
      .toString();

    console.log('trade manifest: ', manifest);

    // fetch commit reciept from gateway api
    let commitReceipt = await submitTransaction(manifest);

    // Show the receipt on the DOM
    console.log(JSON.stringify(commitReceipt.details.receipt, null, 2));
  };

  const withdraw_payment = async () => {};

  async function submitTransaction(manifest) {
    console.log(manifest);

    const result = await rdt
      .sendTransaction({
        transactionManifest: String(manifest),
        version: 1,
      })
      .map((response) => response.transactionHash);

    if (result.isErr()) {
      throw result.error;
    }

    let status = await transactionApi.transactionStatus({
      transactionStatusRequest: {
        intent_hash_hex: result.value.transactionIntentHash,
      },
    });
    console.log('Transaction status: ', status);

    // fetch commit reciept from gateway api
    let commitReceipt = await transactionApi.transactionCommittedDetails({
      transactionCommittedDetailsRequest: {
        transaction_identifier: {
          type: 'intent_hash',
          value_hex: result.value.transactionIntentHash,
        },
      },
    });
    console.log('Commit receipt: ', commitReceipt);

    return commitReceipt;
  }

  return (
    <div className="flex flex-col justify-evenly">
      <radix-connect-button />
      <button onClick={() => instantiate()}>Instantiate</button>

      <button onClick={() => create_and_fund_wallet(1000)}>Create and Fund Wallet</button>

      <button onClick={() => fund_existing_wallet(1000)}>Fund Existing Wallet</button>

      <button onClick={() => trade()}>Trade</button>

      <button onClick={() => withdraw_payment()}>Withdraw Payment</button>
    </div>
  );
}

// export const useStateContext = () => useContext(StateContext);
