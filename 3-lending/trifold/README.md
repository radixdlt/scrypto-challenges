# Trifold

A three factor DeFI lending solution based on the RADIX token and the scrypto library.

## How to run

1. Install node modules
   - `yarn install`
2. Build the smart contract
   - `yarn scrypto`
3. Run the server
   - `yarn dev`
4. Open the browser
   - `localhost:3000`

## How it works

Trifold is built on scrypto and Next.js (React) with Tailwind CSS.

Trifold has many common DeFi lending features:

- Lenders are able to deposit XRD and will be compensated one to one in lnXRD for the amount of XRD they deposit.
- Borrowers are able to borrow XRD from the pool with no collateral, and will be charged interest per epoch.
- Lenders are able to withdraw XRD, along with any profit they have earned through interest, by returning the lnXRD they have recieved, provided the pool has enough liquidity to cover the withdrawal.

As stated earlier, Trifold's namesake is due to its three factors of security.
Each factor provides its own layer of security, and to work in the lender's best interests. The three factors are:

1. [Offchain Verification](#offchain-verification)
2. [Voting Consensus](#voting-consensus)
3. [Karma](#karma)

### Offchain Verification

The instantiator of the contract is responsible for the offchain governance of the contract. They are given an admin badge which is used to approve borrowers. Off the chain, a borrower approaches the instantiator of the contract to request a borrower approval. The instantiator of the contract is then given the opportunity to approve or deny the borrower in real life. If the borrower is approved, the instantiator can then place an approval request on the `/admin` endpoint. The instantiator should provide the name of the borrower (or the company's name), the RADIX address of the holder, and a website which claims responsibility for the address.

### Karma

Karma is a system within the Trifold contract which allows for borrowers to increase their lending limits depending on their past reputation. Borrowers are initially given a set amount of karma, which can be traded 1 to 1 for XRD. If they take out a loan, a corresponding amount of karma will be burned. If the borrower defaults, the borrower will lose the amount of karma they had initially. However, if the borrower pays back the loan, now that interest has been accrued, the borrower will gain more karma than they had initially. This allows for borrowers to continue to increase their lending limits, and prevents malicious borrowers from abusing the system.

### Emergency Shutdown

Holders of the contract can request an emergency shutdown of the contract by sending all of their lnXRD to a specified address. This will compensate the holder with a lockdown special token, and if the amount of lnXRD is over 50% of the total public supply of lnXRD, it shows that a majority of the lnXRD holders believe that the contract is in a bad state. This will cause prevent the contract from accepting any new loans, approving borrowers, withdrawing lends, and all major functions, until the lockdown is ended by withdrawing the lnXRD from the lockdown vault by returning the lockdown token.

## How to use it

Create a new component on the `/` root endpoint. This provides the creator with an admin badge, which they can use to start the borrower approval process.

On the `/lend` endpoint, lenders can deposit XRD and withdraw XRD. They can also vote on borrower approval requests.

On the `/borrow` endpoint, borrowers can borrow XRD from the pool.

On the `/admin` endpoint, the instantiator can approve or deny borrowers.
