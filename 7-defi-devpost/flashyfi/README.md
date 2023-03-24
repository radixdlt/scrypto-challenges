# FlashyFi

FlashyFi is an application that leverages several powerful features of Radix to enable a completely new approach
to flash loans. Unlike other flash loan applications that require users to deposit their tokens into a liquidity pool,
FlashyFi allows each user to lend their tokens and NFTs without ever transferring them out of their accounts.

With many other flash loan applications, the tokens to be lent must first be locked in a smart contract, preventing them
from being used, traded, or transferred directly until they are withdrawn from the contract. However, FlashyFi takes a
different approach by keeping all tokens in their user's account and allowing a central component to temporarily
withdraw them, lend them to another user and return them before the transaction ends. As each flash loan lasts only for
the duration of a single transaction, the tokens never truly leave the account.

# Demo

A demo frontend is available at https://backyard-coder.github.io/flashyfi  
Please take a look at the about page for some usage instructions and a discussion of the demo's limitations.

The publicly hosted frontend is offered as a bonus and should not be the basis for evaluation. The evaluation should be
based solely on the code in the pull request. While I promise that I will not make any substantial changes to the
application after the submission deadline, I reserve the right to fix bugs that significantly affect the liveness or
functionality of the application.

If you would like to deploy the frontend locally, you can follow the instructions described in the
[frontend README](flashyfi-frontend/README.md).

## Technical implementation

FlashyFi is built on the following two characteristic features of the Radix network:

- Transient resources: Transient resources can be used to securely implement flash loans. A user who takes out a loan
  receives not only the lent tokens but also a special receipt NFT. As long as this NFT exists the transaction cannot
  succeed. It thus acts as a safety, preventing the user from taking the loan and then ending the transaction
  without repaying it. The only way to destroy the NFT is to give it back to the FlashyFi component, which will only
  accept it if the loan is repaid.
- Flexible access rule model: The Radix network has a flexible and powerful mechanism for defining access rules,
  including the ability to exchange factors/keys that grant access to an account. FlashyFi takes advantage of this
  feature and defines the account access rules such that not only the user but also the central FlashyFi component can
  withdraw tokens from the account. This is secure because the FlashyFi component always returns the withdrawn tokens.

For the technical details please take a look at the [Scrypto source code](flashyfi-scrypto/src/flashyfi.rs) as it
contains extensive documentation.

## Lender Usage

To lend tokens, users must first "flashyfi" their account by registering it with the central FlashyFi component and
granting that component access to the resources held in the account. Users can use the demo frontend to define which
tokens or NFTs they want to lend and for what fee. Experienced Scryptonauts may examine the transaction manifests shown
in their wallets to learn what's going on in the background.

## Borrower Usage

As a borrower, one would likely use flash loans in the context of automated processes, such as exploiting arbitrage
opportunities. For this, a backend API would be required. This backend would select the optimal accounts from which to
borrow each resource and would take into account the fees set by the account owner.

As such a backend is unsuitable for demonstrations, the FlashyFi frontend includes the ability to simulate a flash loan.
To do this, visit the app's homepage, select a resource, specify an amount to borrow and a transaction manifest will be
generated for you. This manifest performs/simulates the following 4 steps:

1. Borrow tokens from one or more accounts
2. Exploit an arbitrage opportunity (in the generated manifest, the XRD Faucet component is used for this).
3. Repay the borrowed tokens and pay the fee incurred.
4. Deposit the profit generated, if any, into your account.

## Outlook

Currently, FlashyFi is primarily designed for many users to lend tokens from their individual accounts.
However, the FlashyFi blueprint is flexible enough to connect not only account components but also any other components
that implement the methods `withraw_by_amount`, `withraw_by_ids` and `deposit_batch`. This enables users to integrate
components holding significantly more liquidity, such as AMM-style DEXs, giving a second purpose to the tokens that are
currently locked away in those components.
