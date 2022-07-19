use scrypto::prelude::*;

//The centralRepo allows users to create lending offers. The offers are stored in the offers vector until no longer needed
blueprint! {
  struct CentralRepository {
    offers: Vec<(Decimal, ResourceAddress, Decimal, Decimal, u64, ComponentAddress, ResourceAddress, ResourceAddress)>,
    package_address: PackageAddress,
    internal_badge: Vault,
    admin_badge_addr: ResourceAddress,
    oracle_admin_badge_address: ResourceAddress, 
  }

  impl CentralRepository {
    pub fn instantiate(package_address: PackageAddress) -> (ComponentAddress, Bucket) {
      // internal badge to mint the admins badge
      let Internal_badge = ResourceBuilder::new_fungible().initial_supply(1);
      
      // Admin badge which will be given to each sec lending component
      let admin_badge_addr = ResourceBuilder::new_fungible()
          .mintable(rule!(require(internal_badge.resource_address())), LOCKED)
          .burnable(rule!(allow_all), LOCKED)
          .no_initial_supply();

      // Create the badge that allows to update the timestamps for secLending
      let oracle_admin_badge: Bucket = ResourceBuilder::new_fungible()
          .metadata("name", "oracle_admin_badge")
          .divisibility(DIVISIBILITY_NONE)
          .initial_supply(1);
      
      let component_address = Self {
        offers: Vec::new(),
        package_address: package_address,
        internal_badge: Vault::with_bucket(internal_badge),
        admin_badge_addr: admin_badge_addr,
        oracle_admin_badge_address: oracle_admin_badge.resource_address(),
      }
      .instantiate()
      .add_access_check(
          AccessRules::new()
          .default(rule!(allow_all))
      )
      .globalize();
      
      (component_address, oracle_admin_badge)
    }
    
    // when a lender want to create a new offer, it provide information to create the new seclending component, and return the lender badge to the creator
    pub fn new_offer(&mut self, tokens: Bucket, collat_amount: Decimal, collat_resource_address: ResourceAddress ,cost_amount_per_hour: Decimal, fee_resource_address: ResourceAddress,max_borrow_time_in_hours: u64) -> (ComponentAddress, Bucket) {
      let admin_badge = self.internal_badge.authorize(|| borrow_resource_manager!(self.admin_badge_addr).mint(Decimal::ONE));
      let my_address : ComponentAddress = Runtime::actor().component_address().unwrap();
      let tokens_amount = tokens.amount();
      let tokens_address = tokens.resource_address();
      let args = args![tokens, collat_amount, cost_amount_per_hour, max_borrow_time_in_hours, admin_badge, my_address, self.oracle_admin_badge_address, collat_resource_address, fee_resource_address];
      let result = Runtime::call_function(self.package_address, "SecurityLending", "instantiate", args);
      let option_result = scrypto_decode(&result);
      let (comp, badge) : (ComponentAddress, Bucket) = option_result.unwrap();
      self.offers.push((tokens_amount, tokens_address, collat_amount, cost_amount_per_hour, max_borrow_time_in_hours, comp, collat_resource_address, fee_resource_address));
      (comp, badge)
    }

    // when a seclending component receive an accept or cancel order, it remove itself from the central component, to not be listed anymore 
    pub fn remove_offer(&mut self, addr: ComponentAddress, admin_badge: Bucket) {
      assert!(admin_badge.amount().is_positive(), "The badge is empty");
      assert!(admin_badge.resource_address() == self.admin_badge_addr, "The badge provided is incorrect");
      self.offers = self.offers.drain(..).filter(|o| o.5 != addr).collect();
      admin_badge.burn();
    }
  }
}