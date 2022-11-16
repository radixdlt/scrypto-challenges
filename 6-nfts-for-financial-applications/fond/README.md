# Fond 
A platform where users can invest in an asset proposed by the company. Once the users invest they get an NFT representing their share of the item. When the investment goal for an asset is reached, the platform buys the item. It can later sell a previously bought item and distributes the funds to their owners accordingly.

## Initialisation:
Smart contract initiates the company.
- This creates an Admin Badge for the account initialising the smart contract.
- Creates a vault for the currently listed items (items that haven't reached their investment goal yet)
- Creates a vault for the company's inventory (items that have been bought already)
- Creates a structure to keep track of the funds that have been put into an item (e.g.: a HashMap<NonFungibleId, Vault>, where NonFungibleId is the ID of the listed asset and the Vault is where the funds are stored.)

## User roles:

### Admin Badge holder 
⬇️ 1. add admins
  - creates a badge to give to that account

⚠️ 2. create a campaign to invest in a new item.
- INITIALLY: accept RDX only
- LATER: specify an accepted token per item

⚠️ 3. buy an item
- INITIALLY: keep it simple, mint an NFT object and burn the funds
- add to company inventory vault
- LATER: consider oracles for external data exchange

⚠️4. sell items owned by the company
 - this releases the item from the company inventory at a price (retrieved from external source - mocked in our case)
- INITIALLY: we just fix a price (let's say for example it always sells for 10% more of the original price), and simulate selling it to an external source, i.e., we burn the asset and split the acquired funds with the investors of the campaign at amounts relative to their contribution
- LATER: Oracles for external data sources and company taking a cut of the funds

⬇️ 5. move funds from the company vaults to admin accounts
6. can see available campaigns 
7. can see inventory of company 
can't really make my mind up on the last two
### User
⚠️ 1. can see available campaigns to invest in (get_campaigns)
⚠️ 2. can invest in available campaigns (invest_in_campaign)
⬇️ 3. can see inventory of company
⬇️ 4. can go back on their decision and sell their ownership NFT (assuming the item is still listed), for the initial amount they invested

## current script

(First time)
resim reset
resim new-account
export acc1=<account-address>

(cd to /fond)
resim publish . 
export pkg=<package-address>

resim call-function $pkg Fond instantiate_fond  
export comp=<component-address>
resim call-method $comp create_campaign "wine" "red" 6.0 