# SRWA Yield Derivatives

### About

```
This is a simple flexible staking blueprint designed to yield rewards for staking for a specific time. For the account staking the XRD (or other approved asset, like xUSDC), the rewards are taken from the ytXRD dedicated pool (or ytxUSDC).

There is no locking, and the account can unstake at any time, while the rewards would be available only after a predefined time has elapsed.

The package contains component with a simple frontend to demonstrate the functionality and end-user experience. The demo is also available here: https://yield-srwa.netlify.app/
```

### Usage

#### dApp

```
`cd dApp` - go into the dApp folder
`cp .env-example .env` - change the name of the .env file
`npm run dev`: Starts the development server using Vite.
`npm run build`: Builds the project for production using TypeScript and Vite.
`npm run lint`: Lints the source code using ESLint.
`npm run preview`: Previews the production build using Vite.

There is a parameter in the .env file named VITE_COMPONENT_ADDRESS, frontend is already connected with the testnet version, if you want to release your own component version, you should just update the component address.
```

#### Backend

# Yield-Derivatives-Demo

## Resim

resim is a command line tool that is used to interact with local Radix Engine simulator during development.
All of the interactions with the backend are done with resim for now.

On every start or whenever you want to try something new, first command should be `resim reset`. It deletes everything from the simulator.

## Creating a new account

Running this command:
`resim new-account `
creates a new account on the simulator. When done the first time it will automatically set the account as default. Response will be something like this:

`A new account has been created!
Account component address: account_sim1qdu23xcp4jcvurxvnap5e7994xzfza8e0myjaez0s73qd2wye3
Public key: 0208beddb4a109910b5fc9ddfe8b370351bf3e6430874d2ae9e65e3a863b8b6bd6
Private key: 4edf45bf7b6da8ac4d06fec8512c9b6d37a8288c9b39e5ebbb738e60e028a297
NonFungibleGlobalId: resource_sim1qpugpy08q9mp8v9vzcs0y6yyzw5003ratjue48ag2j4s73casc:#1#
Account configuration in complete. Will use the above account as default.`

Save the Account component address, Private and Public keys and NonFungibleGlobalId as you'll need it later.
When you make the account, it will have 1000 XRD on it by default.
If you want to see the details of the account you can use:

`resim show <ACCOUNT_ADDRESS>`

## Publishing the package

To publish the package run this command:

`resim publish .`

At the bottom of the response you'll get the package address (something like this `package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92`),
save it as you'll need it for later.

## Component Instantiation

To instantiate the YieldDerivatives component run this command:

`resim call-function <PACKAGE_ADDRESS> YieldDerivatives instantiate`

You'll get the component and resource addresses in the response, something like this:

`└─ Component: component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn
└─ Resource: resource_sim1t4h3kupr5l95w6ufpuysl0afun0gfzzw7ltmk7y68ks5ekqh4cpx9w`

Component address is the address of the instantiated component and it will be used for all of the transactions later on.
Resource address is the Admin Badge that will be used for creating the Proof for using the admin specific methods.
You can also get it with `resim show <ACCOUNT_ADDRESS>`.

## Interacting with the app

There are several transactions you can use in order to do things on the app.

##### add_new_asset

First transaction must be add_new_asset, it's creating the vault with assets that can be used for deposit and redeem.
When the asset is added, corresponding ytToken token is created (with initial amount of 1 000 000).
It is something that only admin can do so it requires admin badge.

You can add new asset like this:

`resim call-method <COMPONENT_ADDRESS> add_new_asset("<RESOURCE_ADDRESS>", <YIELD_RATE>) --proofs <ADMIN_BADGE>`

Examle:
`resim call-method component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn add_new_asset("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3", 0.1) --proofs resource_sim1t4h3kupr5l95w6ufpuysl0afun0gfzzw7ltmk7y68ks5ekqh4cpx9w:1`

or run it with this command:

`resim run "./src/transactions/add_new_asset.rtm"`

`CALL_METHOD
Address("component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh")
"lock_fee"
Decimal("5000");

CALL_METHOD
Address("account_tdx_2_128qp5d27yepw44ftmq9wa9jzps7tf3y7r20xw0v6xz20wt54s7muwp")
"create_proof_of_amount"
Address("resource_tdx_2_1t5v99d0s7njg7qe65fggvhsj69n4qfukhnhd3egj3j9s0rg0yupp6e")
Decimal("1");

CALL_METHOD
Address("component_tdx_2_1cq9kx6zhxwrnme6g08gkxnngq6hhsetv4u7z2y9t6pmp5t4xenlplg")
"add_new_asset"
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Decimal("0.1");

CALL_METHOD
Address("account_tdx_2_128qp5d27yepw44ftmq9wa9jzps7tf3y7r20xw0v6xz20wt54s7muwp")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>(); `

If XRD is the asset then the asset resource address will be resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3.

##### create_user_and_deposit_principal

Next step is to create a new user and deposit principal token (which is added in previous step).

