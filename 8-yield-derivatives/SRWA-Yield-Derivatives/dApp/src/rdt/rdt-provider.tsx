import React from 'react';

import { Rdt, RdtContext } from './rdt-context';

export const RdtProvider = (
  input: React.PropsWithChildren<{
    value: Rdt;
  }>,
) => <RdtContext.Provider value={input.value}>{input.children}</RdtContext.Provider>;
