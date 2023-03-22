import React from "react";
import { FaBalanceScale, FaInfoCircle } from "react-icons/fa";
import Layout from "../components/Layout";
import { currencyFormatter } from "../utils/priceFormatter";

interface Props {}

function Dashboard(props: Props) {
  const {} = props;

  return (
    <Layout>
      <main className="my-2">
        <div className="flex gap-2">
          <div className="flex flex-col w-2/3 h-60 p-6 item-start justify-center bg-gray-900 rounded-sm shadow-md">
            <p className="flex text-sm items-center gap-1 text-gray-400">
              Total Balance <FaBalanceScale />
            </p>
            <strong className="text-3xl whitespace-nowrap text-primary-1">
              {currencyFormatter(1000000)}
            </strong>
          </div>
          <div className="flex flex-col w-1/2 h-35 item-start justify-center p-6 bg-gray-900 rounded-sm shadow-md">
            <p className="flex  text-sm items-center gap-1 text-gray-400">
              Total Revenue <FaInfoCircle />
            </p>
            <strong className="text-2xl text-primary-1">
              {currencyFormatter(1000)}
            </strong>

            <p className="flex mt-4 text-sm items-center gap-1 text-gray-400">
              Average APY <FaInfoCircle />
            </p>
            <strong className="text-primary-1">7.5% </strong>
          </div>
        </div>

        <h1 className="my-2">
          <strong className="text-gray-400">Recent Transactions</strong>
        </h1>
        <div className="flex flex-wrap gap-2">
          <div className="flex flex-col w-80 h-35 p-6 bg-gray-900 rounded-sm shadow-md">
            <div className="my-2">
              <p className="text-gray-400 text-sm">Protocol:</p>
              <strong>MakerDAO</strong>
            </div>

            <p className="flex  text-sm items-center gap-1 text-gray-400">
              Amount Invested <FaInfoCircle />
            </p>
            <strong className="text-2xl text-primary-1">
              {currencyFormatter(10000)}
            </strong>

            <p className="flex mt-4 text-sm items-center gap-1 text-gray-400">
              APY <FaInfoCircle />
            </p>
            <strong className="text-primary-1">7.5% </strong>
          </div>
          <div className="flex flex-col w-80 h-35 p-6 bg-gray-900 rounded-sm shadow-md">
            <div className="my-2">
              <p className="text-gray-400 text-sm">Protocol:</p>
              <strong>MakerDAO</strong>
            </div>

            <p className="flex  text-sm items-center gap-1 text-gray-400">
              Amount Invested <FaInfoCircle />
            </p>
            <strong className="text-2xl text-primary-1">
              {currencyFormatter(10000)}
            </strong>

            <p className="flex mt-4 text-sm items-center gap-1 text-gray-400">
              APY <FaInfoCircle />
            </p>
            <strong className="text-primary-1">7.5% </strong>
          </div>
          <div className="flex flex-col w-80 h-35 p-6 bg-gray-900 rounded-sm shadow-md">
            <div className="my-2">
              <p className="text-gray-400 text-sm">Protocol:</p>
              <strong>MakerDAO</strong>
            </div>

            <p className="flex  text-sm items-center gap-1 text-gray-400">
              Amount Invested <FaInfoCircle />
            </p>
            <strong className="text-2xl text-primary-1">
              {currencyFormatter(10000)}
            </strong>

            <p className="flex mt-4 text-sm items-center gap-1 text-gray-400">
              APY <FaInfoCircle />
            </p>
            <strong className="text-primary-1">7.5% </strong>
          </div>
          
        </div>
      </main>
    </Layout>
  );
}

export default Dashboard;