It can be done with this ccommand:
`resim call-method <COMPONENT_ADDRESS> create_user_and_deposit_principal <RESOURCE_ADDRESS>:<RESOURCE_AMOUNT>)`

Example:
`resim call-method component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn create_user_and_deposit_principal resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:10`

or run it with this command:

`resim run "./src/transactions/create_user_and_deposit_principal.rtm"`

`CALL_METHOD
Address("component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh")
"lock_fee"
Decimal("5000");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"withdraw"
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Decimal("10");

TAKE_FROM_WORKTOP
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Decimal("10")
Bucket("DepositingBucket");

CALL_METHOD
Address("component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn")
"create_user_and_deposit_principal"
Bucket("DepositingBucket");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>(); `

After running this command, user is created and stored in the app and his principal and yield balances are updated.

Also you should see that you have the User Badge in resources if you run the command `resim show <ACCOUNT_ADDRESS>` , something like this:
`{ amount: 1, resource address: resource_sim1qp2ahm386cw0hcxmyj88r4w249wqrgnyh7ncu66v3lqq2rrts3, name: "User Badge" }`

##### deposit_principal

Principal can be deposited with this function, but depositing is available if the user does not have an active staking deposit.
So, if you have already staked some tokens, you must redeem them first, than deposit new tokens.

It can be done with this ccommand:
`resim call-method <COMPONENT_ADDRESS> deposit_principal(<RESOURCE_ADDRESS>:<RESOURCE_AMOUNT>, <USER_BADGE_PROOF>)`

Example:
`resim call-method component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn deposit_principal("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:10", "resource_sim1nfmyl590n6dttggacglnqx98yu8y2k4vqxjakqeyueqvqtxcqf433s:#1#")`

or run it with this command:

`resim run "./src/transactions/deposit_principal.rtm"`

`CALL_METHOD
Address("component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh")
"lock_fee"
Decimal("5000");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"withdraw"
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Decimal("10");

TAKE_FROM_WORKTOP
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Decimal("10")
Bucket("DepositingBucket");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"create_proof_of_non_fungibles"
Address("resource_sim1nfmyl590n6dttggacglnqx98yu8y2k4vqxjakqeyueqvqtxcqf433s")
Array<NonFungibleLocalId>(
NonFungibleLocalId("#1#")
);

POP_FROM_AUTH_ZONE
Proof("UserBadge");

CALL_METHOD
Address("component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn")
"deposit_principal"
Bucket("DepositingBucket")
Proof("UserBadge");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>(); `

User sends bucket that he wants to deposit.
When the deposit is made, principal and yield balance are updated.

##### redeem

User can redeem the deposit at any time. If redeem is activated before maturity date, user will not receive yield token award.

It can be done with this ccommand:
`resim call-method <COMPONENT_ADDRESS> redeem(<RESOURCE_ADDRESS>, <USER_BADGE_PROOF>)`

Example:
`resim call-method component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn redeem("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3", "resource_sim1nfmyl590n6dttggacglnqx98yu8y2k4vqxjakqeyueqvqtxcqf433s:#1#")`

or run it with this command:

`resim run "./src/transactions/redeem.rtm"`

`CALL_METHOD
Address("component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh")
"lock_fee"
Decimal("5000");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"create_proof_of_non_fungibles"
Address("resource_sim1ngzq4h9deqr8vmwrenzv2ajagf2nggc2kysshsxsphhxes4cn83ymk")
Array<NonFungibleLocalId>(
NonFungibleLocalId("#1#")
);

POP_FROM_AUTH_ZONE
Proof("UserBadge");

CALL_METHOD
Address("component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn")
"redeem"
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Proof("UserBadge");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>(); `

After this transaction user will receive his deposited tokens and reward if all conditions are met.

##### get_users_deposit_balance

You can check deposit balance with this command:
`resim call-method <COMPONENT_ADDRESS> get_users_deposit_balance(<RESOURCE_ADDRESS>, <USER_BADGE_PROOF>)`

Example:
`resim call-method component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn get_users_deposit_balance("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3", "resource_sim1nfmyl590n6dttggacglnqx98yu8y2k4vqxjakqeyueqvqtxcqf433s:#1#")`

or run it with this command:
`resim run "./src/transactions/get_users_deposit_balance.rtm"`

`CALL_METHOD
Address("component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh")
"lock_fee"
Decimal("5000");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"create_proof_of_non_fungibles"
Address("resource_sim1nfmyl590n6dttggacglnqx98yu8y2k4vqxjakqeyueqvqtxcqf433s")
Array<NonFungibleLocalId>(
NonFungibleLocalId("#1#")
);

POP_FROM_AUTH_ZONE
Proof("UserBadge");

CALL_METHOD
Address("component_sim1cq4kl9qul5nsd49u99x25fs8gclv2yd28a9g242l2x0zx4hh6kqfkn")
"get_users_deposit_balance"
Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
Proof("UserBadge");

CALL_METHOD
Address("account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>(); `
