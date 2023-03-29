use crate::dex::pool_state::PoolState;
use crate::stablecoin::issuer_state::{IssuerState, LenderState};
use crate::utils::run_command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::Decimal;
use std::collections::HashMap;
use std::process::Command;

pub struct DaoState {
    dao_address: String,
    pub router_address: String,
    pub issuer_address: String,
    pub issuer_state: IssuerState,
    pub pool_states: HashMap<String, PoolState>,
    pub voter_card_id: u64,
    pub proposals: HashMap<u64, String>,
    pub proposal_id: u64,
    pub total_voting_power: Decimal,
    pub vote_period: i64,
    pub vote_validity_threshold: Decimal,
    pub reserves: HashMap<String, Decimal>,
}

impl DaoState {
    pub fn new(component_address: String, initial_token: String) -> Self {
        Self::from_prompt(component_address, initial_token)
    }

    pub fn update(&mut self) {
        let new_states = Self::from_prompt(self.dao_address.clone(), String::new());

        self.voter_card_id = new_states.voter_card_id;
        self.proposal_id = new_states.proposal_id;
        self.total_voting_power = new_states.total_voting_power;
        self.vote_period = new_states.vote_period;
        self.vote_validity_threshold = new_states.vote_validity_threshold;
        self.proposals = new_states.proposals;

        self.update_issuer_state();
        self.update_pool_states();
        self.update_reserves();
    }

    pub fn assert_variables_are(
        &self,
        voter_card_id: u64,
        proposal_id: u64,
        total_voting_power: Decimal,
        vote_period: i64,
        vote_validity_threshold: Decimal,
    ) {
        assert_eq!(self.voter_card_id, voter_card_id);
        assert_eq!(self.proposal_id, proposal_id);
        assert_eq!(self.total_voting_power, total_voting_power);
        assert_eq!(self.vote_period, vote_period);
        assert_eq!(self.vote_validity_threshold, vote_validity_threshold);
    }

    pub fn assert_pool_states(&self, pool_states: &HashMap<String, PoolState>) {
        let current_states = &self.pool_states;

        assert!(
            current_states.len() == pool_states.len()
                && current_states.keys().all(|k| pool_states.contains_key(k))
        );

        for (token, pool_state) in current_states {
            let other_state = pool_states.get(token).unwrap().clone();

            pool_state.assert_state_is(
                other_state.rate_step,
                other_state.current_step,
                other_state.min_rate,
                other_state.steps.clone(),
                other_state.stable_protocol,
                other_state.other_protocol,
            )
        }
    }

    pub fn assert_issuer_state(
        &self,
        reserves: &HashMap<String, Decimal>,
        lenders: &HashMap<String, LenderState>,
        loan_id: u64,
        flash_mint_id: u64,
    ) {
        self.issuer_state
            .assert_state_is(reserves, lenders, loan_id, flash_mint_id);
    }

    pub fn assert_reserves_state(&self, reserves: &HashMap<String, Decimal>) {
        assert!(
            reserves.len() == self.reserves.len()
                && reserves.keys().all(|k| self.reserves.contains_key(k))
        );

        for (key, value) in reserves {
            let amount = self.reserves.get(key).unwrap();
            assert_eq!(*value, *amount);
        }
    }

    pub fn assert_proposals_state(&self, proposals: &HashMap<u64, String>) {
        assert!(
            proposals.len() == self.proposals.len()
                && proposals.keys().all(|k| self.proposals.contains_key(k))
        );
    }

    pub fn get_proposal(&self, id: u64) -> String {
        self.proposals.get(&id).unwrap().clone()
    }

    fn from_prompt(component_address: String, initial_token: String) -> Self {
        let output = run_command(
            Command::new("resim")
                .arg("show")
                .arg(component_address.clone()),
        );

        lazy_static! {
            static ref DAO_STATE_RE: Regex = Regex::new(r#"ComponentAddress\("(\w*)"\), ComponentAddress\("(\w*)"\), ResourceAddress\("(\w*)"\), Own\("(.*)"\), ResourceAddress\("(\w*)"\), ResourceAddress\("(\w*)"\), (\w*)u64, ResourceAddress\("(\w*)"\), Own\("(.*)"\), Map<U64, ComponentAddress>\((.*)\), (\w*)u64, Own\("(.*)"\), Decimal\("([\d.]*)"\), (\w*)i64, Decimal\("([\d.]*)"\)"#).unwrap();
        }

        let dao_cap = &DAO_STATE_RE.captures(&output).unwrap();

        let dex_comp = String::from(&dao_cap[1]);
        let issuer_comp = String::from(&dao_cap[2]);
        let voter_card_id = String::from(&dao_cap[7]).parse::<u64>().unwrap();
        let proposals_list = &dao_cap[10];
        let proposal_id = String::from(&dao_cap[11]).parse::<u64>().unwrap();
        let total_voting_power = Decimal::from(&dao_cap[13]);
        let vote_period = String::from(&dao_cap[14]).parse::<i64>().unwrap();
        let vote_validity_threshold = Decimal::from(&dao_cap[15]);

        let issuer_state = IssuerState::from(issuer_comp.clone());
        let pool_state = PoolState::from(dex_comp.clone(), initial_token.clone());
        let mut pool_states = HashMap::new();
        pool_states.insert(initial_token, pool_state);

        let proposals = Self::proposals_from(proposals_list);

        Self {
            dao_address: component_address,
            router_address: dex_comp,
            issuer_address: issuer_comp,
            issuer_state,
            pool_states,
            voter_card_id,
            proposals,
            proposal_id,
            total_voting_power,
            vote_period,
            vote_validity_threshold,
            reserves: HashMap::new(),
        }
    }

    fn update_pool_states(&mut self) {
        for (_, pool_state) in &mut self.pool_states {
            pool_state.update();
        }
    }

    fn proposals_from(proposals_list: &str) -> HashMap<u64, String> {
        lazy_static! {
            static ref PROPOSAL_RE: Regex =
                Regex::new(r#"(\w*)u64, ComponentAddress\("(\w*)"\)"#).unwrap();
        }

        let mut proposals = HashMap::new();

        for proposal in PROPOSAL_RE.captures_iter(proposals_list) {
            let id = String::from(&proposal[1]).parse::<u64>().unwrap();
            let comp_address = String::from(&proposal[2]);

            proposals.insert(id, comp_address);
        }
        proposals
    }

    fn update_issuer_state(&mut self) {
        self.issuer_state.update();
    }

    fn update_reserves(&mut self) {
        let output = run_command(
            Command::new("resim")
                .arg("show")
                .arg(self.dao_address.clone()),
        );

        lazy_static! {
            static ref RESOURCE_RE: Regex =
                Regex::new(r#"\{ amount: ([\d.]*), resource address: (\w*)"#).unwrap();
        }

        let mut reserves = HashMap::new();

        for resource_capture in RESOURCE_RE.captures_iter(&output) {
            let amount = Decimal::from(&resource_capture[1]);
            let resource = String::from(&resource_capture[2]);

            if amount != Decimal::ONE && !amount.is_zero() {
                reserves.insert(resource, amount);
            }
        }

        self.reserves = reserves;
    }
}
