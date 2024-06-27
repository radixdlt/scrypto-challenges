import { RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit';
import { createContext } from 'react';

export type Rdt = ReturnType<typeof RadixDappToolkit>;

export const RdtContext = createContext<Rdt | null>(null);
