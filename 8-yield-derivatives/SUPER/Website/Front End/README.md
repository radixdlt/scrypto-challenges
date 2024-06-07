# Getting Started

This frontend uses the ReactJS template provided [here](https://github.com/radixdlt/official-examples/tree/main/getting-started/react-js-dapp). 
It utilizes the Radix dApp Toolkit to interact with the Radix Ledger via the Gateway API and the Radix Wallet. It also uses an express backend to 
store data for faster fetching as opposed to relying entirely on the GatewayAPI.

In the `react-js-dapp` directory: run `npm install` to install the dependencies
and then `npm run dev` to start the development server.

## What is Included

- `index.html` - The main HTML file for the dApp.
- `src/index.jsx` - The main JS file where the React app is initialized and the
  root component `src/App.jsx` is mounted to the DOM.
- `src/App.jsx` - The root component that holds other components
- `src/App.css` - The main CSS file for the dApp.
- `src/pages/` - Pages folder (combinations of different `sections`)
- `src/sections/` - Sections folder (combinations of different `components`)
- `src/components/` - Components folder
- `src/hooks/` - Hooks folder
- `src/manifests/` - Manifests folder
- `src/api/` - API folder (for communicating with backend)
- `src/context/` - Contexts and providers for various things

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
import {GatewayApiClient} from "@radixdlt/babylon-gateway-api-sdk";
import {RadixDappToolkit, RadixNetwork} from "@radixdlt/radix-dapp-toolkit";
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
```

## `src/App.jsx`
This is the main component file for the application. It imports various
pages like HomePage, DocsPage, BuySuperPage, ManageSuperPageV2 and
OwnerPage into a React `BrowserRouter`. These pages within the router
form the primary structure of the app's user interface.

```javascript
import "./App.css";
import HomePage from "./pages/HomePage/index.jsx"
import DocsPage from "./pages/DocsPage/index.jsx"
import BuySuperPage from "./pages/BuySuperPage/index.jsx";
import OwnerPage from "./pages/DevPage/index.jsx";
import SuperPage from "./pages/SuperPage/index.jsx";
import ManageSuperPageV2 from "./pages/ManageSuperPageV2/index.jsx";

function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="Docs" element={<DocsPage/>}/>
                <Route path="/super" element={<SuperPage />}>
                    <Route index element={<Navigate to="/super/buy" />} />
                    <Route path="buy" element={<BuySuperPage />} />
                    <Route path="manage" element={<ManageSuperPageV2/>}/>
                </Route>
                <Route path="DevsOnly" element={<OwnerPage/>}/>
            </Routes>
        </Router>
    );
}

export default App;
```

  
  
<details>
<summary style="font-size: 1.8em; font-weight: bold;">Contexts</summary>

## Contexts
Contexts (created using React's `createContext`) and providers that fill in
those contexts' values.

### [AccountContext.jsx](src%2FAccountContext.jsx)
The AccountProvider component is a React context provider that manages 
account-related state and functionality using the useRdt hook to interact 
with the Radix Dapp Toolkit. It initializes accounts and selectedAccount 
states, and subscribes to wallet updates to keep the account list current. 
The context value, including account data and setters, is memoized for 
performance and provided to child components. The children prop is validated 
with PropTypes to ensure it's always provided and renderable.
```javascript
import {DataRequestBuilder} from "@radixdlt/radix-dapp-toolkit";
import {useRdt} from './hooks/useRdt.js';

// Create a context with a default value of null
export const AccountContext = createContext(null);

const AccountProvider = ({ children }) => {
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);

  const rdt = useRdt();

  useEffect(() => {
    // Set the request data to get at least one account
    rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));

    // Subscribe to wallet data updates
    const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
      console.log("subscription wallet data: ", walletData);

      // Update the accounts state with the received wallet data
      setAccounts(walletData && walletData.accounts ? walletData.accounts : []);
    });

    // Unsubscribe from the wallet data updates on cleanup
    return () => subscription.unsubscribe();
  }, [rdt]);

  // Memoizing the context value to optimize performance
  const contextValue = useMemo(() => ({
    accounts,
    setAccounts,
    selectedAccount,
    setSelectedAccount
  }), [accounts, selectedAccount, setAccounts, setSelectedAccount]);

  return (
      <AccountContext.Provider value={contextValue}>
        {children}
      </AccountContext.Provider>
  );
};
```

### [src/context/xrdAddressContext.jsx](src%2Fcontext%2FxrdAddressContext.jsx)
Defines a context for the XRD Address using React's `createContext` hook.
```javascript
export const XrdAddressContext = createContext("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");
```

### `null` contexts
The following files are initialized with a null value instead of the XRD Resource Address above:
 - [src/context/gatewayApiContext.jsx](src%2Fcontext%2FgatewayApiContext.jsx)
 - [src/context/updateTriggerContext.jsx](src%2Fcontext%2FupdateTriggerContext.jsx)
 - [src/context/rdtContext.jsx](src%2Fcontext%2FrdtContext.jsx)

### Contexts from Environment (`.env`) variables - `src/context/fromEnv/`
This folder contains 4 files:
 - [DappDefinitionCaddy.jsx](src%2Fcontext%2FfromEnv%2FDappDefinitionCaddy.jsx)
 - [NewSuperTxID.jsx](src%2Fcontext%2FfromEnv%2FNewSuperTxID.jsx)
 - [PackageAddy.jsx](src%2Fcontext%2FfromEnv%2FPackageAddy.jsx)
 - [SaleLength.jsx](src%2Fcontext%2FfromEnv%2FsaleLength.jsx)

All 4 take variables from the .env file and export them as contexts using the  
React `createContext` hook, so they can be used throughout the entire code. 
`DappDefinitionCaddy.jsx` is provided below as an example:
```javascript
export const DappDefinitionCaddy = createContext(import.meta.env.VITE_DAPP_ID);
```

### Context Providers - `src/context/providers/`
This folder contains various provider components that wrap their children components, 
allowing them to access the context values.

### [src/context/providers/RdtProvider.jsx](src%2Fcontext%2Fproviders%2FRdtProvider.jsx)
Uses the RdtContext from nullContexts.jsx to provide a React context. RdtProvider takes 
a value prop and wraps its children components, allowing them to access the context's value.
```javascript
import { RdtContext } from "../rdtContext.jsx";

