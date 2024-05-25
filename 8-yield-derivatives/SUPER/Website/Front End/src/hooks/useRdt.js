import { useContext } from "react";
import { RdtContext } from "../context/contexts.jsx";

export const useRdt = () => useContext(RdtContext);
