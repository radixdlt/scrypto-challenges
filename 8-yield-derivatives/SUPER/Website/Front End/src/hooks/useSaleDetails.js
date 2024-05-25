import { useContext } from "react";
import {SaleDetailsContext} from "../context/providers/SaleDetailProvider.jsx"

export const useSaleDetails = () => useContext(SaleDetailsContext);
