import { useRdtState } from "./useRdtState";

export const usePersona = () => {
  const state = useRdtState();

  return state?.persona;
};
