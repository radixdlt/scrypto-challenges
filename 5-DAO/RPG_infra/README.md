Unfortunately, the .rs files associated with this submission are not actual code but are my trash notes. I'm submitting this entry anyway because I think the idea is interesting (see below). I do intend to get some working code going for this one at some point. It seems a good project to start with. To that end, I shall stop writing README.md files and start doing some Scrypto tutorials...

# RPG_infra
## Radical Public Good infrastructure
This thing consists of three components. First, a *multivault* component that accepts deposits/donations of any kind of fungible asset from anyone. Next, a *multivault_controller* component that accepts and burns 'wrapper' tokens representing proportionate ownership of of the contents of the *multivault*. The *multivault controller* returns an appropriate fraction of the *multivault* contents to the caller using a badge to authorise the withdrawal(s). Finally, the *RPG_infra* component, which instantiates the *multivault* and *multivault_controller* components, and issues tranches of 'wrapper' tokens with an owner-specified delay before the issue of each tranche.

!image()
## Usecase 1: fundraising
An organisation wishing to raise funds invites donations of any kind of fungible asset to the *multivault* component. After the specified delay the *RPG_infra* component issues the organisation with an initial tranche of 'wrapper' tokens. The organisation then sells the 'wrapper' tokens on the open market for at least as much as the underlying value of the assets contained in the *multivault*. The organisation does not need to liquidate the multi-asset donations itself. Owners of 'wrapper' tokens can hold or trade them, or burn them in exchange for underlying assets at any time using the *multivault_controller* component.
### Participant motivations: donors
* Donors may be motivated by the nature of the public good.
* Donors may be other crypto projects and may wish to donate their native token without immediately dumping it on the open market. The maximum possible release-rate from the *multivault* is constrained by the issuance-interval of tranches of 'wrapper' token. Moreover, traders may choose to hold the 'wrapper' tokens without ever burning them for the underlying assets.
* If the organisation accepting donations were a registered charity, then donors may wish to donate in order to offset local tax obligations. Donations in the form of any cryptoasset may be a convenient way to acheive this.

Note: as issuance and sale of 'wrapper' tokens proceeds over time, donors to the *multivault* are effectively sending an increasing proportion of their donations to holders/traders of the 'wrapper' token rather than to the organisation itself. This may alter the behaviour of market participants in both predictable and unpredictable ways.
### Participant motivations: direct 'wrapper' token buyers
* Individuals buying the 'wrapper' token directly from the organisation may be motivated by the nature of the public good.
* Buying the 'wrapper' tokens directly from the organisation ensures that the organisation receives the entire purchase price immediately.
* If the organisation is a registered charity, then purchase of 'wrapper' tokens with another asset may be considered a crypto-to-crypto exchange not representing a disposal for tax purposes (though this would require legal precedent).
* Buyers of 'wrapper' tokens may beleive that the value of the *multivault* contents is likely to appreciate over time (through further donations or through appreciation in the value of the existing contents).
* Buyers of 'wrapper' tokens may beleive that their value exceeds - or  is likely to exceed - the value of the claim to the *multivault* contents they represent. That is, that their identification with the organisation, some future utility, etc. confers 'added' value.
### Participant motivations: traders
* Traders may be motivated by the nature of the public good. They may beleive that their ownership of the 'wrapper' token signifies their identification with the organisation.
* Traders may beleive that the value of the 'wrapper' tokens will appreciate over time (see above).
* Traders may wish to acquire 'wrapper' tokens in order to burn them with the *multivault_controller* component in exchange for the underlying assets. Such traders establish a 'floor price' for the 'wrapper' token by buying it on the open market, whilst reducing the total supply of 'wrapper' tokens every time they carry out a burn-transaction.
### Participant motivations: the organisation
* The organisation is motivated to bring a public good to fruition.
* The *RPG_infra* components facilitate donations by accepting donaitons of any asset.
* The *RPG_infra* components facilitate donations by broadening the potential donor pool (to include crypto projects wishing to 'soft-burn' their native token, individuals wishing to offset their tax obligations, etc.)
* As well as direct financial benefits from accrued through sale of 'wrapper' tokens, the organisation benefits from the ecosystem of traders motivated to spread information pertaining to the virtue of the particular public good. 