export const RdtProvider = ({ value, children }) => (
        <RdtContext.Provider value={value}>{children}</RdtContext.Provider>
)
```

### [src/context/providers/GatewayApiProvider.jsx](src%2Fcontext%2Fproviders%2FGatewayApiProvider.jsx)
It uses the RdtContext from GatewayApiContext.jsx to provide a React context.
GatewayApiProvider takes a value prop and wraps its children components, allowing them
to access the context's value.
```javascript
import {gatewayApiContext} from "../gatewayApiContext.jsx";

export const GatewayApiProvider = ({ value, children }) => (
        <gatewayApiContext.Provider value={value}>
          {children}
        </gatewayApiContext.Provider>
);
```

### [src/context/providers/SaleDetailProvider.jsx](src%2Fcontext%2Fproviders%2FSaleDetailProvider.jsx)
It uses the SaleDetailContext from rdtContext.jsx to provide a React context.
SaleDetailProvider takes a value prop and wraps its children components, allowing them
to access the context's value.
```javascript
import {UpdateTriggerContext} from "../updateTriggerContext.jsx";
import {getLatestSaleDetails} from "../../api/get.js";

export const SaleDetailsContext = createContext(null);

export const SaleDetailsProvider = ({ children }) => {
  const [saleDetails, setSaleDetails] = useState(null);
  const { trigger } = useContext(UpdateTriggerContext);

  useEffect(() => {
    const fetchSaleDetails = async () => {
      try {
        const response = await getLatestSaleDetails();
        setSaleDetails(response);
      } catch (error) {
        console.error('Error fetching sale details:', error);
      }
    };

    fetchSaleDetails();
  }, [trigger]);

  return (
          <SaleDetailsContext.Provider value={saleDetails}>
            {children}
          </SaleDetailsContext.Provider>
  );
};
```

### [UpdateTriggerProvider.jsx](src%2Fcontext%2Fproviders%2FUpdateTriggerProvider.jsx)
The UpdateTriggerProvider component provides the UpdateTriggerContext with a trigger state 
and an update function that increments the trigger state. It ensures that any component 
consuming this context can access and update the trigger value, thereby facilitating 
controlled state updates.
```javascript
import {UpdateTriggerContext} from "../updateTriggerContext.jsx";


// Provider component
// eslint-disable-next-line react/prop-types
export const UpdateTriggerProvider = ({ children }) => {
  const [trigger, setTrigger] = useState(0); // initial value 0

  const update = () => {
    console.log("Update trigger pulled, bullet left the chamber");
    setTrigger(prev => prev + 1); // increment to trigger update
  };

  return (
          <UpdateTriggerContext.Provider value={{ trigger, update }}>
            {children}
          </UpdateTriggerContext.Provider>
  );
};
```

</details>


<details>
<summary style="font-size: 1.8em; font-weight: bold;">API</summary>

## API (`src/api`)
API Opener Para

### [get.js](src%2Fapi%2Fget.js)

### [posts.js](src%2Fapi%2Fposts.js)

</details>




<details>
<summary style="font-size: 1.8em; font-weight: bold;">Manifests</summary>

## Manifests (`src/manifests`)
Manifests Opener Para (mention incomplete)

### [endSaleManifest.js](src%2Fmanifests%2FendSaleManifest.js)
Opener
```javascript

```

### [newSuperManifest.js](src%2Fmanifests%2FnewSuperManifest.js)
Opener
```javascript

```

### [splitNftManifest.js](src%2Fmanifests%2FsplitNftManifest.js)
Opener
```javascript

