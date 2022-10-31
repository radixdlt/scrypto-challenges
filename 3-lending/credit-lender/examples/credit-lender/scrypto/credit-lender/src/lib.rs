use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct LenderReceiptNFT {
    
    notes:String,
    account_address: ComponentAddress,
    issued_epoch:u64,
    #[scrypto(mutable)]
    lend_amount:Decimal,
}

#[derive(NonFungibleData)]
pub struct LoanReceiptNFT {
    
    note:String,
    account_address: ComponentAddress,
    #[scrypto(mutable)]
    borrow_amount_usd:Decimal,
   

}

#[derive(NonFungibleData, Debug)]
pub struct LoanNFT {
    
    account_address: ComponentAddress,
    issued_epoch:u64,
    #[scrypto(mutable)]
    fee_epoch:u64,
    #[scrypto(mutable)]
    borrow_amount_usd:Decimal,
    #[scrypto(mutable)]
    borrow_amount_xrd:Decimal,
    #[scrypto(mutable)]
    collateral_amount_xrd:Decimal,
    #[scrypto(mutable)]
    xrd_liquidation_price:Decimal,
}

#[derive(NonFungibleData)]
pub struct CreditReportNFT {

    account_address: ComponentAddress,
    #[scrypto(mutable)]
    open_lend:u8,
    #[scrypto(mutable)]
    close_lend:u8,
    #[scrypto(mutable)]
    open_loans:u8,
    #[scrypto(mutable)]
    closed_loans:u8,
    #[scrypto(mutable)]
    add_collateral:u8,
    #[scrypto(mutable)]
    remove_collateral:u8,
    #[scrypto(mutable)]
    pay_loan:u8,
    #[scrypto(mutable)]
    borrow_more:u8,
    #[scrypto(mutable)]
    liquidations:u8,
    #[scrypto(mutable)]
    credit_score:u64,
    
}

