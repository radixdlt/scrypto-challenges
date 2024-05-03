import { Configuration, RadixNetworkConfigById, StateApi, TransactionApi } from '@radixdlt/babylon-gateway-api-sdk';

import { networkId as networkIdSubject } from '../network/state';

const networkId = networkIdSubject.value;

export const COMPONENT_ADDRESS = import.meta.env.VITE_COMPONENT_ADDRESS;
export const TOTAL_YT_SUPPLY = 1000000;

const gatewayApiConfig = new Configuration({ basePath: RadixNetworkConfigById[networkId].gatewayUrl });

export const stateApi = new StateApi(gatewayApiConfig);

export const transactionApi = new TransactionApi(gatewayApiConfig);

export const formattedNumber = (number: number | null, max = 4): string => {
  if (number === null) return '';

  const formatter: Intl.NumberFormat = new Intl.NumberFormat('en-US', {
    minimumFractionDigits: max === 4 ? 1 : 2,
    maximumFractionDigits: max,
    notation: 'compact',
  });
  return formatter.format(number);
};

export const roundDownNumber = (amount: number, decimal = 2): number => {
  decimal = +decimal;
  const value = +(1 + new Array(decimal + 1).join('0').slice(-decimal));
  return Math.floor(+amount * value) / value;
};

export const calculateDaysLeft = (timestamp: number, daysLeft: number): number => {
  if (isNaN(timestamp)) {
    return 0;
  }

  // Convert Unix timestamp to milliseconds and add 30 days (in milliseconds)
  const futureDate = new Date((timestamp + daysLeft * 24 * 60 * 60) * 1000);

  // Get today's date
  const today = new Date();

  // Calculate the difference in milliseconds between the future date and today's date
  const differenceInMilliseconds = futureDate.getTime() - today.getTime();

  // Convert milliseconds to days
  return Math.ceil(differenceInMilliseconds / (1000 * 60 * 60 * 24));
};
