# Demos

Make sure to build first by running `./build.sh` in the root folder (up one directory from here)

* Simple Swap
  * Request a buy of 200 "M" tokens for some amount of T tokens (...100 T is quoted)
  * Run: `./simple_swap.sh`
  * It logs a whole bunch and is well commented.  Dig in!

* NFT Sale
  * Request a quote to sell the one-of-a-kind demo-nft-family#01 for M tokens (100 M is quoted)
  * Run: `./nft_sale.sh`
  * just a slight modification of Simple Swap
  * a helper blueprint is used to airdrop the NFT during the setup phase for the demo

* Double Swap (ie. complex Taker/Sender)
  * Make your own "router" my combining 2 swaps in a single transaction
  * Run: `./double_swap.sh`
  * Very similar to Simple Swap, just does things twice

* Tokenized Swap (ie. complex Maker/Signer)
  * Get a quote to sell an NFT we don't have yet, tokenize it for later.  Respond to different RFQ to buy the NFT with a callback that resells it immediately pocketing the difference
  * Run: `./middleman.sh`
  * Shows how a Maker can do complex actions during settlement even when they aren't submitting the transaction, and how the HareSwap orders can be turned into non-fungibles

## Other Details

* `baseline.sh` - sets up accounts and things not specific to HareSwap but required for the demos.  
                  Includes hardcoded values instead of parsing resim output.
* `logging.sh` - makes the text output a little prettier
* `maker*_setup.sh` - common setup work for Makers
* `helper/` - a package with blueprint to airdrop an NFT for the demos since `resim new-token-fixed` does not work with NFTs

## Limitations

These have only been tested on Linux, but good chance they work on macOS too.
On Windows you could use WSL, but it will not work with Powershell (sorry)
