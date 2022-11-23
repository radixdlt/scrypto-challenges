use scrypto::prelude::*;

// This is the marketplace component where people list, buy and sell NattyDAO NFTs

// Omar has NFT marketplace boilerplate to use.
// https://github.com/radixdlt/scrypto-examples/tree/main/nft/nft-marketplace
// Requires a method that accepts NFTs (because mint component has giver method, this has a receiver method.)

// Vault in market component accepts this type of NFT
// Need a Vault to store the badges ..
// When you instantiate the marketplace compontent, could have a BTreeMap / HashMap of
// Option Vaults ... or a Vector of Option Vaults (because marketplace insantiated first, need a vault where the NFTs are collected, but its empty.)