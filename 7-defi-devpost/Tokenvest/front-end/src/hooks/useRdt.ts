import { RdtContext } from "@/rdt/rdt-context";
import { useContext } from "react";

export const useRdt = () => {
  const rdt = useContext(RdtContext);

  return rdt;
};
