use scrypto::prelude::*;

blueprint! { 
 struct Accounting {
  accounts_payable: Vault,
  earnings: Vault,
  doa_owned_member_tokens: Vault,
 }

// TODO make this an owned component of the DAO and Instantiate at time of DAO creation
// TODO Set access control rules for founders_badge, operator_badge, and accounting_badge
 impl Accounting {
   pub fn instantiate_accounting() -> ComponentAddress {
     Self {
      accounts_payable: Vault::new(RADIX_TOKEN),
      earnings: Vault::new(RADIX_TOKEN),
      doa_owned_member_tokens: Vault::new(RADIX_TOKEN),
     }
     .instantiate()
     .globalize()
   }

   pub fn generate_earnings_report(member_owned_tokens: Proof) {
    // Calculate earnings_per_token == total_earnings / total_shares
    // calculate earnings per member --> member_owned_tokens * earning_per_token <--
    // return members(member -> {member_address: total_earnings_due})

   }

   pub fn distribute_earnings() {
    // Validate accounting_badge && founders_badge

    // distribute earnings to members

    // transfer doa owned earnings to accounts_payable vault
    
   }

   pub fn pay_expenses() {
    // Validate accounting_badge

    // send expense transactions

   }
 }
}
