use scrypto::prelude::*;

// Assuming an average epoch duration of 35 minutes, 15k epochs roughly fit into one year
// This is a very rough estimate, of course
const EPOCHS_PER_YEAR: u64 = 15_000;

// Currently all position info is centralized in the contract, no data stored in the position badge.
#[derive(NonFungibleData)]
struct PositionData {
    is_liquidated: bool
}

#[derive(std::fmt::Debug, scrypto::Encode, scrypto::Decode, scrypto::TypeId, scrypto::Describe, Copy, Clone)]
struct PositionInfo {
    collateral_amount: Decimal,
    loan_amount: Decimal,
    start_epoch: u64,
}

blueprint! {
    // Store all position data in a map in the contract to allow 3rd party liquidators to call liquidate method on undercollateralized positions.
    struct RaiTest {
        pooled_collateral_vault: Vault,
        positions: HashMap<NonFungibleId, PositionInfo>, // TODO - convert to lazymap
        position_resource: ResourceAddress,
        rai_resource: ResourceAddress,
        minter: Vault,
        interest_rate: Decimal,
        positions_counter: u64,
        is_insolvent: bool,
        oracle_address: ComponentAddress
    }

    impl RaiTest {

        // Create new RAI collateralized claim contract. This contract has sole minting authority over new position identifier nfts and the RAI supply.
        pub fn new(oracle: ComponentAddress) -> (ComponentAddress, Bucket) {

            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "RaiTest Admin Badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let minter = ResourceBuilder::new_fungible()
                .metadata("name", "Minter badge")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(dec!("1"));

            let position_resource: ResourceAddress = ResourceBuilder::new_non_fungible()
                .metadata("name", "RAI Position Badge")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();
            let rai_resource: ResourceAddress = ResourceBuilder::new_fungible()
                .metadata("name", "RAI")
                .mintable(rule!(require(minter.resource_address())), LOCKED)
                .burnable(rule!(require(minter.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("update_interest_rate", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            let mut component = Self {
                pooled_collateral_vault: Vault::new(RADIX_TOKEN),
                positions: HashMap::new(),
                position_resource: position_resource,
                rai_resource: rai_resource,
                minter: Vault::with_bucket(minter),
                interest_rate: dec!("0.05"), // TODO - variable loan interest rate. For now, placeholder 5% interest rate.
                positions_counter: 0,
                is_insolvent: false,
                oracle_address: oracle
            }.instantiate();

            component.add_access_check(rules);
            
            (component.globalize(), admin_badge)
        }

        // Callable by user - deposit collateral into vault, and mint new position badge and store position info.
        pub fn open_position(&mut self, new_position_collateral: Bucket) -> Bucket {
            assert!(
                self.is_insolvent == false,
                "Protocol Insolvent - locked from opening/closing positions and minting/burning RAI"
            );
            assert!(
                new_position_collateral.resource_address() == RADIX_TOKEN,
                "New position collateral required to be in XRD"
            );

            let position_id = NonFungibleId::from_u64(self.positions_counter);
            self.positions_counter += 1;

            let position_badge = self.minter.authorize(|| {
                let resource_manager = borrow_resource_manager!(self.position_resource);
                resource_manager.mint_non_fungible(
                    &position_id, PositionData {is_liquidated: false}
                )
            });

            let position_info = PositionInfo {
                collateral_amount: new_position_collateral.amount(),
                loan_amount: dec!(0),
                start_epoch: Runtime::current_epoch(),
            };
            info!("Open Position - New position id {} {:?}", position_id, position_info);

            self.pooled_collateral_vault.put(new_position_collateral);
            self.positions.insert(position_id, position_info);

            position_badge
        }

        // Callable by user - get price of collateral, allow mint of new RAI token up to 150% collateral ratio, update position info.
        pub fn draw(&mut self, position_badge: Proof, requested_rai: Decimal) -> Bucket {
            assert!(
                self.is_insolvent == false,
                "Protocol Insolvent - locked from opening/closing positions and minting/burning RAI"
            );
            let position_id = position_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_resource,
                    dec!(1)
                ))
                .expect("unauthorized access")
                .non_fungible_id();                                

            let required_collateral_xrd_amount = RaiTest::calc_required_collateral_xrd_amount(requested_rai, self.get_xrd_price());
            let position = self.positions.get_mut(&position_id).unwrap();

            // If collateral is available in position, allow mint.
            info!("Draw - Position ID {} {:?}", position_id, position);
            info!("Minimum collateral required to maintain requested RAI loan - {} XRD", required_collateral_xrd_amount);
            assert!(required_collateral_xrd_amount < position.collateral_amount);
            position.loan_amount = requested_rai;
            position.start_epoch = Runtime::current_epoch();

            let minted_rai = self.minter.authorize(|| {
                let rai_manager: &mut ResourceManager = borrow_resource_manager!(self.rai_resource);
                rai_manager.mint(requested_rai)
            });

            info!("Drew {} RAI for position id {}, {:?}", &requested_rai, &position_id, &position);

            minted_rai
        }

        // Callable by user - provide position id and paydown RAI loan
        pub fn paydown(&mut self, position_badge: Proof, rai_payment: Bucket) -> Bucket {
            trace!("paydown!");
            assert!(
                self.is_insolvent == false,
                "Protocol Insolvent - locked from opening/closing positions and minting/burning RAI"
            );
            assert!(
                rai_payment.resource_address() == self.rai_resource,
                "The rai_payment bucket does not contain RAI resource"
            );
            let position_id = position_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_resource,
                    dec!(1)
                ))
                .expect("unauthorized access")
                .non_fungible_id();                                
            trace!("position_id validated!");

            // Calculate loan principal + interest of position, and apply payment.
            let position = self.positions.get_mut(&position_id).unwrap();
            let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);

            info!("Paydown - Position ID {} {:?}, P&I {} RAI", position_id, position, principal_and_interest);

            // If payment amount exceeds loan balance, paydown complete loan principal and interest.
            let payment_amount = if rai_payment.amount() < principal_and_interest {
                rai_payment.amount()
            } else {
                principal_and_interest
            };
            // Update position after loan payment.
            position.loan_amount = principal_and_interest - payment_amount;
            position.start_epoch = Runtime::current_epoch();

            info!("Position paid down - {:?}", position);

            // Burn RAI payment.
            let remaining_rai_payment = self.burn_rai_payment(rai_payment, payment_amount);

            // Return overpayment if any.
            remaining_rai_payment
        }

        // Callable by user - takes position badge. If position maintains no debt, the badge is burned and the contract 
        // will return the deposited collateral and RAI overpayment if any.
        pub fn close_position(&mut self, position_badge: Bucket) -> Bucket {
            assert!(
                position_badge.resource_address() == self.position_resource,
                "The position_badge bucket does not contain a position badge NFT"
            );

            let position_id = &position_badge.non_fungible::<PositionData>().id();
            let position = self.positions.get_mut(&position_id).unwrap();
            info!("Close Position - position id - {} {:?}", position_id, position);

            assert!(
                position.loan_amount == dec!(0),
                "Position loan balance above 0 - please close position with payment"
            );

            let withdrawn_collateral = self.pooled_collateral_vault.take(position.collateral_amount);

            self.positions.remove(&position_id);
            self.minter.authorize(|| {
                let position_manager = borrow_resource_manager!(self.position_resource);
                position_manager.burn(position_badge)
            });

            withdrawn_collateral
        }


        // Callable by user - takes position badge and RAI payment. If RAI payment is enough to close position, the badge is burned and the contract 
        // will return the deposited collateral and RAI overpayment if any.
        pub fn close_position_with_payment(&mut self, position_badge: Bucket, rai_payment: Bucket) -> (Bucket, Bucket) {
            assert!(
                self.is_insolvent == false,
                "Protocol Insolvent - locked from opening/closing positions and minting/burning RAI"
            );
            assert!(
                position_badge.resource_address() == self.position_resource,
                "The position_badge bucket does not contain a position badge NFT"
            );
            assert!(
                rai_payment.resource_address() == self.rai_resource,
                "The rai_payment bucket does not contain RAI resource"
            );

            let position_id = &position_badge.non_fungible::<PositionData>().id();
            let position = self.positions.get_mut(&position_id).unwrap();
            let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);

            info!("Close Position With Payment - position id - {} {:?}", position_id, position);
            info!("Position Principal and Interest required to close - {} RAI", principal_and_interest);

            assert!(rai_payment.amount() >= principal_and_interest);

            info!("Closed position id {} with {} RAI payment", &position_id, rai_payment.amount());

            let withdrawn_collateral = self.pooled_collateral_vault.take(position.collateral_amount);
            
            // Burn RAI payment.
            let remaining_rai_payment = self.burn_rai_payment(rai_payment, principal_and_interest);

            self.positions.remove(&position_id);
            self.minter.authorize(|| {
                let position_manager = borrow_resource_manager!(self.position_resource);
                position_manager.burn(position_badge)
            });

            (withdrawn_collateral, remaining_rai_payment)
        }

        // Next - use resim to test openposition -> draw -> close position
            // In order to close one position, need to have enough RAI to paydown principal + interest (if epochs have advanced)
            // In order to get enough RAI, open 2nd position to mint enough RAI to cover 1st position. Otherwise, normally would need to purchase additional RAI from the market.

        // Callable by user - take position badge proof and additional collateral, adds it to position
        pub fn add_collateral(&mut self, position_badge: Proof, additional_collateral: Bucket) {
            assert!(
                additional_collateral.resource_address() == RADIX_TOKEN,
                "The additional_collateral bucket does not contain XRD"
            );

            let position_id = position_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_resource,
                    dec!(1)
                ))
                .expect("unauthorized access")
                .non_fungible_id();                                

            let position = self.positions.get_mut(&position_id).unwrap();

            info!("Add additional collateral {} XRD", additional_collateral.amount());
            position.collateral_amount += additional_collateral.amount();
            self.pooled_collateral_vault.put(additional_collateral);
            info!("Position ID {} - {:?}", position_id, position);
        }

        // Callable by user - withdraw collateral, allowed withdrawal until minimum collateral ratio
        pub fn partial_withdraw_collateral(&mut self, position_badge: Proof, requested_withdrawal: Decimal) -> Bucket {
            let position_id = position_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.position_resource,
                    dec!(1)
                ))
                .expect("unauthorized access")
                .non_fungible_id();                                

            let xrd_price = self.get_xrd_price();
            let position = self.positions.get_mut(&position_id).unwrap();
            let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);

            let required_collateral_xrd_amount = RaiTest::calc_required_collateral_xrd_amount(principal_and_interest, xrd_price);

            info!("Partial Withdraw Collateral - Position ID {} - {:?}", position_id, position);
            info!("Position Principal and Interest - {} RAI, minimum collateral required to maintain position - {} XRD", principal_and_interest, required_collateral_xrd_amount);

            assert!(position.collateral_amount - requested_withdrawal > required_collateral_xrd_amount);

            position.collateral_amount -= requested_withdrawal;
            let withdrawal = self.pooled_collateral_vault.take(requested_withdrawal);
            info!("Withdrew {} XRD from position, new position info {:?}", requested_withdrawal, position);

            withdrawal
        }

        // Callable by anyone acting as a liquidator - provide undercollateralized position id and minimum RAI P&I payment to foreclose on position collateral
        pub fn liquidate(&mut self, position_id: NonFungibleId, rai_payment: Bucket) -> (Bucket, Bucket) {
            assert!(
                self.is_insolvent == false,
                "Protocol Insolvent - locked from liquidating positions and minting/burning RAI"
            );
            assert!(
                rai_payment.resource_address() == self.rai_resource,
                "The rai_payment bucket does not contain RAI"
            );
            let xrd_price = self.get_xrd_price();
            
            let position = self.positions.get_mut(&position_id).unwrap();

            let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);
            let required_collateral_xrd_amount = RaiTest::calc_required_collateral_xrd_amount(principal_and_interest, xrd_price);

            assert!(principal_and_interest < required_collateral_xrd_amount);
            info!("Position id {} being liquidated, p&i is {} and required collateral xrd is {}, position only contains {} xrd collateral", 
                position_id, principal_and_interest, required_collateral_xrd_amount, position.collateral_amount);

            assert!(principal_and_interest < rai_payment.amount(), 
                "Liquidation payment not enough to pay off debt");
        
            // If the liquidator can pay off the undercollateralized position, 
            // let them payoff the position debt and send them the collateral along with any overpayment
            let liquidated_collateral = self.pooled_collateral_vault.take(position.collateral_amount);

            // Payoff the debt and burn the RAI payment from supply.
            let remaining_rai_payment = self.burn_rai_payment(rai_payment, principal_and_interest);

            // Delete the position.
            self.positions.remove(&position_id);
            // Update the original position badge - either burn it, or lock it into soulbound token to create a position liquidation history.
            // Since recallable access control is not yet implemented, just update the PositionData of the position badge to reflect that the position has been liquidated.
            self.minter.authorize(|| {
                let position_manager = borrow_resource_manager!(self.position_resource);
                position_manager.update_non_fungible_data(&position_id, 
                    PositionData {
                        is_liquidated: true,
                    })
            });

            info!("Liquidate position id {} successful, releasing {} XRD collateral and returning {} RAI overpayment to the liquidator", 
                position_id, liquidated_collateral.amount(), remaining_rai_payment.amount());

            // After each liquidation, check protocol solvency.
            self.check_protocol_solvency();

            (liquidated_collateral, remaining_rai_payment)
        }

        // When protocol is undercollateralized where locked collateral XRD value < RAI supply value, freeze opening and closing positions and only allow redemptions
        pub fn check_protocol_solvency(&mut self) {
            let rai_manager = borrow_resource_manager!(self.rai_resource);
            let total_rai_supply = rai_manager.total_supply();
            let pooled_collateral_value = self.calc_xrd_value(self.pooled_collateral_vault.amount());
            info!("Collateral pool xrd amount: {} XRD price: {} Pool value: {} Total RAI supply: {}", self.pooled_collateral_vault.amount(), self.get_xrd_price(), pooled_collateral_value, total_rai_supply);
            if total_rai_supply > pooled_collateral_value {
                self.is_insolvent = true;
                info!("!! Protocol is insolvent !! Freezing liquidations and new positions, redemptions against collateral pool allowed now");
            } else {
                info!("Protocol solvent");
            }
        }

        // Redemptions will be distributed based on supply of RAI - each % of RAI supply redeemed will allow redemption of equal % of collateral xrd vault.
        // This percentage based withdrawal means that all RAI holders will be able to withdraw the same amount of collateral regardless of when the 
        // redemption is requested, and prevents the rush to the exits causing a bank run. 
        // This also discourages cascading liquidations from flooding the market with supply, crashing the supply of the collateral.
        pub fn redeem(&mut self, rai_to_redeem: Bucket) -> Bucket {
            assert!(
                self.is_insolvent == true,
                "Redemptions only allowed when protocol is insolvent"
            );
            assert!(
                rai_to_redeem.resource_address() == self.rai_resource,
                "The rai_to_redeem bucket does not contain RAI"
            );
            let rai_manager = borrow_resource_manager!(self.rai_resource);
            let total_rai_supply = rai_manager.total_supply();
            info!("Total RAI supply outstanding: {}", total_rai_supply);
            let percentage_of_total = rai_to_redeem.amount() / total_rai_supply;

            let collateral_redemption_amount = percentage_of_total * self.pooled_collateral_vault.amount();

            let redemption_collateral = self.pooled_collateral_vault.take(collateral_redemption_amount);

            // Burn redemption RAI.
            self.minter.authorize(|| {
                let rai_manager = borrow_resource_manager!(self.rai_resource);
                rai_manager.burn(rai_to_redeem);
            });
            
            info!("Redeem - {}% of RAI supply redemption, returning {}% of collateral pool - {} xrd", percentage_of_total*100, percentage_of_total*100, redemption_collateral.amount());

            redemption_collateral
        }

        // Normally, liquidators would be expected to run bots to subscribe to new `open_position` `draw` `paydown` `close_position`
        // events and maintain an off-ledger database for identifying which vaults are undercollateralized, and call liquidate on
        // the corresponding undercollateralized positions. However since subscription events are not yet available in Babylon,
        // provide this convenience function to print the all outstanding positions and identify if any vaults are available for
        // liquidation and allow manual inspection for liquidation.
        pub fn print_all_positions(&self) {
            let xrd_price = self.get_xrd_price();
            info!("xrd price ${}", xrd_price);
            for position_id in self.positions.keys() {
                let position = self.positions.get(position_id).unwrap();
                trace!("a");
                let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);
                trace!("b");
                let required_collateral_amount = RaiTest::calc_required_collateral_xrd_amount(principal_and_interest, self.get_xrd_price());
                trace!("c");
                let required_collateral_value = required_collateral_amount * xrd_price;
                trace!("d");
                let undercollateralized = if position.collateral_amount < required_collateral_amount { true } else { false } ;
                trace!("e");
                let collateral_value = self.calc_xrd_value(position.collateral_amount);
                info!(
                    "position_id {} {:?}, P&I {} RAI, collateral_value ${}, required_collateral_value ${}, required_collateral_amount {} XRD",
                    position_id, position, principal_and_interest, collateral_value, required_collateral_value, required_collateral_amount
                );
                if undercollateralized {
                    info!("!! position id {} undercollateralized!", position_id);
                }
            }
        }

        // Allow the admin badge holder to update the interest rate for all positions.
        // To keep track of accruing interest across varying interest rates, upon each interest rate change,
        // across all positions calculate the accrued principal & interest and store that value and update the start_epoch field.
        // From that point on, it is treated as a newly originated loan at that start_epoch with the new interest rate.
        // In this fashion, historical interest accrued at previous interest rates is included in calculations as the interest rate varies.
        pub fn update_interest_rate(&mut self, new_interest_rate: Decimal) {
            for (position_id, position) in self.positions.iter_mut() {
                let principal_and_interest = RaiTest::calc_principal_and_interest(position.loan_amount, self.interest_rate, position.start_epoch);
                position.loan_amount = principal_and_interest;
                position.start_epoch = Runtime::current_epoch();
                info!("Compounded P&I under previous interest rate and updated position_id {} {:?}", position_id, position)
            }
            self.interest_rate = new_interest_rate;
            info!("Updated interest rate - new interest rate {}", self.interest_rate)
        }

        fn get_xrd_price(&self) -> Decimal {
            let oracle: OraclePlaceholder = self.oracle_address.into();
            oracle.get_price()
        }

        fn calc_xrd_value(&self, xrd_amount: Decimal) -> Decimal {
            xrd_amount * self.get_xrd_price()
        }

        fn calc_required_collateral_xrd_amount(loan_amount: Decimal, xrd_price: Decimal) -> Decimal {
            let required_collateral_value = loan_amount * dec!("1.50");
            required_collateral_value / xrd_price
        }

        fn calc_principal_and_interest(loan_amount: Decimal, interest_rate: Decimal, start_epoch: u64) -> Decimal {
            trace!("calc_principal_and_interest! {loan_amount} {interest_rate} {start_epoch}");
            let interest_rate_per_epoch = interest_rate / EPOCHS_PER_YEAR;
            let loan_factor = dec!(1) + interest_rate_per_epoch;
            trace!("before number_of_epochs");
            let number_of_epochs = (Runtime::current_epoch() - start_epoch).try_into().expect("error converting epochs u64 to i64");
            trace!("loan_factor {loan_factor}, number_of_epochs {number_of_epochs}");
            let loan_factor = loan_factor.powi(number_of_epochs);
            trace!("loan_factor {loan_factor}!");
            let principal_and_interest = loan_amount * loan_factor;

            trace!("{principal_and_interest}");
            principal_and_interest
        }

        // Burn a specified amount of RAI from the supply and return any remaining RAI in the bucket
        fn burn_rai_payment(&self, mut rai_payment: Bucket, burn_amount: Decimal) -> Bucket {
            self.minter.authorize(|| {
                let rai_manager = borrow_resource_manager!(self.rai_resource);
                rai_manager.burn(rai_payment.take(burn_amount))
            });

            rai_payment
        }
    }
}

