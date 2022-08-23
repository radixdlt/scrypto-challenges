# Indexer

Indexer allows anyone to easily create their own index fund, utilizing resources created on the Radix network. Once a index fund is created, anyone can invest in the fund. Each time someone deposits XRD into the newly created index fund, the XRD is equally split across each token making up the index fund and the depositor receives Indexer Tokens. The Indexer Token is similar to an LP token and keeps track of % ownership of the index fund. 

## Features

1. Utilize each index pool to perform arbitrage between decentralized exchanges.
2. Utilize each index pool to create flash loans.
3. Create (Indexer token / XRD) pair pool on a DEX. This allows users to get exposure to the index fund by purchasing the Indexer Token on a DEX or to utilized the Indexer protocol and manually deposit XRD, and receiving Indexer Tokens. Any premium or discount between the Indexer Token value and the actual value of the Indexer Token calculated by using the weighted price of the tokens making up the index can be balanced by arbitrage. Arbitrage by buying and selling on DEX or creating or redeeming Indexer Tokens using the Indexer protocol. 

## Design Details

A index fund may be created by choosing individual resources to make up the index fund. The XRD token will need to be apart of the index fund. The individual tokens that make up the index fund are stored in vaults. When users deposit XRD into the index fund, the XRD is split evenly across each token making up the index fund. The protocol automatically takes the XRD and swaps them into each resource that makes up the index fund. 

Users will receive Indexer Tokens, which keeps track of % ownership of the index fund. The protocol mints and burns Indexer Tokens when users deposit or redeem their Indexer Tokens. The amount of Indexer Tokens that are minted are based on the amount of XRD divided across the number of index pools. For example, say you invest 1000 XRD into a index fund consisting of 10 tokens. Each token index would receive 100 XRD worth of tokens. 100 Indexer Tokens would be minted. 100 XRD tokens will be swapped for each resource that makes up the index fund and deposited in their vaults. 

When users redeem their Indexer Tokens, an equal number of XRD worth of tokens will be taken from each index pool. Continuing with the same example above, the user presents 100 Indexer Tokens for redemption. 100 XRD worth of tokens will be taken from each index pool swapped to XRD and returned to the user. The user will receive their portion of fees generated from arbitrage and flash loans. 

## Getting Started

1. Lets start off by creating 4 new accounts so we can test the Indexer Protocal. 

```sh
resim reset
OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP3=$(resim new-account)
export PRIV_KEY3=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS3=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP4=$(resim new-account)
export PRIV_KEY4=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS4=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
```

2. Account 1 will be used to set up Ociswap, RaDEX, Oracle1, and Oracle2 and create our first index fund. The other accounts will represent other 
users who invest in the newly created fund.

```sh
resim set-default-account $ACC_ADDRESS1 $PRIV_KEY1
```

3. We will need to create some tokens to add to the index fund. The file [`token_creation.rtm`](./transactions/token_creation.rtm) 
contain the instructions needed for account 1 to create 9 different tokens. To run the transaction file and export values to 
variables run the following command: 

```sh
OP2=$(resim run transactions/token_creation.rtm)
export BITCOIN=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export LITECOIN=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export XRP=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
export DOGECOIN=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '4q;d')
export MONERO=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '5q;d')
export TETHER=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '6q;d')
export BNB=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '7q;d')
export CARDANO=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '8q;d')
export QUANT=$(echo "$OP2" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '9q;d')
export XRD=030000000000000000000000000000000000000000000000000004
```

4. We can now publish the Indexer package and also instantiate a new Indexer, Ociswap, RaDEX, Oracle1, and Oracle2 component by running the following commands:
We are exporting the component addresses for each blueprint and the Indexer Token resource address to variables.

```sh
PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
INDEXER_OP=$(resim call-function $PACKAGE Indexer new)
export INDEXER_COMPONENT=$(echo "$INDEXER_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
export INDEXER_TOKEN=$(echo "$INDEXER_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export FLASH_TOKEN=$(echo "$INDEXER_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
OCI_CP_OP=$(resim call-function $PACKAGE Ociswap new)
export OCI_COMPONENT=$(echo "$OCI_CP_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
RADEX_CP_OP=$(resim call-function $PACKAGE Radex new)
export RADEX_COMPONENT=$(echo "$RADEX_CP_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
ORACLE1_CP_OP=$(resim call-function $PACKAGE Oracle1 new)
export ORACLE1_COMPONENT=$(echo "$ORACLE1_CP_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
ORACLE2_CP_OP=$(resim call-function $PACKAGE Oracle2 new)
export ORACLE2_COMPONENT=$(echo "$ORACLE2_CP_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```
5. Lets quickly set up Ociswap and Radex with some liquidity pools so we can use them to trade tokens.

