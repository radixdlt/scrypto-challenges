[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

The √ Connect Button is a framework agnostic web component to help developers connect users and their Radix Wallet to their dApps.

It appears as a consistent, Radix-branded UI element that helps users identify your dApp website as a Radix dApp. When used with [Radix dApp Toolkit](https://github.com/radixdlt/radix-dapp-toolkit) it is compatible with the Radix Wallet – and it automatically provides a consistent user experience for users to connect with their wallet and see the current status of the connection between dApp and Radix Wallet.

- [Installation](#installation)
- [Usage](#usage)
  - [Getting started](#getting-started)
    - [Properties](#properties)
    - [Events](#events)
- [License](#license)

# Installation

**Using NPM**

```bash
npm install @radixdlt/connect-button
```

**Using Yarn**

```bash
yarn add @radixdlt/connect-button
```

# Usage

## Getting started

Add the `<radix-connect-button />` element in your HTML code and call the configuration function.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  </head>
  <body>
    <radix-connect-button />
  </body>
</html>
```

### Properties

```typescript
const radixConnectButton = document.querySelector('radix-connect-button')!

const handleConnect = () => {
  radixConnectButton.status = 'pending'
}

radixConnectButton.addEventListener('onConnect', handleConnect)
```

```typescript
type ConnectButtonProperties = {
  theme: RadixButtonTheme
  dAppName: string
  personaLabel: string
  connected: boolean
  status: RadixButtonStatus
  loggedInTimestamp: string
  showPopover: boolean
  requestItems: RequestItem[]
  accounts: Account[]
  personaData: PersonaData[]
  isMobile: boolean
  isWalletLinked: boolean
  isExtensionAvailable: boolean
  fullWidth: boolean
  activeTab: 'sharing' | 'requests'
  mode: RadixButtonMode
  avatarUrl: string
}
```

- theme - defaults to `radix-blue`, other values are `black`, `white` and `white-with-outline`
- dAppName - name of the dApp
- personaLabel - label of the connected persona
- connected - set connected state
- status - set current button status, can be one of `default`, `error`, `pending`, `success`
- loggedInTimestamp - timestamp of login
- showPopover - display connect button popover
- requestItems - displays a list of maximum 3 request items in the popover
- accounts - displays a list of connected accounts
- personaData - list of persona fields together with values
- isMobile - display mobile placeholder
- isWalletLinked - marks card during onboarding process as completed
- isExtensionAvailable - marks card during onboarding process as completed
- fullWidth - makes the button full width in its container
- activeTab - changes active tab inside popover
- mode - set styling of a popover

### Events

```typescript
type ConnectButtonEvents = {
  onConnect: () => void
  onDisconnect: () => void
  onCancelRequestItem: (event: CustomEvent<{ id: string }>) => void
  onDestroy: () => void
  onShowPopover: () => void
  onUpdateSharedData: () => void
}
```

- onConnect - triggers when user clicks connect now button
- onDisconnect - triggers when user clicks disconnect wallet button
- onCancelRequestItem - triggers when user cancels connect request
- onDestroy - triggers when button is removed from the DOM. Useful for cleaning up registered event listeners and subscriptions.
- onShowPopover - triggers when users clicks on radix button and popover is being shown
- onUpdateSharedData - triggers when users clicks on "Update Shared Data" button

# License

The √ Connect Button binaries are licensed under the [Radix Software EULA](http://www.radixdlt.com/terms/genericEULA)

The √ Connect Button code is released under [Apache 2.0 license](LICENSE). 

      Copyright 2023 Radix Publishing Ltd

      Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.

      You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0

      Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

      See the License for the specific language governing permissions and limitations under the License.
