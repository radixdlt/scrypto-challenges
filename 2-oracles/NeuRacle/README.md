# NeuRacle: Decentralized, Trustless Oracle solution

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

NeuRacle is a Decentralized, Trustless Oracle built on Radix Ledger that provide decentralized, trustless data validation service to bring-in off-chain data.

![](./src/logo.svg)

## Oracle Trilemma

Most traditional Oracle recent day come to the same [problems](https://encyclopedia.pub/entry/2959), that have to either compromised on decentralization (using trusted identities to bring data on-chain, eg: [ChainLink](https://james-sangalli.medium.com/why-chainlink-is-not-the-oracle-of-the-future-8bb859a81947#:~:text=ChainLink%20does%20not%20have%20a,centralised%20verification%20and%20dispute%20resolution.)), finality (optimistic oracle that use bets to bring data on-chain, eg: [UMA](https://umaproject.org/products/optimistic-oracle)), or security. It's almost the same as the [blockchain trilemma](https://www.ledger.com/academy/what-is-the-blockchain-trilemma).

## From Oracle to Distributed Ledger Technology

Because the Oracle trilemma is almost the same as blockchain trilemma, [choose a blockchain solution as an oracle solution](https://medium.com/@jameslee777/decentralized-trustless-oracles-dto-by-piggybacking-on-timestamp-consensus-rules-2adce34d67b6) will be an innovated approach. There already some oracles that are using this approach to challenge the Oracle Trilemma, eg: [Komodo Trustless Oracles](https://komodoplatform.com/en/blog/the-promise-of-smart-contracts-and-the-oracle-problem/).

Though, blockchain can't solve it's own trilemma.

[Cerberus Concensus Model](https://assets.website-files.com/6053f7fca5bf627283b582c2/608811e3f5d21f235392fee1_Cerberus-Whitepaper-v1.01.pdf) is a DLT concensus model that (on theory and testnet) solved all these trilemma and maintain atomic composability at the same time. While inspired by Komodo to use Consensus Models for validating data off-chain and bring on-chain, NeuRacle will further advance the innovation in oracle space by building on the Radix Network and utilize Cerberus Concensus Model.

## NeuRacle Solution

As an utilization of Cerberus Concensus Model, NeuRacle will have some similar design, for short:

Data Providers ~ [Validator Nodes](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-ii).

NeuRacle Ecosystem ~ [Shardspace](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-ii).

Lively data from an online source ~ [Shard](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-iv).

1 specific data in a particular time ~ [Transaction](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-iii).

Validated Data ~ [Reaching Consensus](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-v).

Sybil Resistance by PoS = [Sybil Resistance by PoS](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-vii).

Users = Users.

Components = Components.

## Quick Start

***For a simple showcase**, this prototype will be un-sharded, that mean each validators will validate all datas at the same time (Not divided into validator sets to bring more scalability or divided into data sources to bring more security). Datas will also be validated (Reaching Consensus) in 1 round of voting. The prototype also won't use **NeuRacle Storage** or **User Incentive Program** (more detail on **System Explaination**)*

*For windows user:* if you has git installed with [git bash](https://www.stanleyulili.com/git/how-to-install-git-bash-on-windows/) and [VSCode](https://code.visualstudio.com/), you should be able to run .sh file through git bash

![](gitbash.PNG)

Clone this git repository: `git clone https://github.com/radixdlt/scrypto-challenges && cd 2-oracles/NeuRacle`

### Backend quick testing

1. Build the package: `cd scrypto && scrypto build`
2. Quick end to end testing: `cd test && ./end_to_end.sh`
3. Check the test explained: [./README.md](./scrypto/test/README.md)

### Frontend testing (Incompleted)

*This frontend is bootstraped with Vite and React.*
*For now, the prototype can only be tested on https://pte01.radixdlt.com/ sever*

1. Ensure all dependent package are installed: `npm install`
2. Testing the local environment: `npm run dev`
3. Go to the variable environment file, delete all current variable and enter your wallet address on "TESTER=''": `./src/NEURACLE.tsx`
4. Back on the page, try publish package and become NeuRacle Admin, follow any instruction that prompt up. (After publish the package you should delete it immediately or it would cause lagging) 
5. Now you have become NeuRacle Admin, you can assign another address as validator, or change into other account, become user, try the UI and staking mechanism.
6. You can also try become an user with this api: http://worldclockapi.com/api/json/est/now

*Note: Current version of NeuRacle UI doesn't support multiple role at one account address, you should try other role in other account instead. You can send NAR token to other account via [pouch](https://plymth.github.io/pouch/).*

## System Explaination

This prototype is made with most dynamicity so anyone can become NeuRacle service provider and create an "oracle value" for their token, even XRD.

Projects using PoS Consensus can also make use of the [Validator](./scrypto/src/validator.rs) blueprint

**Learn about NeuRacle prototype**: `cd scrypto && cargo doc --no-deps --document-private-items --open`

### NeuRacle ecosystem's entities

There are 5 mains entites in NeuRacle ecosystem: **Users**, **Validators**, **NeuRacle Gateway**, **NeuRacle Storage** and **NeuRacle's Native Projects**.

**Validators**, or Data Providers are the people that host NeuRacle Gateway off-chain and ensure the security, connectivity of the Gateway. 

**NeuRacle Gateway** is a **decentralized off-chain entity** that will play role as a medium to automatically read data sources on-chain, use the source to fetch data off-chain, and feeding that data on-chain on validator behalf.

**Users** will have to take responsibility on the accessibility of sources. The data source can be public, or private data. User will have to provide an online and accessible API (and key, if that's a private data) to the NeuRacle Gateway. NeuRacle will also help user to choose (or host) an API that return the exact data user want.

To help the Gateway feedback the precise data that users need, the data source API should return only that one specific data. It shouldn't be a chunk of datas.

After make a request, users can fetch on-chain data through NeuRacle component, if it deemed non-accessible, users will only receive blank data.

User make request for data from 1 source will have to provide neuracle token. The more they provide, the longer they can get validated data. All neuracle token used will be burn.

**NeuRacle Storage** is an off-chain cloud service that do data extracting, parsing, web scraping and hosting on user demand. Those data can only be extracted by NeuRacle Gateway or the user demanded that data for monitor purpose. The data can also be public per user request.

NeuRacle Storage exist for users that can't point to the exact data source they need or can't host the data on their own.

NeuRacle Storage can also be a distributed system for more security.

**NeuRacle's Native Projects** are the projects that's built through NeuRacle blueprint (Eg: USDN stable coin project on the prototype showcase) or other projects that:

- Use the NeuRacle blueprint to fetch off-chain data.
- Have a tokeneconomic that ensure a rate of NAR burning when the project create new value.
- Don't have any method to reroute NeuRacle's data.

Native Projects on NeuRacle ecosystem will receive a badge to freely fetch validated data on-chain.

### User Incentive Program

To further promote the use of NeuRacle services and prevent re-routing of NeuRacle datas, we introduce the **User Incentive Program**. *A percent* of user's payment who demanded same data sources as previous data users will came back to those previous users. The reward will be distributed by the amount which previous users has paid (The more you paid, the more you got reward for the next payment of same data source).

The *incentive percent* will be a hard choice for NeuRacle operator, since if that too much (specifically >=50%), users can just create new account and get those data for free or at reduced price (since most of their payment go back to their other account). Or if it too little, it won't be much of an incentive and can't prevent re-routing of NeuRacle datas.

Undoubtedly, the *incentive percent* is the minimum threshold in which third-party will re-route NeuRacle datas make their user has to pay (based on game-theory). And as the consequence, those third-party also won't have any method to prevent their user won't do the same. In the end, third data market competetion will just destroy themselves and come back to NeuRacle (The loop will make them reduce their data price to the minimum threshold).

That's only possible with a incentive program. As long as the third-party can get enough benefit from re-routing the data, they will continue to do so.

### Delegated Proof of Stake

Anyone can choose a validator to stake, receive reward based on that validator contribution to the network. The Sybil Resistance mechanism worked the same as Radix Network.

PoS reward or punishment will happen on each data refreshing round.

### Data refreshing round

After every round, data will be refreshed, NAR token will also be minted to reward (or burned to punish) validators.

The round call and call-off will run by a racing condition. The individual call (or call-off) a round will receive a reasonable reward. This incentive is to ensure that data valitation round will happen and concluded right after they passed requirement.

Round call requirement is the round-length limited time.

Current time unit of on-chain NeuRacle is transactions history or epoch length.

Round length is the limited time between each data validation round. Data can be validated every 10tx, 100tx or 1epoch,...

Because this time unit is unstable occasionally, the stability of data stream will have to depend on Admin monitor.

Beside data sources, NeuRacle Gateway also have to keep track of the NeuRacle component state to see if new round has started or not.

Right after round start, NeuRacle Gateway will update datas on Validators behalf. After update, the validator will deemed active in that round.

Round concluded requirement is >2/3 active validators.

Datas with >2/3 staked weight of that round will also be validated.

## Some thought about NeuRacle

### Why one source?

Aggregrate data on-chain will be much more computation costly.

Moreover, not every users will want aggregrated data.

Eg: Bob operating a USX stable coin project and using aggregrated "XRD/USD last price" data feed to the system, let user exchange XRD/USX on the feeded data. However, most of the time, there is 1 particular exchange that have it's XRD/USD price lower than the aggregrated data, and unfortunately most of Bob's user use that exchange, so they complain about the data's authenticity. Now Bob have to use that exchange data source instead.

This won't just stay on crypto world, on real world too, different address, location, seller will provide different information. USA oil price will ofc different from Russia oil price. Pork from your locally will ofc different from the farm.

Providers, sellers can also use NeuRacle service to feed their product's price data on-chain and sell NFT proof of owning the product on DeFi market when they **don't even need to know about their buyer**. The product can be any thing like real estate, gold, diamond, or even daily grocery,.. 

Off-chain identity can also do data aggregration and ensure some degree of decentralization (Eg: Flux, SurpraOracle). Therefore, user can buy that data and make a data feeding request on NeuRacle.

### Why don't just use those off-chain decentralized data?

To feed any off-chain data, the oracle still need to rely on "a trusted bridge" or "a trusted medium", lead to a single point of failure or highly centralized, eg: [Ronin bridge](https://cointelegraph.com/news/the-aftermath-of-axie-infinity-s-650m-ronin-bridge-hack). However, NeuRacle use a large set of validators to ensure decentralized, trustless data feeding.

### Other approach

There can be some other approach in how to operate the data-feeding round like choosing leader and including vote mechanism (Same as Cerberus):

- On round call, every validators will update data and cease off-chain data fetching (or just store that last on-chain updated data).
- On voting phase, NeuRacle component will randomly choose a leader for that round, get that leader data and have validators vote on that data. The leader can be "random verified" by only choosing the middle validator that update NeuRacle datas in the stream of data validation transactions done at almost the same time.
- On round conclude, NeuRacle component will compute the voting result, update data, share the reward and punish untruthful behaviors.

**Pros**: Since the last approach will punish any validator that has even the slightly bit difference in data (might come from network delay), this approach will help those validators with low equipment to participate and get reward, further pushing decentralization. 

**Cons**: Since this approach help the validator with low equipment, it will somehow affect security. This approach also contain an extra voting phase in a data-feeding round, make a round more fee costly and create more room for bugs, exploits and malicious behaviors.

## Security, Utility

### What bad things won't happend on NeuRacle?

**Single point of failure**: The data feeding system is decentralized, there is no single point of failure. 

**Slow finality after sharded Cerberus**: With fully sharded Cerberus, all validators can make data feed transactions in parallel. Therefore, as long as NeuRacle has >2/3 truthful validators with required network hosting performance, datas will be finalized right after round start.

### What bad things might happend on NeuRacle?

**Security, Liveness Break**: NeuRacle has the same Sybil Resistance as Radix Network, malicious entities will need >1/3 staked value to break liveness, >2/3 staked value to really conduct a meaningful attack. Based on game-theory, that attack will really hard and costly. With sharded NeuRacle, the validator sets, as well as the data sources they may validate in the next round will all be randomized, make an attack become almost impossible.

**Single point of instability**: As mentioned above, the stability of data stream has to be relied on the Admin's monitor, create a *Single point of instability*. Using an unstable time unit on-chain like transaction history or epoch will affect the *round-length limited time*. Since token burning, minting and data updating all happened on data feeding round, the unstable round-length might lead to an *unstable stream of data* and *unpredicted tokeneconomy*. Stablizing round-length would need to rely on Admin's monitor (Or the NeuRacle DAO in the future).

Although such a bad thing might happen on NeuRacle, it wouldn't be a critical problem of an Oracle. Unstable data updating time won't affect most Oracle usecase as long as the data is frequently updated. However, the data might be delayed more than tolerance and break liveness, lead to other *critical problem* might happened on NeuRacle. (check *Congested Network*)

**Congested Network**: If Radix Network ever become congested, all the DeFi system will be delayed, not only NeuRacle. User is recommended to cease on-chain activity on such an event and wait for the system to cool down.

**Slow finality before sharded Cerberus**: As explained above, before Radix network is fully sharded, validators has to make data feed transactions in sequence.

### What can't this NeuRacle prototype do yet?

- **VRF**: Unfortunately, current NeuRacle prototype can't do VRF. In the future, NeuRacle will include a function to generate random number from a verified seed: "Unix time from NeuRacle service", "Crypto, asset price from NeuRacle service", "The middle address that update NeuRacle datas in the stream of data validation transactions done at almost the same time",... All these data are verified to have a degree of "entropy".

- **Private data feeding**: Currently, all data verified on NeuRacle prototype is stored on NeuRacle component state and is available for any off-chain reading. In the future, NeuRacle will include an Encrypt - Decrypt tool for this specific usecase.

## License & P/s

This work is licensed under MIT and Apache 2.0.

*To complete this work, I has learned a lot from works of many other Radix community members. My best gratitude for [Clement](https://github.com/cbisaillon), [0xOmar](https://github.com/0xOmarA), [Devmannic](https://github.com/devmannic) and [Chris](https://github.com/plymth)*

*I'm still an amateur on cryptography and distributed technology, this work may still contain something wrong or haven't taken into account. I'm glad to have any contributor to this work.*
