![](./web/public/DeMiFi%20logo.png)

# DeMiFi - Decentralized Micro-Finance
This project provides a set of blueprints which when used together
will allow you to offer micro-finance services on the ledger.

It consists of three different blueprints working together, with one
blueprint offering identity services for the participants and the two
other handling loan requests and loan management.

## How to build and test the web front-end
You need to have npm installed on your system. You do not need to
build the blueprints themselves to build and run the front-end.
- From the command line, in the `demifi/web` directory, run `npm
  install && npm run dev`
- Connect your web browser to the URL it shows you

## How to build the blueprints
Make sure you have the necessary toolchain installed, including the
resim tool, see
[here](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html)
for details.
- From the command line, in the `demifi` directory, run `cargo build`

### How to run the test suite
- Make sure you have the resim tool installed, see above.
- From the command line, in the `demifi` directory, run `cargo test --
  --test-threads=1`

### How to generate the documentation
- From the command line, in the `demifi` directory, run `cargo doc`

The generated web pages contain detailed documentation on how the
blueprints work.


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

