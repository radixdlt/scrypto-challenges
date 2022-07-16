# Credit Lender

Credit Lender brings together lenders and borrowers and generates a credit score associated with the users wallet.  This lending protocal is made up of lenders, borrowers, liquidators, and fee harvestors. 

## Features

Lenders can provide liquidity to the protocal and earn yield.  Borrowers can overcollaterlize their $XRD by 150% and take out a low cost loan for 3% APY.  Both lenders and borrowers will start earning $CT crefdt tokens after a 1 week opening thier position.  Lenders earn at half the rate as borrowers.  

Their is a 1% loan orgination fee which is split evenly across the lenders.  Loans that do not maintain their 150% overcollaterlization rate will be liquidated.  When a loan is liquidated the orginal loan amount is paid off, and the liquidator and lenders split the remaining collateral 50/50.  The 3% APY is collected by fee harvesters and split evenly between the harvestor and lenders.

## Design Details 

There are four NFTs that are used in the functionality of the protocal.  They are the lender receiept NFT, borrower receipt NFT, loan NFT, and credit report NFT.  

When a lender add liquidity they get a lenders receipt NFT.  When they close their position the lender receipt is burned.  The lenders receipt NFT data consists of the website information, account address, issued epoch, and liquidity amount.

When a borrower creates a loan they get a borrower receipt NFT and a loan NFT is created and stored on the protocal.  The borrower receipt NFT data consists of the website information, account address, and borrowed amount USD.  The borrower will need the borrower receipt NFT to add/remove collateral, borrow more/pay down, and close out loan.  When a borrower modifies their loan, the loan NFT is updated.  When they close the loan the borrower receipt NFT and the loan NFT are burned.  

The loan NFT is stored in a vault on the protocal.  The loan NFT data includes account address, issued epoch, fee epoch, borrow amount USD, borrow amount XRD, collateral amount XRD, and liquidation price. 

The credit report NFTs are stored in a vault on the protocal.  The credit report NFT ID is the users wallet address.  This ensures that only one credit report NFT will ever be associated with that wallet.  When users are providing liquidity or creating a loan, the protocal will search for an existing credit report NFT, prior to creating one associated with the users account.  The credit report NFT data consists of the account_address, open lend, close lend, open loan, close loan, add collateral, remove collateral, pay loan, borrow more, liquidations, and credit score.  

Liquidators and fee harvestors ensure that lenders liquidity is preserved and loan fees are collected.  When a liquidator liquidates a loan, the loans collateral is withdrawn and distributed and the loan NFT is burned.  When a fee harvestor harvests a loan, fees are taken from the loans collateral and the loan NFT is updated.    

## Getting Started

Below is a walkthrough for how this protocal works.  By defualt the price of $XRD is intially set to $1.    

1.  Let start off by creating 3 new accounts, publishing the blueprint, saving some resource addresses to variables for easy access. NOTE: Be sure to copy/paste new pacakge, lender/borrowers receipts resource address into the code below. 

``` 
resim reset
export op1=$(resim new-account)
export publickey1=$(echo "$op1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey1=$(echo "$op1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account1=$(echo "$op1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export op2=$(resim new-account)
export publickey2=$(echo "$op2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey2=$(echo "$op2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account2=$(echo "$op2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export op3=$(resim new-account)
export publickey3=$(echo "$op3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey3=$(echo "$op3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account3=$(echo "$op3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export xrd=030000000000000000000000000000000000000000000000000004
resim publish .
export package="PASTE NEW PACKAGE HERE"
resim call-function $pkg LendingCredit new
export component="PASTE COMPONENT ADDRESS HERE"
export lender_reciept="PASTE LENDER RECEIPT HERE- 2nd from top resource"
export borrower_receipt="PASTE BORROWER RECEIPT HERE- 3rd from top resource"
```
2. Lets open a lending position for all 3 accounts that add different amounts of liquidity.  Account 1 adds 1000 $XRD, account2 adds 2000 $XRD, and account 3 5000 $XRD.  
```
resim set-default-account $account1 $privatekey1
resim call-method $component add_funds 1000,$xrd $account1
resim set-default-account $account2 $privatekey2
resim call-method $component add_funds 2000,$xrd $account2
resim set-default-account $account3 $privatekey3
resim call-method $component add_funds 5000,$xrd $account3
```
Lets check the component to make sure that a credit report has been created for each account and there is 8000 $XRD in the lending pool.  Lets also check each account to make sure they have a lending receipt NFT, and that open lend and credit score are increment by 1 and 10.  Note the lending receipt captures the amount of liquidity each lender provided.     

```
resim show $component
resim show $account1
resim show $account2
resim show $account3
```
3. Lets create 3 more accounts and have each account create a loan and over collateralize them by 200%.  Account4 creates a loan for 100 $XRD with 200 $XRD collateral.  Account5 creates a loan for 300 $XRD with 600 $XRD collateral.  Account6 creates a loan for 500 $XRD with 1000 $XRD collateral.    
```
export op4=$(resim new-account)
export publickey4=$(echo "$op1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey4=$(echo "$op1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account4=$(echo "$op1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export op5=$(resim new-account)
export publickey5=$(echo "$op2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey5=$(echo "$op2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account5=$(echo "$op2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export op6=$(resim new-account)
export publickey6=$(echo "$op3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey6=$(echo "$op3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account6=$(echo "$op3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
resim set-default-account $account4 $privatekey4
resim call-method $component new_loan 100 200,$xrd $account4
resim set-default-account $account5 $privatekey5
resim call-method $component new_loan 300 600,$xrd $account5
resim set-default-account $account6 $privatekey6
resim call-method $component new_loan 500 1000,$xrd $account6
```
Lets check the component to make sure that a credit report has been created for account4, 5, and 6.  Lets check the lending pool which had a balance of 8000 $XRD, minus 900 $XRD in new loans, plus 9 $XRD from 1% loan orgination fees, and that it is equal to 7109 $XRD.  Note that each loan NFT captures the USD amount, XRD amount, XRD collateral amount, and liquidation price of each loan.  

