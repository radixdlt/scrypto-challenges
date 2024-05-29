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