```sh
BITCOIN_POOL1=$(resim call-method $OCI_COMPONENT create_pool $BITCOIN)
export BITCOIN_POOL1=$(echo "$BITCOIN_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
LITECOIN_POOL1=$(resim call-method $OCI_COMPONENT create_pool $LITECOIN)
export LITECOIN_POOL1=$(echo "$LITECOIN_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
XRP_POOL1=$(resim call-method $OCI_COMPONENT create_pool $XRP)
export XRP_POOL1=$(echo "$XRP_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
DOGECOIN_POOL1=$(resim call-method $OCI_COMPONENT create_pool $DOGECOIN)
export DOGECOIN_POOL1=$(echo "$DOGECOIN_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
MONERO_POOL1=$(resim call-method $OCI_COMPONENT create_pool $MONERO)
export MONERO_POOL1=$(echo "$MONERO_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
TETHER_POOL1=$(resim call-method $OCI_COMPONENT create_pool $TETHER)
export TETHER_POOL1=$(echo "$TETHER_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
BNB_POOL1=$(resim call-method $OCI_COMPONENT create_pool $BNB)
export BNB_POOL1=$(echo "$BNB_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
CARDANO_POOL1=$(resim call-method $OCI_COMPONENT create_pool $CARDANO)
export CARDANO_POOL1=$(echo "$CARDANO_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
QUANT_POOL1=$(resim call-method $OCI_COMPONENT create_pool $QUANT)
export QUANT_POOL1=$(echo "$QUANT_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
XRD_POOL1=$(resim call-method $OCI_COMPONENT create_pool $XRD)
export XRD_POOL1=$(echo "$XRD_POOL1" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

BITCOIN_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $BITCOIN)
export BITCOIN_POOL2=$(echo "$BITCOIN_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
LITECOIN_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $LITECOIN)
export LITECOIN_POOL2=$(echo "$LITECOIN_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
XRP_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $XRP)
export XRP_POOL2=$(echo "$XRP_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
DOGECOIN_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $DOGECOIN)
export DOGECOIN_POOL2=$(echo "$DOGECOIN_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
MONERO_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $MONERO)
export MONERO_POOL2=$(echo "$MONERO_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
TETHER_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $TETHER)
export TETHER_POOL2=$(echo "$TETHER_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
BNB_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $BNB)
export BNB_POOL2=$(echo "$BNB_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
CARDANO_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $CARDANO)
export CARDANO_POOL2=$(echo "$CARDANO_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
QUANT_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $QUANT)
export QUANT_POOL2=$(echo "$QUANT_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
XRD_POOL2=$(resim call-method $RADEX_COMPONENT create_pool $XRD)
export XRD_POOL2=$(echo "$XRD_POOL2" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')
```
6. Now lets add some tokens to the newly created pools.

```sh
resim call-method $OCI_COMPONENT add_liquidity 1000000,$BITCOIN
resim call-method $OCI_COMPONENT add_liquidity 1000000,$LITECOIN 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$XRP 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$DOGECOIN 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$MONERO 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$TETHER 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$BNB 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$CARDANO 
resim call-method $OCI_COMPONENT add_liquidity 1000000,$QUANT 
resim call-method $OCI_COMPONENT add_liquidity 50000,$XRD 

resim call-method $RADEX_COMPONENT add_liquidity 1000000,$BITCOIN
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$LITECOIN 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$XRP 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$DOGECOIN 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$MONERO 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$TETHER 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$BNB 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$CARDANO 
resim call-method $RADEX_COMPONENT add_liquidity 1000000,$QUANT 
resim call-method $RADEX_COMPONENT add_liquidity 50000,$XRD 
```

7. Lets set some price for the Oracle component. Note the prices for Oracle2 are set slightly lower.  We will use this to demonstrate an arbitrage trade.  

```sh
resim call-method $ORACLE1_COMPONENT set_price $BITCOIN 21300
resim call-method $ORACLE1_COMPONENT set_price $LITECOIN 54
resim call-method $ORACLE1_COMPONENT set_price $XRP .34
resim call-method $ORACLE1_COMPONENT set_price $DOGECOIN .07
resim call-method $ORACLE1_COMPONENT set_price $MONERO 146
resim call-method $ORACLE1_COMPONENT set_price $TETHER 1
resim call-method $ORACLE1_COMPONENT set_price $BNB 286
resim call-method $ORACLE1_COMPONENT set_price $CARDANO .45
resim call-method $ORACLE1_COMPONENT set_price $QUANT 105
resim call-method $ORACLE1_COMPONENT set_price $XRD .6

resim call-method $ORACLE2_COMPONENT set_price $BITCOIN 21000
resim call-method $ORACLE2_COMPONENT set_price $LITECOIN 50
resim call-method $ORACLE2_COMPONENT set_price $XRP .30
resim call-method $ORACLE2_COMPONENT set_price $DOGECOIN .05
resim call-method $ORACLE2_COMPONENT set_price $MONERO 140
resim call-method $ORACLE2_COMPONENT set_price $TETHER .96
resim call-method $ORACLE2_COMPONENT set_price $BNB 280
resim call-method $ORACLE2_COMPONENT set_price $CARDANO .40
resim call-method $ORACLE2_COMPONENT set_price $QUANT 100
resim call-method $ORACLE2_COMPONENT set_price $XRD .6
```

