use crate::pool::TickState;
use scrypto::prelude::*;

/**
 * Computes the amount0, amount1 for a given position described by tbe liquidity and range[sqrt_price_low, sqrt_price_high] at a given sqrt_price
 */
pub fn compute_range_amounts_given_liq(
    liq: Decimal,
    sqrt_price: Decimal,
    sqrt_price_low: Decimal,
    sqrt_price_high: Decimal,
) -> (Decimal, Decimal) {
    //the price is bellow the range, the position holds only amount0
    if sqrt_price < sqrt_price_low {
        (
            compute_range_amount0_given_liq(liq, sqrt_price_low, sqrt_price_high),
            Decimal::zero(),
        )
    }
    //the price is in range, the position holds both amount0 and amount1
    else if sqrt_price < sqrt_price_high {
        (
            compute_range_amount0_given_liq(liq, sqrt_price, sqrt_price_high),
            compute_range_amount1_given_liq(liq, sqrt_price_low, sqrt_price),
        )
    }
    //the price is above the range, the position holds only amount1
    else {
        (
            Decimal::zero(),
            compute_range_amount1_given_liq(liq, sqrt_price_low, sqrt_price_high),
        )
    }
}

/**
 * Computes the liquidity for the given amount0, amount1, range[sqrt_price_low, sqrt_price_high] and current sqrt_price
 *
 * Returns the liquidity and the unused amount0 or amount1 (if the current price is out of range)
 */
pub fn compute_range_liq_given_amounts(
    amount0: Decimal,
    amount1: Decimal,
    sqrt_price: Decimal,
    sqrt_price_low: Decimal,
    sqrt_price_high: Decimal,
) -> (Decimal, Decimal, Decimal) {
    //we can use only amount0 until sqrt_price goes up to sqrt_price_low
    if sqrt_price <= sqrt_price_low {
        (
            compute_range_liq_given_amount0(amount0, sqrt_price_low, sqrt_price_high),
            amount0,
            Decimal::zero(),
        )
    }
    // we can use both amount0 and amount1
    else if sqrt_price < sqrt_price_high {
        let liq0 = compute_range_liq_given_amount0(amount0, sqrt_price, sqrt_price_high);
        let liq1 = compute_range_liq_given_amount1(amount1, sqrt_price_low, sqrt_price);
        //choose the lowest to not go overboard with the other required amount
        if liq0 < liq1 {
            (liq0, amount0, compute_range_amount1_given_liq(liq0, sqrt_price_low, sqrt_price))
        } else {
            (liq1, compute_range_amount0_given_liq(liq1, sqrt_price, sqrt_price_high), amount1)
        }
    }
    //we can use only amount1 until sqrt_price goes down to sqrt_price_high
    else {
        (
            compute_range_liq_given_amount1(amount1, sqrt_price_low, sqrt_price_high),
            Decimal::zero(),
            amount1,
        )
    }
}

/**
 * By definition Δ(1/√𝑃)=Δx/L => Δx=L*Δ(1/√𝑃), where Δx = amount0 and Δ(1/√𝑃) = (sqrt_price_high - sqrt_price_low) / sqrt_price_low * sqrt_price_high
 *
 * Returns the amount0 needed to go from sqrt_price_high to sqrt_price_low, given the liquidity
 */
pub fn compute_range_amount0_given_liq(liq: Decimal, sqrt_price_low: Decimal, sqrt_price_high: Decimal) -> Decimal {
    liq * (sqrt_price_high - sqrt_price_low) / (sqrt_price_high * sqrt_price_low)
}

/**
 * By definition Δ𝑦/L=Δ√𝑃 => Δ𝑦=L*Δ√𝑃, where Δy = amount1 and Δ√𝑃 = sqrt_price_high - sqrt_price_low
 *
 * Returns the amount1 needed to go from sqrt_price_low to sqrt_price_high, given the liquidity
 */
pub fn compute_range_amount1_given_liq(liq: Decimal, sqrt_price_low: Decimal, sqrt_price_high: Decimal) -> Decimal {
    liq * (sqrt_price_high - sqrt_price_low)
}

