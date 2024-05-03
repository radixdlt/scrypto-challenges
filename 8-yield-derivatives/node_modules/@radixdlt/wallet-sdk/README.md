[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

This is a TypeScript developer SDK that facilitates communication with the Radix Wallet for two purposes: **requesting various forms of data from the wallet** and **sending transactions to the wallet**.

**Important Note:** This is an early release for development on the Radix Betanet and the Radix Wallet developer preview. This readme describes the intended full interface for the Radix mainnet release, but many features are not yet available (and are flagged as such).

The current version only supports desktop browser webapps with requests made via the Radix Wallet Connector browser extension. It is intended to later add support for mobile browser webapps using deep linking with the same essential interface.

You may wish to consider using this with [dApp toolkit](https://github.com/radixdlt/radix-dapp-toolkit), which works with this SDK to provide additional features for your application and users.

- [Installation](#installation)
  - [Getting started](#getting-started)
- [⬇️ Getting Wallet Data](#️-getting-wallet-data)
  - [💶 Accounts](#-accounts)
    - [Request](#request)
    - [Response](#response)
  - [ℹ️ Persona Data](#ℹ️-persona-data)
    - [Request](#request-1)
    - [Response](#response-1)
  - [🗑️ Reset](#️-reset)
    - [Request](#request-2)
    - [Response](#response-2)
  - [🛂 Auth](#-auth)
    - [Request](#request-3)
    - [Response](#response-3)
- [💸 Send transaction](#-send-transaction)
  - [Build transaction manifest](#build-transaction-manifest)
  - [sendTransaction](#sendtransaction)
  - [Errors](#errors)
- [License](#license)

# Installation

**Using NPM**

```bash
npm install @radixdlt/wallet-sdk
```

**Using Yarn**

```bash
yarn add @radixdlt/wallet-sdk
```

## Getting started

```typescript
import { WalletSdk } from '@radixdlt/wallet-sdk'

const walletSdk = WalletSdk({
  networkId: 12,
  dAppDefinitionAddress:
    'account_tdx_c_1p8j5r3umpgdwpedqssn0mwnwj9tv7ae7wfzjd9srwh5q9stufq',
})
```

```typescript
type Metadata = {
  networkId: number
  dAppDefinitionAddress: string
}
```

| Network  | ID  |
| :------- | :-: |
| Mainnet  |  1  |
| RCNet-V1 | 12  |

- **requires** networkId - Specifies which network to use
- **requires** dAppDefinitionAddress - Specifies the dApp that is interacting with the wallet. Used in dApp verification process on the wallet side.

# ⬇️ Getting Wallet Data

**About oneTime VS ongoing requests**

There are two types of data requests: `oneTime` and `ongoing`.

**OneTime** data requests will **always** result in the Radix Wallet asking for the user's permission to share the data with the dApp.

```typescript
type WalletUnauthorizedRequestItems = {
  discriminator: 'unauthorizedRequest'
  oneTimeAccounts?: AccountsRequestItem
  oneTimePersonaData?: PersonaDataRequestItem
}
```

**Ongoing** data requests will only result in the Radix Wallet asking for the user's permission the first time. If accepted, the Radix Wallet will automatically respond to future data requests of this type with the current data. The user's permissions for ongoing data sharing with a given dApp can be managed or revoked by the user at any time in the Radix Wallet.

```typescript
type WalletAuthorizedRequestItems = {
  discriminator: 'authorizedRequest'
  auth: AuthRequestItem
  reset?: ResetRequestItem
  oneTimeAccounts?: AccountsRequestItem
  oneTimePersonaData?: PersonaDataRequestItem
  ongoingAccounts?: AccountsRequestItem
  ongoingPersonaData?: PersonaDataRequestItem
}
```

The user's ongoing data sharing permissions are associated with a given Persona (similar to a login) in the Radix Wallet. This means that in order to request `ongoing` data, a `identityAddress` must be included.

Typically the dApp should begin with a `login` request which will return the `identityAddress` for the user's chosen Persona, which can be used for further requests (perhaps while the user has a valid session)

## 💶 Accounts

This request type is for getting one or more Radix accounts managed by the user's Radix Wallet app. You may specify the number of accounts desired, and if you require proof of ownership of the account.

### Request

```typescript
type NumberOfValues = {
  quantifier: 'exactly' | 'atLeast'
  quantity: number
}
```

```typescript
type AccountsRequestItem = {
  challenge?: Challenge
  numberOfAccounts: NumberOfValues
}
```

### Response

```typescript
type Account = {
  address: string
  label: string
  appearanceId: number
}
```

```typescript
type AccountProof = {
  accountAddress: string
  proof: Proof
}
```

```typescript
type Proof = {
  publicKey: string
  signature: string
  curve: 'curve25519' | 'secp256k1'
}
```

```typescript
type AccountsRequestResponseItem = {
  accounts: Account[]
  challenge?: Challenge
  proofs?: AccountProof[]
}
```

<details>

<summary>ongoingAccounts example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: { discriminator: 'loginWithoutChallenge' },
  ongoingAccounts: {
    numberOfAccounts: { quantifier: 'atLeast', quantity: 1 },
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: loginWithoutChallenge,
//     persona: Persona
//   },
//   ongoingAccounts: {
//     accounts: Account[]
//   }
// }
const value = result.value
```

</details>

<details>

<summary>oneTimeAccounts example</summary>

```typescript
const result = await walletSdk.request({
  discriminator: 'unauthorizedRequest',
  oneTimeAccounts: { numberOfAccounts: { quantifier: 'atLeast', quantity: 1 } },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "unauthorizedRequest",
//   oneTimeAccounts: {
//     accounts: Account[]
//   }
// }
const value = result.value
```

</details>

<details>

<summary>with proof of ownership example</summary>

```typescript
// hex encoded 32 random bytes
const challenge = [...crypto.getRandomValues(new Uint8Array(32))]
  .map((item) => item.toString(16).padStart(2, '0'))
  .join('')

const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: { discriminator: 'loginWithoutChallenge' },
  ongoingAccounts: {
    challenge,
    numberOfAccounts: { quantifier: 'atLeast', quantity: 1 },
  },
  oneTimeAccounts: {
    challenge,
    numberOfAccounts: { quantifier: 'atLeast', quantity: 1 },
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: loginWithoutChallenge,
//     persona: Persona
//   },
//   ongoingAccounts: {
//     accounts: Account[],
//     challenge,
//     proofs: AccountProof[]
//   },
//   oneTimeAccounts: {
//     accounts: Account[],
//     challenge,
//     proofs: AccountProof[]
//   }
// }
const value = result.value
```

</details>

## ℹ️ Persona Data

This request type is for a list of personal data fields associated with the user's selected Persona.

### Request

```typescript
type PersonaDataRequestItem = {
  isRequestingName?: boolean()
  numberOfRequestedEmailAddresses?: NumberOfValues
  numberOfRequestedPhoneNumbers?: NumberOfValues
}
```

### Response

```typescript
type PersonaDataRequestResponseItem = {
  name?: {
    variant: 'eastern' | 'western'
    family: string
    given: string
  }
  emailAddresses?: NumberOfValues
  phoneNumbers?: NumberOfValues
}
```

<details>

<summary>ongoingPersonaData example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: { discriminator: 'loginWithoutChallenge' },
  ongoingPersonaData: {
    isRequestingName: true,
    numberOfRequestedEmailAddresses: { quantifier: 'atLeast', quantity: 1 },
    numberOfRequestedPhoneNumbers: { quantifier: 'exactly', quantity: 1 },
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: loginWithoutChallenge,
//     persona: Persona
//   },
//   ongoingPersonaData: {
//     name: {
//       variant: 'western',
//       given: 'John',
//       family: 'Conner'
//     },
//     emailAddresses: ['jc@resistance.ai'],
//     phoneNumbers: ['123123123']
//   }
// }

const value = result.value
```

</details>

<details>

<summary>oneTimePersonaData example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'unauthorizedRequest',
  oneTimePersonaData: {
    isRequestingName: true,
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "unauthorizedRequest",
//   oneTimePersonaData: {
//     name: {
//       variant: 'eastern',
//       given: 'Jet',
//       family: 'Li'
//     }
//   }
// }

const value = result.value
```

</details>

## 🗑️ Reset

You can send a reset request to ask the user to provide new values for ongoing accounts and/or persona data.

### Request

```typescript
type ResetRequestItem = {
  accounts: boolean
  personaData: boolean
}
```

### Response

A Reset request has no response.

<details>

<summary>reset example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: { discriminator: 'loginWithoutChallenge' },
  reset: { accounts: true, personaData: true },
})

if (result.isErr()) {
  // code to handle the exception
}
```

</details>

## 🛂 Auth

Sometimes your dApp may want a more personalized, consistent user experience and the Radix Wallet is able to login users with a Persona.

For a pure frontend dApp without any server backend, you may simply want to request such a login from the users's wallet so that the wallet keeps track of data sharing preferences for your dApp and they don't have to re-select that data each time they connect.

If your dApp does have a server backend and you are keeping track of users to personalize their experience, a Persona-based login provides strong proof of user identity, and the ID returned from the wallet provides a unique index for that user.

Once your dApp has a given `identityAddress`, it may be used for future requests for data that the user has given "ongoing" permission to share.

```typescript
type Persona = {
  identityAddress: string
  label: string
}
```

**Login**

This request type results in the Radix Wallet asking the user to select a Persona to login to this dApp (or suggest one already used in the past there), and providing cryptographic proof of control.

```typescript
// Hex encoded 32 random bytes
type Challenge = string
```

This proof comes in the form of a signed "challenge" against an on-ledger Identity component. For each Persona a user creates in the Radix Wallet, the wallet automatically creates an associated on-ledger Identity (which contains none of the personal data held in the wallet). This Identity includes a public key in its metadata, and the signature on the challenge uses the corresponding private key. ROLA (Radix Off-Ledger Authentication) may be used in your dApp backend to check if the login challenge is correct against on-ledger state.

```typescript
type Proof = {
  publicKey: string
  signature: string
  curve: 'curve25519' | 'secp256k1'
}
```

The on-ledger address of this Identity will be the `identityAddress` used to identify that user – in future queries, or perhaps in your dApp's own user database.

If you are building a pure frontend dApp where the login is for pure user convenience, you may safely ignore the challenge and simply keep track of the `identityAddress` in the user's session for use in data requests that require it.

**usePersona**

If you have already identified the user via a login (perhaps for a given active session), you may specify a `identityAddress` directly without requesting a login from the wallet.

<details>

<summary>login example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: { discriminator: 'loginWithoutChallenge' },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: 'loginWithoutChallenge',
//     persona: Persona
//   },
// }
const value = result.value
```

</details>
<details>

<summary>login with challenge example</summary>

```typescript
// hex encoded 32 random bytes
const challenge = [...crypto.getRandomValues(new Uint8Array(32))]
  .map((item) => item.toString(16).padStart(2, '0'))
  .join('')

const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: {
    discriminator: 'loginWithChallenge',
    challenge,
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: 'loginWithChallenge',
//     persona: Persona,
//     challenge: Challenge,
//     proof: Proof
//   },
// }
const value = result.value
```

</details>
<details>

<summary>usePersona example</summary>

```typescript
const result = await sdk.request({
  discriminator: 'authorizedRequest',
  auth: {
    discriminator: 'usePersona',
    identityAddress:
      'identity_tdx_c_1p35qpky5sczp5t4qkhzecz3nm8tcvy4mz4997mqtuzlsvfvrwm',
  },
})

if (result.isErr()) {
  // code to handle the exception
}

// {
//   discriminator: "authorizedRequest",
//   auth: {
//     discriminator: usePersona,
//     persona: Persona
//   },
// }
const value = result.value
```

</details>

### Request

```typescript
type AuthRequestItem = AuthUsePersonaRequestItem | AuthLoginRequestItem
```

```typescript
type AuthUsePersonaRequestItem = {
  discriminator: 'usePersona'
  identityAddress: string
}
```

```typescript
type AuthLoginRequestItem =
  | AuthLoginWithoutChallengeRequestItem
  | AuthLoginWithChallengeRequestItem
```

```typescript
type AuthLoginWithoutChallengeRequestItem = {
  discriminator: 'loginWithoutChallenge'
}
```

```typescript
type AuthLoginWithChallengeRequestItem = {
  discriminator: 'loginWithChallenge'
  challenge: Challenge
}
```

### Response

```typescript
type AuthRequestResponseItem =
  | AuthUsePersonaRequestResponseItem
  | AuthLoginRequestResponseItem
```

```typescript
type AuthUsePersonaRequestResponseItem = {
  discriminator: 'usePersona'
  persona: Persona
}
```

```typescript
type AuthLoginRequestResponseItem =
  | AuthLoginWithoutChallengeResponseRequestItem
  | AuthLoginWithChallengeRequestResponseItem
```

```typescript
type AuthLoginWithoutChallengeRequestResponseItem = {
  discriminator: 'loginWithoutChallenge'
  persona: Persona
}
```

```typescript
type AuthLoginWithChallengeRequestResponseItem = {
  discriminator: 'loginWithChallenge'
  persona: Persona
  challenge: Challenge
  proof: Proof
}
```

# 💸 Send transaction

Your dApp can send transactions to the user's Radix Wallet for them to review, sign, and submit them to the Radix Network.

Radix transactions are built using "transaction manifests", that use a simple syntax to describe desired behavior. See [documentation on transaction manifest commands here](https://docs-babylon.radixdlt.com/main/scrypto/transaction-manifest/intro.html).

It is important to note that what your dApp sends to the Radix Wallet is actually a "transaction manifest stub". It is completed before submission by the Radix Wallet. For example, the Radix Wallet will automatically add a command to lock the necessary amount of network fees from one of the user's accounts. It may also add "assert" commands to the manifest according to user desires for expected returns.

**NOTE:** Information will be provided soon on a ["comforming" transaction manifest stub format](https://docs-babylon.radixdlt.com/main/standards/comforming-transactions.html) that ensures clear presentation and handling in the Radix Wallet.

## Build transaction manifest

We recommend using template strings for constructing simpler transaction manifests. If your dApp is sending complex manifests a manifest builder can be found in [TypeScript Radix Engine Toolkit](https://github.com/radixdlt/typescript-radix-engine-toolkit#building-manifests)

## sendTransaction

This sends the transaction manifest stub to a user's Radix Wallet, where it will be completed, presented to the user for review, signed as required, and submitted to the Radix network to be processed.

```typescript
type SendTransactionInput = {
  transactionManifest: string
  version: number
  blobs?: string[]
  message?: string
}
```

- **requires** transactionManifest - specify the transaction manifest
- **requires** version - specify the version of the transaction manifest
- **optional** blobs - used for deploying packages
- **optional** message - message to be included in the transaction

<details>

<summary>sendTransaction example</summary>

```typescript
const result = await sdk.sendTransaction({
  version: 1,
  transactionManifest: '...',
})

if (result.isErr()) {
  // code to handle the exception
}

const transactionIntentHash = result.value.transactionIntentHash
```

</details>

## Errors

| Error type                                       | Description                                                                                                                                                        | Message                                                                                                                                     |
| :----------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| rejectedByUser                                   | User has rejected the request in the wallet                                                                                                                        |                                                                                                                                             |
| missingExtension                                 | Connector extension is not detected                                                                                                                                |                                                                                                                                             |
| canceledByUser                                   | User has canceled the request                                                                                                                                      |                                                                                                                                             |
| walletRequestValidation                          | SDK has constructed an invalid request                                                                                                                             |                                                                                                                                             |
| walletResponseValidation                         | Wallet sent an invalid response                                                                                                                                    |                                                                                                                                             |
| wrongNetwork                                     | Wallet is currently using a network with a network ID that does not match the one specified in request from Dapp (inside metadata)                                 | "Wallet is using network ID: \(currentNetworkID), request sent specified network ID: \(requestFromP2P.requestFromDapp.metadata.networkId)." |
| failedToPrepareTransaction                       | Failed to get Epoch for Transaction Header                                                                                                                         |                                                                                                                                             |
| failedToCompileTransaction                       | Failed to compile TransactionIntent or any other later form to SBOR using EngineToolkit                                                                            |                                                                                                                                             |
| failedToSignTransaction                          | Failed to sign any form of the transaction either with keys for accounts or with notary key, or failed to convert the signature to by EngineToolkit require format |                                                                                                                                             |
| failedToSubmitTransaction                        | App failed to submit the transaction to Gateway for some reason                                                                                                    |                                                                                                                                             |
| failedToPollSubmittedTransaction                 | App managed to submit transaction but got error while polling it                                                                                                   | "TXID: <TXID_STRING>"                                                                                                                       |
| submittedTransactionWasDuplicate                 | App submitted a transaction and got informed by Gateway it was duplicated                                                                                          | "TXID: <TXID_STRING>"                                                                                                                       |
| submittedTransactionHasFailedTransactionStatus   | App submitted a transaction to Gateway and polled transaction status telling app it was a failed transaction                                                       | "TXID: <TXID_STRING>"                                                                                                                       |
| submittedTransactionHasRejectedTransactionStatus | App submitted a transaction to Gateway and polled transaction status telling app it was a rejected transaction                                                     | "TXID: <TXID_STRING>"                                                                                                                       |

# License

The Wallet SDK code is released under [Apache 2.0 license](LICENSE). Binaries are licensed under the [Radix Software EULA](http://www.radixdlt.com/terms/genericEULA)