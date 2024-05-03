# TweeterOracle

 This oracle stores data from the twitter API. Instantiating this component makes it possible to administer this data and make it available to those who need it within the data ledger
 For example is the radixdlt user account followed by the cyover user account? Or has a tweet been liked by cyover user account ?
 This data can be useful for automating airdrops. An example of component automating the airdrop was created to test this Oracle (AirdropWithTweeterOracle)

## Quick Start 

1. Build scrypto :  `./scrypto/build.sh`
2. Test TweeterOracle with shell: `cd  ./tweeter-oracle/scrypto/tweeter-oracle/demo && ./tweeter_oracle.sh`
3. Test TweeterOracle on web browser : `npm install && npm start` and Open http://localhost:8080 to view it in the browser.

# AirdropWithTweeterOracle 
This component allows airdrop automation. A certain number of tasks are defined by the creators of the airdrop component : Follow 1 and/or more accounts, like a tweet and/or more tweets and/retweet one or more tweets.
Users register for the airdrop via the Register method by specifying their tweeter account and receive in return a non-fungible token to claim the amount of the airdrop when possible.
At the stage of finalizing the airdrop method (finalize_airdrop) the Tweeter_oracle component is used to verify that all tasks have been executed by subscribers.

## Quick Start 
1. tests AirdropWithTweeterOracle with shell: `cd  ./tweeter-oracle/scrypto/tweeter-oracle/demo && ./airdrop_with_tweeter_oracle.sh`


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