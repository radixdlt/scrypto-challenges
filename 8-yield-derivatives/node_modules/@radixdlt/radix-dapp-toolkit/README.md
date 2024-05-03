[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

- [What is Radix dApp Toolkit?](#what-is-radix-dapp-toolkit)
  - [Resources](#resources)
    - [Building a dApp frontend](#building-a-dapp-frontend)
- [Installation](#installation)
- [Usage](#usage)
  - [Getting started](#getting-started)
  - [Login requests](#login-requests)
    - [User authentication](#user-authentication)
    - [Handle user authentication](#handle-user-authentication)
    - [User authentication management](#user-authentication-management)
  - [Wallet data requests](#wallet-data-requests)
      - [Trigger wallet data request programmatically](#trigger-wallet-data-request-programmatically)
    - [Change requested data](#change-requested-data)
    - [Data request builder](#data-request-builder)
      - [`DataRequestBuilder.persona()`](#datarequestbuilderpersona)
      - [`DataRequestBuilder.accounts()`](#datarequestbuilderaccounts)
      - [`OneTimeDataRequestBuilderItem.accounts()`](#onetimedatarequestbuilderitemaccounts)
      - [`DataRequestBuilder.personaData()`](#datarequestbuilderpersonadata)
      - [`OneTimeDataRequestBuilderItem.personaData()`](#onetimedatarequestbuilderitempersonadata)
      - [`DataRequestBuilder.config(input: DataRequestState)`](#datarequestbuilderconfiginput-datarequeststate)
    - [Handle connect responses](#handle-connect-responses)
    - [One Time Data Request](#one-time-data-request)
    - [Data Requests Sandbox](#data-requests-sandbox)
  - [State changes](#state-changes)
  - [Transaction requests](#transaction-requests)
    - [Build transaction manifest](#build-transaction-manifest)
    - [sendTransaction](#sendtransaction)
- [ROLA (Radix Off-Ledger Authentication)](#rola-radix-off-ledger-authentication)
- [√ Connect Button](#-connect-button)
  - [Styling](#styling)
    - [Themes](#themes)
    - [Modes](#modes)
    - [CSS variables](#css-variables)
    - [Compact mode](#compact-mode)
    - [Sandbox](#sandbox)
- [Setting up your dApp Definition](#setting-up-your-dapp-definition)
  - [Setting up a dApp Definition on the Radix Dashboard](#setting-up-a-dapp-definition-on-the-radix-dashboard)
- [Data storage](#data-storage)
- [Examples](#examples)
- [License](#license)

# What is Radix dApp Toolkit?

Radix dApp Toolkit (RDT) is a TypeScript library that automates getting users logged in to your dApp using a Persona, maintains a browser session for that login, and provides a local cache of data the user has given permission to your app to access associated with their Persona. It also provides an interface to request accounts and personal data from the user's wallet, either as a permission for ongoing access or as a one-time request, as well as to submit transaction manifest stubs for the user to review, sign, and submit in their wallet.

The current version only supports desktop browser webapps with requests made via the Radix Wallet Connector browser extension. It is intended to later add support for mobile browser webapps using deep linking with the same essential interface.

**RDT is composed of:**

- **√ Connect Button** – A framework agnostic web component that keeps a minimal internal state and have properties are pushed to it.

- **Tools** – Abstractions over lower level APIs for developers to build their radix dApps at lightning speed.

- **State management** – Handles wallet responses, caching and provides data to √ Connect button.

## Resources

### [Building a dApp frontend](https://docs.radixdlt.com/docs/building-a-frontend-dapp)

# Installation

**Using NPM**

```bash
npm install @radixdlt/radix-dapp-toolkit
```

**Using Yarn**

```bash
yarn add @radixdlt/radix-dapp-toolkit
```

# Usage

## Getting started

Add the `<radix-connect-button />` element in your HTML code and instantiate `RadixDappToolkit`.

```typescript
import { RadixDappToolkit, RadixNetwork } from '@radixdlt/radix-dapp-toolkit'

const rdt = RadixDappToolkit({
  dAppDefinitionAddress:
    'account_tdx_e_128uml7z6mqqqtm035t83alawc3jkvap9sxavecs35ud3ct20jxxuhl',
  networkId: RadixNetwork.RCnetV3,
  applicationName: 'Radix Web3 dApp',
  applicationVersion: '1.0.0',
})
```

**Input**

- **requires** dAppDefinitionAddress - Specifies the dApp that is interacting with the wallet. Used in dApp verification process on the wallet side. [Read more](#setting-up-your-dapp-definition)
- **requires** networkId - Target radix network ID.
- _optional_ applicationName - Your dApp name. It's only used for statistics purposes on gateway side
- _optional_ applicationVersion - Your dApp version. It's only used for statistics purposes on gateway side

## Login requests

The user's journey on your dApp always always starts with connecting their wallet and logging in with a Persona. The "Connect" button always requests a Persona login from the user's wallet.

The default behavior is to request the login alone, but you may also choose to add additional requests for account information or personal data to get at the time of login. This is useful if there is information that you know your dApp always needs to be able to function. You can also however choose to keep the login simple and make other requests later, as needed. Doing it this way allows your dApp to provide a helpful description in its UI of what a given piece of requested information is needed for, such as "please share all of your accounts that you want to use with this dApp" or "providing your email address will let us keep you informed of new features".

The Persona the user logs in with sets the context for all ongoing account and personal data requests for that session. The Radix Wallet keeps track of what permissions the user has provided for each dApp and each Persona they've used with that dApp. RDT automatically keeps track of the currently logged in Persona so that requests to the wallet are for the correct Persona.

After login, RDT also provides your dApp with a local cache of all account information and personal data that a user has given permission to share with your dApp for their chosen Persona.

For a pure frontend dApp (where you have no backend or user database), there is typically no reason for a Persona login to be verified and the login process is completely automated by RDT.

### User authentication

For a full-stack dApp there is also the user authentication flow. Typically, a full-stack dApp would request a persona together with a proof of ownership, which is then verified on the dApp backend using ROLA verification.

**What is a proof of ownership?**

A signature produced by the wallet used to verify that the wallet is in control of a persona or account.

```typescript
// Signed challenge
{
  type: 'persona' | 'account'
  challenge: string
  proof: {
    publicKey: string
    signature: string
    curve: 'curve25519' | 'secp256k1'
  }
  address: string
}
```

The signature is composed of:

|                                       |                                                                                                                                                      |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| **prefix**                            | "R" (as in ROLA) in ascii encoding                                                                                                                   |
| **challenge**                         | 32 random bytes provided by the dApp                                                                                                                 |
| **length of dApp definition address** | String length of the dApp definition address                                                                                                         |
| **dApp definition address**           | The dApp definition address of the requesting dApp                                                                                                   |
| **origin**                            | The origin of the dApp (e.g. `https://dashboard.radixdlt.com`). This is a value that is added to the wallet data request by the Connector extension. |

**Challenge**

In order to request a persona or account with proof of ownership a challenge is needed.

A challenge is a random 32 bytes hex encoded string that looks something like: `4ccb0555d6b4faad0d7f5ed40bf4e4f0665c8ba35929c638e232e09775d0fa0e`

**Why do we need a challenge?**

The challenge plays an important role in the authentication flow, namely preventing replay attacks from bad actors. The challenge ensures that an authentication request payload sent from the client can only be used once. After a challenge is claimed by a request, the subsequent requests can no longer be resolved successfully with the same payload. As a security best practice, a stored challenge should have a short expiration time. In this case, just enough time for a user to interact with the wallet.

**Request persona with proof**

In order to request a proof, it is required to provide a function to RDT that produces a challenge.

```typescript
// type requestChallengeFromDappBackendFn = () => Promise<string>

rdt.walletApi.provideChallengeGenerator(requestChallengeFromDappBackendFn)

rdt.walletApi.setRequestData(DataRequestBuilder.persona.withProof())

// handle the wallet response
rdt.walletApi.dataRequestControl(async (walletData) => {
  const personaProof = walletData.proofs.find(
    (proof) => proof.type === 'persona'
  )
  if (personaProof) await handleLogin(personaProof)
})
```

### Handle user authentication

A typical full stack dApp will require the user to provide proof of ownership. After sending a data request and getting the proof from the wallet, you need authenticate the user through ROLA on the dApp backend.

Use `walletApi.dataRequestControl` to provide a callback function that intercepts the RDT data request response flow. If no error has been thrown inside of the callback function the RDT flow will proceed as usual.

```typescript
rdt.walletApi.dataRequestControl(async (walletData) => {
  const personaProof = walletData.proofs.find(
    (proof) => proof.type === 'persona'
  )
  if (personaProof) await handleLogin(personaProof)
})
```

Throwing an error inside of `walletApi.dataRequestControl` callback will prevent RDT from getting into a logged in state. A full stack dApp may wish to do this to prevent RDT from treating the user as logged in because the ROLA authentication check failed, or for other application-specific reasons why a given user should not be allowed to login.

```typescript
rdt.walletApi.dataRequestControl(async (walletData) => {
  throw new Error('something bad happened...')
})
```

See [ROLA example](https://github.com/radixdlt/rola-examples) for an end-to-end implementation.

### User authentication management

After a successful ROLA verification it is up to the dApp's business logic to handle user authentication session in order to keep the user logged-in between requests. Although RDT is persisting state between page reloads, it is not aware of user authentication. The dApp logic needs to control the login state and sign out a user when needed.

**Expired user auth session**

If a user's auth session has expired it is recommended to logout the user in RDT as well. The dApp needs to call the `disconnect` method in order to but the user in a **not connected** state.

```typescript
rdt.disconnect()
```

The `disconnect` method resets the RDT state, to login anew, a wallet data request needs to be triggered.

## Wallet data requests

For your dApp to access data from a user's wallet, whether account information or personal data, a request must be sent to the wallet. By default, the request will be "ongoing", meaning that the user will be asked for permission to share the information whenever they login to your dApp with their current Persona. A request may also be "one time" if it is for transient use and you do not require the permission to be retained by the user's wallet.

There are two ways to trigger a data request:

1. As part of the login request when the user clicks the √ Connect button's "Connect"
2. Programmatically through the walletApi.sendRequest method

#### Trigger wallet data request programmatically

```typescript
const result = await rdt.walletApi.sendRequest()

if (result.isErr()) return handleException()

// {
//   persona?: Persona,
//   accounts: Account[],
//   personaData: WalletDataPersonaData[],
//   proofs: SignedChallenge[],
// }
const walletData = result.value
```

### Change requested data

By default, a data request requires a Persona to set its context and so if the user is not already logged in, the data request will include a request for login.

Use `walletApi.setRequestData` together with `DataRequestBuilder` to change the wallet data request.

```typescript
rdt.walletApi.setRequestData(
  DataRequestBuilder.persona().withProof(),
  DataRequestBuilder.accounts().exactly(1),
  DataRequestBuilder.personaData().fullName().emailAddresses()
)
```

### Data request builder

The `DataRequestBuilder` and `OneTimeDataRequestBuilder` is there to assist you in constructing a wallet data request.

#### `DataRequestBuilder.persona()`

```typescript
withProof: (value?: boolean) => PersonaRequestBuilder
```

Example: Request persona with proof of ownership

```typescript
rdt.walletApi.setRequestData(DataRequestBuilder.persona().withProof())
```

#### `DataRequestBuilder.accounts()`

```typescript
atLeast: (n: number) => AccountsRequestBuilder
exactly: (n: number) => AccountsRequestBuilder
withProof: (value?: boolean) => AccountsRequestBuilder
reset: (value?: boolean) => AccountsRequestBuilder
```

Example: Request at least 1 account with proof of ownership

```typescript
rdt.walletApi.setRequestData(
  DataRequestBuilder.accounts().atLeast(1).withProof()
)
```

#### `OneTimeDataRequestBuilderItem.accounts()`

```typescript
atLeast: (n: number) => OneTimeAccountsRequestBuilder
exactly: (n: number) => OneTimeAccountsRequestBuilder
withProof: (value?: boolean) => OneTimeAccountsRequestBuilder
```

Example: Exactly 2 accounts

```typescript
rdt.walletApi.sendOneTimeRequest(
  OneTimeDataRequestBuilder.accounts().exactly(2)
)
```

#### `DataRequestBuilder.personaData()`

```typescript
fullName: (value?: boolean) => PersonaDataRequestBuilder
emailAddresses: (value?: boolean) => PersonaDataRequestBuilder
phoneNumbers: (value?: boolean) => PersonaDataRequestBuilder
reset: (value?: boolean) => PersonaDataRequestBuilder
```

Example: Request full name and email address

```typescript
rdt.walletApi.setRequestData(
  DataRequestBuilder.personaData().fullName().emailAddresses()
)
```

#### `OneTimeDataRequestBuilderItem.personaData()`

```typescript
fullName: (value?: boolean) => PersonaDataRequestBuilder
emailAddresses: (value?: boolean) => PersonaDataRequestBuilder
phoneNumbers: (value?: boolean) => PersonaDataRequestBuilder
```

Example: Request phone number

```typescript
rdt.walletApi.sendOneTimeRequest(
  OneTimeDataRequestBuilder.personaData().phoneNumbers()
)
```

#### `DataRequestBuilder.config(input: DataRequestState)`

Use this method if you prefer to provide a raw data request object.

Example: Request at least 1 account and full name.

```typescript
rdt.walletApi.setRequestData(
  DataRequestBuilder.config({
    personaData: { fullName: true },
    accounts: { numberOfAccounts: { quantifier: 'atLeast', quantity: 1 } },
  })
)
```

### Handle connect responses

Add a callback function to `provideConnectResponseCallback` that emits a wallet response.

```typescript
rdt.walletApi.provideConnectResponseCallback((result) => {
  if (result.isErr()) {
    // handle connect error
  }
})
```

### One Time Data Request

One-time data requests do not have a Persona context, and so will always result in the Radix Wallet asking the user to select where to draw personal data from. The wallet response from a one time data request is meant to be discarded after usage. A typical use case would be to populate a web-form with user data.

```typescript
const result = rdt.walletApi.sendOneTimeRequest(
  OneTimeDataRequestBuilder.accounts().exactly(1),
  OneTimeDataRequestBuilder.personaData().fullName()
)

if (result.isErr()) return handleException()

// {
//   accounts: Account[],
//   personaData: WalletDataPersonaData[],
//   proofs: SignedChallenge[],
// }
const walletData = result.value
```

### Data Requests Sandbox

Play around with the different data requests in
* [Stokenet sandbox environment](https://stokenet-sandbox.radixdlt.com/)
* [Mainnet sandbox environment](https://sandbox.radixdlt.com/)

## State changes

Listen to wallet data changes by subscribing to `walletApi.walletData$`.

```typescript
const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
  // {
  //   persona?: Persona,
  //   accounts: Account[],
  //   personaData: WalletDataPersonaData[],
  //   proofs: SignedChallenge[],
  // }
  doSomethingWithAccounts(walletData.accounts)
})
```

When your dApp is done listening to state changes remember to unsubscribe in order to prevent memory leaks.

```typescript
subscription.unsubscribe()
```

Get the latest wallet data by calling `walletApi.getWalletData()`.

```typescript
// {
//   persona?: Persona,
//   accounts: Account[],
//   personaData: WalletDataPersonaData[],
//   proofs: SignedChallenge[],
// }
const walletData = rdt.walletApi.getWalletData()
```

## Transaction requests

Your dApp can send transactions to the user's Radix Wallet for them to review, sign, and submit them to the Radix Network.

Radix transactions are built using "transaction manifests", that use a simple syntax to describe desired behavior. See [documentation on transaction manifest commands here](https://docs.radixdlt.com/docs/transaction-manifest).

It is important to note that what your dApp sends to the Radix Wallet is actually a "transaction manifest stub". It is completed before submission by the Radix Wallet. For example, the Radix Wallet will automatically add a command to lock the necessary amount of network fees from one of the user's accounts. It may also add "assert" commands to the manifest according to user desires for expected returns.

**NOTE:** Information will be provided soon on a ["comforming" transaction manifest stub format](https://docs.radixdlt.com/docs/conforming-transaction-manifest-types) that ensures clear presentation and handling in the Radix Wallet.

### Build transaction manifest

We recommend using template strings for constructing simpler transaction manifests. If your dApp is sending complex manifests a manifest builder can be found in [TypeScript Radix Engine Toolkit](https://github.com/radixdlt/typescript-radix-engine-toolkit#building-manifests)

### sendTransaction

This sends the transaction manifest stub to a user's Radix Wallet, where it will be completed, presented to the user for review, signed as required, and submitted to the Radix network to be processed.

```typescript
type SendTransactionInput = {
  transactionManifest: string
  version?: number
  blobs?: string[]
  message?: string
  onTransactionId?: (transactionId: string) => void
}
```

- **requires** transactionManifest - specify the transaction manifest
- **optional** version - specify the version of the transaction manifest
- **optional** blobs - used for deploying packages
- **optional** message - message to be included in the transaction
- **optional** onTransactionId - provide a callback that emits a transaction ID

<details>

<summary>sendTransaction example</summary>

```typescript
const result = await rdt.walletApi.sendTransaction({
  transactionManifest: '...',
})

if (result.isErr()) {
  // code to handle the exception
}

const transactionIntentHash = result.value.transactionIntentHash
```

</details>

# ROLA (Radix Off-Ledger Authentication)

ROLA is method of authenticating something claimed by the user connected to your dApp with the Radix Wallet. It uses the capabilities of the Radix Network to make this possible in a way that is decentralized and flexible for the user.

ROLA is intended for use in the server backend portion of a Full Stack dApp. It runs "off-ledger" alongside backend business and user management logic, providing reliable authentication of claims of user control using "on-ledger" data from the Radix Network.

The primary use for ROLA is to authenticate the user's Persona login with the user's control of account(s) on Radix. Let's say that Alice is subscribed to an online streaming service on the Radix network called Radflix, which requires a subscription badge to enter the website. Alice logs in with her Persona to Radflix and now needs to prove that she owns an account that contains a Radflix subscription badge. By using Rola we can verify that Alice is the owner of the account that contains the Radflix subscription badge. Once we have verified that Alice is the owner of the account, we can then use the account to check for the Radflix subscription badge and verify that Alice has a valid subscription.

**Read more**

- [ROLA example](https://github.com/radixdlt/rola-examples)
- [Full-stack dApp](https://docs.radixdlt.com/docs/building-a-full-stack-dapp)

# √ Connect Button

Provides a consistent and delightful user experience between radix dApps. Although complex by itself, RDT is off-loading the developer burden of having to handle the logic of all its internal states.

Just add the HTML element in your code, and you're all set.

```html
<radix-connect-button />
```

## Styling

Configure the √ Connect Button to fit your dApp's branding.

### Themes

Available themes:

- `radix-blue` (default)
- `black`
- `white-with-outline`
- `white`

```typescript
rdt.buttonApi.setTheme('black')
```

### Modes

Available modes:

- `light` (default)
- `dark`

```typescript
rdt.buttonApi.setMode('dark')
```

### CSS variables

There are three CSS variables available:

- `--radix-connect-button-width` (default 138px)
- `--radix-connect-button-height` (default 42px)
- `--radix-connect-button-border-radius` (default 0px)

```css
body {
  --radix-connect-button-width: 200px;
  --radix-connect-button-height: 42px;
  --radix-connect-button-border-radius: 12px;
}
```

### Compact mode

Setting `--radix-connect-button-width` below `138px` will enable compact mode.

### Sandbox

Play around with the different configurations on the
[sandbox environment](https://connect-button-storybook.radixdlt.com/)

# Setting up your dApp Definition

A dApp Definition account should be created after you’ve built your dApp’s components and resources, and created a website front end for it. dApp Definition account is a special account on the Radix Network with some metadata set on it that does some nice things, like:

- Provides the necessary unique identifier (the dApp Definition’s address) that the Radix Wallet needs to let users login to your dApp and save sharing preferences for it.

- Defines things like name, description, and icon so the Radix Wallet can inform users what they are interacting with.

- Lets you link together everything associated with your dApp – like websites, resources, and components – so that the Radix Wallet knows what they all belong to.

Creating a dApp Definition for your dApp will provide the necessary information for clients like the Radix Wallet to let users interact with your dApp in a way that is easy, safe, and informative. It also acts as a hub that connects all your dApp pieces together.

You can read more about dApp Definitions [here](https://docs.radixdlt.com/docs/metadata-for-verification).

## Setting up a dApp Definition on the Radix Dashboard

1. **Create a new account in the Radix Wallet.** This is the account which we will convert to a dApp Definition account.

2. **Head to the Radix Dashboard’s Manage dApp Definitions page**. This page provides a simple interface to set the metadata on an account to make it a dApp Definition.

3. **Connect your Radix Wallet to the Dashboard** and make sure you share the account that you just created to be a dApp Definition. Select that account on the Dashboard page.

4. **Now check the box for “Set this account as a dApp Definition”, and fill in the name and description you want to use for your dApp.** Later you’ll also be able to specify an icon image, but that’s not ready just yet.

5. **Click “Update”** and an approve transaction should appear in your Radix Wallet. Done!

Provide account address as the the dApp Definition address that you just created, and it will be sent to the Radix Wallet whenever a user connects or receives a transaction from your dApp. The Wallet will then look up that dApp Definition address on the Radix Network, pull the latest metadata, and show it to the user. When a user logins to your dApp, an entry in the wallet’s preferences for your dApp will appear too. Try it out for yourself!

# Data storage

To provide a consistent user experience RDT stores data to the browser’s local storage. This will enable state rehydration and keep state between page reloads.

To understand which wallet responses that get stored we need to understand the difference between one-time and regular data requests.

One-time data requests do not register the dApp in the wallet and the connect button does not display that data in the UI. The data is meant to be used temporarily by the dApp and discarded thereafter.

A user connecting her wallet will be the first user flow in the majority of dApps. The connect flow is a bit different from subsequent data request flows. Its purpose is to provide the dApp with a minimal amount of user data in order for the user to be able to use the dApp, e.g. the minimal amount of data for a DEX dApp is an account.

RDT handles writing and reading data to the browser’s local storage so that it will be persisted between page loads. The dApp frontend logic can at any time ask RDT to provide the stored data by subscribing to the `walletApi.walletData$` observable or calling `walletApi.getWalletData`. One time data requests or requests that can not be resolved by the internal state are sent as data requests to the wallet.

# Examples

The `examples` directory contains a react dApp that consumes RDT. Its main purpose is to be used by us internally for debugging but can also serve as a source of inspiration.

# License

The Radix Dapp Toolkit binaries are licensed under the [Radix Software EULA](http://www.radixdlt.com/terms/genericEULA).

The Radix Dapp Toolkit code is released under [Apache 2.0 license](LICENSE). 

      Copyright 2023 Radix Publishing Ltd

      Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.

      You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0

      Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

      See the License for the specific language governing permissions and limitations under the License.
