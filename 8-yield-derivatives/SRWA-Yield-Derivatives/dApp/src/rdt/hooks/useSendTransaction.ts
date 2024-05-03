import { useCallback } from 'react';

import { useRdt } from './useRdt';

export const useSendTransaction = (message?: string) => {
  const rdt = useRdt();

  return useCallback(
    (transactionManifest: string) => {
      return rdt.walletApi.sendTransaction({ transactionManifest, version: 1, message: message });
    },
    [rdt],
  );
};
