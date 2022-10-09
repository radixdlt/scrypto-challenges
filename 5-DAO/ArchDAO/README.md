![](./ArchDAO.png)

# ArchDAO - A proposal DAO
This project provides a blueprint that will take votes from proposers 
for approving proposal proposed by the DAO funders for being help in their decision.

It consists of a main blueprint, the ArchDAO itself, as well as a
support blueprint (Proposal) which contains... TODO

There is also a third blueprint, ProposalMock, used in running
the test suite.

## How to build the blueprints
Make sure you have the necessary toolchain installed, including the
resim tool, see
[here](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html)
for details. You will need Scrypto 0.4.1.
- From the command line, in the `archdao` directory, run `cargo build`

### How to run the test suite
- Make sure you have the resim tool installed, see above.
- From the command line, in the `archdao` directory, run `cargo test -- --test-threads=1 --nocapture`

### How to generate the documentation
- From the command line, in the `archdao` directory, run `cargo doc`

The generated web pages contain detailed documentation on how the
blueprints work.

### Useful command
scrypto test -- --nocapture
cargo test -- --test-threads=1 --nocapture


