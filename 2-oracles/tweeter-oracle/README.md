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
