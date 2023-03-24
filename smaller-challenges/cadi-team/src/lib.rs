se scrypto::prelude::*;

 blueprint! {
     struct CadiTeamEscrow {
         vaults: BTreeMap<ResourceSpecifier, Vault>,
         obligation_non_fungible_resource: ResourceAddress,
         is_escrow_fulfilled: bool
     }

     impl CadiTeamEscrow {
         pub fn instantiate_escrow(
             to_be_paid_by_party_1: ResourceSpecifier,
             to_be_paid_by_party_2: ResourceSpecifier,
         ) -> (ComponentAddress, Bucket) {
             assert!(
                 to_be_paid_by_party_1.validate().is_ok(),
                 "First resource is invalid"
             );
             assert!(
                 to_be_paid_by_party_1.validate().is_ok(),
                 "Second resource is invalid"
             );
             assert_ne!(
                 to_be_paid_by_party_1, to_be_paid_by_party_2,
                 "they are not equal"
             );

             let party_1_obligation: EscrowObligation = EscrowObligation {
                 amount_to_pay: to_be_paid_by_party_1.clone(),
                 amount_to_get: to_be_paid_by_party_2.clone()
             };

             let party_2_obligation: EscrowObligation = EscrowObligation {
                 amount_to_pay: to_be_paid_by_party_2.clone(),
                 amount_to_get: to_be_paid_by_party_1.clone()
             };

             let escrow_obligations: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Escrow Obligation")
                .metadata("symbol", "ESCROW")
                .metadata("description", "This resource describes the obligation of the two parties involved in the exchange")
                .metadata("team-member-1-github-username", "andrealupini")
                .metadata("team-member-2-github-username", "gitpck")
                .metadata("team-member-1-github-username", "diogosequeira94")
                .metadata("team-member-2-github-username", "ianmac2")
                 .initial_supply([
                     (
                         NonFungibleId::from_u32(1),
                         party_1_obligation
                     ),
                     (
                         NonFungibleId::from_u32(2),
                         party_2_obligation
                     ),
                 ]);


             let mut vaults: BTreeMap<ResourceSpecifier, Vault> = BTreeMap::new();
             vaults.insert(
                 to_be_paid_by_party_1.clone(),
                 Vault::new(to_be_paid_by_party_1.resource_address())
             );
             vaults.insert(
                 to_be_paid_by_party_2.clone(),
                 Vault::new(to_be_paid_by_party_2.resource_address())
             );

             let component_address: ComponentAddress = Self {
                 vaults,
                 obligation_non_fungible_resource: escrow_obligations.resource_address(),
                 is_escrow_fulfilled: false
             }
             .instantiate()
             .globalize();

             (component_address, escrow_obligations)

         }

         pub fn deposit(&mut self, obligation_badge: Proof, mut funds: Bucket) -> Bucket {
             /// First we need to make sure if the person passed the proper obligation badge (we have created)
             let obligation_badge: ValidatedProof = obligation_badge
                 .validate_proof(self.obligation_non_fungible_resource)
                 .expect("Invalid badge");
             /// After this lets get the data on the badge
             let obligation: EscrowObligation = obligation_badge.non_fungible().data();
             let vault: &mut Vault = self.vaults.get_mut(&obligation.amount_to_pay).unwrap();

             let funds_to_deposit: Bucket = match obligation.amount_to_pay {
                 ResourceSpecifier::Fungible { amount, .. } => funds.take(amount),
                 ResourceSpecifier::NonFungible { non_fungible_ids, .. } => funds.take_non_fungibles(&non_fungible_ids),
             };

             vault.put(funds_to_deposit);
             funds
         }

         pub fn withdraw(&mut self, obligation_badge: Proof) -> Bucket {
             assert!(
                 self.is_escrow_fulfilled(),
                 "You can not withdraw your funds unless the  escrow is not concluded",
             );

             let obligation_badge: ValidatedProof = obligation_badge
                 .validate_proof(self.obligation_non_fungible_resource)
                 .expect("invalid badge provider");

             let obligation: EscrowObligation = obligation_badge.non_fungible().data();
             let vault: &mut Vault = self.vaults.get_mut(&obligation.amount_to_get).unwrap();
             vault.take_all()
         }

         pub fn is_escrow_fulfilled(&mut self) -> bool {
             if self.is_escrow_fulfilled {
                 self.is_escrow_fulfilled
             } else {
                 self.is_escrow_fulfilled = self.vaults
                     .iter()
                     .map(|(resource_specifier, vault)| {
                         match resource_specifier {
                             ResourceSpecifier::Fungible {
                                 resource_address,
                                 amount,
                             } => {
                                 vault.resource_address() == *resource_address
                                     && vault.amount() >= *amount
                             }

                             ResourceSpecifier::NonFungible {
                                 resource_address,
                                 non_fungible_ids,
                             } => {
                                 vault.resource_address() == *resource_address
                                     && vault
                                         .non_fungible_ids()
                                         .iter()
                                         .all(|x| non_fungible_ids.contains(x))
                             }
                         }
                     })
                     .all(|x| x);
                 self.is_escrow_fulfilled
             }

         }
     }
 }

 #[derive(Debug, NonFungibleData)]
 pub struct EscrowObligation {
     /// The amount of tokens which this party needs to pay to the other party.
     amount_to_pay: ResourceSpecifier,
     /// The amount of tokens paid by the other party to this party.
     amount_to_get: ResourceSpecifier,
 }

 #[derive(Debug, TypeId, Encode, Decode, Describe, Ord, PartialOrd, Eq, PartialEq, Clone)]
 pub enum ResourceSpecifier {
     /// A variant used to specify the amount of a fungible resource through the [`ResourceAddress`]
     /// of the resource the amount of that resource as a [`Decimal`].
     Fungible {
         resource_address: ResourceAddress,
         amount: Decimal,
     },
     /// A variant used to specify non-fungible of that resource based on the [`ResourceAddress`] of
     /// the resource and a set of the [`NonFungibleId`]s being specified by the enum.
     NonFungible {
         resource_address: ResourceAddress,
         non_fungible_ids: BTreeSet<NonFungibleId>,
     },
 }

 impl ResourceSpecifier {
     pub fn validate(&self) -> Result<(), ()> {
         match self {
             Self::Fungible { amount, .. } => {
                 if *amount <= Decimal::zero() {
                     Err(())
                 } else {
                     Ok(())
                 }
             }
             Self::NonFungible {
                 non_fungible_ids, ..
             } => {
                 if non_fungible_ids.is_empty() {
                     Err(())
                 } else {
                     Ok(())
                 }
             }
         }
     }

     pub fn resource_address(&self) -> ResourceAddress {
         match self {
             Self::Fungible {
                 resource_address, ..
             }
             | Self::NonFungible {
                 resource_address, ..
             } => *resource_address,
         }
     }
}