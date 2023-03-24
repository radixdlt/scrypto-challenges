use crate::utils::run_command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::Decimal;
use std::collections::HashMap;
use std::process::Command;

pub struct LenderState {
    collateral_amount: Decimal,
    loan_to_value: Decimal,
    interest_rate: Decimal,
    liquidation_threshold: Decimal,
    protocol_liquidation_share: Decimal,
}

impl LenderState {
    pub fn from(
        collateral_amount: Decimal,
        loan_to_value: Decimal,
        interest_rate: Decimal,
        liquidation_threshold: Decimal,
        protocol_liquidation_share: Decimal,
    ) -> Self {
        Self {
            collateral_amount,
            loan_to_value,
            interest_rate,
            liquidation_threshold,
            protocol_liquidation_share,
        }
    }

    pub fn from_output(str_output: &str) -> LenderState {
        lazy_static! {
            static ref LENDER_RE: Regex = Regex::new(r#"Array<Decimal>\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\)"#).unwrap();
        }

        let lender_cap = LENDER_RE.captures(str_output).unwrap();

        Self {
            collateral_amount: Decimal::from(&lender_cap[1]),
            loan_to_value: Decimal::from(&lender_cap[2]),
            interest_rate: Decimal::from(&lender_cap[3]),
            liquidation_threshold: Decimal::from(&lender_cap[4]),
            protocol_liquidation_share: Decimal::from(&lender_cap[5]),
        }
    }

    fn assert_lenders_state(
        lenders_1: &HashMap<String, LenderState>,
        lenders_2: &HashMap<String, LenderState>,
    ) {
        assert!(
            lenders_1.len() == lenders_2.len()
                && lenders_1.keys().all(|k| lenders_2.contains_key(k))
        );

        for (key, value) in lenders_1 {
            let state = lenders_2.get(key).unwrap();

            assert_eq!(value.collateral_amount, state.collateral_amount);
            assert_eq!(value.loan_to_value, state.loan_to_value);
            assert_eq!(value.interest_rate, state.interest_rate);
            assert_eq!(value.liquidation_threshold, state.liquidation_threshold);
            assert_eq!(
                value.protocol_liquidation_share,
                state.protocol_liquidation_share
            );
        }
    }
}

pub struct IssuerState {
    component_address: String,
    reserves: HashMap<String, Decimal>,
    lenders: HashMap<String, LenderState>,
    loan_id: u64,
    flash_mint_id: u64,
}

impl IssuerState {
    pub fn from(component_address: String) -> Self {
        Self {
            component_address,
            reserves: HashMap::new(),
            lenders: HashMap::new(),
            loan_id: 0,
            flash_mint_id: 0,
        }
    }

    pub fn update(&mut self) {
        let output = run_command(
            Command::new("resim")
                .arg("show")
                .arg(&self.component_address),
        );

        self.update_reserves(&output);

        lazy_static! {
            static ref ISSUER_RE: Regex = Regex::new(r#"Tuple\(Map<ResourceAddress, Own>\((.*)\), Map<ResourceAddress, Tuple>\((.*)\), ResourceAddress\("(\w*)"\), ResourceAddress\("(\w*)"\), (\w*)u64, ResourceAddress\("(\w*)"\), (\w*)u64,"#).unwrap();
        }

        let issuer_cap = &ISSUER_RE.captures(&output).unwrap();

        let lenders_map = &issuer_cap[2];
        let loan_id = String::from(&issuer_cap[5]).parse::<u64>().unwrap();
        let flash_mint_id = String::from(&issuer_cap[7]).parse::<u64>().unwrap();

        lazy_static! {
            static ref LENDER_ADDRESS_RE: Regex =
                Regex::new(r#"ResourceAddress\("(\w*)"\)"#).unwrap();
        }

        let mut new_lenders = HashMap::new();

        for lender_address in LENDER_ADDRESS_RE.captures_iter(lenders_map) {
            let token = String::from(&lender_address[1]);
            let new_output = run_command(
                Command::new("resim")
                    .arg("call-method")
                    .arg(&self.component_address)
                    .arg("get_lender_state")
                    .arg(&token),
            );
            let lender_state = LenderState::from_output(&new_output);

            new_lenders.insert(token, lender_state);
        }

        self.lenders = new_lenders;
        self.loan_id = loan_id;
        self.flash_mint_id = flash_mint_id;
    }

    pub fn assert_state_is(
        &self,
        reserves: &HashMap<String, Decimal>,
        lenders: &HashMap<String, LenderState>,
        loan_id: u64,
        flash_mint_id: u64,
    ) {
        self.assert_reserves_state(reserves);
        LenderState::assert_lenders_state(&self.lenders, lenders);
        assert_eq!(self.loan_id, loan_id);
        assert_eq!(self.flash_mint_id, flash_mint_id);
    }

    fn assert_reserves_state(&self, reserves: &HashMap<String, Decimal>) {
        assert!(
            reserves.len() == self.reserves.len()
                && reserves.keys().all(|k| self.reserves.contains_key(k))
        );

        for (key, value) in reserves {
            let amount = self.reserves.get(key).unwrap();
            assert_eq!(*value, *amount);
        }
    }

    fn update_reserves(&mut self, output: &str) {
        lazy_static! {
            static ref RESOURCE_RE: Regex =
                Regex::new(r#"\{ amount: ([\d.]*), resource address: (\w*)"#).unwrap();
        }

        let mut reserves = HashMap::new();

        for resource_capture in RESOURCE_RE.captures_iter(output) {
            let amount = Decimal::from(&resource_capture[1]);
            let resource = String::from(&resource_capture[2]);

            if amount != Decimal::ONE {
                reserves.insert(resource, amount);
            }
        }

        self.reserves = reserves;
    }
}
