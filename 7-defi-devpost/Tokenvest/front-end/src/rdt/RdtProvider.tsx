import { RdtContext } from "./rdt-context";
import { Rdt } from "./types";

export const RdtProvider = (
  input: React.PropsWithChildren<{
    value: Rdt;
  }>
) => (
  <RdtContext.Provider value={input.value}>
    {input.children}
  </RdtContext.Provider>
);