import { useContext } from "react";
import {gatewayApiContext} from "../context/gatewayApiContext.jsx";

export const useGatewayApi = () => useContext(gatewayApiContext);
