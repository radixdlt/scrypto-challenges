import { useState, useEffect } from 'react';
import Sdk, { ManifestBuilder } from '@radixdlt/alphanet-walletextension-sdk';
import {
  // StateApi,
  TransactionApi,
  // StatusApi,
} from '@radixdlt/alphanet-gateway-api-v0-sdk';
const BuyTokens = (props) => {
  const [account, setAccount] = useState(
    'account_tdx_a_1qd9eafyqjh750uv7scsy474xdceh2x2cjqdccus5k0ls06kddh'
  );
  // TODO handle passing Component props
  const [component, setComponent] = useState(
    'component_tdx_a_1qgq6augflx3els05k97ccslfyjxhtgkawtjt23s0lasskjxtyp'
  );
  // Initialize the SDK
  const sdk = Sdk();
  const transactionApi = new TransactionApi();
  useEffect(() => {
    const getAddress = async () => {
      const result = await sdk.request({
        accountAddresses: {},
      });
      console.log('buy tokens accountAddresses: ', result.value);
      const { accountAddresses } = result.value;
      setAccount(accountAddresses[0].address);
    };
    getAddress();
    return () => {};
  }, [sdk]);

  const buyMemberToken = async () => {
    let manifest = new ManifestBuilder()
      .callMethod(account, 'lock_fee', ['Decimal("100")'])
      .withdrawFromAccountByAmount(
        account,
        33,
        'resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9'
      )
      .takeFromWorktopByAmount(
        33,
        'resource_tdx_a_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqegh4k9',
        'xrd_bucket'
      )
      .callMethod(component, 'buy_member_tokens', ['Bucket("xrd_bucket")'])
      .callMethod(account, 'deposit_batch', ['Expression("ENTIRE_WORKTOP")'])
      .build()
      .toString();

    // Send manifest to extension for signing
    const hash = await sdk
      .sendTransaction(manifest)
      .map((response) => response.transactionHash);

    if (hash.isErr()) throw hash.error;

    // Fetch the receipt from the Gateway SDK
    const receipt = await transactionApi.transactionReceiptPost({
      v0CommittedTransactionRequest: { intent_hash: hash.value },
    });
    console.log('token receipt: ', receipt);
  };
  return (
    <div>
      <h2>Buy Memeber Tokens</h2>
      <button
        className="mt-2 mr-5 bg-green-700 hover:bg-green-500"
        onClick={buyMemberToken}
      >
        Buy Member Tokens
      </button>
    </div>
  );
};

export default BuyTokens;
