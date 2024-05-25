import {GatewayApiClient} from "@radixdlt/babylon-gateway-api-sdk";
import {RadixDappToolkit, RadixNetwork} from "@radixdlt/radix-dapp-toolkit";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.jsx";
import {GatewayApiProvider} from "./context/providers/GatewayApiProvider.jsx";
import {RdtProvider} from "./context/providers/RdtProvider.jsx";
import {AccountProvider} from "./AccountContext.jsx";
import {SaleDetailsProvider} from "./context/providers/SaleDetailProvider.jsx";
import {UpdateTriggerProvider} from "./context/providers/UpdateTriggerProvider.jsx";


// You can create a dApp definition in the dev console at https://stokenet-console.radixdlt.com/configure-metadata
// then use that account for your dAppId
const dAppId = import.meta.env.VITE_DAPP_ID;

// Initialize the Gateway API for network queries and the Radix Dapp Toolkit for connect button and wallet usage.
const applicationVersion = "1.0.0";
const applicationName = "Super IYO";
const networkId = RadixNetwork.Stokenet; // network ID 2 for the stokenet test network, 1 for mainnet

// Instantiate Gateway API
const gatewayApi = GatewayApiClient.initialize({
    networkId,
    applicationName,
    applicationVersion,
});
console.log("gatewayApi: ", gatewayApi);

// Instantiate DappToolkit
const rdt = RadixDappToolkit({
    dAppDefinitionAddress: dAppId,
    networkId,
    applicationName,
    applicationVersion,
});

console.log("dApp Toolkit: ", rdt);

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <GatewayApiProvider value={gatewayApi}>
            <RdtProvider value={rdt}>
                <AccountProvider>
                    <UpdateTriggerProvider>
                        <SaleDetailsProvider>
                                <App/>
                        </SaleDetailsProvider>
                    </UpdateTriggerProvider>
                </AccountProvider>
            </RdtProvider>
        </GatewayApiProvider>
    </React.StrictMode>
);
