use crate::utils::run_command;
use lazy_static::lazy_static;
use regex::Regex;
use scrypto::prelude::Decimal;
use std::collections::HashMap;
use std::process::Command;

#[derive(Clone)]
pub struct StepState {
    stable: Decimal,
    other: Decimal,
    rate: Decimal,
    stable_fees_per_liq: Decimal,
    other_fees_per_liq: Decimal,
    stable_fees: Decimal,
    other_fees: Decimal,
}

impl StepState {
    pub fn from(
        stable: Decimal,
        other: Decimal,
        rate: Decimal,
        stable_fees_per_liq: Decimal,
        other_fees_per_liq: Decimal,
        stable_fees: Decimal,
        other_fees: Decimal,
    ) -> Self {
        Self {
            stable,
            other,
            rate,
            stable_fees_per_liq,
            other_fees_per_liq,
            stable_fees,
            other_fees,
        }
    }

    pub fn from_output(str_output: &str) -> HashMap<u16, Self> {
        let mut steps = HashMap::new();

        lazy_static! {
            static ref STEP_STATE_RE: Regex = Regex::new(r#"Tuple\((\w*)u16, Array<Decimal>\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\)\)"#).unwrap();
        }

        for step_cap in STEP_STATE_RE.captures_iter(str_output) {
            let step = String::from(&step_cap[1]).parse::<u16>().unwrap();
            let step_state = StepState {
                stable: Decimal::from(&step_cap[2]),
                other: Decimal::from(&step_cap[3]),
                rate: Decimal::from(&step_cap[4]),
                stable_fees_per_liq: Decimal::from(&step_cap[5]),
                other_fees_per_liq: Decimal::from(&step_cap[6]),
                stable_fees: Decimal::from(&step_cap[7]),
                other_fees: Decimal::from(&step_cap[8]),
            };

            steps.insert(step, step_state);
        }

        steps
    }

    fn assert_step_state(&self, other_step: &StepState) {
        assert_eq!(self.rate, other_step.rate);
        assert_eq!(self.stable, other_step.stable);
        assert_eq!(self.other, other_step.other);
        assert_eq!(self.stable_fees_per_liq, other_step.stable_fees_per_liq);
        assert_eq!(self.other_fees_per_liq, other_step.other_fees_per_liq);
        assert_eq!(self.stable_fees, other_step.stable_fees);
        assert_eq!(self.other_fees, other_step.other_fees);
    }

    pub fn assert_step_states(
        step_states_1: &HashMap<u16, StepState>,
        step_states_2: &HashMap<u16, StepState>,
    ) {
        // Checks that both maps have the same amount of keys and that the keys match
        assert!(
            step_states_1.len() == step_states_2.len()
                && step_states_1.keys().all(|k| step_states_2.contains_key(k))
        );

        for (key, value) in step_states_1 {
            let state_2 = step_states_2.get(key).unwrap();
            value.assert_step_state(state_2);
        }
    }
}

pub struct PoolState {
    pub router_address: String,
    pub other: String,
    pub rate_step: Decimal,
    pub current_step: u16,
    pub min_rate: Decimal,
    pub steps: HashMap<u16, StepState>,
    pub stable_protocol: Decimal,
    pub other_protocol: Decimal,
}

impl PoolState {
    pub fn from(router_address: String, other: String) -> Self {
        PoolState {
            router_address,
            other,
            rate_step: Decimal::ZERO,
            current_step: 0,
            min_rate: Decimal::ZERO,
            steps: HashMap::new(),
            stable_protocol: Decimal::ZERO,
            other_protocol: Decimal::ZERO,
        }
    }

    pub fn update(&mut self) {
        let output = run_command(
            Command::new("resim")
                .arg("call-method")
                .arg(&self.router_address)
                .arg("get_pool_state")
                .arg(&self.other),
        );

        lazy_static! {
            static ref STATE_MATCH_RE: Regex = Regex::new(r#"├─ Tuple\(Decimal\("([\d.]*)"\), (\w*)u16, Decimal\("([\d.]*)"\), Tuple\(Decimal\("([\d.]*)"\), Decimal\("([\d.]*)"\)\), Array<Tuple>\((.*)\)"#).unwrap();
        }
        let capture = &STATE_MATCH_RE.captures(&output).unwrap();
        self.rate_step = Decimal::from(&capture[1]);
        self.current_step = String::from(&capture[2]).parse::<u16>().unwrap();
        self.min_rate = Decimal::from(&capture[3]);
        self.stable_protocol = Decimal::from(&capture[4]);
        self.other_protocol = Decimal::from(&capture[5]);
        self.steps = StepState::from_output(&capture[6]);
    }

    pub fn assert_state_is(
        &self,
        rate_step: Decimal,
        current_step: u16,
        min_rate: Decimal,
        steps: HashMap<u16, StepState>,
        stable_protocol: Decimal,
        other_protocol: Decimal,
    ) {
        assert_eq!(self.rate_step, rate_step);
        assert_eq!(self.min_rate, min_rate);
        StepState::assert_step_states(&self.steps, &steps);
        assert_eq!(self.current_step, current_step);
        assert_eq!(self.stable_protocol, stable_protocol);
        assert_eq!(self.other_protocol, other_protocol);
    }
}
