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