/**
 * By definition Δ(1/√𝑃)=Δx/L => L=Δx/Δ(1/√𝑃), where Δx = amount0 and Δ(1/√𝑃) = (sqrt_price_high - sqrt_price_low) / sqrt_price_low * sqrt_price_high
 *
 * Returns the liquidty that makes a given amount0 to move the price from sqrt_price_high to sqrt_price_low
 */
fn compute_range_liq_given_amount0(amount0: Decimal, sqrt_price_low: Decimal, sqrt_price_high: Decimal) -> Decimal {
    (amount0 * sqrt_price_low * sqrt_price_high) / (sqrt_price_high - sqrt_price_low)
}

/**
 * By definition Δ𝑦/L=Δ√𝑃 => L=Δ𝑦/Δ√𝑃, where Δy = amount1 and Δ√𝑃 = sqrt_price_high - sqrt_price_low
 *
 * Returns the liquidty that makes a given amount1 to move the price from sqrt_price_low to sqrt_price_hugh
 */
fn compute_range_liq_given_amount1(amount1: Decimal, sqrt_price_low: Decimal, sqrt_price_high: Decimal) -> Decimal {
    amount1 / (sqrt_price_high - sqrt_price_low)
}

/**
 * Returns the fees(fee0, fee1) corresponding to a given range[low_tick, high_tick], given the total fees(fee_growth_global0, fee_growth_global1) and the current_tick.
 *
 * By definition the fee corresponding to a range[low_tick, high_tick] for either resource0,1 is:
 *
 * fee = fee_global - fee_bellow_low_tick - fee_above_high_tick, where
 *
 * 1. fee_bellow_low_tick = fee_outside_tick, if current_tick >= tick
 *    fee_bellow_low_tick = fee_global - fee_outside_tick, if current_tick < tick
 *
 * 2. fee_above_high_tick = fee_global - fee_outside_tick, if current_tick >= tick or
 *    fee_above_high_tick = fee_outside_tick, if current_tick < tick
 *
 * 3. fee_outside_tick = fee_global - fee_outside_tick (updated everytime the tick is crossed bellow or above)
 *
 * 4. When the first position containing a tick is created, it's assumed that all fees were generated bellow the tick, so:
 *    fee_outside_tick = 0, if current_tick < tick or
 *    fee_outside_tick = fee_global, if current_tick >= tick
 */
pub fn compute_range_fees(
    current_tick: i32,
    fee_global0: Decimal,
    fee_global1: Decimal,
    low_tick_state: &TickState,
    high_tick_state: &TickState,
) -> (Decimal, Decimal) {
    let fee_bellow_low_tick0 = if current_tick >= low_tick_state.tick {
        low_tick_state.fee_outside0
    } else {
        fee_global0 - low_tick_state.fee_outside0
    };

    let fee_above_high_tick0 = if current_tick >= high_tick_state.tick {
        fee_global0 - high_tick_state.fee_outside0
    } else {
        high_tick_state.fee_outside0
    };

    let fee_bellow_low_tick1 = if current_tick >= low_tick_state.tick {
        low_tick_state.fee_outside1
    } else {
        fee_global1 - low_tick_state.fee_outside1
    };

    let fee_above_high_tick1 = if current_tick >= high_tick_state.tick {
        fee_global1 - high_tick_state.fee_outside1
    } else {
        high_tick_state.fee_outside1
    };

    (
        fee_global0 - fee_bellow_low_tick0 - fee_above_high_tick0,
        fee_global1 - fee_bellow_low_tick1 - fee_above_high_tick1,
    )
}