8. Now we need to add the Ociswap, RaDEX, Oracle1, and Oracle2 component address to the Indexer component, add the Oracle1 component address to the Ociswap component, and Oracle2 component address to the RaDEX component.  

```sh
resim call-method $INDEXER_COMPONENT oci_address $OCI_COMPONENT
resim call-method $INDEXER_COMPONENT oracle1_address $ORACLE1_COMPONENT
resim call-method $INDEXER_COMPONENT radex_address $RADEX_COMPONENT
resim call-method $INDEXER_COMPONENT oracle2_address $ORACLE2_COMPONENT
resim call-method $OCI_COMPONENT oracle1_address $ORACLE1_COMPONENT
resim call-method $RADEX_COMPONENT oracle2_address $ORACLE2_COMPONENT
```

9. Now we can finally create our custom index fund. We will add 10 tokens to our index fund.

```sh
resim call-method $INDEXER_COMPONENT create_index_pool $BITCOIN
resim call-method $INDEXER_COMPONENT create_index_pool $LITECOIN
resim call-method $INDEXER_COMPONENT create_index_pool $XRP
resim call-method $INDEXER_COMPONENT create_index_pool $DOGECOIN
resim call-method $INDEXER_COMPONENT create_index_pool $MONERO
resim call-method $INDEXER_COMPONENT create_index_pool $TETHER
resim call-method $INDEXER_COMPONENT create_index_pool $BNB
resim call-method $INDEXER_COMPONENT create_index_pool $CARDANO
resim call-method $INDEXER_COMPONENT create_index_pool $QUANT
resim call-method $INDEXER_COMPONENT create_index_pool $XRD
```

10. Lets switch to account 2, 3, and 4 and deposit 20000, 30000, and 40000 XRD into the Indexer protocol

```sh
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
resim call-method $INDEXER_COMPONENT deposit 10000,$XRD
resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
resim call-method $INDEXER_COMPONENT deposit 20000,$XRD
resim set-default-account $ACC_ADDRESS4 $PRIV_KEY4
resim call-method $INDEXER_COMPONENT deposit 30000,$XRD
```

11. Let check to see the index fund token balances.

```sh
resim call-method $INDEXER_COMPONENT show_index_pool
```

12. Lets generate some yield using the index pool tokens and performing arbitrage trades with tokens trading at a discount on RaDEX. We will sell the tokens on Ociswap and buy them back cheaper on RaDEX.

```sh
resim call-method $INDEXER_COMPONENT arb_oci_radex $LITECOIN
resim call-method $INDEXER_COMPONENT arb_oci_radex $BITCOIN
resim call-method $INDEXER_COMPONENT arb_oci_radex $CARDANO
resim call-method $INDEXER_COMPONENT arb_oci_radex $QUANT
resim call-method $INDEXER_COMPONENT arb_oci_radex $XRP
```

13. Let take out a few flash loans using the XRP, DOGECOIN, and CARDANO index pools

```sh
resim run transactions/flash_loan1.rtm
resim run transactions/flash_loan2.rtm
resim run transactions/flash_loan3.rtm
```

14. Lets redeem the Indexer Tokens for accounts 2, 3, and 4.

```sh
resim call-method $INDEXER_COMPONENT withdraw 3000,$INDEXER_TOKEN
resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
resim call-method $INDEXER_COMPONENT withdraw 2000,$INDEXER_TOKEN
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
resim call-method $INDEXER_COMPONENT withdraw 1000,$INDEXER_TOKEN
```

15. Lets check the account balance of accounts 2, 3, and 4. Each account balance starts with 1 million XRD. Anything in addition to this amount represents the fees accrued. The fees accrued are proportional to the initial amount deposited by accounts 2, 3, and 4. Account 4 accrued fees should be equal to the sum of account 2 and 3 accrued fees. Account 3 accrued fees should be double that of Account 2. 

```sh
resim show $ACC_ADDRESS2
resim show $ACC_ADDRESS3
resim show $ACC_ADDRESS4
```

16. Useful methods...

    16.1 Show token balances in index pools

    ```sh 
    resim call-method $INDEXER_COMPONENT show_index_pool
    ```

    16.2 Show token balance in Ociswap liquidity pools

    ```sh
    resim call-method $OCI_COMPONENT show_liquidity_pool
    ```

    16.3 Show token balanace in RaDEX liquidity pools

    ```sh
    resim call-method $RADEX_COMPONENT show_liquidity_pool
    ```
    
