# Align Blueprint package

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

This submission provides a blueprint package to create a Decentralized Autonomous Organization (DAO) on Web3. It used many advanced Decentralized Governance (DeGov) techiques to create solid alignment of interests while at the same time foster individual accountabilities and allow flexibility between trustlessness or affordable trustness.

The package consists of 5 blueprints: DAO blueprint to instantiate the DAO's core component, Community, Proposal, Treasury, LocalOracle blueprints to support interacting with the DAO.

There are also many helpful modules, contain strong typed structs to support these blueprints.

There are two extra blueprints for test purpose: TestFundraising and TestProposal blueprints.

## Quick start

1. Clone this git repository: `git clone https://github.com/radixdlt/scrypto-challenges && cd 5-DAO/Align`
2. Quick test: `./tests.sh`
3. Study the tests through logs and test each function, methods of the protocol.

**Documentation**:

1. Linux/Mac: `./doc.sh`
2. Windows: `cd scrypto && cargo doc --no-deps --document-private-items --open`

**Alphanet intergration**: By the time of the challenge's deadline, there's some issues when trying to deploy large package into Alphanet. The author intended to test this package on Alphanet after the Radix team got it fixed.

## License

This project is licensed under [Apache 2.0](https://github.com/radixdlt/scrypto-challenges/blob/main/LICENSE).