/**
 * If √P0, L, ΔX (initial sqrt_price, liq, delta_amount0) are given, we want to compute ΔY(delta_amount1) and the new price using them.
 *
 * By definition ΔY=Δ(√𝑃)*L (1) and L=ΔX/Δ(1/√𝑃) => Δ(1/√𝑃)=ΔX/L (2)
 * Also, by definition 1/√P1=1/√P0+Δ(1/√𝑃) => from(2) that 1/√P1=1/√P0+ΔX/L => √P1=1/(1/√P0+ΔX/L) (3)
 * But, by defintion √P1=√P0-Δ(√𝑃) => from (3) that 1/(1/√P0+ΔX/L)=√P0-Δ(√𝑃) => Δ(√𝑃)=√P0-1/(1/√P0+ΔX/L) => from (1) that ΔY=(√P0-1/(1/√P0+ΔX/L))*L
 *
 * Returns the new sqrt_price and the corresponding amount1.
 */
pub fn compute_swap_amount0_price_and_amount1(liq: Decimal, sqrt_price: Decimal, amount0: Decimal) -> (Decimal, Decimal) {
    let delta_sqrt_price_inv = amount0 / liq;
    let new_sqrt_price_inv = (Decimal::one() / sqrt_price) + delta_sqrt_price_inv;
    let new_sqrt_price = Decimal::one() / new_sqrt_price_inv;

    //price going down, maintaining delta positive
    let delta_sqrt_price = sqrt_price - new_sqrt_price;
    let amount1 = delta_sqrt_price * liq;

    (new_sqrt_price, amount1)
}

/**
 * If √P0, L, ΔY (initial sqrt_price, liq, delta_amount1) are given, we want to compute ΔX(delta_amount0) and the new price using them.
 *
 * By definition L=ΔX/Δ(1/√𝑃) and ΔY=Δ(√𝑃)*L, then ΔX=L*Δ(1/√𝑃) (1) and Δ(√𝑃)=ΔY/L (2)
 * Also, by definition √P1=√P0+Δ(√P) => from (2) that √P1=√P0+ΔY/L (3)
 * But, by definition Δ(1/√𝑃)=1/√P0-1/√P1 => from (3) that Δ(1/√𝑃)=1/√P0-1/(√P0+ΔY/L) => from (1) that ΔX=L/(1/√P0-1/(√P0+ΔY/L))
 *
 * Returns the new sqrt_price and the corresponding amount0.
 */
pub fn compute_swap_amount1_price_and_amount0(liq: Decimal, sqrt_price: Decimal, amount1: Decimal) -> (Decimal, Decimal) {
    let delta_sqrt_price = amount1 / liq;
    let new_sqrt_price = sqrt_price + delta_sqrt_price;

    //price going up, maintaining delta positive
    let delta_sqrt_price_inv = Decimal::one() / sqrt_price - Decimal::one() / new_sqrt_price;
    let amount0 = delta_sqrt_price_inv * liq;

    (new_sqrt_price, amount0)
}

/**
 * Computes the fees for a position
 */