blueprint! {
    struct CreditLender {

    //VAULTS
       
        //This vault stores the loan NFTs.   
        loan_vault: Vault,

        //This vault stores the protocal users credit report NFTs.
        credit_report_vault: Vault,

        //This vaults stores the lending pool funds
        lending_pool: Vault,

        //This is this vault the minting_badge will be stored
        minting_badge_vault:Vault,

        //This is the vault that will hold all the borrows collateral
        collateral_vault: Vault,

        //This is the price oracle 
        xrd_price_oracle: Decimal,
        
    //RESOURCE ADDRESS 

        //This is the credit token
        //credit_token: ResourceAddress,  //TODO not used erase

        //This is the credit report NFT
        credit_report: ResourceAddress,

        //This is the lenders receipt
        lender_receipt: ResourceAddress,

        //This is the loan NFT
        loan_nft: ResourceAddress,

        //This is the borrowers receipt
        loan_receipt: ResourceAddress,

    //DECIMALS

        //Counter to keep track of % contributed to lending pool and earned fees
        lending_counter:Decimal,

    }

    impl CreditLender {
        
        pub fn new() -> ComponentAddress {

            //Minting badge use for component minting authority 
            let minting_badge:Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Minting Badge")
                .initial_supply(1);

            //This is the lender receipt
            let lender_receipt:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Lender Receipt")
                .mintable(rule!(require(minting_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minting_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minting_badge.resource_address())), LOCKED)
                .no_initial_supply();

             //This is the borrower receipt
            let loan_receipt:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan Receipt")
                .mintable(rule!(require(minting_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minting_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minting_badge.resource_address())), LOCKED)
                .no_initial_supply();

            //This is the loan NFT
            let loan_nft:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Loan")
                .mintable(rule!(require(minting_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minting_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minting_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let credit_report:ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "Credit Report")
                .mintable(rule!(require(minting_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minting_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minting_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            
            Self {
                minting_badge_vault:Vault::with_bucket(minting_badge),
                loan_vault:Vault::new(loan_nft),
                credit_report_vault:Vault::new(credit_report),
                lending_pool:Vault::new(RADIX_TOKEN),
                collateral_vault:Vault::new(RADIX_TOKEN),
                lending_counter: dec!(0),
                lender_receipt:lender_receipt,
                loan_receipt: loan_receipt,
                xrd_price_oracle: dec!("1"),
                loan_nft:loan_nft,
                credit_report:credit_report,
            }
            .instantiate()
            .globalize()
        }

//This is the xrd price oracle.  This is just to show functionality of this blueprint.  
        pub fn set_xrd_price(&mut self, new_price:Decimal) {
            self.xrd_price_oracle = new_price;
            info!("xrd price has been set to {}", new_price);
        }

//Add funds to lending pool.  Lenders will receive a lenders receipt NFT 
    //TODO add a % ratio for ownership of entire pool similar to an LP token

        pub fn add_funds(&mut self, amount:Bucket, account_address:ComponentAddress ) -> Bucket {
            
            info!("You have added {} to the lending pool!", amount.amount());
            //Get lending amount 
            let lend_amount: Decimal = amount.amount();

            //Increase lending counter by lending amount
            self.lending_counter += lend_amount;

            //Add funds to lending pool
            self.lending_pool.put(amount);

            //Assign data to lender receipt NFT
            let nft_data = LenderReceiptNFT {
                notes: "Thanks for lending @ www.lend.com".to_string(),
                account_address: account_address,
                issued_epoch:Runtime::current_epoch(),
                lend_amount: lend_amount,
            };

            //Mint lenders receipt NFT
            let nft_bucket:Bucket = self.minting_badge_vault.authorize(||{
                borrow_resource_manager!(self.lender_receipt)
                    .mint_non_fungible(&NonFungibleId::random(), nft_data)
            });

            //Check credit report vault to see if lender has a credit report
            //Create credit report is lender does not have a credit report

            //Convert component address to NonFungibleId
            let account = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Get list of NonFungibleId from credit report vault
            let report_list = self.credit_report_vault.non_fungible_ids();

            //If account address which is the credit report ID# if found, update credit report data
            if report_list.contains(&account){

                //Take credit report from credit report vault and put in bucket
                let credit_nft_bucket:Bucket = self.credit_report_vault.take_non_fungible(&account);

                //Get the credit report data
                let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();
                
                //Increment the # of lending positions by 1 
                credit_data.open_lend += 1;

                //update the credit report NFT data
                self.minting_badge_vault.authorize(|| {
                    credit_nft_bucket.non_fungible().update_data(credit_data)
                });

                //Put the credit report NFT back in the credit report vault
                self.credit_report_vault.put(credit_nft_bucket);

            //If account address which is the credit report ID# if not found, create a credit report NFT
            } else {

                //Assign nft data to credit report NFT
                let nft_data = CreditReportNFT {
                    account_address: account_address,
                    open_lend:1, //This is the first lending position
                    close_lend:0,
                    open_loans:0,
                    closed_loans:0,
                    add_collateral:0,
                    remove_collateral:0,
                    pay_loan:0,
                    borrow_more:0,
                    liquidations:0,
                    credit_score:10, //Lender recieves 10 point for opening their first lending position
                }; 
    
                //Mint credit report NFT
                let nft_bucket = self.minting_badge_vault.authorize(||{
                    borrow_resource_manager!(self.credit_report)
                        .mint_non_fungible(&account, nft_data)
                });
    
                //Put the credit report NFT back in the credit report vault
                self.credit_report_vault.put(nft_bucket);

            }
            
            //return lender receipt NFT to lender
            return nft_bucket;

        }

//Remove Funds from lending pool
        pub fn remove_funds(&mut self, lend_receipt:Bucket, remove_amount:Decimal) -> (Bucket, Option<Bucket>) {

            //Get the nft data from the lending receipt
            let lender_nft_data:LenderReceiptNFT = lend_receipt.non_fungible().data();

            //Get account address from NFT data
            let account_address = lender_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Assert that the requested amount to remove from the lending pool is <= to orginal amount lent to the lending pool
            assert!(lender_nft_data.lend_amount >= remove_amount, "Request removal amount is > orginal lent amount");

            //Assert that there is enough funds in the lending pool to cover the requested removal amount
            assert!(self.lending_pool.amount() >= remove_amount, "There is not enough liquidity in the lending pool");

            //Calculate % of lending pool to remove
            let calc_remove_amount = remove_amount * (self.lending_pool.amount() / self.lending_counter);

            //Remove funds from lending pool
            let remove_funds_bucket = self.lending_pool.take(calc_remove_amount);

            //Reduce lending counter by amount removed
            self.lending_counter -= remove_amount;

            //If the requested amount of funds to be remove is == to the amount orginally lent
            //Burn the lenders reciept 
            if remove_amount == lender_nft_data.lend_amount {

                //burn lending receipt NFT
                self.minting_badge_vault.authorize({|| 
                    lend_receipt.burn()
                });

                //Take credit report NFT from credit report vault
                let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account);

                //Get the credit report data
                let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

                //Increment the close lend position by 1
                //TODO add credit tokens based on length of lending
                credit_data.close_lend += 1;
                
                //Calculate how long the lending position was open
                let length_of_lend = Runtime::current_epoch() - lender_nft_data.issued_epoch;

                //Borrowers earn 2 credit point per epoch if they have the loan open for more than a week.
                //30min = 1 epoch, 7days*24hrs*2epoch
                if length_of_lend > 336 {
                    credit_data.credit_score += length_of_lend * 1;
                };

                //Update the credit report NFT data
                self.minting_badge_vault.authorize(|| {
                    credit_nft_bucket.non_fungible().update_data(credit_data)
                });

                //Put the credit report back into the credit report vault
                self.credit_report_vault.put(credit_nft_bucket);

                //Return the orginal amount of funds orginally lent
                return (remove_funds_bucket, None);
                                
            } else {
                //TODO this is not working right
                //update lender recipet NFT
                //Updata loan NFT

                //Get the lenders receipt NFT
                let mut loan_nft_data:LenderReceiptNFT = lend_receipt.non_fungible().data();

                //Modify the lend amount by subtracting the requested removal amount
                loan_nft_data.lend_amount -= remove_amount;

                //Update the lending receipt NFT data
                self.minting_badge_vault.authorize(|| {
                    lend_receipt.non_fungible().update_data(loan_nft_data)
                });

                //Return the requested amount of funds to be removed from the lending pool
                return (remove_funds_bucket, Some(lend_receipt));
                
            }
        }

//Create a loan

        pub fn new_loan(&mut self, loan_amount:Decimal, mut collateral:Bucket, account_address:ComponentAddress)-> (Bucket, Bucket){

            //Check credit report vault to see if borrower has a credit report
            //Convert component address to NonFungibleId
            let account = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Get list of NonFungibleId from credit report vault 
            let report_list = self.credit_report_vault.non_fungible_ids();

            //If account address which is the credit report ID# if found, update credit report data
            if report_list.contains(&account){

                //Take credit report from credit report vault and put in bucket
                let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account);

                //Get list of NonFungibleId from credit report vault
                let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

                //Modify credit report by incrementing open loans by 1, and 20 points for creating a loan
                credit_data.open_loans += 1;
                credit_data.credit_score += 20;

                //Update credit report NFT data
                self.minting_badge_vault.authorize(|| {
                    credit_nft_bucket.non_fungible().update_data(credit_data)
                });

                //Put the credit report NFT back in the credit report vault
                self.credit_report_vault.put(credit_nft_bucket);

            //If account address which is the credit report ID# if not found, create a credit report NFT
            } else {

                //Assign nft data to credit report NFT
                let nft_data = CreditReportNFT {
                    account_address: account_address,
                    open_lend:0,
                    close_lend:0,
                    open_loans:1,
                    closed_loans:0,
                    add_collateral:0,
                    remove_collateral:0,
                    pay_loan:0,
                    borrow_more:0,
                    liquidations:0,
                    credit_score:20,
                }; 
    
                let nft_bucket:Bucket = self.minting_badge_vault.authorize(||{
                    borrow_resource_manager!(self.credit_report)
                        .mint_non_fungible(&account, nft_data)
                });
    
                //Put the credit report NFT back in the credit report vault
                self.credit_report_vault.put(nft_bucket);

            }

            //Calculate loan initiation fee, 1% of loan, which is taken out of the collateral
            let loan_inititation_fee = loan_amount * dec!("0.01");

            //Get the collateral "amount" in decimal of the XRD in the collateral bucket minus initiation fee
            let collateral_amount:Decimal = collateral.amount() - loan_inititation_fee;
            
            //Assert that the 150% collateralization rate is met 
            assert!(collateral_amount/(loan_amount*dec!("1.5")) > dec!("1"), "[ERROR] Check borrow and collateral amount ratio"); 

            //Take loan initiation fees from collateral bucket
            let initiation_fee_bucket = collateral.take(loan_inititation_fee);

            //Put loan initiation fees in the lenders pool
            self.lending_pool.put(initiation_fee_bucket);

            //Put the remaining collateral into the collateral vault
            self.collateral_vault.put(collateral);

            //Assert that the lending pool has enough funds for loan amount 
            assert!(!(self.lending_pool.amount() < loan_amount), "Not enough funds in the lending pool");

            //Take loan amount from lending pool
            let loan_bucket = self.lending_pool.take(loan_amount); 

            //Generate NFT ID, loan receipt NFT ID and loan NFT ID are the same 
            let nft_id = NonFungibleId::random();

            //Assign loan receipt NFT data
            let loan_receipt_nft_data = LoanReceiptNFT {

                note:"You have a loan open with lend.com".to_string(),
                account_address: account_address,
                borrow_amount_usd:loan_amount*self.xrd_price_oracle,
                
            };

            //Mint a loan receipt NFT
            let loan_receipt_nft_bucket = self.minting_badge_vault.authorize(||{
                borrow_resource_manager!(self.loan_receipt)
                    .mint_non_fungible(&nft_id, loan_receipt_nft_data)
            });

            //Assign Loan NFT data
            let loan_nft_data = LoanNFT {
            
                account_address: account_address,
                issued_epoch:Runtime::current_epoch(),
                fee_epoch:Runtime::current_epoch(),
                borrow_amount_xrd:loan_amount,
                borrow_amount_usd:loan_amount*self.xrd_price_oracle,
                collateral_amount_xrd:collateral_amount, 
                xrd_liquidation_price:(loan_amount*self.xrd_price_oracle * dec!("1.5"))/(collateral_amount),
            };

            //Mint loan NFT 
            let loan_nft_bucket = self.minting_badge_vault.authorize(||{
                borrow_resource_manager!(self.loan_nft)
                    .mint_non_fungible(&nft_id, loan_nft_data)
            });

            //Put the loan NFT in the credit report vault
            self.loan_vault.put(loan_nft_bucket);

            //Return loan receipt NFT and loan to borrower
            return (loan_receipt_nft_bucket, loan_bucket);

        }

//Close a loan
        pub fn close_loan(&mut self, loan_receipt:Bucket, mut repayment_amount:Bucket) -> (Bucket, Bucket){
            
            //Get loan receipt NFT ID to retrive loan NFT, since the IDs are the same
            let loan_id = loan_receipt.non_fungible::<LoanReceiptNFT>().id(); 

            //Take loan NFT out of loan vault 
            let loan_nft_bucket:Bucket = self.loan_vault.take_non_fungible(&loan_id);

            //Get loan receipt NFT data
            let loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            //Get account address
            let account_address = loan_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account_address = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Assert the loan is not in liquidation by checking liquidation price vs current xrd price
            assert!(loan_nft_data.xrd_liquidation_price < self.xrd_price_oracle, "Loan is in liquidation status");

            //Assert that the repayment amount is >= to the borrow amount
            assert!(loan_nft_data.borrow_amount_xrd <= repayment_amount.amount(), "Loan repayment amount is < orginal loan amount");

            //Take the orginal loan amount out of the repayment bucket 
            let lending_pool_repayment = repayment_amount.take(loan_nft_data.borrow_amount_xrd);

            //Put the orginal loan amount in the lending pool
            self.lending_pool.put(lending_pool_repayment);

            //Take collateral out of the collateral pool
            let mut return_collateral_bucket = self.collateral_vault.take(loan_nft_data.collateral_amount_xrd);

            //Find how may epoch since orgination or last fee harvest
            let loan_length = Runtime::current_epoch() - loan_nft_data.fee_epoch;

            //17520 epoch in a year.  At 3% APY == 0.000171%/epoch
            let loan_fee = loan_nft_data.borrow_amount_usd * loan_length * dec!("0.00000171");

            //Take any fees that are owed out of the return collateral bucket
            self.lending_pool.put(return_collateral_bucket.take(loan_fee/self.xrd_price_oracle));

            info!("Thank you for closing your loan.  Loan Amount = {} XRD, Collateral Amount = {} XRD, Loan Length = {} EPOCH, Loan Fee Outstanding {} USD",
                 loan_nft_data.borrow_amount_usd, loan_nft_data.collateral_amount_xrd, loan_length, loan_fee);
            
            //Burn the loan receipt NFT
            self.minting_badge_vault.authorize(||{
                loan_receipt.burn();
            });

            //Burn the loan NFT
            self.minting_badge_vault.authorize(||{
                loan_nft_bucket.burn();
            });

            //Take credit report from credit report vault and put in bucket
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account_address);

            //Get credit report NFT data
            let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Increment closed loan by 1
            credit_data.closed_loans += 1;

            //Calculate how long the loan was open
            let length_of_loan = Runtime::current_epoch() - loan_nft_data.issued_epoch;

            //Borrowers earn 2 credit point per epoch if they have the loan open for more than a week.
            //30min = 1 epoch, 7days*24hrs*2epoch
            if length_of_loan > 336 {
                credit_data.credit_score += length_of_loan * 2;
            };
            
            //Updata credit report NFT data
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.credit_report_vault.put(credit_nft_bucket);

            info!("returned collateral {}", return_collateral_bucket.amount());

            //Return any change associated with repayment amount and return collateral to borrower
            return (repayment_amount, return_collateral_bucket);
        }

//Add collateral to existing loan
            pub fn add_collateral(&mut self, loan_receipt:Bucket, added_collateral:Bucket)->Bucket{

            //Get loan receipt NFT ID to retrive loan NFT, since the IDs are the same
            let loan_id = loan_receipt.non_fungible::<LoanReceiptNFT>().id(); 

            //Take loan NFT out of loan vault 
            let loan_nft_bucket:Bucket = self.loan_vault.take_non_fungible(&loan_id);

            //Get loan receipt NFT data
            let mut loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            //Get account address
            let account_address = loan_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account_address = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Get the added collateral "decimal" from the add collateral bucket
            let added_collateral_amount: Decimal = added_collateral.amount();

            //Assert the loan is not in liquidation by checking liquidation price vs current xrd price
            assert!(loan_nft_data.xrd_liquidation_price < self.xrd_price_oracle, "Loan is in liquidation status");

            //Add collateral to the collateral pool
            self.collateral_vault.put(added_collateral);

            //Updata loan NFT
            //Modify loan receipt NFT data collateral amount by added added collatral amount
            loan_nft_data.collateral_amount_xrd += added_collateral_amount;

            //Modify loan receipt NFT data liquidation price 
            loan_nft_data.xrd_liquidation_price = loan_nft_data.borrow_amount_usd * dec!("1.5") / loan_nft_data.collateral_amount_xrd;

            //Update loan NFT data 
            self.minting_badge_vault.authorize(|| {
                loan_nft_bucket.non_fungible().update_data(loan_nft_data)
            });

            //Put loan receipt back in loan vault
            self.loan_vault.put(loan_nft_bucket);

            //Update credit report 
            //Take borrowers credit report from credit report vault
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account_address);

            //Get credit report data
            let mut credit_nft_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Modify credit report data by incrementing added collatral by 1
            credit_nft_data.add_collateral += 1;

            //Update credit report NFT data
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_nft_data)
            });

            //Put credit report NFT in credit report vault
            self.credit_report_vault.put(credit_nft_bucket);
            
            //Return credit report receipt to borrower
            return loan_receipt;

        }

