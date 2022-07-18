use scrypto::prelude::*;

#[derive(Debug, PartialEq, sbor::Decode, sbor::Encode, sbor::Describe, sbor::TypeId)]
// When instantiated, the components functions react differently depending on the State the component is in. 
enum State {
    StateWaitForRenter,
    StateRenting,
    StateReturned,
    StateDefaulted,
    StateSettled,
    StateCancelled,
}

// Used to identify the borrower, and help the frontend to list the relevant component
fn create_borrower_badge(component_address: ComponentAddress) -> Bucket {
    ResourceBuilder::new_fungible()
        .metadata("name", "borrower badge")
        .metadata("central_exchange", "MiCroqLending")
        .metadata("component_address", component_address.to_string())
        .divisibility(DIVISIBILITY_NONE)
        .burnable(rule!(allow_all), LOCKED)
        .updateable_metadata(rule!(allow_all), LOCKED)
        .initial_supply(1)
}

// Used to identify the lender
fn create_lender_badge() -> Bucket {
    ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .burnable(rule!(allow_all), LOCKED)
        .updateable_metadata(rule!(allow_all), LOCKED)
        .initial_supply(1)
}

// Adds the securityLending component address which allows to match userAcc and component in the frontend
fn update_lender_badge(lender_badge_address: ResourceAddress, component_address: ComponentAddress) {
    let resource_manager: &ResourceManager = borrow_resource_manager!(lender_badge_address);
    resource_manager.update_metadata(HashMap::from([
      (String::from("name"), String::from("lender badge")),
      (String::from("central_exchange"), String::from("MiCroqLending")),
      (String::from("component_address"), component_address.to_string())
    ]));
}

