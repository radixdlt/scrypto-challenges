## Resim

Resim is a command line tool that is used to interact with local Radix Engine simulator during development.
All of the interactions with the backend are done with resim for now.

On every start or whenever you want to try something new, first command should be `resim reset`, it deletes everything from the simulator.

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

Save the address, private and public keys and NonFungibleGlobalId as you'll need it later.
When you make the account it will have 1000 XRD on it.
If you want to see the details of the account you can use:

`resim show <ACCOUNT_ADDRESS>`

At the bottom of the response there is a list of all of the resources that the account has, for example:

`Resources:
├─ { amount: 898.6816111, resource address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety, name: "Radix", symbol: "XRD" }
├─ { amount: 1, resource address: resource_sim1qzmm88ant9lrace0ya6x63gapqnkg3v644kcqd8y0m2qngeg6w, name: "Admin Badge" }
├─ { amount: 1, resource address: resource_sim1qp2ahm386cw0hcxmyj88r4w249wqrgnyh7ncu66v3lqq2rrts3, name: "User Badge" }
│  └─ NonFungible { id: NonFungibleLocalId("#1#"), immutable_data: Tuple(), mutable_data: Tuple() }
├─ { amount: 1, resource address: resource_sim1qpugpy08q9mp8v9vzcs0y6yyzw5003ratjue48ag2j4s73casc, name: "Owner badge" }
│  └─ NonFungible { id: NonFungibleLocalId("#1#"), immutable_data: Tuple(), mutable_data: Tuple() }`

You can create as much accounts as you want but if you want to interact with the app but every account that interacts wit the app must be changed to be the default.

## Changing the default account

Run the new account command:

`resim new-account`

You’ll see something like this:

`A new account has been created!
Account component address: account_sim1q0vg5el883j9t9jkdz0g8r80nvv7rfrk8pq66e4sm26qrwlptl
Public key: 021a67d56f6c4b839f7c524ed62618b5e23bc3fee4d6c48f85fdcb9d1c15421945
Private key: daa71067e280834e2ebd092ef5ada70b08ee6661e1c605c141b89a9dfc3fbb4f`

Transfer owner badge to new account:

`resim transfer 1 <OWNER_BADGE> <NEW_ACCOUNT_ADDRESS>`

Set new account as default:

`resim set-default-account <NEW_ACCOUNT_ADDRESS>
<NEW_ACCOUNT_PRIVATE_KEY> <OLD_ACCOUNT_NonFungibleGlobalId>`



## Publishing the package

To publish the package run this command:

`resim publish .`

At the bottom of the response you'll get the package address, save it as you'll need it for later.

## Component Instantiation

To instantiate the Lending component run this command:

`resim call-function <PACKAGE_ADDRESS> Lend instantiate_lending`

You'll get the component and resource addresses in the response, something like this:

`└─ Component: component_sim1q2mm88ant9lrace0ya6x63gapqnkg3v644kcqd8y0m2qkkpn9x
└─ Resource: resource_sim1qzmm88ant9lrace0ya6x63gapqnkg3v644kcqd8y0m2qngeg6w`

Component address is the address of the instantiated component and it will be used for all of the transactions later on. 
Resource address is the Admin Badge that will be used for creating the Proof for using the admin methods.
You can also get it with `resim show <ACCOUNT_ADDRESS>`.

## Transactions

Transactions are stored in the transactions folder in the project. 
Transaction consists of several parts:

##### Locking the fee payment

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");`

Every transaction needs to have this othervise it will not work.
User is always paying the fee, so you'll be using the same account address as for everything else in the transaction.

##### Creating the Proof

In order to use some of the methods on the app you must be authenticated. That is done through Proofs. They are created using your account address and your badge, for now it can be Admin Badge or User Badge.

First step is to create the proof:

`CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<ADMIN_OR_USER_BADGE_RESOURCE_ADDRESS>");`

Proof is created and put in auth zone, so the next step is to get it so you can use it.

`CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<ADMIN_OR_USER_BADGE_RESOURCE_ADDRESS>")
    Proof("NAME_OF_THE_PROOF");`

Name of the proof can be anything, for the sake of keeping things simple we will be using admin_badge and user_badge.

##### Creating the bucket to send resource

If you need to send some resource, it should be taken from the worktop and put in the bucket:

`CALL_METHOD 
    ComponentAddress("<ACCOUNT_ADDRESS>") 
    "withdraw_by_amount" 
    Decimal("<AMOUNT>") 
    ResourceAddress("<RESOURCE_ADDRESS>");
TAKE_FROM_WORKTOP 
    ResourceAddress("<RESOURCE_ADDRESS>") 
    Bucket("<BUCKET_NAME>");`

Resource address for XRD is always the same for now, it's: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety
If you need some other resource it can be found in the account details (`resim show <ACCOUNT_ADDRESS>`)
Bucket name can be anything, same as the proof name.

##### Taking the response and putting it in your account

In case there is something sent by the app, for example XRDs when you borrow them, you should take them from the worktop and put it in your account:

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

## Interacting with the app

There are several transactions you can use in order to do things on the app.