Lets also check accoun4, 5, and 6, to make sure they have a borrow receipt NFT and that the value of the new loan is correct.  

resim show $component
resim show $account4
resim show $account5
resim show $account6

4. Lets assume 1000 epoch later the $xrd price is $2.  We can simulate this by setting the current epoch calling a oracle method.

```
resim set-current-epoch 1000
resim call-method $component set_xrd_price 2
```

Account 4 decided to take profits and close out their loan.  This can be done by presenting the borrower receipt and amount borrowed to the close_loan method.

```
resim set-default-account $account4 $privatekey4
resim call-method $component close_loan 1,$borrower_receipt 100,$xrd
```
Lets check the lending pool to make sure the orginal loan amount of 100 $XRD was returned plus the fee.  The lending pool was at 7190 $XRD, plus 100 $xrd from the orginal loan, plus 0.00855 $XRD, which equals 7209.00855.

```
resim show $component
```

Fees are calcualted using the following assumptions...
1 epoch = 30min -> 17520 epoch = 1 year 
3% APY/epochs in a year -> 0.03/17520 = 0.00000171 per epoch

So in this case the original loan is 100XRD and the price $100.  3% APY calculated using the USD value of the loan.  $100 * 1000 epoch * 0.00000171/epoch = $0.171.  With a $XRD = $2, this is equivalant to $0.171/$2 per XRD = 0.00855 $XRD.

Also note, when viewing the component that account4 now has a credit score of 2020.  Borrowers earn 20 $CT credit token at a loan creation and at a rate of 2 $CT per epoch opened.  Account4 credit report also shows that 1 loan was created and closed.   

If we check balances for account4...

```
resim show $account 4
```

Account4 started with 1000000 $XRD, minus 1 $XRD for loan orgination fee, minus 0.00855 $XRD for fee, which equals 999998.99145.

5. Lets add additional collateral to account5, which has a 300XRD loan 597 $XRD collateral and a liquidation price of 0.753.  

```
resim set-default-account $account5 $privatekey5
resim call-method $component add_collateral 1,$borrower_receipt 600,$xrd
resim show $component
```
Looking at the loan NFT data in the component, the collateral had increased by 600XRD and the liquidation price is now 0.375.

Lets assume the account5 is in need of funds and wants to remove the collateral they just added.  

```
resim call-method $component remove_collateral 1,$borrower_receipt 600
resim show $component 
```
Looking at the loan NFT data, the collateral and liquidation price have returned to what they were previously at.  

6. Account5 still needs $300 more so lets borrow more from the loan.  

```
resim call-method $component borrow_more 1,$borrower_receipt 300
resim show $component
```
Looking at the Loan NFT data on in the component, the amount borrowed USD is now $600, amount borrowed XRD is 450, and liquidation price is 1.507.

7. Four Arrows Capital and Fahrenheit hedge funds have caused the crypto markets to plunge.  The price of $XRD drop to $1.35 overnight.  Lets create account7 and liquidate account5 loan.  The bad loan id is found in the loan NFT vault in the component.  This will be needed to call the liquidate method.  Account5 borrowed 450 $XRD and had 597 $XRD in collateral.  The liquidation method takes 450 $XRD from the collateral and puts it in the lending pool.  The remaining 197 $XRD is split 50/50 and added to the lending pool and to account7 balance.  

```
resim call-method $component set_xrd_price 1.35
export op7=$(resim new-account)
export publickey7=$(echo "$op7" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey7=$(echo "$op7" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account7=$(echo "$op7" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
resim set-default-account $account7 $privatekey7
resim call-method $component liquidate "INPUT LOAN ID HERE"
resim show $component
```

Note that the liquidated loan NFT has been burned and is no longer in the component.  Account5 credit report NFT data now shows that there has been 1 liquidation.  The lending pool previsouly had 7059.00855, plus 450 XRD from orginal loan, plus 73.5 XRD for liquidation fee, which equate to 7582.508.

```
resim show $account7
```

Note the new account7 balance of 1000073.5 XRD

8. Lets travel to the future and set the current epoch to 10000.  Lets also create an account8 and harvest the fees for the account6 loan.  We will need the loan NFT ID from the loan vault on the component.   

```
resim set-current-epoch 10000
export op8=$(resim new-account)
export publickey8=$(echo "$op8" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export privatekey8=$(echo "$op8" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account8=$(echo "$op8" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
resim set-default-account $account8 $privatek7585.752166666666666666ey8
resim call-method $component harvest_fee "INPUT LOAN NFT ID HERE"
```
The loan fee is calculated using the length of time between the loan orgination or the last time the fee from the where harvested.  Account6 loan borrowed $500 for 10000 epochs.  $500 * 10000 epochs * 0.00000171/epoch = $8.55.  With the current price of $XRD at $1.35, that is equivalant to $8.55/$1.35/XRD = 6.333 $XRD.  This fee is split 50/50 between the lending pool and account8.

The lending pool was at 7059.0855, plus 3.166 which equate to 7585.674


















 










