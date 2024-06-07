import { useContext } from "react";
import { RdtContext } from "../context/rdtContext.jsx";

export const useRdt = () => useContext(RdtContext);