##### add_asset

First transaction must be add_asset, it's creating the vault with assets that can be used for lending.
When the asset is added, corresponding SRWA token is created.
It is something that only admin can do so it requires admin badge.

Run it with this command:

`resim run "./src/transactions/add_asset.rtm"`

`CALL_METHOD
    ComponentAddress("<ADMIN_ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<ADMIN_BADGE_ADDRESS>");

CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("ADMIN_BADGE_ADDRESS")
    Proof("admin_badge");

CALL_METHOD ComponentAddress("<ADMIN_ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<COMPONENT_ADDRESS>")
    "new_asset"
    ResourceAddress("<ASSET_RESOURCE_ADDRESS>")
    Decimal("<LTV_RATIO>"); `

If XRD is the asset then the asset resource address will be resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety.
LTV ratio is a decimal number between 0 and 1.

##### create_user

Run it with this command:

`resim run "./src/transactions/create_user.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<COMPONENT_ADDRESS>")
    "new_user";

CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

After running this command, user is created and stored in the app and if you run the command `resim show <ACCOUNT_ADDRESS>` you should see that you have the User Badge in resources, something like this:

`{ amount: 1, resource address: resource_sim1qp2ahm386cw0hcxmyj88r4w249wqrgnyh7ncu66v3lqq2rrts3, name: "User Badge" }`

##### deposit

User can now make a deposit, run it with this command:

`resim run "./src/transactions/deposit.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge");

CALL_METHOD 
    ComponentAddress(<ACCOUNT_ADDRESS>) 
    "withdraw_by_amount" 
    Decimal("<AMOUNT>") 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");
TAKE_FROM_WORKTOP 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") 
    Bucket("deposit");
    
CALL_METHOD 
    ComponentAddress("<COMPONENT_ADDRESS>") 
    "deposit_asset" 
    Bucket("deposit")
    Proof("user_badge");

CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

AMOUNT is decimal number of resources (XRDs) that the user wants to deposit.
When the deposit is made, SRWA token is minted to be sent to the users account.
After running this command you’ll get some amount of srXRD tokens, if you run the command `resim show <ACCOUNT_ADDRESS>` you should have something like this in resources:

`{ amount: 100, resource address: resource_sim1qq70n0f4x6g66cqkuns9gck0eu8njwayeevf2v4rg3lsxdcyej, name: "SRWA Token", symbol: "srXRD" }`

##### withdraw

User can withdraw the deposit or part of it by running this command:

`resim run "./src/transactions/withdraw.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge");

CALL_METHOD 
    ComponentAddress("<COMPONENT_ADDRESS>") 
    "withdraw_asset" 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
    Decimal("<AMOUNT>")
    Proof("user_badge");

CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

After this transaction the amount that the user requested should be put in his account.
Accrued interest is staying on the user account.

##### borrow

User can borrow using the deposited assets as collateral running this command:

`resim run "./src/transactions/borrow.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge");

CALL_METHOD
    ComponentAddress("<COMPONENT_ADDRESS>")
    "borrow_asset"
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
    Decimal("<AMOUNT>")
    Proof("user_badgef");

CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

AMOUNT is the amount that te user wants to borrow.
After running this transaction, user should get the borrowed assets on his account.

##### repay

User can repay his debt by running this command: 

`resim run "./src/transactions/deposit.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");

CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge_proof");

CALL_METHOD 
    ComponentAddress("<ACCOUNT_ADDRESS>") 
    "withdraw_by_amount" 
    Decimal("5") 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety");
TAKE_FROM_WORKTOP 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety") 
    Bucket("repay");
    
CALL_METHOD 
    ComponentAddress("<COMPONENT_ADDRESS>") 
    "repay_asset" 
    Bucket("repay")
    Proof("user_badge_proof");

CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "deposit_batch" Expression("ENTIRE_WORKTOP");`

If there’s some interest, it will be calculated and addet to the amount that has to be repaid. If the amount that is sent by the user is smaller then the debt, app is going to calculate how much of the debt is left to be repaid, if it’s greater, it’s going to give back the rest back to the user after it takes the amount needed.

##### get_borrow_balance

User can see the amount that he borrowed by running:

`resim run "./src/transactions/get_borrow_balance.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge_proof");

CALL_METHOD 
    ComponentAddress("<COMPONENT_ADDRESS>") 
    "get_resource_borrow_balance" 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
    Proof("user_badge_proof");`

##### get_deposit_balance

User can see the amount that he deposited by running:

`resim run "./src/transactions/get_deposit_balance.rtm"`

`CALL_METHOD ComponentAddress("<ACCOUNT_ADDRESS>") "lock_fee" Decimal("10");

CALL_METHOD
    ComponentAddress("<ACCOUNT_ADDRESS>")
    "create_proof"
    ResourceAddress("<USER_BADGE_ADDRESS>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<USER_BADGE_ADDRESS>")
    Proof("user_badge_proof");

CALL_METHOD 
    ComponentAddress("<COMPONENT_ADDRESS>") 
    "get_resource_deposit_balance" 
    ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety")
    Proof("user_badge_proof");`













