# Getting Started

This example is the template for a simple decentralized application (dApp) using
React JS. It utilizes the Radix dApp Toolkit to interact with the Radix Ledger
via the Gateway API and the Radix Wallet.

In the `react-js-dapp` directory: run `npm install` to install the dependencies
and then `npm run dev` to start the development server.

## What is Included

- `index.html` - The main HTML file for the dApp.
- `src/index.jsx` - The main JS file where the React app is initialized and the
  root component `src/App.jsx` is mounted to the DOM.
- `src/App.jsx` - The root component that holds other components
- `src/App.css` - The main CSS file for the dApp.
- `src/components/` - Components folder
- `src/hooks/` - Hooks folder

The project is bootstrapped with a React JS Vite project. This gives you a hot
reload development server out of the box and we add the preconfigured Radix dApp
toolkit, a walk through demonstrating how to set the Radix Wallet for Dev mode,
and a pre-deployed scrypto component to interact with on stokenet (the Radix
Public Test Network).

## `index.html`

It functions as the primary entry point when the application is loaded in a
browser. This file is crucial for setting up the basic structure of the
application.

For styling purposes, `index.html` includes links to Google Fonts, allowing us
to incorporate the `IBM Plex Sans` font family. Also, there's a reference to a
favicon `hello-token-fav.svg`. The core of `index.html` is the `<div>` element
with `id="root"` that acts as the mounting point for our entire React
application. When React starts, it latches onto this div and renders the app's
components within it. At the end of the body section, index.html includes a
script tag that imports the `index.jsx` file. This script is the entry point for
the React JavaScript code, kicking off the React application's execution.

## `src/index.jsx`

This JavaScript file, serves as the main entry point for initializing and
rendering the React application. The file begins by setting up the
RadixDappToolkit with a specific dApp ID.

It uses the Radix dApp Toolkit to interact with the Radix Wallet and the Gateway
API to interact with the Radix Ledger. You can find examples of how to connect
to the Radix Ledger and send tokens. These examples provide core building blocks
for creating a dApp on the Radix Ledger. Key Features of the Radix dApp Toolkit
include:

- User persona and account information
- Constructing and sending transactions
- Listening for transaction status updates & retrieving comitted transaction
  details.

```javascript
// You can create a dApp definition in the dev console at https://stokenet-console.radixdlt.com/dapp-metadata
// then use that account for your dAppId
const dAppId =
  "account_tdx_2_128jm6lz94jf9tnec8d0uqp23xfyu7yc2cyrnquda4k0nnm8gghqece";
// Initialize the Gateway API for network queries and the Radix Dapp Toolkit for connect button and wallet usage.
const applicationVersion = "1.0.0";
const applicationName = "Hello Token dApp";
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
```

The configured `rdt` (RadixDappToolkit instance) and `gatewayApi` are then
passed as a value to the their respective providers. These providers are React
context providers that makes the toolkit and API accessible to any component
within the app, facilitating a seamless integration of Radix Network
functionalities throughout the application. The root React component, App, is
rendered into the DOM within the `<div>` with the id root. This is done using
the ReactDOM.createRoot method, encapsulated within React's StrictMode for
highlighting potential problems in an application.

```javascript
ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <GatewayApiProvider value={gatewayApi}>
      <RdtProvider value={rdt}>
        <AccountProvider>
          <App />
        </AccountProvider>
      </RdtProvider>
    </GatewayApiProvider>
  </React.StrictMode>
);
```

### `src/RdtProvider.jsx`

It uses the RdtContext from rdt-context.js to provide a React context.
RdtProvider takes a value prop and wraps its children components, allowing them
to access the context's value.

```javascript
export const RdtProvider = ({ value, children }) => (
  <RdtContext.Provider value={value}>{children}</RdtContext.Provider>
);

RdtProvider.propTypes = {
  value: PropTypes.any,
  children: PropTypes.node.isRequired,
};
```

### `src/rdt-context.js`

Here, a new context is created using React's createContext function. This
context is used to share data across components in the application, specifically
data related to the Radix Dapp Toolkit.

```javascript
import { createContext } from "react";

export const RdtContext = createContext(null);
```

## `src/App.jsx`

