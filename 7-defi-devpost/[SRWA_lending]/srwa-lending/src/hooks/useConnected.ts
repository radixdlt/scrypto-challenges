import { useRdtState } from "./useRdtState";

export const useConnected = () => {
  const state = useRdtState();

  return state?.connected ?? false;
};