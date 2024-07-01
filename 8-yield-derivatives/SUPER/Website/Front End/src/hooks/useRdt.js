import { useContext } from "react";
import { RdtContext } from "../context/rdtContext.jsx";

/**
 * Custom hook to access RDT
 *
 * @returns {Object} The value of the RdtContext.
 */
export const useRdt = () => useContext(RdtContext);