Broadly, the reason for an organisation to use this mode of fundraising is with the intent of aligning the behavious of a wide decentralised community of participants who may have differing interests and motivations.
## Usecase 2: internal market equipment booking system
### Problem
This usecase uses the same component-primatives as above, but in a different way and to address a different problem. The problem concerns the efficient use of resources by large institutions. Some large institutions - such as Universities - operate internal markets for facilities such as equipment. Semi-autonomous employee-fundholders - for example research groups - pay the institution to access its equipment. The funds received by the institution are used to cover running costs (which encompasses support staff costs, service contracts, reagent costs, repair, electricity, etc.). However, where running costs of specific items of equipment are used to price their booking-costs, internal market inefficiencies can arise.

An example of the progression of an internal market inefficiency is as follows. First the cost of booking an item of equipment is assigned based on its estimated useage over a given timeperiod. Then, useage over that timeperiod is less than expected. Then, cost of booking is increased to cover the deficit. Then, useage falls further due to the increased price; perhaps an external provider is now cheaper. Then, price is increased again. Then, the item falls into disuse, or breaks without a service contract in place. Then, the item is discarded.

Clearly, the aim of the institution should be to maximise efficient equipment use by the semi-autonomous-employee-fundholders as well as to cover costs. Therefore, a better approach would be to reduce the booking costs of an item when its useage falls. If the useage of an item continues to fall even when it can be booked cheaply, then this is a good indication of its obsolescence. However, a reduced price might alternatively spark renewed utilisation. Moreover, semi-autonomous-employee-fundholders with limited funds may for a time be able to access equipment that they would formerly have been 'priced-out' of. Thus, internal market efficiency could be improved.

The *RPG_infra* components could support the second model of equipment booking according to the following workflow.

### Workflow
![image](https://github.com/marktwh/scrypto-challenges/blob/main/5-DAO/RPG_infra/booking_system.png)
* The institution instantiates a set of *RPG_infra* components for each individual item of equipment.
* The institution mints a generic 'machine-time' tokens (for internal use only) and deposits these into the *multivault*s corresponding to each item, in proportion to the relative running costs of each item.
* The institution is issued specific 'wrapper' tokens for each item of equipment. These are then used by the *booking_system* (not implemented), within which they represent the right to book each item for a set amount of time.
* When semi-autonomous users pay to book an item, the *booking system* automatically burns the appropriate quantity of'wrapper' tokens via the appropriate *multivault_controller* and collects 'machine-time' tokens for audit purposes.
* Accrued 'machine-time' tokens are indicative of efficiency of resource-use, institution-wide.
* The institution 'banks' unused 'wrapper' tokens prior to each new issue of each 'wrapper' token (again for audit purposes). The number of unused tokens for each item corresponds to the amount of time for which the item wasn't in use. The amount of time for which the item *was* in use is given by *tokens issued* minus *tokens unused*.
* Upon each new issuance event, the institution re-prices the 'wrapper' tokens representing the costs of booking each item of equipment. Repricing aims to maximise equipment use as well as to cover costs.

Note that in this workflow, the 'machine-time' tokens and 'wrapper' tokens are used within the internal market only, whereas the semi-autonomous users pay for services with 'real' money.
### Rationale
The use of the above workflow provides the institution with live, granular, information. Importantly, data on utilisation of equipment institution-wide is given by the quantity and rate of 'machine-time' token accrual by the *booking system*. By formalising 'machine-time' as a currency-like fungible entity within the internal market, a metric for overall efficient use of resources becomes visible to accounting and auditing processes. It becomes actionable information.

Data on total funds received from semi-autonomous users, funds recieved for use of each item of equipment, utilisation of each item, and user-responses to re-pricing events, are all easily accessible. Further, the modular nature of the workflow means that it is straightforward to add or remove equipment from the system dynamically. Furthermore, the data can be used to refine the parameters for re-pricing, equipment purchase, and equipment retirement, to acheive an optimal balance of internal-revenue and efficient equipment use over time.

Notably, many institutions may prefer to keep the details of their accounts and internal markets private. However, making these operational details public on a distributed ledger could be of particular benefit to an organisation focussed on acheving a specific public good. The open demonstration of efficient utilisation of donated funds by such an organisation could encourage more donors to donate. Further, detailed voluntary open accounting may improve the chances of a public-good-enterprise being officially recognised as a charity. Finally, a public-public-good-ecosystem could comprise a resource from which pre-refined parameters for resource-management componentry could be selected.