// Allows user to lend tokens in exchange for a fee and with a collateral. User can specify the maximum duration of lending. We use real-world time coming from an oracle (the owner of the central repository)
blueprint! {
    struct SecurityLending {
      token_address: ResourceAddress, // Address of the token to borrow
      token_quantity: Decimal, // Quantity of token available to borrow
      collat_amount: Decimal, // Collateral amount to be provided
      cost_amount_per_hour: Decimal, // Fee for borrowing the security
      max_borrow_time_in_hours: u64, // Maximum amount before the lender can declare default
      unix_borrow_start: u64, // Time when the offer has been accepted and the borrowing started
      state: State, // Status of the component
      fee_resource_address: ResourceAddress, // Address of the token used for the fees (usually Radix)
      collat_resource_address: ResourceAddress, // Address of the token used for collateral (usually Radix)
      max_lending_cost_amount: Decimal, // max borrow time * cost
      token_vault: Vault, // vault to store the borrowed tokens
      collat_vault: Vault, // vault to store the collateral
      lending_fee_vault: Vault, // vault to store the fees
      admin_badge: Vault, // vault to store the badge, which allow the component to unreference itself from the central component
      lender: ResourceAddress, // address of the lender badge
      central_repo: ComponentAddress, // address of the central component
      oracle_admin_badge_address: ResourceAddress, // address of the oracle badge
      
      // needed for the interaction with the oracle
      unix_time: u64, // last time sent by the oracle
      unix_time_is_valid: bool // did the time is still valid (the oracle will invalid it after a bit of time)
    }

    impl SecurityLending {
        pub fn instantiate(tokens: Bucket, collat_amount: Decimal, cost_amount_per_hour: Decimal, max_borrow_time_in_hours: u64, admin_badge: Bucket,central_repo: ComponentAddress ,oracle_admin_badge_address: ResourceAddress, collat_resource_address: ResourceAddress, fee_resource_address: ResourceAddress,) -> (ComponentAddress, Bucket) {
            assert!(
                collat_amount.is_positive() || collat_amount.is_zero(),
                "collateral amount should be positive"
            );
            assert!(
                cost_amount_per_hour.is_positive() || cost_amount_per_hour.is_zero(),
                "cost amount should be positive"
            );
            
            let lender_badge = create_lender_badge();
            
            // guard oracle methods should
            let access_rules = AccessRules::new()
                 .method("update_unix_time", rule!(require(oracle_admin_badge_address)))
                 .method("invalidate_unix_time", rule!(require(oracle_admin_badge_address)))
                 .default(rule!(allow_all));

            let component_address = Self {
                token_address: tokens.resource_address(),
                token_quantity: tokens.amount(),
                collat_amount,
                cost_amount_per_hour,
                max_borrow_time_in_hours,
                unix_borrow_start: 0,
                state: State::StateWaitForRenter,
                collat_resource_address: collat_resource_address,
                fee_resource_address: fee_resource_address, 
                max_lending_cost_amount: cost_amount_per_hour * max_borrow_time_in_hours, 
                
                token_vault: Vault::with_bucket(tokens),
                collat_vault: Vault::new(collat_resource_address),
                lending_fee_vault: Vault::new(fee_resource_address),
                admin_badge: Vault::with_bucket(admin_badge),
                lender: lender_badge.resource_address(),
                central_repo: central_repo,
                oracle_admin_badge_address: oracle_admin_badge_address,
               
                unix_time: 0,
                unix_time_is_valid: false
            }
            .instantiate()
            .add_access_check(
                access_rules
            ).globalize();
            update_lender_badge(lender_badge.resource_address(), component_address);
            (component_address, lender_badge)
        }
        
        fn remove_from_central_repository(&mut self) {
          let admin_badge = self.admin_badge.take_all();
          let my_address : ComponentAddress = Runtime::actor().component_address().unwrap();
          Runtime::call_method(self.central_repo, "remove_offer", args![my_address,admin_badge]);
        }

       
        // unix time is supposed to only be valid for 10 minutes so we can get close to the real lending time
        // This method is guarded. Therefore, only the owner of the oracle_badge can call it.
        pub fn update_unix_time(&mut self,new_unix_time: u64) {
            self.unix_time = new_unix_time;
            self.unix_time_is_valid = true;
        
        }

        // The oracle calls invalidate_unix_time() 10min after the update, which leads to some methods throwing an assertError.
        // This method is guarded. Therefore, only the owner of the oracle_badge can call it.
        pub fn invalidate_unix_time(&mut self) {
            self.unix_time_is_valid = false;
        }

        // The cancellation is protected by the borrower badge, which is destroyed once called
        // This method call the central component to remove this component from the list of active offer
        pub fn cancel(&mut self, badge: Bucket) -> Bucket {
            assert!(badge.resource_address() == self.lender, "Only the lender can cancel the contract");
            assert!(self.state == State::StateWaitForRenter, "You can only cancel a contract if the renter has not yet been found");
            self.state = State::StateCancelled;
            badge.burn();
            self.remove_from_central_repository();
            self.token_vault.take_all()
        }
        
        // The default is protected by the oracle timer and by the borrower badge, which is destroyed once called
        pub fn default(&mut self, badge: Bucket) -> (Bucket, Bucket) {
            assert!(self.unix_time_is_valid == true, "Ask the oracle to update the time first");
            assert!(badge.resource_address() == self.lender, "Only the lender can default the contract");
            assert!(self.state == State::StateRenting, "Contract can't be defaulted");
            // Check if the max_borrow_time in hours is already reached. 3600000 is miliseconds per hour
            assert!((self.unix_borrow_start + 3600000 * self.max_borrow_time_in_hours) > self.unix_time, "Contract not in default, yet");
            self.state = State::StateDefaulted;
            badge.burn();
            (self.collat_vault.take_all(), self.lending_fee_vault.take_all())
        }
        
        // The settlement is protected by the borrower badge, which is destroyed once called
        pub fn settle(&mut self, badge: Bucket) -> (Bucket, Bucket) {
            assert!(badge.resource_address() == self.lender, "Only the lender can settle the contract");
            assert!(self.state == State::StateReturned, "Token has not been returned yet");
            self.state = State::StateSettled;
            badge.burn();
            (self.token_vault.take_all(), self.lending_fee_vault.take_all())
        }
        
        // The borrower badge doesn't really protect this method, because we don't care who is reimbursing.
        // we'll burn the badge to not encumber the wallet of the borrower
        pub fn return_asset(&mut self, mut token: Bucket, borrower_badge: Bucket) -> (Bucket, Bucket, Bucket) {
            assert!(self.unix_time_is_valid == true, "Ask the oracle to update the time first");
            assert!(self.state == State::StateRenting, "Token cannot be returned anymore");
            assert!(token.resource_address() == self.token_vault.resource_address(), "incorrect token type");
            assert!(self.token_quantity <= token.amount(), "not enough tokens");
            self.state = State::StateReturned;
            self.token_vault.put(token.take(self.token_quantity));

            // Calculates how much of the lending_fee will be given back to lender 
            let hours_passed = ((self.unix_time - self.unix_borrow_start) / 3600000) as u64;
            let mut percentage_of_max_hours = hours_passed / self.max_borrow_time_in_hours;
            if percentage_of_max_hours > 1{percentage_of_max_hours = 1}
            let lending_fee_leftover = self.lending_fee_vault.take(self.lending_fee_vault.amount() * percentage_of_max_hours);
            borrower_badge.burn();
            (self.collat_vault.take_all(), token, lending_fee_leftover)
        }
        
        // The borrower call this method to accept the offer, he receive a badge in return to help the frontend to keep track of it
        // This method call the central component to remove this component from the list of active offer
        pub fn borrow(&mut self, mut collat: Bucket, mut lending_fee: Bucket) -> (Bucket, Bucket, Bucket, Bucket) {
            assert!(self.unix_time_is_valid == true, "Ask the oracle to update the time so the contract knows when you start borrowing");
            assert!(self.collat_vault.resource_address() == collat.resource_address(), "collateral resource is of wrong type");
            assert!(self.lending_fee_vault.resource_address() == lending_fee.resource_address(), "lending fee resource is of wrong type");
            assert!(self.state == State::StateWaitForRenter, "Token cannot be borrowed anymore");
            assert!(collat.resource_address() == self.collat_vault.resource_address(), "incorrect collateral type");
            assert!(lending_fee.resource_address() == self.lending_fee_vault.resource_address(), "incorrect cost currency");
            assert!(collat.amount() >= self.collat_amount, "not enough collateral");
            assert!(lending_fee.amount() >= (self.cost_amount_per_hour * self.max_borrow_time_in_hours), "need more to cover borrowing the cost");
            self.unix_borrow_start = self.unix_time;
            self.unix_time_is_valid = false; 
            self.state = State::StateRenting;
            self.collat_vault.put(collat.take(self.collat_amount));
            self.lending_fee_vault.put(lending_fee.take(self.cost_amount_per_hour * self.max_borrow_time_in_hours));
            let component_address : ComponentAddress = Runtime::actor().component_address().unwrap();
            let borrower_badge = create_borrower_badge(component_address);
            self.remove_from_central_repository();
            (self.token_vault.take_all(),collat,lending_fee, borrower_badge)
        }
    }
}