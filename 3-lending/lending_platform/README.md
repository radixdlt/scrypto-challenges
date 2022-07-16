# Lending Platform

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

- Creates a test admin account
- Creates 2 test user accounts
- Creates an instance of the lending platform
- Adds XRD to the lending pool with an LTV of 0.85
- Registers the 2 user accounts with the lending platform
- Deposits assets for both users
- Withdrawals assets for the 2nd user
- 2nd User Borrows XRD against the collateral deposited by the 1st user
- 2nd User Repays borrowed XRD

To perform the above scenario, run the following: `source build_rtm.sh`
