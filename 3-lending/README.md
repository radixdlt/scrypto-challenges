# Lending App
In this example, we will create an uncollateralized loan application. everyone can lend or borrow. 
level badges are assigned based on usage.

## Setting up the environment
We will show how you can use the transaction manifest to call multiple components in the same transaction. For this example, we will build a transaction that will first lend some money to the pool, then take the money back. You can run this command to setup the LendinApp and radiswap components:
## How to run
1. Reset your environment: `resim reset`
2. Create a new account: `resim new-account` -> save into **$account**
3. Build and deploy the blueprint on the local ledger: `resim publish .` -> save into **$package**
4. Call the `instantiate_pool` function to instantiate a component: `resim call-function $package LendingApp instantiate_pool 100,$xrd 100 10 7` -> save into **$component**
5. Call the `lend_money` method on the component: `resim call-method $component lend_money 2,$xrd`
6. Call the `take_money_back` method on the component: `resim call-method $component take_money_back 2,$lnd`
7. Verify that you received a token by running `resim show $account`

## How to run
1. Run the transaction manifest: `resim run lending_example.rtm`