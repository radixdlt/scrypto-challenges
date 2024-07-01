import { useContext } from "react";
import {gatewayApiContext} from "../context/gatewayApiContext.jsx";

/**
 * Custom hook to access the Gateway Api.
 *
 * @returns {Object} The value of the gatewayApiContext.
 */
export const useGatewayApi = () => useContext(gatewayApiContext);
