# Lending Platform - Scrypto Blueprint

## About

Radix Scrypto blueprint for creating a lending platform

This blueprint allows you to perform the following actions:

- Add assets/tokens to the platform using an admin badge
- Create accounts on the lending platform
- Deposit multiple types of assets/tokens as 'deposits'
- Withdrawal deposited assets
- Borrow against deposited assets
- Repay loans

The platform currently tracks user balances w/ a HashMap and utilizes admin specified LTV values when calculating
collateral

Next features to add:

- Prices per asset (All assets currently have a 1:1 ratio with XRD)
- Price oracles (Calling from an external source instead of from within the component)

## Blueprint Functions

### Admin authenticated functions

Before calling the admin functions, you must first authenticate by creating a proof as follows

#### Creating an admin proof

Format

```
# Withdrawing the admin badge from the admin account. When we call the `create_proof` methods on the account component
# the returned proof is automatically put in our auth zone so we do not need to manually manage it.
CALL_METHOD
    ComponentAddress("<<<INSERT_ADMIN_ACCOUNT_HERE>>>")
    "create_proof"
    ResourceAddress("<<<INSERT_LENDING_PLATFORM_ADMIN_BADGE_HERE>>>");

# The `create_proof` method returns as Proof. As soon as a proof comes back to the transaction worktop, it gets sent
# directly to the auth zone. Therefore, in the following instruction we're creating a `Proof` out of the badge in the
# auth zone.
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<<<INSERT_LENDING_PLATFORM_ADMIN_BADGE_HERE>>>")
    Proof("admin_badge");
```

Example

```
# Withdrawing the admin badge from the admin account. When we call the `create_proof` methods on the account component
# the returned proof is automatically put in our auth zone so we do not need to manually manage it.
CALL_METHOD
    ComponentAddress("020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a")
    "create_proof"
    ResourceAddress("036f251943d65956cf768885119fc77003a4c1deefb0b526744464");

# The `create_proof` method returns as Proof. As soon as a proof comes back to the transaction worktop, it gets sent
# directly to the auth zone. Therefore, in the following instruction we're creating a `Proof` out of the badge in the
# auth zone.
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("036f251943d65956cf768885119fc77003a4c1deefb0b526744464")
    Proof("admin_badge");
```

#### Function `new_asset`

Format

```
# Call the `new_asset` function to add a possible assets in the liquidity pool
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "new_asset"
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>")
    Decimal("<<<INSERT_LOAN_TO_VALUE_RATIO_HERE>>>");
```

Example

```
CALL_METHOD
    ComponentAddress("02fe2636176e5253ae0b91e8eb9a63c26631e8679110e34d3b0509")
    "new_asset"
    ResourceAddress("030000000000000000000000000000000000000000000000000004")
    Decimal("0.85");
```

### Non Authenticated Functions

#### Function `new_user`

Format

```
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "new_user";
CALL_METHOD_WITH_ALL_RESOURCES
    ComponentAddress("<<<INSERT_USER_ACCOUNT_HERE>>>")
    "deposit_batch";
```

Example

```
CALL_METHOD
    ComponentAddress("02f59582b222e59a5561aab9677116599e64d128a90698c95ae5de")
    "new_user";
CALL_METHOD_WITH_ALL_RESOURCES
    ComponentAddress("02b61acea4378e307342b2b684fc35acf0238a4accb9f91e8a4364")
    "deposit_batch";
```

### User Authenticated Functions

Before calling the user authenticated functions, we must first create a user badge proof which we will use when calling
the user authenticated functions.

#### Creating a user badge proof

Format

```
# Withdrawing the user lending badge from the user account. When we call the `create_proof` methods on
# the account component the returned proof is automatically put in our auth zone so we do not need to
# manually manage it.
CALL_METHOD
    ComponentAddress("<<<INSERT_USER_ACCOUNT_HERE>>>")
    "create_proof"
    ResourceAddress("<<<INSERT_USER_LENDING_PLATFORM_BADGE_HERE>>>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<<<INSERT_USER_LENDING_PLATFORM_BADGE_HERE>>>")
    Proof("user_badge_proof");
```

Example

```
CALL_METHOD
    ComponentAddress("02b61acea4378e307342b2b684fc35acf0238a4accb9f91e8a4364")
    "create_proof"
    ResourceAddress("03b097db9c47ffdd689238025535ec7cc55fcef22a963b0f23deb5");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("03b097db9c47ffdd689238025535ec7cc55fcef22a963b0f23deb5")
    Proof("user_badge_proof");
```

#### Function `deposit_asset`

Note: In order to deposit an asset, you must have a bucket of assets to deposit

Format for creating a bucket of assets to deposit

