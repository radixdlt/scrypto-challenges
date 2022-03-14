# TODO

* [X] handle all code TODOs
* quality
  * remove all dead code
  * convert comments to trace/debug/info logging
  * [x] rename Account to SharedAccount
  * remove dead imports/dependencies
  * reorganize file structure, of main package, and of cli package
  * run clippy
  * use scrypto_statictypes
* features
  * [x] add order deadline (from signer)
  * [x] add direction flag to support sell request (instead of just the buy request we have now) -- compare with Airswap/Swap documentation
  * handle swap NFTs (should only need CLI changes?)
* testing
  * add scenerio where the sender composes a more interesting tx, where the get the money from somewhere else first (like a flash loan) or they combine 2 RFQs to do their own routing
  * add scenerio where non default maker callback is used (and they do something interesting, like maybe swap for the needed token using an AMM)
  * [x] delete/cleanup tests/lib.rs
* documentation
  * document EVERY function
  * document every file
  * write up front README
  * [x] make sure to document how to use `cargo doc`


# Docs

Run: `cargo doc --no-deps --document-private-items --open`