This is the main component file for the application. It imports various
components like DevModeInstruction, PrimaryNavbar, DocumentationSection, and
HelloTokenSection. These components are then rendered inside a div with the id
container, forming the primary structure of the app's user interface.

```javascript
function App() {
  return (
    <div id="container">
      <PrimaryNavbar />
      <DevModeInstruction />
      <HelloTokenSection />
      <DocumentationSection />
    </div>
  );
}
```

### `src/components/NavBar.jsx`

This is where we inject the `radix-connect-button` web component into the DOM.
This component is a part of the Radix dApp Toolkit and is used to connect the
Radix Wallet to the dApp.

There are also two image elements in the PrimaryNavbar to display the Radix logo and
developer image.

### `src/components/DevModeInstruction.jsx`

Display useful information for the dApp, guide users through setting up
development mode for their wallet.

### `src/components/HelloTokenSection`

This component serves as the main interface for the token claim functionality.
It uses the `useAccounts.js` hook to fetch and display user accounts in a
dropdown menu. Users can select an account from this dropdown. The component
also includes a `ClaimHello.jsx` component, which is the actual button used to
claim the token. The dropdown's visibility is managed by local state, and a
function is defined to handle account selection.

#### `src/components/useAccounts.js`

This custom hook is responsible for fetching the user accounts. It subscribes to
wallet data updates and sets the accounts in its state. The hook exposes the
list of accounts, the currently selected account, and a function to set the
selected account.

Following the instantiation of the Radix dApp Toolkit, we have an example of how
to get user information:

```javascript
rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));

const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
  console.log("subscription wallet data: ", walletData);
  if (walletData && walletData.accounts.length > 0) {
    setAccounts(walletData.accounts);
  }
});
```

#### `src/components/ClaimHello.jsx`

This component represents the button that users click to claim the "Hello
Token". It uses the `useSendTransaction.js` hook to handle the actual
transaction process. Upon clicking the button, it constructs a transaction
manifest and sends it using the sendTransaction function. It also handles the
validation to ensure a user account is selected before attempting the
transaction.

Next we have an example that shows how to construct a transaction manifest:

```javascript
const componentAddress =
  "component_tdx_2_1crmw9yqwfaz9634qf3tw9s89zxnk8fxva958vg8mxxeuv9j6eqer2s";
const accountAddress = selectedAccount.selectedAccount;

let manifest = `
  CALL_METHOD
    Address("${componentAddress}")
    "free_token"
    ;
  CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
    ;
`;
```

#### `src/components/useSendTransaction.js`

This is another custom hook that provides functionality to send a transaction.
It encapsulates the process of sending a transaction and fetching its receipt,
abstracting away the complexity from the components.

Next we have an example that shows how to construct and send a transaction to
the Radix wallet, and then fetch the results committed to the ledger from the
gateway API:

```javascript
const sendTransaction = useCallback(
  // Send manifest to extension for signing
  async (transactionManifest, message) => {
    const transactionResult = await rdt.walletApi.sendTransaction({
      transactionManifest,
      version: 1,
      message,
    });

    if (transactionResult.isErr()) throw transactionResult.error;
    console.log("transaction result:", transactionResult);

    // Get the details of the transaction committed to the ledger
    const receipt = await gatewayApi.transaction.getCommittedDetails(
      transactionResult.value.transactionIntentHash
    );
    return { transactionResult: transactionResult.value, receipt };
  },
  [gatewayApi, rdt]
);
```

For more information about the hello-token you can find the scrypto project in
the Radix Official-Examples repository
[here](https://github.com/radixdlt/official-examples/tree/main/getting-started/hello-token)
This project is a simple example of a Radix component that can be used to
interact with the Radix Ledger. It is pre-deployed on the stokenet network and
can be interacted with using the Radix dApp Toolkit. It contains a simple
blueprint that allows users to claim a token and deposit it into their wallet.
The other point of interest is the example of how to set up the
`dapp_definition` metadata for 2 way verification in the Radix Wallet. This is a
key feature of the Radix Wallet that allows users to verify the dApp they are
interacting with is the correct one.

## License

The Radix Official Examples code is released under Radix Modified MIT License.

    Copyright 2023 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.
