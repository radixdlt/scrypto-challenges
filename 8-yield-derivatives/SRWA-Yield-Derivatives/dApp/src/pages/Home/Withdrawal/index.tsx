import { useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

import { AssetsItemModel } from '../../../app/shared/models/index.model';
import featuresService from '../../../features';
import { COMPONENT_ADDRESS } from '../../../helpers';
import { useSendTransaction } from '../../../rdt/hooks/useSendTransaction';

type DepositProps = {
  onActionChange: (action: string) => void;
  userBadge: string | undefined;
  accountAddress: string;
  accountAssets: AssetsItemModel[];
};
type FormData = {
  asset: string;
};

function Withdrawal({ onActionChange, userBadge, accountAddress, accountAssets }: DepositProps) {
  const sendTransaction = useSendTransaction();

  const {
    register,
    reset,
    formState: { errors },
    handleSubmit,
  } = useForm<FormData>();
  const [isLoading, setIsLoading] = useState(false);

  const handleActionChange = (action: string) => {
    onActionChange(action);
  };
  const onSubmit: SubmitHandler<FormData> = async (data) => {
    setIsLoading(true);

    const response = await featuresService.withdrawAsset(
      sendTransaction,
      userBadge,
      accountAddress,
      data.asset,
      COMPONENT_ADDRESS,
    );

    if (response) {
      reset();
      await featuresService.fetchAssets(COMPONENT_ADDRESS, 'component');
      await featuresService.fetchAssets(accountAddress, 'account');
      await featuresService.fetchComponentAssets(COMPONENT_ADDRESS);
    }

    setIsLoading(false);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 mt-10" noValidate>
      <h3 className="text-center text-xl font-bold leading-9 tracking-tight text-gray-900">WITHDRAWAL</h3>

      <p className="block text-md font-medium leading-6 text-gray-900">YT IS UNLOCKED</p>
      <p className="block text-md font-medium leading-6 text-gray-900">
        YT IS LOCKED(UNLOCKING PERIOD HAS NOT EXPIRED)
      </p>
      <div className="text-left">
        <label htmlFor="asset" className="block text-sm font-medium leading-6 text-gray-900">
          Asset
        </label>
        <div className="mt-2">
          <select
            {...register('asset', {
              required: 'This field is required',
            })}
            id="asset"
            className="block w-full rounded-md border-0 p-1.5 text-gray-900 ring-1 ring-gray-300 placeholder:text-gray-400 sm:text-sm sm:leading-6"
          >
            {accountAssets.map((item) => (
              <option key={item.address} value={item.address}>
                {item.as_string}
              </option>
            ))}
          </select>
          {errors.asset && (
            <p role="alert" className="text-red-600 text-xs">
              {errors.asset.message}
            </p>
          )}
        </div>
      </div>
      <div className="flex gap-x-10">
        <button
          onClick={() => handleActionChange('')}
          type="button"
          className="flex w-1/2 justify-center rounded-md bg-rose-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-rose-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-600"
        >
          CANCEL
        </button>
        <button
          type="submit"
          className={`flex w-1/2 justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 relative ${
            isLoading ? 'btn-is-loading' : ''
          }`}
        >
          UNSTAKE
        </button>
      </div>
    </form>
  );
}

export default Withdrawal;
