![](./Radfolio%20logo.png)

# Radfolio - An automated portfolio management system
This project provides a blueprint that will take funds from investors
and distribute them across any number of preconfigured investment
vehicles.

It consists of a main blueprint, the Radfolio itself, as well as a
support blueprint (InvestmentVehicle) which defines how investment
opportunities must be packaged for Radfolio to be able to interact
with them.

There is also a third blueprint, InterestBearingMock, used in running
the test suite.

## How to build the blueprints
Make sure you have the necessary toolchain installed, including the
resim tool, see
[here](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html)
for details. You will need Scrypto 0.4.1.
- From the command line, in the `radfolio` directory, run `cargo build`

### How to run the test suite
- Make sure you have the resim tool installed, see above.
- From the command line, in the `radfolio` directory, run `cargo test --
  --test-threads=1`

### How to generate the documentation
- From the command line, in the `radfolio` directory, run `cargo doc`

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