//Remove collateral from existing loan
        pub fn remove_collateral(&mut self, loan_receipt:Bucket, remove_collateral:Decimal)-> (Bucket, Bucket) {

            //Get loan receipt NFT ID to retrive loan NFT, since the IDs are the same
            let loan_id = loan_receipt.non_fungible::<LoanReceiptNFT>().id(); 

            //Take loan NFT out of loan vault 
            let loan_nft_bucket:Bucket = self.loan_vault.take_non_fungible(&loan_id);

            //Get loan receipt NFT data
            let mut loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            //Get account address
            let account_address = loan_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account_address = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Assert the loan is not in liquidation by checking liquidation price vs current xrd price
            assert!(loan_nft_data.xrd_liquidation_price < self.xrd_price_oracle, "Loan is in liquidation status");

            //Calculate new collateral amount
            let new_collateral_amount:Decimal = loan_nft_data.collateral_amount_xrd - remove_collateral;

            //Assert that the request amount to remove from the collateal does not cause the loan to go into liquidation
            assert!((loan_nft_data.borrow_amount_usd * dec!("1.5") / new_collateral_amount < self.xrd_price_oracle) , "Too little collateral");

            //Remove requests amount of collateral from the collateral pool
            let collateral_bucket:Bucket = self.collateral_vault.take(remove_collateral);

            //Updata loan NFT
            //Modify loan NFT data. Reduce collateral amount by requested removal account
            loan_nft_data.collateral_amount_xrd -= remove_collateral;

            //Modfiy loan NFT data liquidation price
            loan_nft_data.xrd_liquidation_price = loan_nft_data.borrow_amount_usd * dec!("1.5") / loan_nft_data.collateral_amount_xrd;

            //updata loan NFT data
            self.minting_badge_vault.authorize(|| {
                loan_nft_bucket.non_fungible().update_data(loan_nft_data)
            });

            //Put loan NFT back in the loan NFT vault
            self.loan_vault.put(loan_nft_bucket);

            //update credit report 
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account_address);

            //Get credit report NFT data
            let mut credit_nft_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Modify credit report NFT data by incremeting remove collateral by 1
            credit_nft_data.remove_collateral += 1;

            //Update credit report NFT data 
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_nft_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.credit_report_vault.put(credit_nft_bucket);

            //Return loan receipt removed collateral to borrower
            return (loan_receipt, collateral_bucket);

        }