```
# Add XRD to the worktop
CALL_METHOD
    ComponentAddress("<<<INSERT_USER_ACCOUNT_HERE>>>")
    "withdraw_by_amount"
    Decimal("2000")
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>");

# Put worktop XRD into a bucket
TAKE_FROM_WORKTOP_BY_AMOUNT
    Decimal("2000")
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>")
    Bucket("assets_bucket");
```

Example for creating a bucket of assets to deposit

```
CALL_METHOD
    ComponentAddress("02b61acea4378e307342b2b684fc35acf0238a4accb9f91e8a4364")
    "withdraw_by_amount"
    Decimal("2000")
    ResourceAddress("030000000000000000000000000000000000000000000000000004");
TAKE_FROM_WORKTOP_BY_AMOUNT
    Decimal("2000")
    ResourceAddress("030000000000000000000000000000000000000000000000000004")
    Bucket("assets_bucket");
```

Format for calling the `deposit_asset` function using the `assets_bucket` bucket

```
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "deposit_asset"
    Bucket("assets_bucket")
    Proof("user_badge_proof");
```

Example for calling the `deposit_asset` function using the `assets_bucket` bucket

```
CALL_METHOD
    ComponentAddress("02f59582b222e59a5561aab9677116599e64d128a90698c95ae5de")
    "deposit_asset"
    Bucket("deposit_1")
    Proof("user_badge_proof");
```

#### Function `withdrawal_asset`

Format

```
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "withdrawal_asset"
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>")
    Decimal("1000")
    Proof("user_badge_proof");
```

Example

```
CALL_METHOD
    ComponentAddress("02f59582b222e59a5561aab9677116599e64d128a90698c95ae5de")
    "withdrawal_asset"
    ResourceAddress("030000000000000000000000000000000000000000000000000004")
    Decimal("1000")
    Proof("user_badge_proof");
```

#### Function `borrow_asset`

Format

```
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "borrow_asset"
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>")
    Decimal("500")
    Proof("user_badge_proof");
```

Example

```
CALL_METHOD
    ComponentAddress("02f59582b222e59a5561aab9677116599e64d128a90698c95ae5de")
    "borrow_asset"
    ResourceAddress("030000000000000000000000000000000000000000000000000004")
    Decimal("500")
    Proof("user_badge_proof");
```

#### Function `repay_asset`

Note: Must have an asset bucket in order to call this function. See `deposit_asset` for an example.

Format

```
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "repay_asset"
    Bucket("assets_bucket")
    Proof("user_badge_proof");
```

Example

```
CALL_METHOD
    ComponentAddress("02f59582b222e59a5561aab9677116599e64d128a90698c95ae5de")
    "repay_asset"
    Bucket("assets_bucket")
    Proof("user_badge_proof");
```

## Example Scenario

Included with this example is a file called `build_rtm.sh` that performs the following:

* Creates a test admin account
* Creates 2 test user accounts
* Creates an instance of the lending platform
* Adds XRD to the lending pool with an LTV of 0.85
* Registers the 2 user accounts with the lending platform
* Deposits assets for both users
* Withdrawals assets for the 2nd user
* 2nd User Borrows XRD against the collateral deposited by the 1st user
* 2nd User Repays borrowed XRD

To perform the above scenario, run the following: `source build_rtm.sh`

## Deploying Lendi to Public Test Environment (PTE)

### Prerequisites

Install the PTE Scrypto Terminal
Client: https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-terminal.html

### Building and publishing

Run the following commands:

* Build the package: `scrypto build`
* Open the resim-client: `resim-client --address pte02-socket.radixdlt.com:8010`
* In the resim terminal run:
    * `resim publish /<INSERT_PATH_TO_LENDING_PLATFORM_FOLDER_HERE>/scrypto/target/wasm32-unknown-unknown/release/lending_platform.wasm`
* Take note of the package address
    * Example `Success! New Package: 01bf1510f852a54d36f169a283cc3c8dd66eb1784987c62de6e473`
        * The package address would be `01bf1510f852a54d36f169a283cc3c8dd66eb1784987c62de6e473`

### Creating an instance

At this point, you have published the package to the PTE. The next step is to create an instance of the blueprint.

In the resim terminal run the following:

#### Create new account

Run `resim new-account`

Take note of the account address as we will need it in order to set up the lending pool in
the `Setting up the lending pool section`

#### Create component instance

Run `resim call-function <<<INSERT_PACKAGE_ADDRESS_HERE>>> LendingPlatform instantiate_lending_platform`

Ex. `resim call-function 01bf1510f852a54d36f169a283cc3c8dd66eb1784987c62de6e473 LendingPlatform instantiate_lending_platform`

Denote the `Component` address (instance address)

Example

```
Logs: 0
New Entities: 2
└─ Component: 020ddb692e8836ce8e12a03ab1d7a98e8c6327e60c4cc3bd5c8609
└─ Resource: 0345803d58dc62f31a4d04ef850efa77a7c091e3919ec97edfb343
```

