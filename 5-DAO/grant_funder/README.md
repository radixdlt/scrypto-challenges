# Grant Funder

Grant Funder allows users to create a grant proposal and receive funding. The proposal owner must put down 50% of the requested grant amount. Funds are released in 4 stages. Each proposal stage is voted on by the community. The proposal owner must show the community their progress, answer questions, and address suggestions that arise, to earn enough votes to proceed to the next stage. If a proposal does not earn enough points in the alloted time, the proposal is burned. Stage 1 releases 20%, stage 2 releases 40%, stage 3 released 70%, and stage 4 releases 100% of the approved funds. 

## Design Details

1. User creates a grant proposal which mints a NFT containing information about the grant and requested XRD amount. The user will receive a receipt NFT referencing the proposal NFT id. 

2. There are two vaults located in the component. Users can vote on a proposal by sending 1 XRD to the vaults. One vault represents a YES vote the other a NO vote. 

3. Votes are counted and if the proposal is > 50% then the proposal NFT data stage is incremented and returned to the proposal vault.

4. The user can use the update_receipt method to see if their proposal was approved. If it was approved their proposal receipt NFT data will be updated. 

5. The user can take their updated proposal receipt NFT and use it to retrieve their funds. 

## Getting Started

```
resim reset
export XRD=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag
```

1. Lets create a new account

```
OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACCOUNT_ADDRESS1=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
```

2. Lets publish the blueprint
```
PK_OP=$(resim publish ".")
echo $PK_OP
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
```

3. Lets fund the Grant Funder Protocal with 700 XRD and instantiate the component
```
COMPONENT_OP=$(resim call-function $PACKAGE GrantFunder new 700,$XRD)
echo $COMPONENT_OP

export COMPONENT=$(echo "$COMPONENT_OP" | sed -nr "s/.* Component: ([[:alnum:]_]+)/\1/p" | sed '1q;d')

export PROPOSAL_RECEIPT=$(echo "$COMPONENT_OP" | sed -nr "s/.* Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
```

4.  Lets create a grant proposal for 200 XRD by putting 100 XRD down.
```
resim call-method $COMPONENT create_proposal 100,$XRD first_grant 200
```

5.  Lets vote of the proposal 
```
resim call-method $COMPONENT vote_yes 1,$XRD
resim call-method $COMPONENT vote_yes 1,$XRD
resim call-method $COMPONENT vote_yes 1,$XRD
resim call-method $COMPONENT vote_yes 1,$XRD
resim call-method $COMPONENT vote_yes 1,$XRD
resim call-method $COMPONENT vote_no 1,$XRD
resim call-method $COMPONENT vote_no 1,$XRD
resim call-method $COMPONENT vote_no 1,$XRD
```

6. Lets grab the proposal NFT id from inside the component

```
resim show $COMPONENT
```

7. We will use the proposal NFT id for the count_votes method.  If the proposal is voted for then the proposal NFT is incremented and placed back in the component.   
```
resim call-method $COMPONENT count_votes ---PASTE NFT ID HERE---
```

8. Lets see if our grant proposal was approved by running the update_receipt method.  If the proposal moved forward the proposal receipt NFT stage data is incremented.   
```
resim call-method $COMPONENT update_receipt 1,$PROPOSAL_RECEIPT 
```

9. Lets retrieve our grant funds using the updated proposal receipt NFT.
```
resim call-method $COMPONENT collect_xrd 1,$PROPOSAL_RECEIPT
```

10. Typically the YES NO vaults will get emptied after each vote.  But for this example we will run the count_votes, update_receipt, and collect_xrd method a few more times just to illustrate the life cycle of the grant proposal.
```
resim call-method $COMPONENT count_votes ---PASTE NFT ID HERE---
resim call-method $COMPONENT update_receipt 1,$PROPOSAL_RECEIPT
resim call-method $COMPONENT collect_xrd 1,$PROPOSAL_RECEIPT
```

