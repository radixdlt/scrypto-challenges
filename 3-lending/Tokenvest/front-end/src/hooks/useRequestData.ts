import { DataRequestInput } from "@radixdlt/radix-dapp-toolkit";
import { useCallback } from "react";
import { useRdt } from "./useRdt";

export const useRequestData = () => {
  const rdt = useRdt()!;

  return useCallback(
    (value: DataRequestInput) => rdt.requestData(value),
    [rdt]
  );
};
