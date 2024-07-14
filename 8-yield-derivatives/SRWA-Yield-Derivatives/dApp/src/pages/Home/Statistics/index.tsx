import { AssetsItemModel } from '../../../app/shared/models/index.model';
import { TOTAL_YT_SUPPLY, calculateDaysLeft, formattedNumber } from '../../../helpers';

type StatisticsProps = {
  onActionChange: (action: string) => void;
  accountAssets: AssetsItemModel[];
  yieldAssets: AssetsItemModel[];
  componentAssets: AssetsItemModel[];
  userBadge: string;
};

function Statistics({ onActionChange, accountAssets, yieldAssets, componentAssets, userBadge }: StatisticsProps) {
  const handleActionChange = (action: string) => {
    onActionChange(action);
  };

  return (
    <div>
      <div className="mt-10 flex flex-col justify-center gap-x-10 text-lg text-gray-600 font-medium border-b border-slate-300 pb-4">
        <p className="block">REWARDS</p>
        <table className="table-auto border-b mb-2">
          <thead>
            <tr>
              <th>ASSET</th>
              <th>TOTAL</th>
              <th>REALIZED</th>
              <th>PENDING</th>
              <th>AVAILABLE</th>
            </tr>
          </thead>
          <tbody>
            {yieldAssets.map((asset: AssetsItemModel) => (
              <tr key={asset.address}>
                <td>{asset.as_string}</td>
                <td>{formattedNumber(TOTAL_YT_SUPPLY, 2)}</td>
                <td>{formattedNumber(TOTAL_YT_SUPPLY - (asset.amount ?? 0), 2)}</td>
                <td>{formattedNumber(asset.total_balances, 2)}</td>
                <td>{formattedNumber((asset.amount ?? 0) - (asset.total_balances ?? 0), 2)}</td>
              </tr>
            ))}
          </tbody>
        </table>

        <p className="block">DEPOSITS</p>
        <table className="table-auto">
          <thead>
            <tr>
              <th>ASSET</th>
              <th>TOTAL</th>
              <th>AVAILABLE</th>
            </tr>
          </thead>
          <tbody>
            {componentAssets.map((item: AssetsItemModel) => (
              <tr key={item.address}>
                <td>{item.as_string}</td>
                <td>{formattedNumber(item.total_balances, 2)}</td>
                <td>{formattedNumber(TOTAL_YT_SUPPLY / item.yield_rates - item.total_balances || 0, 2)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div
        className={`mt-4 flex flex-col justify-center gap-x-10 text-lg text-gray-600 font-medium pb-4 ${accountAssets.length > 1 ? 'border-b border-slate-300' : ''}`}
      >
        {userBadge ? (
          <table className="table-auto">
            <thead>
              <tr>
                <th>ASSET</th>
                <th>MY DEPOSIT</th>
                <th>MY YIELD</th>
                <th>UNLOCKING IN</th>
              </tr>
            </thead>
            <tbody>
              {accountAssets
                .filter(
                  (item: AssetsItemModel) =>
                    'deposited_at' in item && 'principal_balance' in item && 'yield_balance' in item,
                )
                .map((item: AssetsItemModel) => (
                  <tr key={item.address}>
                    <td>{item.as_string}</td>
                    <td>{formattedNumber(item.principal_balance, 2)}</td>
                    <td>{formattedNumber(item.yield_balance, 2)} YT</td>
                    <td>{calculateDaysLeft(item.deposited_at, 30)} days</td>
                  </tr>
                ))}
            </tbody>
          </table>
        ) : null}

        {accountAssets.length > 0 && (
          <div className="flex justify-center gap-x-10 pt-4">
            <button
              onClick={() => handleActionChange('Stake')}
              type="button"
              className="flex w-32 justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            >
              STAKE
            </button>
            {userBadge ? (
              <button
                onClick={() => handleActionChange('Unstake')}
                type="button"
                className="flex w-32 justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                UNSTAKE
              </button>
            ) : null}
          </div>
        )}
      </div>
    </div>
  );
}

export default Statistics;