import! { r#"
{
    "package_address": "package_sim1qy49wq0lyfvq6wn8vhq6fvxfp3rhf5phsnalj0yefnvsy86pq6",
    "blueprint_name": "OraclePlaceholder",
    "abi": {
      "structure": {
        "type": "Struct",
        "name": "OraclePlaceholder",
        "fields": {
          "type": "Named",
          "named": [
            [
              "xrd_price",
              {
                "type": "Custom",
                "type_id": 161,
                "generics": []
              }
            ]
          ]
        }
      },
      "fns": [
        {
          "ident": "new",
          "mutability": null,
          "input": {
            "type": "Struct",
            "name": "OraclePlaceholder_new_Input",
            "fields": {
              "type": "Named",
              "named": []
            }
          },
          "output": {
            "type": "Custom",
            "type_id": 129,
            "generics": []
          },
          "export_name": "OraclePlaceholder_new"
        },
        {
          "ident": "set_price",
          "mutability": "Mutable",
          "input": {
            "type": "Struct",
            "name": "OraclePlaceholder_set_price_Input",
            "fields": {
              "type": "Named",
              "named": [
                [
                  "arg0",
                  {
                    "type": "Custom",
                    "type_id": 161,
                    "generics": []
                  }
                ]
              ]
            }
          },
          "output": {
            "type": "Unit"
          },
          "export_name": "OraclePlaceholder_set_price"
        },
        {
          "ident": "get_price",
          "mutability": "Immutable",
          "input": {
            "type": "Struct",
            "name": "OraclePlaceholder_get_price_Input",
            "fields": {
              "type": "Named",
              "named": []
            }
          },
          "output": {
            "type": "Custom",
            "type_id": 161,
            "generics": []
          },
          "export_name": "OraclePlaceholder_get_price"
        }
      ]
    }
  }
"# 
}