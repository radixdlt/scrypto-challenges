import { useEffect, useState } from 'react';
import Sdk from '@radixdlt/alphanet-walletextension-sdk';

import BuyTokens from '../components/BuyTokens';

const Marketplace = () => {
  const [account, setAccount] = useState(
    'account_tdx_a_1q06j4qxaqmdg7qm2vq04a9smz4nnx6x8we7xwm5fvueqd9pz2n'
  );

  const sdk = Sdk();

  useEffect(() => {
    const getAddress = async () => {
      const result = await sdk.request({
        accountAddresses: {},
      });
      console.log('accountAddresses: ', result.value);
      const { accountAddresses } = result.value;
      setAccount(accountAddresses[0].address);
    };
    getAddress();
    return () => {};
  }, [sdk]);

  return (
    <div>
      <h2 className="text-2xl font-bold mb-2">
        Welcome To The DAO Marketplace
      </h2>
      <p>
        Find new projects to support, contribute to, sell or trade member tokens
        with other community members.
      </p>
      <BuyTokens account={account} />
    </div>
  );
};

export default Marketplace;