pub fn compute_pos_fees(
    liq: Decimal,
    pos_range_fee0: Decimal,
    pos_range_fee1: Decimal,
    range_fee0: Decimal,
    range_fee1: Decimal,
) -> (Decimal, Decimal) {
    ((range_fee0 - pos_range_fee0) * liq, (range_fee1 - pos_range_fee1) * liq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tick_math;

    #[test]
    fn range_liq_given_amount0() {
        let amount0 = 1000_i32.into();
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        assert_eq!(
            dec!("3215.402030677817590191"),
            compute_range_liq_given_amount0(amount0, sqrt_price_low, sqrt_price_high)
        );
    }

    #[test]
    fn range_liq_given_amount1() {
        let amount1 = 1001_i32.into();
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        assert_eq!(
            dec!("2216.258812884945669253"),
            compute_range_liq_given_amount1(amount1, sqrt_price_low, sqrt_price_high)
        );
    }

    #[test]
    fn range_amount0_given_liq() {
        let liq = dec!("5005.34");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        assert_eq!(
            dec!("1556.676257663760152155"),
            compute_range_amount0_given_liq(liq, sqrt_price_low, sqrt_price_high)
        );
    }

    #[test]
    fn range_amount1_given_liq() {
        let liq = dec!("6006.12");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        assert_eq!(
            dec!("2712.736475111362401894"),
            compute_range_amount1_given_liq(liq, sqrt_price_low, sqrt_price_high)
        );
    }

    #[test]
    fn range_liq_given_amounts_and_price_lower_than_range() {
        let amount0 = 1000_i32.into();
        let amount1 = 1001_i32.into();
        let sqrt_price = dec!("0.5");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (liq, req_amount0, req_amount1) =
            compute_range_liq_given_amounts(amount0, amount1, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, req_amount0, req_amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_liq_given_amounts_and_price_higher_than_range() {
        let amount0 = 1000_i32.into();
        let amount1 = 1001_i32.into();
        let sqrt_price = dec!("1.5");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (liq, req_amount0, req_amount1) =
            compute_range_liq_given_amounts(amount0, amount1, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, req_amount0, req_amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_liq_given_amounts_and_price_is_in_range_and_liq0_greater_than_liq1() {
        let amount0 = 1000_i32.into();
        let amount1 = 1001_i32.into();
        let sqrt_price = dec!("1.25");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (liq, req_amount0, req_amount1) =
            compute_range_liq_given_amounts(amount0, amount1, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, req_amount0, req_amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_liq_given_amounts_and_price_is_in_range_and_liq1_greater_than_liq0() {
        let amount0 = 1000_i32.into();
        let amount1 = 3001_i32.into();
        let sqrt_price = dec!("1.25");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (liq, req_amount0, req_amount1) =
            compute_range_liq_given_amounts(amount0, amount1, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, req_amount0, req_amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_amounts_given_liq_and_price_lower_than_range() {
        let liq = dec!("5005.34");
        let sqrt_price = dec!("0.1453");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (amount0, amount1) = compute_range_amounts_given_liq(liq, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, amount0, amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_amounts_given_liq_and_price_greater_than_range() {
        let liq = dec!("6006.12");
        let sqrt_price = dec!("1.5");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (amount0, amount1) = compute_range_amounts_given_liq(liq, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, amount0, amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_amounts_given_liq_and_price_is_in_range() {
        let liq = dec!("6006.12");
        let sqrt_price = dec!("1.25");
        let sqrt_price_high = dec!("1.451912069310634604"); //tick 7458
        let sqrt_price_low = dec!("1.000250018750312399"); //tick 5
        let (amount0, amount1) = compute_range_amounts_given_liq(liq, sqrt_price, sqrt_price_low, sqrt_price_high);
        assert_real_reserves_invariant(liq, amount0, amount1, sqrt_price_low, sqrt_price_high);
    }

    #[test]
    fn range_fees_given_current_tick_lower_than_range() {
        let low_tick = 5;
        let mut low_tick_state = TickState::new(low_tick);
        low_tick_state.fee_outside0 = dec!("25.24");
        low_tick_state.fee_outside1 = dec!("26.25");
        let high_tick = 7458;
        let mut high_tick_state = TickState::new(high_tick);
        high_tick_state.fee_outside0 = dec!("15.14");
        high_tick_state.fee_outside1 = dec!("16.15");

        let fee_growth_global0 = dec!("70");
        let fee_growth_global1 = dec!("70");

        let current_tick = 3;

        assert_eq!(
            (dec!("10.1"), dec!("10.1")),
            compute_range_fees(
                current_tick,
                fee_growth_global0,
                fee_growth_global1,
                &low_tick_state,
                &high_tick_state,
            )
        );
    }

    #[test]
    fn range_fees_given_current_tick_higher_than_range() {
        let low_tick = 5;
        let mut low_tick_state = TickState::new(low_tick);
        low_tick_state.fee_outside0 = dec!("25.24");
        low_tick_state.fee_outside1 = dec!("26.25");
        let high_tick = 7458;
        let mut high_tick_state = TickState::new(high_tick);
        high_tick_state.fee_outside0 = dec!("35.24");
        high_tick_state.fee_outside1 = dec!("36.25");

        let fee_growth_global0 = dec!("70");
        let fee_growth_global1 = dec!("70");

        let current_tick = 8000;

        assert_eq!(
            (dec!("10"), dec!("10")),
            compute_range_fees(
                current_tick,
                fee_growth_global0,
                fee_growth_global1,
                &low_tick_state,
                &high_tick_state,
            )
        );
    }

    #[test]
    fn range_fees_given_current_tick_in_range() {
        let low_tick = 5;
        let mut low_tick_state = TickState::new(low_tick);
        low_tick_state.fee_outside0 = dec!("25.24");
        low_tick_state.fee_outside1 = dec!("26.25");
        let high_tick = 7458;
        let mut high_tick_state = TickState::new(high_tick);
        high_tick_state.fee_outside0 = dec!("35.24");
        high_tick_state.fee_outside1 = dec!("36.25");

        let fee_growth_global0 = dec!("70");
        let fee_growth_global1 = dec!("70");

        let current_tick = 1000;

        assert_eq!(
            (dec!("9.52"), dec!("7.5")),
            compute_range_fees(
                current_tick,
                fee_growth_global0,
                fee_growth_global1,
                &low_tick_state,
                &high_tick_state,
            )
        );
    }

    #[test]
    fn swap_amount0_price_and_amount1() {
        let live_liq = dec!("205051.662681070198680358");
        let sqrt_price = Decimal::one();
        let amount0 = dec!("9999.999999999999969789");
        let amount1 = dec!("10000");
        let sqrt_price_low = tick_math::sqrt_price_at_tick(-1000);
        let sqrt_price_high = tick_math::sqrt_price_at_tick(1000);

        let amount0_to_swap = dec!("1000");
        

        let (liq, a0, a1) = compute_range_liq_given_amounts(dec!("10000"), dec!("10000"), sqrt_price, sqrt_price_low, sqrt_price_high);

        println!("liq: {:?}, a0={:?}, a1={:?}", liq, a0, a1);

        let (_new_sqrt_price, delta_amount1) = compute_swap_amount0_price_and_amount1(live_liq, sqrt_price, amount0_to_swap);

        assert_real_reserves_invariant(
            live_liq,
            amount0 + amount0_to_swap,
            amount1 - delta_amount1,
            sqrt_price_low,
            sqrt_price_high,
        );
    }

    #[test]
    fn swap_amount1_price_and_amount0() {
        let live_liq = dec!("205051.662681070198680358");
        let sqrt_price = Decimal::one();
        let amount0 = dec!("10000");
        let amount1 = dec!("9999.999999999999969789");
        let sqrt_price_low = tick_math::sqrt_price_at_tick(-1000);
        let sqrt_price_high = tick_math::sqrt_price_at_tick(1000);

        let amount1_to_swap = dec!("1000");

        let (_new_sqrt_price, delta_amount0) = compute_swap_amount1_price_and_amount0(live_liq, sqrt_price, amount1_to_swap);

        assert_real_reserves_invariant(
            live_liq,
            amount0 - delta_amount0,
            amount1 + amount1_to_swap,
            sqrt_price_low,
            sqrt_price_high,
        );
    }

    pub fn assert_real_reserves_invariant(
        liq: Decimal,
        amount0: Decimal,
        amount1: Decimal,
        sqrt_price_low: Decimal,
        sqrt_price_high: Decimal,
    ) {
        let left = (amount0 + liq / sqrt_price_high) * (amount1 + liq * sqrt_price_low);
        let right = liq * liq;
        let diff = left - right;
        let epsilon = dec!("0.0000001");

        println!("Assessing real reserves invariant for input: ");
        println!("liq: {:?}", liq);
        println!("amount0: {:?}", amount0);
        println!("amount1: {:?}", amount1);
        println!("sqrt_price_low: {:?}", sqrt_price_low);
        println!("sqrt_price_high: {:?}", sqrt_price_high);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
        println!("diff: {:?}", diff);
        println!("epsilon: {:?}", epsilon);

        assert!(-epsilon < diff);
        assert!(diff < epsilon);
    }
}
