/// This smart contract is used to create the startuP, invest into it and withdraw the investments.
/// Built with Scrypto v0.8

use scrypto::prelude::*;
use std::collections::HashMap;


#[blueprint]
mod investment {
    struct Investment {
        // Name of the startup
        startup_name: String,
        // HashMap with investors and the amount invested
        investors: HashMap<String,Decimal>,
        // Vault where the xrd payments will be stored.
        xrd_tokens_vault: Vault,
        // Investment goal for the startup
        investment_goal: Decimal,
    }

    impl Investment {
        // "new" function initializes the smart contract and sets up the goal for the startup
        pub fn new(investment_goal: Decimal, startup: String) -> (ComponentAddress, Bucket) {

            //Creates an owner badge
            let owner_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Startup Owner Badge")
                .metadata("symbol", "SUOwner")
                .mint_initial_supply(1);

            //Gives owner the access to use the "withdraw" method
            let access_rules: AccessRules = AccessRules::new()
            .method("withdraw", rule!(require(owner_badge.resource_address())), LOCKED)
            .default(rule!(allow_all), LOCKED);

            //Fills up the variables in "Investment" struct
            let mut investment_component: InvestmentComponent = Self {
                startup_name: startup,
                investors: HashMap::default(),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                investment_goal: investment_goal,
            }
            .instantiate();
        investment_component.add_access_check(access_rules);
        let investment_component_address: ComponentAddress = investment_component.globalize();
        return (investment_component_address,owner_badge);
        }
        
        // "invest" method allows the users to invest in the product
        pub fn invest(&mut self, funds: Bucket, investor: String) -> Decimal {
            let new_investment_amount = funds.amount();
            self.xrd_tokens_vault.put(funds);
            self.investors.insert(investor,new_investment_amount);
            self.xrd_tokens_vault.amount()
        }

        // "withdraw" method allows the startup owner to withdraw the investments
        pub fn withdraw(&mut self) -> Bucket {
            //Checks if the collected amount is more or equal to the investment goal
            if self.xrd_tokens_vault.amount() >= self.investment_goal {  
            //returns the amount collected
            self.xrd_tokens_vault.take_all()
        }
        // Returns an error log and an empty bucket if the amount is not enough 
        else {
        error!("NOT ENOUGH INVESTED");
        self.xrd_tokens_vault.take(0)
    }
   }
  }
 }
