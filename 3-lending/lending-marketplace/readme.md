# Goodwill Lending protocol ❤️

⚠️This project is **incomplete!** no webapp have been deployed , and the blueprint logic is in early stage⚠️

## Introduction 😀

Goodwill Lending protocol is a zero interest lending protocol, At its core, it uses NFTs as collateral.

the protocol works with a predetermined set of NFT collections that are part of the GoodWill program, Holder of those NFTs can borrow a set amount of predetermined token in exchange for their NFT  for a given period of time . If the borrower doesn’t repays before the due date, his assets are sold in a dutch auction.


## How it works 🧭

we have three blueprint : 
`bootstrap.rs` : contains a blueprint which creates a number of test NFTs for us which we will need to use for the purposes of testing.

`Dutch_auctions.rs` : contains the implementation of the dutch auction

`LendingMarketPlaceV2.rs` : contains  a very basic implantation of the lending protocol . 
