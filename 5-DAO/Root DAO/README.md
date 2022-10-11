# Root DAO

Hello, my names is Tomas. I would like to guide you through my Root DAO idea.

**_Disclaimer_**
*Unfortunately I did not have enough time to completely code my idea so I am submitting at least the concept without actual code.
Although this entry won´t be eligible for participating in the Scrypto DAO challenge, because it does not contains code, I still hope that it may serve as an inspiration for others and I will be eligible for at least early adopter NFT badge.*

> following section contains questions one should ask and reasons why I have designed my DAO the way I did
#### DAO and Justice

For a longe time I thought about DAO structure that would not bee ruled by whales.
Some DAOs implement quadratic principle of 1 token = 1 vote, 4 tokens = 2 votes to prevent whales from ruling the protocol.
However as a student of economical oriented programme on uni I have tried to regard the DAO as a unit securing and managing wealth.
When you look at quadratic principle of DAO voting, you might realize the correlation between quadratic principle and proggresive taxation in the real world.

For the ones who does not know what progressive taxes are:
*A progressive taxation imposes a lower tax rate on low-income earners than on those with a higher income. This is usually achieved by creating tax brackets that group taxpayers by income ranges. 


Nevertheless I think that fair is that who earns more, pays more in total but the percentage should be same for everybody.
When the tax rate is flat, everybody pays on taxis same percentage although 20 % form 7 5000 USD is in total different amount than 20 % from 15 000 USD.
Progressive tax rates disincentivize honest work because the more you earn, the less of your income goes to our pocket and that is not fair! 

Maybe, you are asking yourself, what the heck has this in common with DAO? Well the answer is simple. If we take DAO as a protocol securing, managing and multiplying wealth, then should be the ones that provided the least value (in USD, crypto, ...) incentivized the most? I do not think so. The think is that the ones that provided the least value are the ones at lowest risk. On the other hand big players that provided a lot of value have a lot in stakes. Consequently, this prevents them from malicious behaviour because thay have their skin in the game. That being said, I do not think that quadratic principle is the ideal solution. You incentivize the "poor" such as me and disincentivized the ones whose skin is in a game. 

However crypto as such is about decentralization and power of people and community. So I do not think that 1 token = 1 vote principle is optimal solution as well. That would lead to a protocol ruled by whales. I do not think, that it must be neccesarilly bad, however I think that this is something crypto community tries to avoid. Because then the power is concentrated in hands of very narrow group of "elites" and the rest, which accounts for majority of users, has no power over the protocol.

You might ask: Okay Tomas, so what you wanna do when you do not want to disincentivize wealth and honest work but want to avoid a protocol that is ruled by the whales.
Well, I think that ideal answer does not exist. However I have tried to look at the solution from different point of view. Rather then asking what to prevent or who to disincentivize I had asked myself: " Who I want to incentivize. Who is the perfect DAO user, that adds the most value to the protocol?

The answer is as usually some sort of trade-off. Whales usually provides value in form of capital. Small users usually provide value in form of human capital such ass ideas, discussions, community moderation and community engagement. So the ideal user should be somewhere in an equlibrium of these 2 sources of added value. Thus being said, the ideal user base is the middle class. Those are the people that have their skin in the game however does not have enough resources to rule the protocol on their own.

## Voting power concepts
Vote weight should be combination of regressive and progressive concepts. What I mean by this is that 1 token will be equal to fraction of a vote, depending on the fact how many tokens user has. 

I will take as an example DAO with 1 000 000 tokens. If we assume that our DAO expected user base is about 10 K people then optimal amount of tokens owned by 1 person would be 1 M / 10 K = 100 tokens per person. Consequently 100 per person we make our optimal amount and that will be our middle class. We do  not expect everybody to have same amount. That is not our aim. Our goal is that about 60 % has circa 100 tokens.

Ceteris paribus, we want the people holding 100 tokens exactly incentivize the most and everybody having more or less incentivize less. By incentivize I mean giving the holders different token = Xxvote. Then we want more incentivize the smallest holders than the whales. So that is our hierarchy.

We will try to achieve our goal of delegating the most power to middle class by following series of points and functions going through them.
The vote weight is calculated for every token separately.

###### Interesting points table
<img src="https://github.com/tomashla/root_dao/blob/main/interesting%20points.PNG" alt="Table of interesting points" title="Table of interesting points" width="360">

###### Functions table
<img src="https://github.com/tomashla/root_dao/blob/main/funkce.PNG" alt="Table of functions" title="Table of functions" width="600">

###### Very simple chart how the result function/curve looks like
<img src="https://github.com/tomashla/root_dao/blob/main/graf0.PNG" alt="Vote weight chart" title="Vote weight chart" >

## Features of Root DAO
*This is just list of features / capabilities that are for DAO in my opinion nice to have

1) Option to delegate votes to someone else => if proposals are regular, not everybody has time to check each
2) Option to cancel his delegation and vote otherwise if the person does not agree with the delegate 
3) On-chain governance 
(code is in the proposal and when passed it will be uploaded directly onto the legder - I think that in Scrypto it could be done by having 2 Components - one storing the actual DAO state and the other would be for everythng else - holding the proposal code, voting and with right to change the DAO Component by uploading proposal code if the proposal gets passed)