```

### [startSaleManifest.js](src%2Fmanifests%2FstartSaleManifest.js)


</details>




<details>
<summary style="font-size: 1.8em; font-weight: bold;">Hooks</summary>

## Hooks (`src/hooks`)
hooks Opener Para (mention incomplete)

### [useAccount.jsx](src%2Fhooks%2FuseAccount.jsx)
Opener
```javascript

```

### [useCombinedNftData.jsx](src%2Fhooks%2FuseCombinedNftData.jsx)
Opener
```javascript

```

### [useComponentDetails.js](src%2Fhooks%2FuseComponentDetails.js)
Opener
```javascript

```

### [useGatewayApi.js](src%2Fhooks%2FuseGatewayApi.js)
```javascript

```

### [useGetEventInCommitDetails.js](src%2Fhooks%2FuseGetEventInCommitDetails.js)
Opener
```javascript

```

### [useGetEventInReceipt.js](src%2Fhooks%2FuseGetEventInReceipt.js)
Opener
```javascript

```

### [useRdt.js](src%2Fhooks%2FuseRdt.js)
Opener
```javascript

```

### [useSaleDetails.js](src%2Fhooks%2FuseSaleDetails.js)
```javascript

```
### [useSaleTimeDetails.js](src%2Fhooks%2FuseSaleTimeDetails.js)
Opener
```javascript

```

### [useSendTransaction.js](src%2Fhooks%2FuseSendTransaction.js)
Opener
```javascript

```

### [useUpdateSaleDetails.js](src%2Fhooks%2FuseUpdateSaleDetails.js)
Opener
```javascript

```


</details>




<details>
<summary style="font-size: 1.8em; font-weight: bold;">Pages, Sections, and Components</summary>

## Pages, Sections, and Components
For the sake of clarity, the following section is presented in a manner where various pages 
(from `src/pages`) are presented. Every page is the combination of various sections, so we'll 
go from pages to sections (from `src/sections`), followed by the components (from`src/components`) 
that they employ.

We'll follow the order of the `BrowserRouter` in `src/App.jsx`:
```javascript
<Router>
  <Routes>
    <Route path="/" element={<HomePage />} />
    <Route path="Docs" element={<DocsPage/>}/>
    <Route path="/super" element={<SuperPage />}>
      <Route index element={<Navigate to="/super/buy" />} />
      <Route path="buy" element={<BuySuperPage />} />
      <Route path="manage" element={<ManageSuperPageV2/>}/>
    </Route>
    <Route path="DevsOnly" element={<OwnerPage/>}/>
  </Routes>
</Router>
```
### [HomePage](src%2Fpages%2FHomePage)
EXPLAIN
```javascript
function HomePage() {
  return (
          <>
            <PrimaryNavbar />
            <main>
              <DevModeInstruction />
            </main>
          </>
  );
}

export default HomePage;
```

#### [PrimaryNavbar.jsx](src%2Fcomponents%2FPrimaryNavbar.jsx)

#### [DevModeInstruction.jsx](src%2Fcomponents%2FDevModeInstruction.jsx)


### [DocsPage](src%2Fpages%2FDocsPage)
EXPLAIN
```javascript
import "../../App.css"
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import DocumentationSection from "../../sections/DocumentationSection.jsx";

function DocsPage() {
    return (
        <>
            <PrimaryNavbar />
            <main>
                <DocumentationSection />
            </main>
        </>
    );
}

export default DocsPage;
```

#### [DocumentationSection.jsx](src%2Fsections%2FDocumentationSection.jsx)


### [SuperPage](src%2Fpages%2FSuperPage)
EXPLAIN
```javascript
import PrimaryNavbar from "../../components/PrimaryNavbar.jsx";
import SecondaryNavbar from "../../components/SecondaryNavBar.jsx";
import { Outlet } from "react-router-dom";

function SuperPage() {
  return (
    <>
      <PrimaryNavbar />
      <SecondaryNavbar />
      <main>
      {/* The Outlet will render child routes */}
      <Outlet />
      </main>
    </>
  );
}

export default SuperPage;
```



### [BuySuperPage](src%2Fpages%2FBuySuperPage)
```javascript
import BuySuperSection from "../../sections/BuySuperSection.jsx";

function BuySuperPage() {
    return (
        <>
            <main>
                <BuySuperSection />
            </main>
        </>
    );
}

export default BuySuperPage;
```

#### [BuySuperSection.jsx](src%2Fsections%2FBuySuperSection.jsx)
```javascript
import {useState, useEffect} from "react";
import BuySuper from "../components/BuySuper.jsx";
import ExchangeRatePic from "../components/ExchangeRatePic.jsx";
import AccountDropdown from "../components/AccountDropdown.jsx";
import {useAccount} from "../hooks/useAccount.jsx";


