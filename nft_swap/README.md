#NFT SWAP & Royalties 

In this example we will mint an NFT collection, purchase 1 NFT using account 1, and swap them with accounts 2, 3, and 4.  Account 1 will purchase a NFT for 1000 XRD.  Account 1 will swap this NFT with Account 2 for 1000 XRD.  Account 2 will swap with Account 3 for 1000 XRD.  Account 3 will swap with Account 4 for 1000 XRD.  10% of each swap value will be sent to a nft_royalty_vault.  Each seller's address is added to a nft_owners vector.  Nft_shares are minted at a 1:1 ratio to XRD and evenly distributed to all the nft_owners in the vector. Nft mutable metadate "generation" is incremented during each swap. All one-time owners of these NFTs will receive royalties for every future swap.  Orginal owners total royalties will be larger than later owners total royalties.   

##Using transaction manifest files

1. Reset

```
resim reset
```

2. Create 4 accounts 

```
export op1=$(resim new-account)
export pk1=$(echo "$op1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export a1=$(echo "$op1" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
export op2=$(resim new-account)
export pk2=$(echo "$op2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export a2=$(echo "$op2" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
export op3=$(resim new-account)
export pk3=$(echo "$op3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export a3=$(echo "$op3" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
export op4=$(resim new-account)
export pk4=$(echo "$op4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export a4=$(echo "$op4" | sed -nr "s/Account address: ([[:alnum:]_]+)/\1/p")
```

3. Publish

```
resim publish .
```

4. Instantiate component, and export component address to variable  

```
resim run transactions/component.rtm
export component=029bcee04344d0ca6d747e764a64e30a2d01dbc6d940fb1d11fa37
```

4.1 At anytime during this process you can look at each account and component using the following commands

```
resim show $a1
resim show $a2
resim show $a3
resim show $a4
resim show $component
```

5. Puchase NFT

```
resim run transactions/buy_nft.rtm
```

6. Swap Account 1 with Account 2 

```
resim run transactions/swap1.rtm --signers $pk1 --signers $pk2
```

7. Swap Account 2 with Account 3

```
resim run transactions/swap2.rtm --signers $pk2 --signers $pk3
```

8. Swap Account 2 with Account 4

```
resim run transactions/swap3.rtm --signers $pk3 --signers $pk4
```

9. Exchange XRD from nft_royalty_vault using nft_shares

```
resim run transactions/withdraw1.rtm --signers $pk1
resim run transactions/withdraw2.rtm --signers $pk2
resim run transactions/withdraw3.rtm --signers $pk3
```

