![](./archdao_black.jpg)

# ArchDAO - A proposal DAO

This project aims to manage the votes of a group of people who wish to express their choices with respect to a series of projects that are already partially funded by the founders of the DAO and who therefore wish to be helped in defining the priorities of each individual project.

ArchDAO implements a crypto dao that serves for decentralized governance of business proposal from the initial stage to the approval and the execution stage

Proposers receive fungible vote tokens in return for their registering, with vote tokens representing shares of decision of the
whole proposal approval process. 
Right to vote can be transferred and whoever holds them can at any time vote for a proposal. 

Proposers can also fund the ArchDao with XRD tokens and receive back ARCH tokens, as the projects funded by ArchDAO makes profits the value of each ARCH token will increase.

The ArchDAO's managers can dynamically add and remove proposal, or ask to verify if the proposal trigger is satisfied for promoting a proposal into a into a project ready to be funded.

The ArchDAO can charge protocol fees, intended at producing profits for the ArchDAO's managers. Some or all of these protocol fees can go
towards rewards for both the managers and the voters.

Both the managers and the voters can earn rewards when the project gets executed and when the execution produces a profit.

## Example

For this kind of project there are many use cases, since the cases can be innumerable then let's try to describe a specific use case for Radix.

Let's say that during one of the appointments defined 'RoundTable' it is decided that an extra funding of 1,000,000xrd should be divided between the departments, so each one will propose his idea which will then be voted through the DAO to obtain funding for each project that is proportional the result of the vote obtained.

## How to build the blueprints

Make sure you have the necessary toolchain installed, including the
resim tool, see
[here](https://docs.radixdlt.com/main/scrypto/getting-started/install-scrypto.html)
for details. You will need Scrypto 0.6.0.
- From the command line, in the `archdao` directory, run `cargo build`

### How to run the test suite

- Make sure you have the resim tool installed, see above.
- From the command line, in the `archdao` directory, run `cargo test -- --test-threads=1 --nocapture`

### How to generate the documentation

- From the command line, in the `archdao` directory, run `cargo doc`

The generated web pages contain detailed documentation on how the blueprints work.

### Useful command
scrypto test -- --nocapture
cargo test -- --test-threads=1 --nocapture


