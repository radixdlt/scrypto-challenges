![](./web/public/DeMiFi logo.png)

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
