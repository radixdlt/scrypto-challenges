# Cup Perps PoC
The frontend is using adapted code from [Gumball Machine Scrypto example](https://github.com/radixdlt/scrypto-examples/tree/main/full-stack/dapp-toolkit-gumball-machine) using the [Radix dApp Toolkit](https://github.com/radixdlt/radix-dapp-toolkit#readme)

The scrypto code is original and the technical documentation is provided in the [Technical Documentation](CupPerps.pdf)

Rust-generated documentation is available after running `cargo doc` from the `scrypto` directory

## Pre-requisites
1. Node >= 12.17.0
2. The Betanet wallet & Radix-connector browser extenstion installed. Instructions [here](https://docs-babylon.radixdlt.com/main/getting-started-developers/wallet-and-connector.html)
3. Scrypto v0.8.0. Instructions to install [here](https://docs-babylon.radixdlt.com/main/getting-started-developers/first-component/install-scrypto.html) and update [here](https://docs-babylon.radixdlt.com/main/getting-started-developers/first-component/updating-scrypto.html)

## Building the Scrypto code
1. Enter the scrypto directory in a terminal: `cd scrypto`
1. Build the code: `scrypto build`
1. Two important files (`gumball_machine.abi` and `gumball_machine.wasm`) will be generated in `scrypto/target/wasm32-unknown-unknown/release/`. You will need them for the next step.

## Deploy the package to Betanet (TODO REWRITE, not needed)
1. Go to the [Betanet Dashboard Website](https://betanet-dashboard.radixdlt.com/)
2. Connect the Wallet Via the Connect Button
3. Navigate to Deploy Package & choose an account and badge or have one created for you if you don't have one yet using the link below. (Which appears once you have selected an account)
4. Upload both `cup_perps.abi` and `cup_perps.wasm`
5. Click on "publish package"
6. The wallet should open up and ask you to approve the transaction
7. On the wallet click on "sign transaction"
8. The deployed package address should get displayed. **You will need it for the next step**.

## Interacting with our package (TODO REWRITE)
1. In a terminal go back to the root of this project (dapp-toolkit-gumball-machine)
2. Install the npm dependencies: `npm install`
3. Start the local server with `npm start`
4. Open up your browser at the provided url if it doesn't open automatically.
5. Make sure you created an account on the wallet and added funds via the faucet by clicking on account name and then the three dots a button to get XRD from faucet should open.
6. Click on the connect button to fetch your wallet address. You should see your address appearing 
7. Fill the package address you got in the previous section and enter a symbol name for your gumball to display in the wallet then click on "instantiate gumball machine"
8. Your wallet will again open up. Click on "sign transaction". You should now see the instantiated component address and Gumball resource address on the page.
9. Buy a gumball by clicking on "10 XRD Long"
10. Your wallet will open up. Click on "sign transaction". The transaction receipt will get displayed on the page.
11. Check the number of Long LP tokens you have by clicking on the account name in your wallet and viewing the tokens tab.