import { useState, useEffect } from 'react';
// Import Radix Wallet and Gateway SDKs
import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import {
  StateApi,
  TransactionApi,
  // StatusApi,
} from '@radixdlt/alphanet-gateway-api-v0-sdk';

const Withdraw = () => {
  const [account, setAccount] = useState(
    'account_tdx_a_1qd9eafyqjh750uv7scsy474xdceh2x2cjqdccus5k0ls06kddh'
  );
  const [component, setComponent] = useState(
    'component_tdx_a_1qgq6augflx3els05k97ccslfyjxhtgkawtjt23s0lasskjxtyp'
  );
  const [founders_badge, setFounders_badge] = useState(
    'resource_tdx_a_1qq3nnfj68j04urt997paq6hc7mf6zkmjyqu29nmua9aqxq5d23'
  );

  // Initialize the SDK
  const sdk = Sdk();
  const transactionApi = new TransactionApi();
  const stateApi = new StateApi();
  // const statusApi = new StatusApi();

  useEffect(() => {
    const getAddress = async () => {
      const result = await sdk.request({
        accountAddresses: {},
      });
      console.log('accountAddresses: ', result.value);
      const { accountAddresses } = result.value;
      setAccount(accountAddresses[0].address);
      // get corresponding founders badge
    };
    getAddress();
    return () => {};
  }, [sdk]);

  const withdrawFoundersFunds = async () => {
    let manifest = new ManifestBuilder()
      .callMethod(account, 'lock_fee', ['Decimal("100")'])
      .createProofFromAccount(
        account,
        'resource_tdx_a_1qq3nnfj68j04urt997paq6hc7mf6zkmjyqu29nmua9aqxq5d23'
      )
      .callMethod(account, 'deposit_batch', ['Expression("ENTIRE_WORKTOP")'])
      .build()
      .toString();
    console.log('withdraw manifest: ', manifest);

    const hash = await sdk
      .sendTransaction(manifest)
      .map((response) => response.transactionHash);

    if (hash.isErr()) throw hash.error;
    console.log('hash: ', hash);
    // Fetch the receipt from the Gateway SDK
    const receipt = await transactionApi.transactionReceiptPost({
      v0CommittedTransactionRequest: { intent_hash: hash.value },
    });
    console.log('receipt: ', receipt);
  };

  return (
    <div className="border-2">
      <h2>Withdraw Funds</h2>
      <p>NOTE: This is still Work In Progress</p>
      <button
        className="mt-2 mr-5 bg-green-700 hover:bg-green-500"
        onClick={withdrawFoundersFunds}
      >
        Withdraw Token Sale Funds
      </button>
    </div>
  );
};

export default Withdraw;