The component address would be `020ddb692e8836ce8e12a03ab1d7a98e8c6327e60c4cc3bd5c8609`

### Setting up the lending pool

We are only going to add XRD for now, but you can use the same `new_asset` function to add other assets as well

Using the same account from `Creating an instance` run the following:

#### Obtain XRD address

* Format: `resim show <<<ACCOUNT_ADDRESS>>>`
* Example: `resim who 02973b7c75c73c5c348b96c4104cd93305522d34825c1922f7dcf5`
* Example output:
    * `{ amount: 1000000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix",
      symbol: "XRD" }`
* Denote the resource address for the next step

#### Create new asset

From a regular terminal, create a transaction manifest file as follows: (NOTE: this is kind of a janky way to do this,
but we cannot easily create manifest files from within the resim-client)

Format: `echo "CALL_METHOD ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")" "new_asset" ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>") Decimal("<<<INSERT_LOAN_TO_VALUE_RATIO_HERE>>>");" > <<<INSERT_PATH_FOR_MANIFEST_FILE>>>`

Example: `echo "CALL_METHOD ComponentAddress("020ddb692e8836ce8e12a03ab1d7a98e8c6327e60c4cc3bd5c8609") "new_asset" ResourceAddress("030000000000000000000000000000000000000000000000000004") Decimal("0.85");" > /home/defi/new_asset_manifest_file.rtm`

Next, run the transaction manifest file from the resim-client

* Format: `resim run <<<INSERT_PATH_FOR_MANIFEST_FILE>>>`
* Example: `resim run /home/defi/new_asset_manifest_file.rtm`

## Testing using the PTE browser plugin

- Install the PTE browser
  extension: https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html
- Select the pte02 environment (this is the one we published our package to)
- Create an account using the PTE browser extension
- Run the example manifest commands from the `Blueprint Functions` section in the PTE browser extension

### Full Examples

#### Create Account

Format
```
CALL_METHOD
    ComponentAddress("<<<lending_platform_component_address>>>")
    "new_user";
    CALL_METHOD_WITH_ALL_RESOURCES
    ComponentAddress("<<<account_1_address>>>")
    "deposit_batch";
```

Example
```
CALL_METHOD
    ComponentAddress("020ddb692e8836ce8e12a03ab1d7a98e8c6327e60c4cc3bd5c8609")
    "new_user";
    CALL_METHOD_WITH_ALL_RESOURCES
    ComponentAddress("02e70830fe32de80be11c710bc272ac0fd3ddaabe8dc9d48f05825")
    "deposit_batch";
```

#### Deposit XRD
Format
```
CALL_METHOD
    ComponentAddress("<<<INSERT_USER_ACCOUNT_HERE>>>")
    "create_proof"
    ResourceAddress("<<<INSERT_USER_LENDING_PLATFORM_BADGE_HERE>>>");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("<<<INSERT_USER_LENDING_PLATFORM_BADGE_HERE>>>")
    Proof("user_badge_proof");
CALL_METHOD
    ComponentAddress("<<<INSERT_USER_ACCOUNT_HERE>>>")
    "withdraw_by_amount"
    Decimal("2000")
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>");
TAKE_FROM_WORKTOP_BY_AMOUNT
    Decimal("2000")
    ResourceAddress("<<<INSERT_TOKEN_ADDRESS_HERE>>>")
    Bucket("assets_bucket");
CALL_METHOD
    ComponentAddress("<<<INSERT_LENDING_PLATFORM_COMPONENT_ADDRESS_HERE>>>")
    "deposit_asset"
    Bucket("assets_bucket")
    Proof("user_badge_proof");
```

Example
```
CALL_METHOD
    ComponentAddress("02e70830fe32de80be11c710bc272ac0fd3ddaabe8dc9d48f05825")
    "create_proof"
    ResourceAddress("03de4479a4ccd0953fa6539aaf93aa4a85062beb3f8f6fec5fffec");
CREATE_PROOF_FROM_AUTH_ZONE
    ResourceAddress("03de4479a4ccd0953fa6539aaf93aa4a85062beb3f8f6fec5fffec")
    Proof("user_badge_proof");
CALL_METHOD
    ComponentAddress("02e70830fe32de80be11c710bc272ac0fd3ddaabe8dc9d48f05825")
    "withdraw_by_amount"
    Decimal("2000")
    ResourceAddress("030000000000000000000000000000000000000000000000000004");
TAKE_FROM_WORKTOP_BY_AMOUNT
    Decimal("2000")
    ResourceAddress("030000000000000000000000000000000000000000000000000004")
    Bucket("assets_bucket");
CALL_METHOD
    ComponentAddress("020ddb692e8836ce8e12a03ab1d7a98e8c6327e60c4cc3bd5c8609")
    "deposit_asset"
    Bucket("assets_bucket")
    Proof("user_badge_proof");
```