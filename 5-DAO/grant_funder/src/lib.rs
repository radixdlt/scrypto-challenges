use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Proposal {
    name:String,
    amount:Decimal,
    #[scrypto(mutable)]
    stage:u8,
}

#[derive(NonFungibleData)]
pub struct ProposalReceipt {

    amount:Decimal,
    #[scrypto(mutable)]
    stage:u8,
    proposal_id:NonFungibleId,
    #[scrypto(mutable)]
    stage1_funded:bool,
    #[scrypto(mutable)]
    stage2_funded:bool,
    #[scrypto(mutable)]
    stage3_funded:bool,
    #[scrypto(mutable)]
    stage4_funded:bool,
}

blueprint! {
    struct GrantFunder {
        //Vault that holds the admin badge
        admin_badge_vault:Vault,

        //Vault that holds the proposal NFT
        proposal_vault:Vault,

        //Vault that signifies YES votes
        yes_vote_vault:Vault,

        //Vault that signifies NO votes
        no_vote_vault:Vault,

        //Proposal NFT resource address
        proposal_resource_address:ResourceAddress,

        //Proposal receipt for accessing funds 
        proposal_receipt_resource_address:ResourceAddress,

        //Vault that holds XRD for grants
        grant_vault:Vault,
    }

    impl GrantFunder {
        
        pub fn new(grant_xrd:Bucket) -> ComponentAddress {

            let admin_badge:Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Admin Badge")
                .initial_supply(1);
           
            let proposal_resource_address:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Proposal")
                .mintable(rule!(require(admin_badge.resource_address(),)), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let proposal_receipt_resource_address:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Proposal Receipt")
                .mintable(rule!(require(admin_badge.resource_address(),)), LOCKED)
                .burnable(rule!(require(admin_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(admin_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let component = Self {
                yes_vote_vault:Vault::new(RADIX_TOKEN),
                no_vote_vault:Vault::new(RADIX_TOKEN),
                admin_badge_vault:Vault::with_bucket(admin_badge),
                proposal_vault:Vault::new(proposal_resource_address),
                proposal_resource_address,
                grant_vault:Vault::with_bucket(grant_xrd),
                proposal_receipt_resource_address,
            }
            
            .instantiate()
            .globalize();

            return component;
        }

        //Create grant proposal, enter name and amount
        pub fn create_proposal(&mut self, 
            down_payment:Bucket,
            name:String, 
            grant_amount:Decimal, 
                ) -> Bucket {

            assert!(down_payment.amount() >= grant_amount / 2, "Down payment must be 50% of grant");
            
            //Put down payment in grant vault
            self.grant_vault.put(down_payment);
            
            //Proposal NFT data
            let proposal_nft_data = Proposal {
                name:name,
                amount:grant_amount,
                stage:0,
            };

            //Mint proposal NFT
            let proposal_nft_bucket = self.admin_badge_vault.authorize(||
                borrow_resource_manager!(self.proposal_resource_address).mint_non_fungible(
                    &NonFungibleId::random(),
                    proposal_nft_data,
                )
            );

            //Proposal receipt NFT data
            let receipt_nft_data = ProposalReceipt {

                amount:grant_amount,
                stage:0,
                proposal_id:proposal_nft_bucket.non_fungible::<Proposal>().id(),
                stage1_funded:false,
                stage2_funded:false,
                stage3_funded:false,
                stage4_funded:false,
            };

            //Mint proposal receipt NFT
            let receipt_nft_bucket = self.admin_badge_vault.authorize(||
                borrow_resource_manager!(self.proposal_receipt_resource_address).mint_non_fungible(
                    &NonFungibleId::random(),
                    receipt_nft_data,
                )
            );

            //Put the proposal NFT back in the proposal NFT vault
            self.proposal_vault.put(proposal_nft_bucket);

            //Return proposal receipt to user
            return receipt_nft_bucket;

            }

            //Vote yes
            pub fn vote_yes(&mut self, mut vote_xrd:Bucket) -> Bucket{
                self.yes_vote_vault.put(vote_xrd.take(dec!("1")));
                return vote_xrd;
                
            }

            //Vote no
            pub fn vote_no(&mut self, mut vote_xrd:Bucket) -> Bucket{
                self.no_vote_vault.put(vote_xrd.take(dec!("1")));
                return vote_xrd;
                
            }

            //This method counts the total of yes and no votes
            //If there are more yes than no votes increment the proposal NFT data
            pub fn count_votes(&mut self, nft_id:NonFungibleId) {

                //Take the proposal from the proposal NFT vault
                let nft_bucket = self.proposal_vault.take_non_fungible(&nft_id);

                //Get the proposal NFT data
                let mut nft_data:Proposal = nft_bucket.non_fungible().data();

                //Determine the amount of yes and no votes
                let yes_votes:Decimal = self.yes_vote_vault.amount();

                info!("Yes votes{}", yes_votes);

                let no_votes:Decimal = self.no_vote_vault.amount();

                info!("No votes{}", no_votes);

                //If yes votes > no votes increment the proposal NFT data
                if yes_votes > no_votes {

                    //Increment NFT data stage
                    nft_data.stage +=1;

                    //Update the proposal NFT data
                    self.admin_badge_vault.authorize(|| nft_bucket.non_fungible().update_data(nft_data));
                    
                    //Put the proposal NFT back into the proposal vote
                    self.proposal_vault.put(nft_bucket);

                    //Remove all funds from the yes no vaults and place them in the grant fund

                } else {
                    //Burn proposal
                    self.admin_badge_vault.authorize(|| nft_bucket.burn());
                }
            }

            pub fn update_receipt(&mut self, receipt:Bucket) -> Bucket {
                //Get NFT data from receipt NFT
                let mut receipt_nft_data:ProposalReceipt = receipt.non_fungible().data();

                //Get the proposal id from the NFT data
                let proposal_id:NonFungibleId = receipt_nft_data.proposal_id.clone();

                //Use the proposal id to find proposal NFT in proposal vault
                let proposal_nft:Bucket = self.proposal_vault.take_non_fungible(&proposal_id);

                //Get the NFT data from the proposal NFT
                let proposal_nft_data:Proposal = proposal_nft.non_fungible().data();

                //Get the stage from the proposal id
                let proposal_stage:u8 = proposal_nft_data.stage;

                //Update the receipt NFT stage 
                receipt_nft_data.stage = proposal_stage;

                //Update the receipt NFT data
                self.admin_badge_vault.authorize( || receipt.non_fungible()
                    .update_data(receipt_nft_data));

                //Return proposal receipt to vault
                self.proposal_vault.put(proposal_nft);

                //Return receipt to user
                return receipt;
                
            }

            pub fn collect_xrd(&mut self, proposal_receipt:Bucket) -> Option<(Bucket, Bucket)> {

                //Get data from proposal receipt
                let mut receipt_nft_data:ProposalReceipt = proposal_receipt.non_fungible().data();
                let amount = receipt_nft_data.amount;
                let stage = receipt_nft_data.stage;

                //Verify proposal receipt stage and pay grant funds and update proposal nft data
                if stage == 1 && receipt_nft_data.stage1_funded == false {
                    info!("Here are your stage 1 funds"); 
                    let funds = self.grant_vault.take((amount/10)*2);
                    receipt_nft_data.stage1_funded = true;
                    self.admin_badge_vault.authorize(|| proposal_receipt.non_fungible().update_data(receipt_nft_data));
                    return Some((funds, proposal_receipt));
                    
                } else if stage == 2 && receipt_nft_data.stage2_funded == false{
                    info!("Here are your stage 2 funds");
                    let funds = self.grant_vault.take((amount/10)*2);
                    receipt_nft_data.stage2_funded = true;
                    self.admin_badge_vault.authorize(|| proposal_receipt.non_fungible().update_data(receipt_nft_data));
                    return Some((funds, proposal_receipt));
                    
                } else if stage == 3 && receipt_nft_data.stage3_funded == false{
                    info!("Here are your stage 3 funds");
                    let funds = self.grant_vault.take((amount/10)*3);
                    receipt_nft_data.stage3_funded = true;
                    self.admin_badge_vault.authorize(|| proposal_receipt.non_fungible().update_data(receipt_nft_data));
                    return Some((funds, proposal_receipt));
                    
                } else if stage == 4 && receipt_nft_data.stage4_funded == false{
                    info!("Here are your stage 4 funds");
                    let funds = self.grant_vault.take((amount/10)*3);
                    receipt_nft_data.stage4_funded = true;
                    self.admin_badge_vault.authorize(|| proposal_receipt.non_fungible().update_data(receipt_nft_data));
                    return Some((funds, proposal_receipt));

                } else {
                    info!("Invalid Receipt"); 
                    return None;
                };
            }  
    }
}