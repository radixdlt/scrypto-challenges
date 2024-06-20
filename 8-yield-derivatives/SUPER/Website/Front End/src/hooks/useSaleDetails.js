import { useContext } from "react";
import {SaleDetailsContext} from "../context/providers/SaleDetailProvider.jsx"

/**
 * Custom hook to access the SaleDetailsContext.
 *
 * @returns {Object} The value of the SaleDetailsContext.
 */
export const useSaleDetails = () => useContext(SaleDetailsContext);
