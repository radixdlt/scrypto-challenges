# Collaborative dApp

The Collaborative dApp is a decentralized application where users can collaborate to a portfolio management solution where no users profiling exist and it is completely permissionless, any user can deposit asset and any user can put asset at work towards the simulated available deFi applications.

It is a kind of a social trading application but it differs from it because we think that financial knowledge is spread and also the final user has a great understanding and could operate in such a broad market for opportunities.

Collaborative dApp has only some simple rules:
* users can deposit asset
* users can reedem asset at any moment
* users can put asset at work until 10X of its asset value
* users can close any current operation only if it's current value is lower than initial value
* users can close their operations anytime

The rules aim: 
   - to incentivates users that have not sufficient capital to put it at work but have instead knowledge 
   - to help users to close their losing position that is usually a difficult decision to put in action

Operations history will be registered on each user so anyone can evaluate each other's.

Simulated deFi applications used in this portfolio management solution are the following:
- Lending application (we'll use LendingdApp developed for the previous challenge)
- Trading application
- Swap application

# Design

Blueprint can create new components with only a single vault and a map containing all the info about the operation opened/in place/closed.

Component has some very simple method for depositing/taking from the main vault:
- deposit(bucket) -> tokens are put in the main vault -> account receives an nft (transferable) who states the amount of tokens deposited
- take(proof, amount) -> a bucket gets created with tokens from the main vault and sent back if the account has presented a valid proof 

And some others for executing orders/operations:
- buy(proof, amount, resource_address) -> a buy order is issued using the 'trading blueprint' for the amount specified and the resource address
- sell(proof, amount, resource_address) -> a sell order is issued using the 'trading blueprint' for the amount specified and the resource address
- register() -> a register is asked to the LendingdApp component
- lend(proof, bucket) -> a bucket has lent to the LendingdApp component 
- take_back(proof, amount) -> a bucket is created and sent back to the account

And also some for closing orders/operations:
- close_operation(proof) -> account that has opened the operation can close it anytime
- list_open_operation() -> list of open operation and its account creator 
- close_someone_else_operation(operation_id) -> close an operation opened by someone else, available only if the operation is losing


The following methods should update the soulbound token of the account that has created the operation:
- sell      -> should update in the sbt the number of positive operation if the result has been positive, otherwise no
- take_back -> should update in the sbt the number of positive operation if the result has been positive, otherwise no

The following methods should update the main map containing the info about all the operations:
- buy       -> should insert in the map the new operation with operation_id, amount, date_opened
- sell      -> should update in the map the closed operation with date_closed
- lend      -> should insert in the map the new operation with operation_id, amount, date_opened
- take_back -> should update in the map the closed operation with date_closed
- close_operation -> should find all the opening and close everything
- close_someone_else_operation -> 

The data about the operation contains the following:
- operation_id: id created random
- amount: size of the operation
- date_opened: epoch when it has been opened
- date_closed: epoch when it has been closed
- current_standing: actual result (profitable/losing position)
- number_of_request_for_autoclosing: number or request needed for the operation to be closed even if creator does not agree
- [current_requestor_for_closing]: account requesting its closing
 
# Preparazione simulatore

//eseguo il publish
resim publish .
export package=

//creo i tokens
resim new-token-fixed --name bitcoin --symbol btc 10000
resim new-token-fixed --name ethereum --symbol eth 1000
resim new-token-fixed --name leonets --symbol leo 10000
export xrd=030000000000000000000000000000000000000000000000000004

//creo l'account
resim new-account
export account=

//creo il component
-- resim call-function $package TradingApp create_market 1000,$xrd 10,$btc 1000,$eth 1000,$leo
export component=0273ab5508a644b26b3b2f06e56edea9e40e16d99c5b9f03130791
//per poter eseguire il test del componente devo crearlo senza bucket ma con i resource address e poi farne il funding
resim call-function $package TradingApp create_market $xrd $btc $eth $leo


--dopo la creazione dei fixed token questi si trovano tutti nell'account principale

//procedo con il funding del market
resim call-method $component fund_market 1000,$xrd 1000,$btc 1000,$eth 1000,$leo

//eseguo un'operazione di acquisto (non funziona)
resim call-method $component buy_generic 1000,$xrd $eth

//eseguo un'operazione di acquisto e poi una di vendita (funziona)
resim call-method $component buy 1000,$xrd "no"
resim call-method $component sell 25,$btc "no"

//aggiorno il package
resim publish . --package-address $package



# Test unitari

Eseguire 'scrypto test' 
