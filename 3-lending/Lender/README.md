# scrypto-challenges

'Lender' is aiming to make lending easy for everyone and to provide a safe way to lend money to borrowers by establishing a way for both clients and members
to gain fidelity levels inside a lending network so that they can borrow/lend with less collateral.

## Lender
'Lender' is the main component of this project. It combines together 3 additional components:
'LockedLoanCollateral', 'TrustedPartnerNetwork' and 'DepositContributors' into a power tool for lending that can be used by anyone without any KYC
The design of this blueprint is in line with Scrypto 0.5 where dapps can be composed of multiple small components
each with its own purpose, that are maintained and tested as separate projects. 
The lender component is acting as a liason between these three components by connecting them together and providing the api
for borrowers to interact with them
While LockedLoanCollateral and DepositContributors are used as intra-package (composition) and the Lender instantiates them locally, 
The TrustedPartnerNetwork is designed as a cross-package communication between these two blueprints because the 
TrustedPartnerNetwork needs its own stored across multiple lenders in order to establish a measure of trustworthiness 
for both clients and members. 
Using a TrustedPartnerNetwork is also optional for lender, they can accept clients that are not part of the network and 
lenders are not required to be part of any network in order to provide lending services. But this will be at their own risk 
because without a central component to monitor if a client is trustworthy, they cannot provide loans with less collateral 
as they have no guarantee that a client will pay his loan.
The Lender also has the option to make custom offers to clients that he deems trustworthy (or friends that are in need) 
and this can be done by minting a custom lending offer nft and offering it to those clients
The lender component is also designed to support any kind of collateral as long as it can be evaluated through external means (oracle) 
and the LockedLoanCollateral just keeps track of the progress of the refund by counting the number of tokens needed to unlock the collateral.
The endpoint of the borrower is to refund enough tokens in order to unlock his collateral, if the time allocated for the loan has passed
and the amount he has to pay is greater than the evaluation of the collateral, then the borrower looses his collateral and the loan ca be liquidated. 
The logic for unlocking the collateral and increasing/decreasing the amount the borrower needs to refund based on early/late installment payments 
is all separated into the 'lockedLoanCollateral' component for further reause by other dapps. 
Enabling everyone to become a lender is the purpose of DeFi but one of the downsides besides the risk is that it tends to become 
fragmented and taking large loans would mean to take multiple small loans, which is not in the interest of borrowers. 
This is the role of the 'DepositContributors' components - which aims to provide benefits to both contributors and the owner. 
The contributors provide funds to this 'Lender' and get a reward based on how many loans are taken with their funds. The reward is based on 
the percentage of funds that they are contributing to that loans, so everyone gets their fair share. Contributors funds are allocated in 
a cyclic manner so that each contributor gets his turn. 
The owner of the 'DepositContributors' will always have priority, so he has nothing to lose in allowing other contributors. 
The benefit in his case is that if there is a bad client that didn't pay his loan and he caused a loss besides liquidating his collateral, the losses
are split between all of the contributors so he will not be affected that much. 
This component could be improved by providing additional incentives to the owner in the sense of him getting a small percentage of the profit 
from each contributor besides the loss mitigation. 

## LockedLoanCollateral
LockedLoanCollateral is a structure that encapsulated the logic and calculation for repaying loans and keeps the deposit locked until the amount required for unlock is met
The threshold is computed based on the interest rate and is adapted each time a new deposit is made based on early/late installment payments
If the borrower didn't manage to restore the threshold amount until the installment date, then he will have to pay penalty fees 
to account for the additional time the lender is without his funds
Because we have collateral we increase the debt even more as a penalty method if the borrower doesn't meet his deadlines until the loan passes its deadline
and the collateral evaluation is less than the remaining debt. We liquidate the loan when the borrower does not have any more reasons to continue repaying because the 
amount he must repay is larger than the collateral


## DepositContributors
While DeFi will revolutionize banking because everyone will be able to participate in financial apps, a major downside is that tends to become very fragmented
This class aims to fill that gap so that people can join together their funds and provide larger funds for dapps 
If the aim is to replace banks with DeFi, the solution is to provide benefits for ordinary people to join their funds
`DepositContributors` is a struct that manages funds windrawal from a vault that is split between multiple contributors in a similar manner to a cyclic resource allocator
where each contribution occupies a continuous amount of tokens
There's no such thing as a sustainable constant APY, that's why the approach of this struct is that each contributor earns based on how many allocation are made with his contribution, 
just like a shop earns based on the sell amount
In order to make it worth it for the owner of the deposit to accept contributors, he will always have priority to allocations 
For all the other contributors, the algorithm works in a queue manner. Chunks of free tokens are taken from the queue for allocations so that everyone gets their turn in a cyclic manner
The benefit for the owner is that the amount of losses is mitigated because it is split between all participants of the deposit 
The contributors benefit from using the perks of the owner (like a high level trusted network rank)
So it is a win-win for both. 


## TrustedPartnerNetwork
The 'TrustedPartnerNetwork' component is similar to a guild. It represents a group of partners that have trust in each other that they are not bad actors. 
Only leader can invite other members and new leaders are promoted by the votes of at least half the active leaders 
Thea idea behind the trusted partner network is that without KYC you cannot trust clients unless you have a network of trusted partners that are verified and don't act maliciously 
Without this kind of network design, there is no way to guarantee that a client is a friend of a random lender and will do loans back and forth between them to gain fidelity levels 
once the client gained enough fidelity levels to borrow with less collateral then he can go to other members, borrow from them and will keep the borrowed money (which is more than his collateral)
without returning anything. 

The creator of this component will provide a list of fidelity levels that both clients and members will have to gain in order to access better offers
One metric for fidelity levels could be the amount of profit that the client generated inside the network by taking loans for example.
The more profit he generated by taking loans, the further levels he gains and can take loans with less collateral.
Deciding the exact levels and benefits is a tough decision to make and needs to be careful analyzed by the creators based on the risk/reward. 
If clients can gain fidelity levels fast, then they should not have too much collateral reduction. 
The levels should be geared towards certain borrowers. For example, institutions will borrow large sums and generate a huge profit so a 'TrustedPartnerNetwork' for them 
should have levels adjusted to the amount of tokens that big players are borrowing. 

Clients that don't repay their loan have their client_nfts burnt and they will need to start again from 0 in order to gain fidelity levels.
Banning clients is useless because without KYC they can always create a new account and start from there. 
If crypto will be regulated and clients will have a badge that will link to a real-life person (like a nation ID), then banning clients is certainly something that can be done
but as long as there is no way to associated an account to that person, then is no valid use case for banning. 

This struct also has a fallback mechanism for punishing members that helped bad clients gain fidelity levels
We trace the history of the client and register the loss he caused as minus points to the trustworthiness of the member
In order to gain back his trustworthiness he must register profits from other clients to regain enough levels to account for the losses
Clients will be encouraged to loan from members with similar trustworthiness. If a client with higher fidelity will try to loan from a low fidelity member, then
that client will only have a discount proportional to the minimum between his level and the members level 

Improvements in the future:
What can be improved is to add a membership fee and the profits from the membership fees will be used to account for the losses similar to an insurance