import { useRdtState } from "./useRdtState";

export const useAccounts = () => {
  const state = useRdtState();

  return state?.accounts ?? [];
};