const BuySuperSection = () => {
  const { accounts } = useAccount();
  const [selectedAccount, setSelectedAccount] = useState(null);
  const [enableButtons, setEnableButtons] = useState(false);
  const [xrdAmount, setXrdAmount] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    // Automatically enable buttons if accounts are available
    setEnableButtons(accounts.length > 0);
  }, [accounts]);

  useEffect(() => {
    if (accounts.length > 0) {
      setEnableButtons(true);
    } else {
      setEnableButtons(false);
    }
  }, [accounts]); // Only re-run the effect if count changes


  const isNumeric = num => !isNaN(num);
  const handleChange = (e) => {
    const val = e.target.value.trim();
    if (val === '' || (isNumeric(val))) {
      setXrdAmount(val);
      setError('');
    } else {
      setError('Please enter a numeric value.');
    }
  };

  return (
      <>

        <div className="buy-super-container">

          <div className="go-buy-super">

            <h2>Go</h2> <h2 className='h2-cyan'>SUPER</h2>

          </div>

          <ExchangeRatePic/>

          <div className='buy-super-input-container'>

            <AccountDropdown
                selectedAccount={selectedAccount}
                setSelectedAccount={setSelectedAccount}
                enableDropdown={enableButtons}
            />

            <div className="buy-super-input-wrapper">

              <input
                  type={"text"}
                  id={"buy-super-input"}
                  onChange={handleChange}
                  value={xrdAmount}
                  placeholder="Enter XRD Amount"
                  style={{marginBottom: '0.625rem'}}
              />

              <p id={'input-suffix'}>
                XRD
              </p>

            </div>

            <BuySuper
                selectedAccount={selectedAccount}
                enableButtons={enableButtons}
                xrdAmount={xrdAmount}
                error={error}
            />

          </div>


        </div>

      </>
  );
};

export default BuySuperSection;
```

##### [AccountDropdown.jsx](src%2Fcomponents%2FAccountDropdown.jsx)
```javascript
import React, {useEffect} from 'react';
import PropTypes from "prop-types";
import {useAccount} from "../hooks/useAccount.jsx";



