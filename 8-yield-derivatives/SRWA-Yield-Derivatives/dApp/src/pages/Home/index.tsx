import { useEffect, useState } from 'react';

import Deposit from './Deposit';
import Statistics from './Statistics';
import Withdrawal from './Withdrawal';
import featuresService from '../../features';
import { COMPONENT_ADDRESS } from '../../helpers';
import { useBearStore } from '../../store';

function Home() {
  const accountAddress = useBearStore((state) => state.accountAddress);
  const accountAssets = useBearStore((state) => state.accountAssets);
  const yieldAssets = useBearStore((state) => state.yieldAssets);
  const componentAssets = useBearStore((state) => state.componentAssets);
  const userBadge = useBearStore((state) => state.userBadge);

  const [activeAction, setActiveAction] = useState('');

  const handleActionChange = (action: string) => {
    setActiveAction(action);
  };

  useEffect(() => {
    (async () => {
      if (!accountAddress) return;
      await featuresService.fetchUserBadge(accountAddress);
      await featuresService.fetchAssets(COMPONENT_ADDRESS, 'component');
      await featuresService.fetchAssets(accountAddress, 'account');
      await featuresService.fetchComponentAssets(COMPONENT_ADDRESS);
    })();
  }, [accountAddress]);

  console.log('accountAssets: ', accountAssets);
  console.log('yieldAssets: ', yieldAssets);
  console.log('componentAssets: ', componentAssets);
  // console.log('userBadge: ', userBadge);

  return (
    <div className="relative isolate px-6 lg:px-8">
      <div
        className="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80"
        aria-hidden="true"
      >
        <div
          className="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
          style={{
            clipPath:
              'polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)',
          }}
        />
      </div>
      <div className="mx-auto max-w-lg py-32">
        <div className="text-center">
          <div className="border-b border-slate-300 pb-4">
            <h1 className="text-4xl font-bold tracking-tight text-gray-900">
              30 days
              <span className="block">FLEXIBLE TOKEN STAKING</span>
            </h1>
            <p className="mt-6 text-lg text-gray-900">
              STAKE YOUR XRD & ENJOY 20% REWARDS
              <span className="block">ON UNSTAKING AFTER 30 days expiry</span>
              <span>Withdraw YOUR XRD AT ANY TIME!</span>
            </p>
          </div>
          <Statistics
            onActionChange={handleActionChange}
            accountAssets={accountAssets}
            yieldAssets={yieldAssets}
            componentAssets={componentAssets}
            userBadge={userBadge}
          />
          {accountAssets.length > 0 && (
            <>
              {activeAction === 'Stake' && (
                <Deposit
                  onActionChange={handleActionChange}
                  userBadge={userBadge}
                  accountAddress={accountAddress}
                  accountAssets={accountAssets}
                />
              )}
              {activeAction === 'Unstake' && (
                <Withdrawal
                  onActionChange={handleActionChange}
                  userBadge={userBadge}
                  accountAddress={accountAddress}
                  accountAssets={accountAssets}
                />
              )}
            </>
          )}
        </div>
      </div>
      <div
        className="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]"
        aria-hidden="true"
      >
        <div
          className="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]"
          style={{
            clipPath:
              'polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)',
          }}
        />
      </div>
    </div>
  );
}

export default Home;