//Borrow more from loan

        pub fn borrow_more(&mut self, loan_receipt: Bucket, borrow_amount:Decimal) -> (Bucket, Bucket) {

            //Get loan receipt NFT ID to retrive loan NFT, since the IDs are the same
            let loan_id = loan_receipt.non_fungible::<LoanReceiptNFT>().id(); 

            //Take loan NFT out of loan vault 
            let loan_nft_bucket:Bucket = self.loan_vault.take_non_fungible(&loan_id);

            //Get loan receipt NFT data
            let mut loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            //Get account address
            let account_address = loan_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account_address = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Assert the loan is not in liquidation by checking liquidation price vs current xrd price
            assert!(loan_nft_data.xrd_liquidation_price < self.xrd_price_oracle, "Loan is in liquidation status");

            //Calculate new borrow amount
            let new_borrow_amount_usd:Decimal = loan_nft_data.borrow_amount_usd + borrow_amount;

            //Assert that the request amount to borrow does not cause the loan to go into liquidation
            assert!(new_borrow_amount_usd* dec!("1.5") / loan_nft_data.collateral_amount_xrd < self.xrd_price_oracle, "The new borrow amount puts the loan into liquidation status");

            //Calculate new borrow amount XRD based on the current price of XRD
            let new_borrow_amount_xrd:Decimal = borrow_amount / self.xrd_price_oracle;

            //Asset there are enough funds in the lending pool 
            assert!(self.lending_pool.amount() >= new_borrow_amount_xrd, "Not enough funds in the lending pool");

            //Take requested borrow amount from lending pool
            let new_borrow_bucket = self.lending_pool.take(new_borrow_amount_xrd);

            //update loan reciept NFT
            //Get loan receipt NFT data
            let mut receipt_nft_data:LoanReceiptNFT = loan_receipt.non_fungible().data();

            //Modify receipt NFT data borrow amount USD by adding additional amount borrowed 
            receipt_nft_data.borrow_amount_usd += borrow_amount;

            //Update receipt NFT data
            self.minting_badge_vault.authorize(|| {
                loan_receipt.non_fungible().update_data(receipt_nft_data)
            });

            //update loan NFT
            //Modify loan NFT data borrow amount USD by adding additional amount borrowed 
            loan_nft_data.borrow_amount_usd += borrow_amount;

            //Modify loan NFT data borrow amount XRD by adding additional amount borrowed 
            loan_nft_data.borrow_amount_xrd +=new_borrow_amount_xrd;

            //Modify loan NFT data updated liquidation price
            loan_nft_data.xrd_liquidation_price = loan_nft_data.borrow_amount_usd * dec!("1.5") / loan_nft_data.collateral_amount_xrd;

            //update loan NFT data
            self.minting_badge_vault.authorize(|| {
                loan_nft_bucket.non_fungible().update_data(loan_nft_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.loan_vault.put(loan_nft_bucket);

            //updata credit report 
            
            //Take credit report NFT from credit report vault
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account_address);

            //Get credit report NFT data
            let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Modify credit report NFT data by incrementing borrow more by 1
            credit_data.borrow_more += 1;

            //Updata credit report NFT 
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.credit_report_vault.put(credit_nft_bucket);

            //Return loan receipt and new borrowed amount to borrower
            return (loan_receipt, new_borrow_bucket);
            
        }

//Make Loan Payment 

        pub fn pay_loan (&mut self, loan_receipt: Bucket, pay:Bucket) -> Bucket {

            

            //Get loan receipt NFT ID to retrive loan NFT, since the IDs are the same
            let loan_id = loan_receipt.non_fungible::<LoanReceiptNFT>().id(); 

            //Take loan NFT out of loan vault 
            let loan_nft_bucket:Bucket = self.loan_vault.take_non_fungible(&loan_id);

            //Get loan receipt NFT data
            let mut loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            //Get account address
            let account_address = loan_nft_data.account_address;

            //Convert component address to NonFungibleId
            let account_address = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Get payment amount "demcimal" from pay bucket
            let pay_amount = pay.amount();

            //Assert the loan is not in liquidation by checking liquidation price vs current xrd price
            assert!(loan_nft_data.xrd_liquidation_price < self.xrd_price_oracle, "Loan is in liquidation status");

            //Assert pay amount < total loan amount
            assert!(pay.amount() < loan_nft_data.borrow_amount_xrd, "Use the close loan method to pay off entire loan"); 

            self.lending_pool.put(pay);

            //updata reciept NFT
            let mut receipt_nft_data:LoanReceiptNFT = loan_receipt.non_fungible().data();

            //Modify loan receipt NFT data borrow amount USD by subtracting additional amount borrowed 
            receipt_nft_data.borrow_amount_usd = receipt_nft_data.borrow_amount_usd - pay_amount*self.xrd_price_oracle;

            //Update loan receipt NFT data
            self.minting_badge_vault.authorize(|| {
                loan_receipt.non_fungible().update_data(receipt_nft_data)
            });

            //update loan NFT 
            //Modify loan NFT data borrow amount USD by subtracting additional amount borrowed 
            loan_nft_data.borrow_amount_usd = loan_nft_data.borrow_amount_usd - pay_amount*self.xrd_price_oracle;

            //Modify loan NFT data borrow amount XRD by subtracting additional amount borrowed
            loan_nft_data.borrow_amount_xrd = loan_nft_data.borrow_amount_xrd - pay_amount;

            //Modify loan NFT data updated liquidation price
            loan_nft_data.xrd_liquidation_price = loan_nft_data.borrow_amount_usd * dec!("1.5") / loan_nft_data.collateral_amount_xrd;

            //Updata loan NFT data
            self.minting_badge_vault.authorize(|| {
                loan_nft_bucket.non_fungible().update_data(loan_nft_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.loan_vault.put(loan_nft_bucket);

            //updata credit report 
            //Take credit report NFT from credit report vault
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account_address);

            //Get credit report NFT data
            let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Modify credit report data by incrementing pay loan by 1 
            credit_data.pay_loan += 1;

            //Update credit report data
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.credit_report_vault.put(credit_nft_bucket);

            //Return loan receipt to borrower
            return loan_receipt;

        }

//Liquidate
        pub fn liquidate(&mut self, nft_bucket:NonFungibleId) -> Bucket{

            //Take loan NFT from loan vault
            let nft_loan_bucket = self.loan_vault.take_non_fungible(&nft_bucket);

            //Get loan NFT data
            let nft_loan_data:LoanNFT = nft_loan_bucket.non_fungible().data();

            //Get component address from loan NFT data
            let account_address = nft_loan_data.account_address;

            //Convert component address to NonFungibleId
            let account = NonFungibleId::from_str(&account_address.to_string()).unwrap();

            //Assert the current price of XRD is less than the liquidation price of the loan
            assert!(!(nft_loan_data.xrd_liquidation_price <= self.xrd_price_oracle), "Can't liquidate this loan since the price of XRD is >= liquidation price");

            //Take the liquidated loan collateral out of the collateral vault
            let mut collateral_bucket:Bucket = self.collateral_vault.take(nft_loan_data.collateral_amount_xrd);

            //Take the borrowed amount XRD from the collateral bucket
            let borrower_bucket:Bucket = collateral_bucket.take(nft_loan_data.borrow_amount_xrd);

            //Return borrowed amount the lending pool
            self.lending_pool.put(borrower_bucket);

            let liquidation_fee = collateral_bucket.amount() / dec!(2);

            //Take half of the remaining funds in the collateral bucket and place put in the lending pool
            self.lending_pool.put(collateral_bucket.take(liquidation_fee));

            //Burn the liquidated loan NFT
            self.minting_badge_vault.authorize({|| 
                nft_loan_bucket.burn()
            });

            //Updata credit report NFT
            //Take credit report NFT from credit report vault
            let credit_nft_bucket = self.credit_report_vault.take_non_fungible(&account);

            //Get credit report data
            let mut credit_data:CreditReportNFT = credit_nft_bucket.non_fungible().data();

            //Modify credit report data by incrementing liquidations by 1
            credit_data.liquidations += 1;

            //TODO Reduce credit score by 25% for getting liquidated

            //Update credit report NFT data
            self.minting_badge_vault.authorize(|| {
                credit_nft_bucket.non_fungible().update_data(credit_data)
            });

            //Put the credit report NFT back in the credit report vault
            self.credit_report_vault.put(credit_nft_bucket);

            //Return remaing funds in collateral bucket to liquidator 
            return collateral_bucket;
        
        }
//Harvest fee.  Harvesters can find loans that have matured and earn 50% of the difference between loan orgination 
        //or the last harvest epoch and the current epoch.  
        pub fn harvest_fee(&mut self, id:NonFungibleId) -> Bucket {

            let loan_nft_bucket = self.loan_vault.take_non_fungible(&id);

            let mut loan_nft_data:LoanNFT = loan_nft_bucket.non_fungible().data();

            let loan_length = Runtime::current_epoch() - loan_nft_data.fee_epoch;

            //17520 epoch in a year.  At 3% APY == 0.000171%/epoch
            let loan_fee = loan_nft_data.borrow_amount_usd * loan_length * dec!("0.00000171");

            info!("Loan Fee Harvested {} XRD", loan_fee );
            // info!("XRD price {}", self.xrd_price_oracle );
            // info!("loan fee / price {}", loan_fee/self.xrd_price_oracle );
            // info!("loan fee / 2 {}", loan_fee * dec!(".5") );

            //Find loan fee denominated in XRD
            let loan_fee_xrd = loan_fee/self.xrd_price_oracle;

            //Take collateral out of collateral vault
            let mut loan_fee_bucket = self.collateral_vault.take(loan_fee_xrd);

            info!("loan fee {}", loan_fee_xrd );

            //Put half the loan fee into the lending pool
            self.lending_pool.put(loan_fee_bucket.take(loan_fee_xrd/2));

            //Modify Loan NFT data
            loan_nft_data.collateral_amount_xrd -= loan_fee;

            //Modfiy loan NFT data liquidation price
            loan_nft_data.xrd_liquidation_price = loan_nft_data.borrow_amount_usd * dec!("1.5") / loan_nft_data.collateral_amount_xrd;

            //Modify loan NFT data fee epoch
            loan_nft_data.fee_epoch = Runtime::current_epoch();

            //Update Loan NFT data
            self.minting_badge_vault.authorize(|| {
                loan_nft_bucket.non_fungible().update_data(loan_nft_data)
            });

            self.loan_vault.put(loan_nft_bucket);

            //Return remaining loan fee to harvester 
            return loan_fee_bucket;
        }
    }
}