const AccountDropdown = (props) => {
    const { selectedAccount, setSelectedAccount, enableDropdown } = props;

    const { accounts } = useAccount();
    const [dropdownOpen, setDropdownOpen] = React.useState(false);
    const [selectStyle, setSelectStyle] = React.useState({
        width: "100%",
        fontSize: "1.15rem",
        backgroundColor: "var(--grey-2)",
        padding: "0.675rem 1rem",
        border: "1px solid var(--grey-5)",
        borderRadius: "8px",
        cursor: "pointer",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
    });
    const [active, setActive] = React.useState(false);

    useEffect(() => {
        if (accounts.length === 1) {
            setSelectedAccount(accounts[0].address);
            handleSelectAccount(accounts[0])
        }
    }, [accounts, setSelectedAccount]);

    const toggleDropdown = () => {
        setActive(!active);
        setDropdownOpen(!dropdownOpen);
    };

    const handleSelectAccount = (account) => {
        setSelectedAccount(account.address);
        setSelectStyle({
            ...selectStyle,
            background: `var(--account-appearance-${account.appearanceId})`,
            border: "none",
        });
        setActive(false);
        setDropdownOpen(false);
    };

    const renderAccountLabel = (account) => {
        const shortAddress = `${account.address.slice(0, 4)}...${account.address.slice(-6)}`;
        return `${account.label || "Account"} ${shortAddress}`;
    };

    return (
        <div className={"custom-select" + (active ? " active" : "")}>

            <button
                className={
                    selectedAccount ? "select-button-account" : "select-button"
                }
                role="combobox"
                aria-haspopup="listbox"
                aria-expanded={dropdownOpen}
                onClick={toggleDropdown}
                aria-controls="select-dropdown"
                disabled={!enableDropdown}
                style={selectStyle}
            >
                <span className="selected-value">
                    {!enableDropdown
                        ? "Setup Dev Mode to choose an account"
                        : selectedAccount && enableDropdown
                            ? renderAccountLabel(accounts.find((acc) => acc.address === selectedAccount))
                            : "Select an Account"}
                </span>
                <span className={selectedAccount ? "arrow-account" : "arrow"} />
            </button>

            {dropdownOpen && (
                <ul
                    className="select-dropdown"
                    role="listbox"
                    id="select-dropdown"
                >

                    {accounts.map((account) => (
                        <li
                            key={account.address}
                            role="option"
                            style={{
                                background: `var(--account-appearance-${account.appearanceId})`,
                            }}
                            onClick={() => handleSelectAccount(account)}
                        >
                            <label>{renderAccountLabel(account)}</label>
                            <input
                                type="radio"
                                name={account.label}
                                value={account.address}
                                defaultChecked={selectedAccount === account.address}
                            />
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};
AccountDropdown.propTypes = {
    selectedAccount: PropTypes.string,
    setSelectedAccount: PropTypes.func,
    enableDropdown: PropTypes.bool,
};
export default AccountDropdown;
```

##### [BuySuper.jsx](src%2Fcomponents%2FBuySuper.jsx)
```javascript
import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy, useXrdAddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {SendNewNftToMongo} from "../api/posts.js";


BuySuper.propTypes = {
    selectedAccount: PropTypes.string,
    enableButtons: PropTypes.bool,
    xrdAmount: PropTypes.string,
    error: PropTypes.string,
};

function BuySuper(props) {
    const [receipt, setReceipt] = useState(null);

    const {selectedAccount, enableButtons, xrdAmount, error} = props;

    const sendTransaction = useSendTransaction();

    const xrdAddy = useXrdAddy();
    const componentAddy = useComponentAddy();

    const handleBuySuper = async () => {

        if (!selectedAccount) {
            alert("Please select an account first.");
            return;
        }
        if (error) {
            alert("Fix the errors before submitting.");
            return;
        }

        const accountAddress = selectedAccount;

        let manifest = `
            CALL_METHOD
                Address("${accountAddress}")
                "withdraw"
                Address("${xrdAddy}")
                Decimal("${xrdAmount}");
            
            TAKE_FROM_WORKTOP
                Address("${xrdAddy}")
                Decimal("${xrdAmount}")
                Bucket("bucket1");
            
            CALL_METHOD
                Address("${componentAddy}")
                "deposit"
                Bucket("bucket1");
            
            CALL_METHOD
                Address("${accountAddress}")
                "deposit_batch"
                Expression("ENTIRE_WORKTOP");
            `;

        console.log("manifest", manifest);

        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    const CreateYieldNFTEvent = useGetEventInReceipt(receipt, "CreateYieldNFTEvent");

    useEffect(() => {
        // Check if the receipt is not null and call the function
        if (receipt) {
            if(CreateYieldNFTEvent) {
                // Call the function when receipt is updated
                SendNewNftToMongo(CreateYieldNFTEvent)
            }
        }
    }, [receipt, CreateYieldNFTEvent]); // This hook will re-run whenever receipt changes

    return (
        <div>
            <button
                id="buy-super-button"
                onClick={handleBuySuper}
                disabled={!selectedAccount || !enableButtons}>
                Buy SUPER
            </button>
        </div>
    );
}

export default BuySuper;
```
#### [ExchangeRatePic.jsx](src%2Fcomponents%2FExchangeRatePic.jsx)
```javascript
function ExchangeRatePic() {
    const RadixLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets-global.website-files.com/6053f7fca5bf627283b582c2/6266da23999171a63bcbb2a7_Radix-Icon-Round-200x200.svg"
                    alt="Radix Token"
                />

                <h2> 10 XRD </h2>

            </span>
        );
    };

    const SuperYieldNFTLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/png/tp/white/yield_nft.png"
                    alt="SuperYield"
                />

                <h2>SUPER Yield NFT </h2>

            </span>
        )
    }

    const SuperLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/white/super_s.svg"
                    alt="SuperYieldNFT"
                />

                <h2> 10 SUPER </h2>

            </span>
        )
    }

    const SuperTrustLogo = () => {
        return (
            <span className='exchange-amount-container'>

                <img
                    src="https://assets.floww.fi/images/logo/svg/bg/white/super_t.svg"
                    alt="SuperYield"
                />

                <h2>6 SUPERt</h2>

            </span>

        )
    }

    return (
        <span className="ExchangeRateFormula">

            <RadixLogo/>
            <h2 className='exchange-symbols'>=</h2>
            <SuperLogo/>
            <h2 className='exchange-symbols'>+</h2>
            <SuperTrustLogo/>
            <h2 className='exchange-symbols'>+</h2>
            <SuperYieldNFTLogo/>

        </span>
    )

}

export default ExchangeRatePic;
```



### [ManageSuperPageV2](src%2Fpages%2FManageSuperPageV2)
```javascript
import SelectNftSection from "../../sections/SelectNFTSection.jsx";
import {useEffect, useState} from "react";
import {useAccount} from "../../hooks/useAccount.jsx";
import {useYieldNftRaddy} from "../../hooks/useComponentDetails.js";
import SplitNFTSectionV2 from "../../sections/SplitNFTSectionV2.jsx";

const ManageSuperPage = () => {
    const { accounts } = useAccount();
    // State to manage the selected account and NFT-related options
    const [selectedAccount, setSelectedAccount] = useState(null);
    // eslint-disable-next-line no-unused-vars
    const [enableSelectNft, setEnableSelectNft] = useState(true); // Assuming this might be toggled based on some conditions
    // eslint-disable-next-line no-unused-vars
    const [selectedNft, setSelectedNft] = useState(null);
    // eslint-disable-next-line no-unused-vars
    const [enableInput, setEnableInput] = useState(false);
    const [enableButtons, setEnableButtons] = useState(false);
    const YieldNftRaddy = useYieldNftRaddy();

    useEffect(() => {
        // Automatically enable buttons if accounts are available
        setEnableButtons(accounts.length > 0);
        setEnableSelectNft(accounts.length > 0);

    }, [accounts]);

    // You might also manage other states related to specific functionalities on this page
    useEffect(() => {
        if (selectedNft && selectedAccount) {
                setEnableInput(true)
            } else {
                setEnableInput(false)
            }
        }, [selectedNft, selectedAccount]);

    return (
        <div className="manage-super-page">

            <h1>Manage Your Super Assets</h1>

            <SelectNftSection
                selectedAccount={selectedAccount}
                setSelectedAccount={setSelectedAccount}
                enableButtons={enableButtons}
                enableSelectNft={enableSelectNft}
                YieldNftRaddy={YieldNftRaddy} // This would be dynamically fetched or set based on your application's logic
                setSelectedNft={setSelectedNft}
                setEnableInput={setEnableInput}
            />

            <SplitNFTSectionV2
                selectedAccount={selectedAccount}
                selectedNft={selectedNft}
                YieldNftRaddy={YieldNftRaddy}
                enableInput = {enableInput}
            />

            {/* Other sections or components related to managing super assets could also be added here */}
        </div>
    );
};

export default ManageSuperPage;
```

#### [SelectNFTSection.jsx](src%2Fsections%2FSelectNFTSection.jsx)
```javascript
import PropTypes from 'prop-types';
import AccountDropdown from '../components/AccountDropdown.jsx';
import YieldNFTDropdown from '../components/YieldNFTDropdown.jsx';

const SelectNftSection = ({ selectedAccount, setSelectedAccount, enableButtons, enableSelectNft, YieldNftRaddy, setSelectedNft, setEnableInput }) => {
    return (
        <>
            <div className="choose-owner-container">

                <div className="choose-owner-heading-section">
                    <p className="head-text">Select the account and the SUPER Yield NFT you want to manage.</p>
                </div>

                <div className="choose-owner-dropdown-container">

                    <AccountDropdown
                        selectedAccount={selectedAccount}
                        setSelectedAccount={setSelectedAccount}
                        enableDropdown={enableButtons} // Assuming this prop controls dropdown enable state
                    />
                </div>

                <div className="select-nft-dropdown-container">

                <YieldNFTDropdown
                        selectedAccount={selectedAccount}
                        enableSelectNft={enableSelectNft} // This might control the dropdown enable state based on NFT specifics
                        YieldNftRaddy={YieldNftRaddy}
                        setSelectedNft={setSelectedNft}
                        setEnableInput={setEnableInput}
                    />

                </div>

                <div className="manage-button-container">
                    
                </div>

            </div>
        </>
    );
};

SelectNftSection.propTypes = {
    selectedAccount: PropTypes.string,
    setSelectedAccount: PropTypes.func.isRequired,
    enableButtons: PropTypes.bool,
    enableSelectNft: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    setSelectedNft: PropTypes.func.isRequired,
    setEnableInput: PropTypes.func.isRequired
};

export default SelectNftSection;
```

##### [YieldNFTDropdown.jsx](src%2Fcomponents%2FYieldNFTDropdown.jsx)
```javascript
import {useCallback, useEffect, useState} from 'react';
import PropTypes from 'prop-types';
import {useCombinedNftData} from "../hooks/useCombinedNftData.jsx";


const YieldNFTDropdown = ({ selectedAccount, enableSelectNft, YieldNftRaddy, setSelectedNft, setEnableInput }) => {

    const AccountNftsWithData = useCombinedNftData(YieldNftRaddy)

    if (AccountNftsWithData.length > 0 && enableSelectNft) {
        console.log("Account w/ nft data: ", AccountNftsWithData)
    }

    const [selectedAccountNfts, setSelectedAccountNfts] = useState([]);

    useEffect(() => {
        if (selectedAccount && AccountNftsWithData.length > 0) {

            const nftsForSelectedAccount = AccountNftsWithData.filter(address => address.address === selectedAccount);
            if (nftsForSelectedAccount) {
                setSelectedAccountNfts(nftsForSelectedAccount[0]); // Assuming you want the first match or handle multiple matches appropriately
                console.log("selectedAccountNfts",selectedAccountNfts);
            }
        } else {
            setSelectedAccountNfts([]); // Reset when selected account is not defined
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectedAccount]);  // Ensure AccountNftsWithData is in the dependency array



    const [selectedNFT, setSelectedNFT] = useState(null);
    const [dropdownOpen, setDropdownOpen] = useState(false);
    const initialStyleState = {
        width: "100%",
        fontSize: "1.15rem",
        background: "var(--grey-2)",
        color: "white",
        padding: "0.675rem 1rem",
        border: "0.0625rem solid var(--grey-5)",
        borderRadius: "0.5rem",
        cursor: "pointer",
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center"
    }
    const [selectStyle, setSelectStyle] = useState(initialStyleState);

    useEffect(() => {
        // Reset the selected NFT when selectedAccount changes
        setSelectedNFT(null);
        setDropdownOpen(false);
        setSelectStyle(initialStyleState);
        setSelectedNft(null);
        setEnableInput(false)
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectedAccount]);

    const toggleDropdown = useCallback(() => {
        setDropdownOpen(prevOpen => !prevOpen);
    }, []);

    const handleSelectNFT = useCallback((nft, index) => {
        const bgSelector = index % 2 === 0 ? 'even' : 'odd';
        const fontSelector = index % 2 === 0 ? 'white' : 'var(--grey-1)';
        setSelectedNFT(nft);
        setSelectedNft(nft);
        setEnableInput(true);
        console.log("Selected NFT", nft);
        setSelectStyle({
            ...selectStyle,
            background: `var(--nft-appearance-${bgSelector}-bg)`,
            color: `${fontSelector}`,
            border: "none",
        });
        setDropdownOpen(false);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectStyle]);

    const renderSelectedNFT = () => {
        if (selectedNFT) {
            return (
                <span className="nft-dropdown-option">
                    <span className="nft-dropdown-option-label">
                        NFT {selectedNFT.label}
                    </span>

                    <span className="nft-dropdown-option-data">
                        {`${selectedNFT.data.n_super_minted} SUPER Minted at Hour ${selectedNFT.data.hour_of_mint}`}
                    </span>
                </span>
            )
        } else {
            return "Select NFT ID";
        }
    };


        return (
            <div className={"custom-select" + (dropdownOpen ? " active" : "")}>

                <button
                    className="select-button"
                    role="combobox"
                    aria-haspopup="listbox"
                    aria-expanded={dropdownOpen}
                    disabled={!enableSelectNft}
                    onClick={toggleDropdown}
                    style={selectStyle}
                >
                    <span className="nft-dropdown-option">{renderSelectedNFT()}</span>
                    <span className="arrow" />
                </button>

                {dropdownOpen && (
                    <ul
                        className="select-dropdown"
                        role="listbox"
                        style={{ border: "0.0625rem solid var(--grey-5)", borderRadius: "0.5rem" }}
                    >

                        {selectedAccountNfts.nfts && selectedAccountNfts.nfts.length > 0 ? (
                            selectedAccountNfts.nfts.map((nft, index) => (
                                <li
                                    key={index}
                                    role="option"
                                    className={index % 2 === 0 ? 'nft-appearance-even' : 'nft-appearance-odd'}
                                    style={{
                                        padding: "0.5rem 0.625rem",
                                        cursor: "pointer"
                                    }}
                                    onClick={() => handleSelectNFT(nft, index)}
                                >
                                    {<span className="nft-dropdown-option">
                                        <span className="nft-dropdown-option-label">
                                            NFT {nft.label}
                                        </span>

                                        <span className="nft-dropdown-option-data">
                                            {`${nft.data.n_super_minted} SUPER Minted at Hour ${nft.data.hour_of_mint}`}
                                        </span>
                                    </span>}
                                </li>
                            ))
                        ) : (
                            <li style={{ padding: "1rem 2rem" }}>No NFTs found</li>
                        )}
                    </ul>
                )}
            </div>
        );

};

YieldNFTDropdown.propTypes = {
    selectedAccount: PropTypes.string,
    enableSelectNft: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    setSelectedNft: PropTypes.func,
    setEnableInput: PropTypes.func,
};


export default YieldNFTDropdown;
```


#### [SplitNFTSectionV2.jsx](src%2Fsections%2FSplitNFTSectionV2.jsx)
```javascript
import React, {useEffect, useState} from "react";
import PropTypes from "prop-types";
import SplitNftButton from "../components/SplitNftButton.jsx";

const SplitNftSectionV2 = ({ selectedAccount, selectedNft, YieldNftRaddy, enableInput }) => {

    const [numSplits, setNumSplits] = useState(0);
    const [error, setError] = useState('');
    const [enableButton, setEnableButton] = useState(false);
    const [input, setInput] = useState(""); // Changed to use state
    const [nftLabel, setNftLabel] = useState("")

    useEffect(() => {
        if (selectedNft) {
            setNftLabel(selectedNft.label)
        }
        else {
            setNftLabel("")
        }
    }, [selectedNft]);

    const isNumeric = num => !isNaN(num);
    const isInteger = num => Number.isInteger(num);
    const isLowerThan50 = num => num <= 50;

    useEffect(() => {
        if (input !== "") {
            const val = parseFloat(input);
            if (val === '' || (isNumeric(val) && isInteger(val) && isLowerThan50(val))) {
                setNumSplits(val);
                setEnableButton(true);
                setError('');
            } else {
                setError('Please enter a integer value (Max. 50).');
                setEnableButton(false);
            }
        }
    }, [input]);

    return (
        <>


            <div className="buy-super-container">

                <div className="go-buy-super">
                    <h2>Split NFT</h2>
                </div>

                <div className="split-nft-input-container">

                    <div className="split-nft-input-first-line">


                        <p id="nft-prefix">Split NFT {nftLabel}</p>


                    </div>

                    <div className="split-nft-input-second-line">

                        <p id="split-input-prefix">into </p>
                        <input
                            type={"text"}
                            id={"split-input"}
                            value={input}
                            onChange={e => setInput(e.target.value)} // Added onChange handler
                            disabled={!enableInput}
                            placeholder="# of Splits"
                            style={{marginBottom: '0.625rem'}}
                        />
                        <p id='split-input-suffix'>
                            equivalent NFTs
                        </p>

                    </div>

                    <p> {error} </p>

                </div>

                <SplitNftButton
                    selectedAccount={selectedAccount}
                    enableButton = {enableButton}
                    YieldNftRaddy={YieldNftRaddy}
                    selectedNft={selectedNft}
                    numSplits={numSplits.toString()}
                />
            </div>
        </>
    );
};

SplitNftSectionV2.propTypes = {
    selectedAccount: PropTypes.string,
    selectedNft: PropTypes.object,
    YieldNftRaddy: PropTypes.string,
    enableInput: PropTypes.bool
};

const MemoizedSplitNftSection = React.memo(SplitNftSectionV2);

export default MemoizedSplitNftSection;
```

##### [SplitNftButton.jsx](src%2Fcomponents%2FSplitNftButton.jsx)
```javascript
import { useSendTransaction } from "../hooks/useSendTransaction.js";
import PropTypes from "prop-types";
import {useEffect, useState} from "react";
import {useComponentAddy} from "../hooks/useComponentDetails.js";
import useGetEventInReceipt from "../hooks/useGetEventInReceipt.js";
import {SendNewNftToMongo} from "../api/posts.js";
import {splitNftManifest} from "../manifests/splitNftManifest.js";


SplitNftButton.propTypes = {
    selectedAccount: PropTypes.string,
    enableButton: PropTypes.bool,
    YieldNftRaddy: PropTypes.string,
    selectedNft: PropTypes.object,
    numSplits: PropTypes.string,
};

function SplitNftButton(props) {

    const [receipt, setReceipt] = useState(null);
    const {selectedAccount, enableButton, YieldNftRaddy, selectedNft, numSplits} = props;
    const sendTransaction = useSendTransaction();
    const componentAddy = useComponentAddy();

    const handleBuySuper = async () => {

        if (!selectedAccount || !enableButton) {
            alert("Please select an account first.");
            return;
        }

        let manifest = splitNftManifest(selectedAccount, componentAddy, YieldNftRaddy, selectedNft.value, numSplits);

        console.log("manifest", manifest);

        // eslint-disable-next-line no-unused-vars
        const { TxnResult, events } = await sendTransaction(manifest);
        await setReceipt(events);
    };

    const SplitNFTEvent = useGetEventInReceipt(receipt, "SplitNFTEvent");

    useEffect(() => {
        // Check if the receipt is not null and call the function
        if (receipt) {
            if(SplitNFTEvent) {
                // Call the function when receipt is updated
                SendNewNftToMongo(SplitNFTEvent)
            }
        }
    }, [receipt, SplitNFTEvent]); // This hook will re-run whenever receipt changes

    return (
        <div>
            <button
                id="buy-super-button"
                onClick={handleBuySuper}
                disabled={!selectedAccount || !enableButton}>
                Split NFT
            </button>
        </div>
    );
}

export default SplitNftButton;
```

### [OwnerPage](src%2Fpages%2FDevPage)



### [AccountDropdown.jsx](src%2Fcomponents%2FAccountDropdown.jsx)
Opener
```javascript

```

### [BuySuper.jsx](src%2Fcomponents%2FBuySuper.jsx)
Opener
```javascript

```

### [YieldNFTDropdown.jsx](src%2Fcomponents%2FYieldNFTDropdown.jsx)
Opener
```javascript

```

### [DevModeInstruction.jsx](src%2Fcomponents%2FDevModeInstruction.jsx)
Opener
```javascript

```

### [EndCountdown.jsx](src%2Fcomponents%2FEndCountdown.jsx)
```javascript

```
### [EndSale.jsx](src%2Fcomponents%2FEndSale.jsx)
Opener
```javascript

```

### [ExchangeRatePic.jsx](src%2Fcomponents%2FExchangeRatePic.jsx)
Opener
```javascript

```

### [instantiateSuper.jsx](src%2Fcomponents%2FinstantiateSuper.jsx)
Opener
```javascript

```

### [PrimaryNavbar.jsx](src%2Fcomponents%2FPrimaryNavbar.jsx)
```javascript

```

### [SecondaryNavBar.jsx](src%2Fcomponents%2FSecondaryNavBar.jsx)
Opener
```javascript

```

### [SaleActiveStatus.jsx](src%2Fcomponents%2FSaleActiveStatus.jsx)
Opener
```javascript

```



### [SplitNftButton.jsx](src%2Fcomponents%2FSplitNftButton.jsx)
Opener
```javascript

```

### [StartSale.jsx](src%2Fcomponents%2FStartSale.jsx)
Opener
```javascript

```



</details>


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
