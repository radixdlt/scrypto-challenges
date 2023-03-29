import React, { useContext, createContext } from 'react';

import {
  RadixDappToolkit,
  ManifestBuilder,
  Decimal,
  Bucket,
  Expression,
  ResourceAddress,
  
} from '@radixdlt/radix-dapp-toolkit';

// There are four classes exported in the Gateway-SDK These serve as a thin wrapper around the gateway API
// API docs are available @ https://betanet-gateway.redoc.ly/
import { TransactionApi, StateApi, StatusApi, StreamApi } from '@radixdlt/babylon-gateway-api-sdk';

const StateContext = createContext();
export const StateContextProvider = ({ children }) => {
  const [accountAddress, setAccountAddress] = useState();
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

  const rdt = RadixDappToolkit(
    { dAppDefinitionAddress: dAppId, dAppName: 'AlkyneFi' },
    (requestData) => {
      requestData({
        accounts: { quantifier: 'atLeast', quantity: 1 },
      }).map(({ data: { accounts } }) => {
        // add accounts to dApp application state
        console.log('account data: ', accounts);
        setAccountName(accounts[0].label);
        setAccountAddress(accounts[0].address);
      });
    },
    { networkId: 11 }
  );

  console.log('dApp Toolkit: ', rdt);
  
  const instantiate = async () => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress, 1000, xrdAddress)
      .takeFromWorktopByAmount(1000, xrdAddress, 'xrd_bucket')
      .callMethod(accountAddress, 'create_proof', [ResourceAddress(xrdAddress)])
      .callFunction(alkyneFi_package, '', 'instantiate_tradex', [Bucket('xrd_bucket'), "", Decimal('1'),  xrdAddress, {}])
      .build()
      .toString();
    console.log('Instantiate Manifest: ', manifest);
    
    // Submit transaction
    let commitReceipt = await submitTransaction(manifest);

    // componentAddress = commitReceipt.details.referenced_global_entities[0];
    // resourceAddress = commitReceipt.details.referenced_global_entities[1];
  };

  const create_and_fund_wallet = async (amount) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
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

  const fund_existing_wallet = async (amount) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .callMethod(accountAddress, 'lock_fee', ['Decimal("20")'])
      .withdrawFromAccountByAmount(accountAddress, Number(amount), xrdAddress)
      .takeFromWorktopByAmount(Number(amount), xrdAddress, 'xrd_bucket')
      .callMethod(componentAddress, 'create_and_fund_wallet', [Bucket('xrd_bucket')])
      .build()
      .toString();

    // Submit transaction
    let commitReceipt = await submitTransaction(manifest);

    console.log(JSON.stringify(commitReceipt.details.receipt, null, 2));
  };

  const trade = async (pool_address, amount, ) => {
    // ************ Create the manifest for the transaction ************
    let manifest = new ManifestBuilder()
      .callMethod(componentAddress, 'trade', [Decimal('1')])
      .build()
      .toString();

    console.log('trade manifest: ', manifest);

    // fetch commit reciept from gateway api
    let commitReceipt = await submitTransaction(manifest);

    // Show the receipt on the DOM
    console.log(JSON.stringify(commitReceipt.details.receipt, null, 2));
  };

  const withdraw_payment = async () => {

  };

  async function submitTransaction(manifest) {
    console.log(manifest);
    const result = await rdt
      .sendTransaction({
        transactionManifest: manifest,
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
    <StateContext.Provider
      value={{
        address,
        instantiate,
        create_and_fund_wallet,
        fund_existing_wallet,
        trade,
        withdraw_payment,
      }}
    >
      {children}
    </StateContext.Provider>
  );
};

export const useStateContext = () => useContext(StateContext);
