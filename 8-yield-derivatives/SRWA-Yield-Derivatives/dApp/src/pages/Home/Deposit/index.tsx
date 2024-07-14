import { ChangeEvent, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

import { AssetsItemModel } from '../../../app/shared/models/index.model';
import featuresService from '../../../features';
import { COMPONENT_ADDRESS, formattedNumber, roundDownNumber } from '../../../helpers';
import { useSendTransaction } from '../../../rdt/hooks/useSendTransaction';

type DepositProps = {
  onActionChange: (action: string) => void;
  userBadge: string | undefined;
  accountAddress: string;
  accountAssets: AssetsItemModel[];
};
type FormData = {
  amount: string;
  asset: string;
};

function Deposit({ onActionChange, userBadge, accountAddress, accountAssets }: DepositProps) {
  const sendTransaction = useSendTransaction();
  const [selectedAsset, setSelectedAsset] = useState<string>(accountAssets[0].address);

  const {
    register,
    reset,
    formState: { errors },
    handleSubmit,
    setValue,
  } = useForm<FormData>();
  const [isLoading, setIsLoading] = useState(false);

  const handleActionChange = (action: string) => {
    onActionChange(action);
  };
  const onSubmit: SubmitHandler<FormData> = async (data) => {
    setIsLoading(true);

    if (!userBadge) {
      const response = await featuresService.depositAndCreateUser(
        sendTransaction,
        Number(data.amount),
        accountAddress,
        data.asset,
        COMPONENT_ADDRESS,
      );

      if (response) {
        await featuresService.fetchUserBadge(accountAddress);
        await onSuccessSubmit();
      }
      setIsLoading(false);
    } else {
      const response = await featuresService.depositAsset(
        sendTransaction,
        Number(data.amount),
        userBadge,
        accountAddress,
        data.asset,
        COMPONENT_ADDRESS,
      );

      if (response) {
        await onSuccessSubmit();
      }

      setIsLoading(false);
    }
  };
  const onSuccessSubmit = async () => {
    reset();
    await featuresService.fetchAssets(COMPONENT_ADDRESS, 'component');
    await featuresService.fetchAssets(accountAddress, 'account');
    await featuresService.fetchComponentAssets(COMPONENT_ADDRESS);
  };
  const handleAssetChange = (event: ChangeEvent<HTMLSelectElement>) => {
    setSelectedAsset(event.target.value);
    setValue('amount', '');
  };
  const handleSetMaxAmount = () => {
    setValue('amount', handleMaxAmount().toString());
  };
  const handleMaxAmount = () => {
    return Number(activeAccountAsset?.amount ?? 0);
  };

  const activeAccountAsset = accountAssets.find((asset) => asset.address === selectedAsset);

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 mt-10" noValidate>
      <h3 className="text-center text-xl font-bold leading-9 tracking-tight text-gray-900">DEPOSIT</h3>

      <div className="text-left">
        <label htmlFor="asset" className="block text-sm font-medium leading-6 text-gray-900">
          Asset
        </label>
        <div className="mt-1">
          <select
            {...register('asset', {
              required: 'This field is required',
            })}
            onChange={handleAssetChange}
            id="asset"
            className="block w-full rounded-md border-0 p-1.5 text-gray-900 ring-1 ring-gray-300 placeholder:text-gray-400 sm:text-sm sm:leading-6"
          >
            {accountAssets.map((item) => (
              <option key={item.address} value={item.address}>
                {item.as_string}
              </option>
            ))}
          </select>
          <small className="mt-2 block">
            Wallet Balance: {formattedNumber(roundDownNumber(activeAccountAsset?.amount ?? 0), 2)}{' '}
            {activeAccountAsset?.as_string}
          </small>
          {errors.asset && (
            <p role="alert" className="text-red-600 text-xs">
              {errors.asset.message}
            </p>
          )}
        </div>
      </div>
      <div className="text-left">
        <label htmlFor="amount" className="block text-sm font-medium leading-6 text-gray-900">
          Amount
        </label>
        <div className="mt-1 flex gap-x-2">
          <input
            {...register('amount', {
              required: 'This field is required',
              validate: {
                notGreaterThanMax: (value) =>
                  Number(value) <= handleMaxAmount() || `Value cannot be greater than ${handleMaxAmount()}`,
              },
            })}
            id="amount"
            type="text"
            className="w-10/12 rounded-md border-0 p-1.5 text-gray-900 ring-1 ring-gray-300 placeholder:text-gray-400 sm:text-sm sm:leading-6"
          />
          <button
            onClick={handleSetMaxAmount}
            type="button"
            className="flex w-2/12 justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
            MAX
          </button>
        </div>
        {errors.amount && (
          <p role="alert" className="text-red-600 text-xs">
            {errors.amount.message}
          </p>
        )}
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
          STAKE
        </button>
      </div>
    </form>
  );
}

export default Deposit;
