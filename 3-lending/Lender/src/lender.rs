use scrypto::prelude::*;
use crate::locked_loan_collateral::{LockedLoanCollateral, NFTLoan, LoanContract};
use crate::deposit_contributors::DepositContributors;
use crate::trusted_network::{TrustedPartnerNetwork, TrustedPartnerNetworkMember};
use std::cmp;

#[derive(NonFungibleData)]
struct SpecialOffer {
    pub interest_rate_reduction : Decimal,
    pub increase_max_loan_duration : u64,
    pub increase_max_loan_amount : Decimal, // in percentage
}


blueprint! {

    /// 'Lender' is the main component of this project. It combines together 3 additional components:
    /// 'LockedLoanCollateral', 'TrustedPartnerNetwork' and 'DepositContributors' into a power tool for lending that can be used by anyone without any KYC
    /// The design of this blueprint is in line with Scrypto 0.5 where dapps can be composed of multiple small components
    /// each with its own purpose, that are maintained and tested as separate projects. 
    /// The lender component is acting as a liason between these three components by connecting them together and providing the api
    /// for borrowers to interact with them
    /// While LockedLoanCollateral and DepositContributors are used as intra-package (composition) and the Lender instantiates them locally, 
    /// The TrustedPartnerNetwork is designed as a cross-package communication between these two blueprints because the 
    /// TrustedPartnerNetwork needs its own stored across multiple lenders in order to establish a measure of trustworthiness 
    /// for both clients and members. 
    /// Using a TrustedPartnerNetwork is also optional for lender, they can accept clients that are not part of the network and 
    /// lenders are not required to be part of any network in order to provide lending services. But this will be at their own risk 
    /// because without a central component to monitor if a client is trustworthy, they cannot provide loans with less collateral 
    /// as they have no guarantee that a client will pay his loan.
    /// The Lender also has the option to make custom offers to clients that he deems trustworthy (or friends that are in need) 
    /// and this can be done by minting a custom lending offer nft and offering it to those clients
    /// The lender component is also designed to support any kind of collateral as long as it can be evaluated through external means (oracle) 
    /// and the LockedLoanCollateral just keeps track of the progress of the refund by counting the number of tokens needed to unlock the collateral.
    /// The endpoint of the borrower is to refund enough tokens in order to unlock his collateral, if the time allocated for the loan has passed
    /// and the amount he has to pay is greater than the evaluation of the collateral, then the borrower looses his collateral and the loan ca be liquidated. 
    /// The logic for unlocking the collateral and increasing/decreasing the amount the borrower needs to refund based on early/late installment payments 
    /// is all separated into the 'lockedLoanCollateral' component for further reause by other dapps. 
    /// Enabling everyone to become a lender is the purpose of DeFi but one of the downsides besides the risk is that it tends to become 
    /// fragmented and taking large loans would mean to take multiple small loans, which is not in the interest of borrowers. 
    /// This is the role of the 'DepositContributors' components - which aims to provide benefits to both contributors and the owner. 
    /// The contributors provide funds to this 'Lender' and get a reward based on how many loans are taken with their funds. The reward is based on 
    /// the percentage of funds that they are contributing to that loans, so everyone gets their fair share. Contributors funds are allocated in 
    /// a cyclic manner so that each contributor gets his turn. 
    /// The owner of the 'DepositContributors' will always have priority, so he has nothing to lose in allowing other contributors. 
    /// The benefit in his case is that if there is a bad client that didn't pay his loan and he caused a loss besides liquidating his collateral, the losses
    /// are split between all of the contributors so he will not be affected that much. 
    /// This component could be improved by providing additional incentives to the owner in the sense of him getting a small percentage of the profit 
    /// from each contributor besides the loss mitigation. 
    /// The `Lender` type is a struct that contains a `Vault` for the lender's funds, a
    /// `DepositContributors` for the funds contributed by other users, a `Vault` for the NFTs that
    /// represent the contributors, a boolean that indicates if the lender allows contributors, a `HashMap`
    /// for the loans, a `Vec` for the liquidated collaterals, a `Decimal` for the interest rate, a
    /// `Decimal` for the missed installment penalization, a `u64` for the max loan duration,
    /// 
    /// Properties:
    /// 
    /// * `auth_vault`: Vault,
    /// * `deposit`: This is the amount of funds that the lender has deposited in the lending contract.
    /// * `contributor_nft`: 
    /// * `allow_contributors`: if true, the lender will accept funds from other contributors. If false, the
    /// lender will only accept funds from himself.
    /// * `loans`: This is a hashmap that stores the loans that are currently active. The key is the
    /// NonFungibleId of the loan. The value is the LockedLoanCollateral.
    /// * `liquidated_collaterals`: This is the vault where the lender will store the NFTs that were used 
    /// as collateral for the loans and were liquidated
    /// * `interest_rate`: the interest rate that the lender will charge for the loan.
    /// * `missed_installment_penalization`: the percentage of the collateral that is lost if the borrower
    /// misses an installment
    /// * `max_loan_duration`: the maximum duration of a loan in epochs
    /// * `installment_frequency`: the number of epochs between each installment payment
    /// * `max_loan_percentage`: This is the maximum percentage of the collateral that can be borrowed. 0
    /// means that the borrower should provide 100% of the collateral.
    /// * `trusted_network_memberships`: this is the list of trusted network memberships that the lender
    /// has. The lender can be a member of multiple trusted networks.
    /// * `client_member_nfts`: This is a map of the client nfts that are kept so that the clients cannot 
    /// borrow money from other lender while he still has this one active. The key is
    /// the client's address and the value is the vault where the member nft is stored.
    /// * `custom_lending_offer_address`: ResourceAddress, // resource address for creating custom lending
    /// offer to some clients. The idea is that some people may want to lend some money to a friend in need
    /// and they can do that by making a special offer to that friend
    struct Lender {
        auth_vault: Vault,
        
        // funds contribution for lending tokens
        deposit : DepositContributors,
        contributor_nft : Vault,
        allow_contributors : bool,

        // loan data
        loans : HashMap<NonFungibleId, LockedLoanCollateral>,
        liquidated_collaterals : Vec<Vault>, // nfts or other coins that are used as collateral
        interest_rate : Decimal,// measured in percent
        missed_installment_penalization : Decimal, // measured in percent
        max_loan_duration: u64,  // measure in epochs
        installment_frequency : u64,
        max_loan_percentage : Decimal, // percentage of the collateral that can be borrowed. 0 means that the borrower should provide 100% of the collateral 

        // trusted network membership nft
        trusted_network_memberships : Vec<Vault>, // TODO add support for multiple networks, for the moment we just use the first one
        // Clients can't borrow from two members at the same time. This is a protection mechanism because without a KYC, no client can be trusted. 
        // Someone bad intended could borrow using some collateral, then go to another member and use the lended money to borrow a higher sum and so on without refunding any of his borrowed money
        // this would results in a loss for every lender 
        // member_nfts are returned once the loan is finished or burned if they didn't finish the loan
        client_member_nfts : HashMap<NonFungibleId, Vault>,
        
        // offers 
        custom_lending_offer_address: ResourceAddress, // resource address for creating custom lending offer to some clients 
    }

    impl Lender {

        pub fn instantiate(name : String, initial_liquidity: Bucket) -> ComponentAddress 
        {
            let auth_token = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin authority for loaning")
                .burnable(rule!(allow_all), LOCKED)
                .initial_supply(Decimal::one());

            let offer_nft_resource = ResourceBuilder::new_non_fungible()
            .mintable(rule!(require(auth_token.resource_address())), LOCKED) // only the owner can mint new leader nfts
            .burnable(rule!(allow_all), LOCKED) // if someone doesn't want the offer, he can burn it
            .metadata("LenderName", name.clone())
            .updateable_non_fungible_data(rule!(deny_all), LOCKED) // nobody can change the offer data
            .restrict_withdraw(rule!(deny_all), LOCKED) // this resource is soulbound and cannot be traded
            .no_initial_supply();

            let access_rules: AccessRules = AccessRules::new()
            .method("add_trusted_partner_network_member_badge", rule!(require(auth_token.resource_address())))
            .method("recommend_for_promotion", rule!(require(auth_token.resource_address())))
            .method("mint_custom_lending_offer", rule!(require(auth_token.resource_address())));

            let (deposit_component, nft) = DepositContributors::instantiate(initial_liquidity, auth_token.resource_address());

            Self {
                auth_vault: Vault::with_bucket(auth_token),
                deposit : deposit_component.into(),
                contributor_nft : Vault::with_bucket(nft),
                allow_contributors : true,
                loans : HashMap::new(),
                liquidated_collaterals : Vec::new(),
                installment_frequency : 480, // there are 1440 minutes in a day * 30 / 90 minutes (which is the max for the epoch)
                max_loan_percentage : dec!("0.1"), // by default clients can only get 1% more than what they deposit. They need to build up trust with a lending partner network in order to borrow more
                interest_rate : dec!("0.01"), // default interest rate is 1% per installment
                missed_installment_penalization : dec!("0.05"), // if a borrower doesn't pay his installment on time, a 5% 
                max_loan_duration : 2880, // ~ 6 months
                trusted_network_memberships : Vec::new(),
                client_member_nfts : HashMap::new(),
                custom_lending_offer_address : offer_nft_resource
            }
            .instantiate()
            .add_access_check(access_rules)
            .globalize()
        }

        /// > If the contributor_nft is empty, create a new deposit, otherwise increase the deposit
        /// 
        /// Arguments:
        /// 
        /// * `tokens`: The amount of tokens to contribute to the deposit.
        /// * `contributor_nft`: The NFT that the contributor is using to contribute to the deposit.
        /// 
        /// Returns:
        /// 
        /// The return value is a Bucket with the updated contributor_nft
        pub fn contribute_to_deposit(&mut self, tokens : Bucket, contributor_nft : Bucket) -> Bucket {
            ComponentAuthZone::push(self.auth_vault.create_proof());
            if contributor_nft.is_empty() {
                assert!(self.allow_contributors, "The owner of this component does not allow contributors");
                return self.deposit.create_deposit(tokens)
            }
            else {
                assert!(contributor_nft.resource_address() == self.contributor_nft.resource_address(), "invalid contributor nft");
                return self.deposit.increase_deposit(tokens, contributor_nft)
            }
        }

        /// > Function called by contrbitors in order to stop contributing to the deposit of thise component and get their tokens
        /// If the tokens are still in use, then the contributor has to wait untill the funds are unlocked
        /// 
        /// Arguments:
        /// 
        /// * `contributor_nft`: The NFT that the contributor is using to contribute to the deposit.
        /// 
        /// Returns:
        /// 
        /// The return value is a Bucket with the updated contributor_nft
        pub fn close_deposit_constribution(&mut self,contributor_nft : Bucket) -> Bucket {
            assert!(!self.contributor_nft.is_empty(), "The owner of this component does not allow contributors");
            ComponentAuthZone::push(self.auth_vault.create_proof());
            
            return self.auth_vault.authorize(|| {self.deposit.close_deposit(contributor_nft)})
        }

        /// The function takes a collateral and returns a tuple of the collateral's value and the collateral
        /// itself
        /// 
        /// Arguments:
        /// 
        /// * `collateral`: The collateral that the user is providing
        fn evaluate_collateral(&mut self, collateral : Bucket) -> (Decimal, Bucket) {

            if collateral.resource_address() == self.deposit.get_deposit_resource_type() {
                return (collateral.amount(), collateral)
            }
            else { 
                // TODO: if the user has a different token type or a nft, this function should consult an oracle to get an approximate evaluation
                // The evaluation should take into consideration the price and the risk of holding those tokens for the time of the loan
                // For nfts the evaluation should have some sort of mechanism that looks at a nft market and compute the approximate the price
                // An example would be https://skins.cash/ for selling steam skins which calculates a price directly based on 
                // the historical transactions of that skin inside steam and you can sell it directly for real money or crypto
                return (Decimal::zero(), collateral)
            }
        }

        /// > The function takes a collateral bucket, an amount of tokens to borrow, a duration for the loan,
        /// and a client NFT (if the lender is part of a trusted network). It then evaluates the collateral,
        /// checks that the amount of tokens requested is not greater than the maximum allowed for the offer,
        /// and that the duration of the loan is not greater than the maximum allowed by the lender. It then
        /// withdraws the requested amount of tokens from the lender's deposit, creates a loan contract, and
        /// instantiates a locked collateral NFT. Finally, it stores the client NFT in the lender's vault so
        /// that the client cannot borrow from other lenders in the same trusted network
        /// 
        /// Arguments:
        /// 
        /// * `collateral`: the collateral that the borrower is providing to the lender
        /// * `amount`: the amount of tokens to be loaned
        /// * `duration`: the duration of the loan in seconds
        /// * `network_client_nft`: this is the NFT that the borrower has received from the trusted network. It
        /// is used to determine the borrower's fidelity level and thus the maximum loan percentage that the
        /// borrower can get.
        pub fn take_loan(&mut self, collateral : Bucket, amount : Decimal, duration : u64, mut network_client_nft : Bucket) -> (Bucket, Bucket) 
        {
           let (collateral_evaluation, collateral) = self.evaluate_collateral(collateral);
           assert!(collateral_evaluation > Decimal::zero(), "no collateral provided or it couldn't be evaluated");

            let mut max_loan_percentage = self.max_loan_percentage;
            if !network_client_nft.is_empty() && !self.trusted_network_memberships.is_empty() { 
                let member_data = self.trusted_network_memberships[0].non_fungible::<TrustedPartnerNetworkMember>().data();
                assert!(network_client_nft.resource_address() == member_data.client_nft_resource_address, "invalid trusted network client nft");
                let network: TrustedPartnerNetwork = member_data.component.into();

                let discount;
                (network_client_nft, discount) = network.get_client_fidelity_level(self.trusted_network_memberships[0].create_proof(), network_client_nft);

                max_loan_percentage += discount;
            }

            assert!(amount <= collateral_evaluation + collateral_evaluation * max_loan_percentage, "the amount of tokens requested is greater than max allowed for your offer");
            assert!(duration <= self.max_loan_duration, "Cannot take a loan beyond the max time limit set by the lender");

            ComponentAuthZone::push(self.auth_vault.create_proof());
            let (loaned_tokens, allocation_id) = self.deposit.withdraw_funds(amount); // will assert if funds are not enough
            ComponentAuthZone::pop().drop();

            let nb_installments = Decimal::from(duration / self.installment_frequency);

            let current_epoch = scrypto::prelude::Runtime::current_epoch();
            let loan_contract = LoanContract::create(current_epoch, duration, self.installment_frequency,
                                                                        self.interest_rate, self.missed_installment_penalization,
                                                                        amount / nb_installments, amount, collateral_evaluation);
 
            let (locked_collateral, loan_nft) = LockedLoanCollateral::instantiate(collateral, amount, loan_contract, allocation_id.clone(), self.auth_vault.resource_address());
            self.loans.insert(loan_nft.non_fungible::<crate::locked_loan_collateral::NFTLoan>().id(), locked_collateral);

            // store the client nft so that he cannot borrow from other lenders in this partner network
            if !network_client_nft.is_empty() && !self.trusted_network_memberships.is_empty() {
                self.client_member_nfts.insert(allocation_id.clone(), Vault::with_bucket(network_client_nft));
            }
            return (loaned_tokens, loan_nft)
        }


        /// `mint_custom_lending_offer` is a function that mints a custom lending offer for the borrower.
        /// 
        /// Arguments:
        /// 
        /// * `interest_rate_reduction`: The amount of interest rate reduction that the offer provides.
        /// * `increase_max_loan_duration`: This is the number of blocks that the loan duration can be increased
        /// by.
        /// * `increase_max_loan_amount`: This is the amount that the borrower can borrow in addition to the
        /// base amount.
        /// 
        /// Returns:
        /// 
        /// The bucket is being returned.
        pub fn mint_custom_lending_offer(&mut self, interest_rate_reduction : Decimal, increase_max_loan_duration : u64,  increase_max_loan_amount : Decimal) -> Bucket{
            return self.auth_vault.authorize(|| { 
                borrow_resource_manager!(self.custom_lending_offer_address)
                    .mint_non_fungible(&NonFungibleId::random(), SpecialOffer {
                        interest_rate_reduction : interest_rate_reduction, 
                        increase_max_loan_duration  : increase_max_loan_duration,
                        increase_max_loan_amount : increase_max_loan_amount
                        })
            })
        }


        // Function mostly intented for uses cases like having a friend in need and you want to lend him some money 
        // You make him a custom offer and then he can accept it or not 
        pub fn take_loan_with_offer(&mut self, collateral : Bucket, amount : Decimal, duration : u64, special_offer : Bucket) -> (Bucket, Bucket) 
        {
            let (collateral_evaluation, collateral
            ) = self.evaluate_collateral(collateral);
            assert!(collateral_evaluation > Decimal::zero(), "no collateral provided or it couldn't be evaluated");
          
            let mut max_loan_duration = self.max_loan_duration;
            let mut max_loan_percentage = self.max_loan_percentage;
            let mut interest_rate = self.interest_rate;
            if !special_offer.is_empty() { 
                assert!(special_offer.resource_address() == self.custom_lending_offer_address, "invalid special offer");
                let special_offer_data = special_offer.non_fungible::<SpecialOffer>().data();
                max_loan_duration += special_offer_data.increase_max_loan_duration;

                interest_rate -= special_offer_data.interest_rate_reduction;
                interest_rate = cmp::min(self.interest_rate, Decimal::zero());

                max_loan_percentage += special_offer_data.increase_max_loan_amount;
            }

            assert!(amount <= collateral_evaluation + collateral_evaluation * max_loan_percentage, "the amount of tokens requested is greater than max allowed for your offer");
            assert!(duration <= max_loan_duration, "Cannot take a loan beyond the max limit set by the lender");

            ComponentAuthZone::push(self.auth_vault.create_proof());
            let (loaned_tokens, allocation_id) = self.deposit.withdraw_funds(amount); // will assert if funds are not enough
            ComponentAuthZone::pop().drop();
            
            let nb_installments = Decimal::from(duration / self.installment_frequency);

            
            let current_epoch = scrypto::prelude::Runtime::current_epoch();
            let loan_contract = LoanContract::create(current_epoch, duration, self.installment_frequency,
                                                                        interest_rate, self.missed_installment_penalization,
                                                                        amount / nb_installments, amount, collateral_evaluation);

            let (locked_collateral, loan_nft) = LockedLoanCollateral::instantiate(collateral, amount, loan_contract, allocation_id, self.auth_vault.resource_address());
            self.loans.insert(loan_nft.non_fungible::<crate::locked_loan_collateral::NFTLoan>().id(), locked_collateral);

            return (loaned_tokens, loan_nft)
        }

        /// The function `close_loan` takes a `LockedLoanCollateral` and returns a `Bucket` and a
        /// `LockedLoanCollateral`. 
        /// The `Bucket` is the notification that the loan has been closed. 
        /// The `LockedLoanCollateral` is the collateral that was used to secure the loan. 
        /// The function is called when the loan is finished. 
        /// The function checks if the loan can be liquidated. 
        /// If the loan can be liquidated, the function liquidates the loan. 
        /// If the collateral is the same as the deposit resource type, the function adds the collateral to the
        /// deposit. 
        /// If the collateral is not the same as the deposit resource type, the function adds the collateral to
        /// the liquidated collaterals. 
        /// 
        /// The function
        /// 
        /// Arguments:
        /// 
        /// * `loan`: LockedLoanCollateral
        fn close_loan(&mut self, mut loan : LockedLoanCollateral) -> (Bucket, LockedLoanCollateral) {
            assert!(loan.is_finished(), "loan not finished");

            ComponentAuthZone::push(self.auth_vault.create_proof());
            if loan.can_be_liquidated() { 
                let collateral = loan.liquidate(self.auth_vault.create_proof());
                if collateral.resource_address() == self.deposit.get_deposit_resource_type() {
                    self.deposit.add_funds(collateral, loan.get_id());
                } else {
                    self.liquidated_collaterals.push(Vault::with_bucket(collateral));
                }
            }

            let profit : Decimal = self.deposit.close_withdrawal(loan.get_id());
            ComponentAuthZone::pop().drop();

            return (self.notify_trusted_partner_network(loan.get_id(), profit), loan)
        }

        /// The function takes in a bucket of tokens and a bucket of loan NFTs, and returns a bucket of
        /// tokens, a bucket of client NFTs, a bucket of loan NFTs, and a bucket of tokens.
        /// 
        /// The function is called by the client, and the client is expected to pass in a bucket of
        /// tokens and a bucket of loan NFTs. The function then returns a bucket of tokens, a bucket of
        /// client NFTs, a bucket of loan NFTs, and a bucket of tokens.
        /// 
        /// Arguments:
        /// 
        /// * `tokens`: The amount of tokens the borrower is repaying.
        /// * `loan_nft`: The NFT that represents the loan.
        pub fn repay(&mut self, mut tokens : Bucket, mut loan_nft : Bucket) -> (Bucket, Bucket, Bucket, Bucket) 
        {
            assert!(tokens.resource_address() == self.deposit.get_deposit_resource_type(), "invalid resource type");
            assert!(loan_nft.amount() == Decimal::one(), "invalid loan nft count");
            let loan_id = loan_nft.non_fungible::<crate::locked_loan_collateral::NFTLoan>().id();
            assert!(self.loans.contains_key(&loan_id), "loan not found");

            let mut locked_collateral = self.loans.remove(&loan_id).unwrap();
            let (loan_status, overflow_amount) = locked_collateral.repay(tokens.amount(), self.auth_vault.create_proof(), loan_nft.create_proof());

            self.auth_vault.authorize( || { self.deposit.add_funds(tokens.take(tokens.amount() - overflow_amount), loan_id) });
            let mut client_nft = Bucket::new(tokens.resource_address());
            let mut collateral = Bucket::new(tokens.resource_address());
            if locked_collateral.is_finished() {
                (client_nft, locked_collateral) =  self.close_loan(locked_collateral);
                collateral = locked_collateral.unlock_collateral(self.auth_vault.create_proof(), loan_nft.create_proof());
                loan_nft.take(Decimal::one()).burn();
            } 
            else {
                self.auth_vault.authorize( || { loan_nft.non_fungible::<crate::locked_loan_collateral::NFTLoan>().update_data( NFTLoan {
                    loan_contract : locked_collateral.get_loan_contract().clone(),
					loan_refund_status : loan_status.clone()
                }) });

                self.loans.insert(locked_collateral.get_id(), locked_collateral);
            }


            return (collateral, client_nft, loan_nft, tokens) // return empty buckets
        }

        /// > If a loan can be liquidated, then remove it from the `loans` map and burn the client's NFT
        pub fn liquidate_loans(&mut self) 
        {
            let mut removed_loan_ids : Vec<NonFungibleId> = Vec::new();

            for (id, loan) in self.loans.iter() {
                if loan.can_be_liquidated() { 
                    removed_loan_ids.push(id.clone())
                }
            }

            for removed_loan_id in removed_loan_ids.iter() {
                let locked_collateral = self.loans.remove(&removed_loan_id).unwrap();
                let (client_nft, _locked_collateral) = self.close_loan(locked_collateral);
                client_nft.burn();
            }
        }

        /// > The function `notify_trusted_partner_network` is called when a loan is paid off. It takes the loan
        /// id and the profit made on the loan as arguments. It then checks if the loan id is in the
        /// `client_member_nfts` map. If it is, it then checks if the `trusted_network_memberships` list is
        /// empty. If it is not empty, it then gets the `TrustedPartnerNetworkMember` data from the first
        /// element in the list, and then gets the `TrustedPartnerNetwork` component from the
        /// `TrustedPartnerNetworkMember` data. It then removes the `client_member_nfts` entry for the loan id,
        /// and calls the `register_client_profit` function on the `TrustedPartnerNetwork` component, passing in
        /// the first element in the `trusted_network_memberships` list, the `client_member_n
        /// 
        /// Arguments:
        /// 
        /// * `loan_id`: The ID of the loan that the client is paying back.
        /// * `profit`: Decimal
        /// 
        /// Returns:
        /// 
        /// A Bucket of tokens.
        fn notify_trusted_partner_network(&mut self, loan_id : NonFungibleId, profit : Decimal) -> Bucket {
            if !self.client_member_nfts.contains_key(&loan_id) {
                return Bucket::new(RADIX_TOKEN)
            }
            if self.trusted_network_memberships.len() == 0 {
                return Bucket::new(RADIX_TOKEN)
            }
            let member_data = self.trusted_network_memberships[0].non_fungible::<TrustedPartnerNetworkMember>().data();
            let network: TrustedPartnerNetwork = member_data.component.into();
            let client_nft = self.client_member_nfts.remove(&loan_id).unwrap().take_all();
            return network.register_client_profit(self.trusted_network_memberships[0].create_proof(), client_nft, profit);
        }

  
        pub fn add_trusted_partner_network_member_badge(&mut self, member_nft : Bucket) {
            self.trusted_network_memberships.push(Vault::with_bucket(member_nft));
        }

        pub fn get_trusted_network_member_id(&self) -> NonFungibleId {
            assert!(!self.trusted_network_memberships.is_empty(), "component does not have any memberships");
            return self.trusted_network_memberships[0].non_fungible::<TrustedPartnerNetworkMember>().id()
        }

        pub fn recommend_for_promotion(&mut self, member_id : u32) {
            assert!(!self.trusted_network_memberships.is_empty(), "component does not have any memberships");
            let member_data = self.trusted_network_memberships[0].non_fungible::<TrustedPartnerNetworkMember>().data();
             // need to verify if this type of cross-package communication is possible
            let network: TrustedPartnerNetwork = member_data.component.into();
            network.recommend_member_for_promotion(self.trusted_network_memberships[0].create_proof(), NonFungibleId::from_u32(member_id));
        }
    }
}
