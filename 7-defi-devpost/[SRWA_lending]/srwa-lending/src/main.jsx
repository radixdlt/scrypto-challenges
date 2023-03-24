import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { RdtProvider } from "./RdtProvider"
import { RadixDappToolkit } from "@radixdlt/radix-dapp-toolkit"
// import './index.css'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <RdtProvider
      value={RadixDappToolkit(
        {
          dAppName: "lending",
          dAppDefinitionAddress:
            "account_tdx_b_1pqux68wudcgxs4l70390qacqxzqm8q9rxdzns4grmhqsynz5al",
        },
        (requestData) => {
          requestData({
            accounts: { quantifier: "atLeast", quantity: 1 },
          });
        },
        {
          networkId: 11,
        }
      )}
    >
      <App />
    </RdtProvider>
  </React.StrictMode>
);
