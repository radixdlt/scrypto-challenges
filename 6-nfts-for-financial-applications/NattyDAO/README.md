THIS IS TOTALLY UNFINISHED, but wanted to submit it just for the sake of 'why the hell not?'.

Basically there's a relay script that checks an external API for changes, loads a file with those changes, and then mints them as NFTs using an RTM. This relay is TBD (to be decentralized).

That RTM is calling into [scrypto/inatty-oracle/src/mint.rs](scrypto/inatty-oracle/src/mint.rs) in the `create_nft` method.

This is not an NFT for finance use cases, but an NFT for "Regenerative Finance" use cases. 

------------------

## <This is all conceptual until Babylon. Please join now and help build this project with me!>

See [shardeez.xyz/refi](https://shardeez.xyz/refi) for resources on Regenerative Finance (ReFi). Let's learn about ReFi and build together!

## Idea for NattyDAO

### Mission

To support members of the Radix community to `touch grass` & `touch trash`.

DAO funds small-scale environmental benefits such as: wildland trash pickups, beach cleanups, de-paving (removing concrete and replacing w/ native plants), gardening projects, pollinator homes, and more. 

### Mechanism

- Members propose a project, request funding from treasury.
  
- Holders of INATTY token vote to pass or reject proposal.
  
- Quadratic funding mechanism w/ a matching pool.
  
- Treasury funded by sale of NFTs, donations.

### iNATTY Token

The first token backed by relationships to nature. It's used to buy NFTs from the NattyDAO. It's also used to vote on proposals to fund environmental projects.

### iNATTY NFTs (THIS PART USED FOR SUBMISSION TO SCRYPTO CHALLENGE)

NFTs minted by the NattyDAO that represent a member outside experiencing nature. These NFTs are minted using data from the **iNatty Oracle**. Whenever a registered member experiences nature, an NFT is minted. This nature experience is provable because of integration with the iNaturalist API.

They are then offered for sale in an NFT marketplace. They can only be purchased with INATTY, and 50% of the funds go to the NattyDAO treasury, and 50% go to the member.

### iNATTY Oracle 

Relays information about the member's associated iNaturalist accounts to the NattyDAO components. One part watches for new registrations of [iNaturalist.org](https://inaturalist.org) accounts and keeps track of the member's activity in a Scrypto Component. Another part watches for new observations on iNaturalist and mints NFTs for the member (metadata includes the natural experience, including image, species identified, location, and date).

### NattyDAO Treasury

This is used to incentivize members to participate in the NattyDAO. It is funded by the sale of iNATTY NFTs. Proposals can be made to the NattyDAO to spend funds from the treasury.

### Optimistic Funding (provable cleanups idea)

- Once funded the project is given 1 week to complete and prove to the community their efforts.
  
- If no dispute about their efforts, XRD automatically sent to their address
  
- Someone disputing must bond some XRD.
  
- Their dispute is voted on, if found out true then they get their bonded XRD back + some reward.
  
- If their dispute isn't corroborated by community, they lose their bond.

### References

- [Open Forest Protocol Whiteboard Series (YouTube)](https://www.youtube.com/watch?v=ZjFT2KoUgks&list=PLWJdg32OtDLUbxcE_Qr0GTHQ0L07mikej)