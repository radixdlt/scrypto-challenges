import { useContext } from "react";
import { gatewayApiContext } from "../context/contexts.jsx";

export const useGatewayApi = () => useContext(gatewayApiContext);
