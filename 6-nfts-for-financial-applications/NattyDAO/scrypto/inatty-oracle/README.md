## iNatty Oracle

### How It Works

There's an iNatty relay (python script), that is called periodically with user names to process. This builds a database of nature experiences, with meta data to be used in minting NFTs.

### Scrypto Components

- src/mint.rs - Keeps track of which NFTs have been minted (by iNaturalist ID). Creates new NFTs and mints them. 

- src/register.rs - Used to hold the iNaturalist ID of the user. Takes a payment of INATTY token to register.

- src/market.rs - This is the marketplace component where people list, buy and sell NattyDAO NFTs w/ INATTY token.