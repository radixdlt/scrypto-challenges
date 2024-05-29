# Align Blueprint package

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

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

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