use scrypto::prelude::*;
use scrypto_math::*;

// 365 days in sconds
const PERIOD_SIZE: Decimal = dec!(31536000);

    /// Calculates the exchange rate based on the proportion of the trade, 
    /// rate scalar, and rate anchor.
    pub fn calc_exchange_rate(
        proportion: Decimal,
        rate_anchor: PreciseDecimal,
        rate_scalar: Decimal,
    ) -> PreciseDecimal {
        
        let ln_proportion = log_proportion(proportion);

        let exchange_rate = ln_proportion
            .checked_div(rate_scalar)
            .and_then(|result| result.checked_add(rate_anchor)
        )
        .unwrap();

        assert!(
            exchange_rate > PreciseDecimal::ONE,
            "Exchange rate must be greater than 1."
        );

        return exchange_rate
    }

    /// Calculates the size of the trade in relation
    /// to pool size in terms of PT sent or receiving.
    pub fn calc_proportion(
        net_pt_amount: Decimal,
        total_pt: Decimal,
        total_asset: Decimal,
    ) -> Decimal {
            
        let numerator = total_pt.checked_sub(net_pt_amount).unwrap();

        let proportion = 
            numerator
            .checked_div(total_pt.checked_add(total_asset).unwrap())
            .unwrap();

        return proportion
    }

    /// Natural logarithm of the proportion to make computation 
    /// easier apparently.
    pub fn log_proportion(
        proportion: Decimal
    ) -> PreciseDecimal {

        assert_ne!(proportion, Decimal::ONE);

        let logit_p: PreciseDecimal = 
            proportion
            .checked_div(
                PreciseDecimal::ONE
                .checked_sub(proportion)
                .unwrap()
            )
            .unwrap();

        return logit_p.ln().unwrap()
    }

    /// Calculates the scalar rate as a function of time to maturity.
    /// The scalar rate determines the steepness of the curve. A higher 
    /// scalar rate flattens the curve (less slippage) while a lower scalar 
    /// rate steepens the curve (more slippage). It is based is based on an 
    /// initial immutable scalar root value. As the market matures, the scalar 
    /// rate increases, which ultimately flattens the curve over time. It is
    /// important that the curve flattens over time as it narrows... 
    pub fn calc_rate_scalar(
        scalar_root: Decimal,
        time_to_expiry: i64
    ) -> Decimal {

        let rate_scalar: Decimal = scalar_root
            .checked_mul(PERIOD_SIZE)
            .and_then(|result| result.checked_div(time_to_expiry)
        )
        .unwrap();

        // Check if rate scalar is less then 0
        assert!(rate_scalar >= Decimal::ZERO);

        return rate_scalar
    }

    /// Calculates the rate anchor
    /// The rate anchor determines where the curve starts and where exchange rates
    /// are initially anchored (and ultimately the implied rate of the market).
    /// E.g: A rate anchor of 1.05 means that the exchange rate will be around ~1.05
    /// pending other factors such as the rate scalar, size of the trade, and fees.
    pub fn calc_rate_anchor(
        last_ln_implied_rate: PreciseDecimal,
        proportion: Decimal,
        time_to_expiry: i64, 
        rate_scalar: Decimal
    ) -> PreciseDecimal {

        // Calculate the last exchange rate from last implied rate.
        let last_exchange_rate = calc_exchange_rate_from_implied_rate(
            last_ln_implied_rate, 
            time_to_expiry
        );

        // Exchange rate always needs to be greater than one.
        assert!(
            last_exchange_rate > PreciseDecimal::ONE,
            "Exchange rate must be greater than 1. Exchange rate: {:?}",
            last_exchange_rate
        );

        let ln_proportion = log_proportion(proportion);

        let new_exchange_rate: PreciseDecimal = 
            ln_proportion
            .checked_div(rate_scalar)
            .unwrap();

        // The rate anchor = last implied rate (last_exchange_rate) - new exchange rate 
        let rate_anchor: PreciseDecimal = 
            last_exchange_rate
            .checked_sub(new_exchange_rate)
            .unwrap();

        return rate_anchor
    }

    /// Calculates and applies fees based on the direction of the trade.
    /// Since fees are a function of time to maturity, the fees will decrease
    /// as the market matures and contributes to flattening the curve over time.
    pub fn calc_fee(
        fee_rate: PreciseDecimal,
        time_to_expiry: i64,
        net_pt_amount: Decimal,
        exchange_rate: PreciseDecimal,
        pre_fee_amount: PreciseDecimal
    ) -> PreciseDecimal {

        // In this case, the fee rate is the implied rate.
        let fee_rate = calc_exchange_rate_from_implied_rate(
            fee_rate, 
            time_to_expiry
        );

        let fee_amount;

        // Multiply the trade if the direction of the trade is from LSU ---> PT
        // Divide the fee if the direciton of the trade is from PT ---> LSU
        if net_pt_amount > Decimal::ZERO {
            let post_fee_exchange_rate = exchange_rate.checked_div(fee_rate).unwrap();

            assert!(
                post_fee_exchange_rate > PreciseDecimal::ONE,
                "Can't be less than one. Exchange rate: {:?}",
                post_fee_exchange_rate
            );

            // pre_fee_amount is negative but because fee_rate is subtracted by 1, 
            // fee_rate is also a negative. Multiplying together makes the result positive.
            fee_amount = pre_fee_amount
                .checked_mul(PreciseDecimal::ONE.checked_sub(fee_rate).unwrap())
                .unwrap();
        } else {

            fee_amount = pre_fee_amount
                .checked_mul(PreciseDecimal::ONE.checked_sub(fee_rate).unwrap())
                .and_then(|result: PreciseDecimal| result.checked_div(fee_rate))
                .and_then(|result: PreciseDecimal| result.checked_neg())
                .unwrap();
        };

        return fee_amount
    }

    /// Converts implied rate to an exchange rate given a time to expiry.
    pub fn calc_exchange_rate_from_implied_rate(
        ln_implied_rate: PreciseDecimal, 
        time_to_expiry: i64
    ) -> PreciseDecimal {

        let rt: PreciseDecimal = ln_implied_rate.checked_mul(time_to_expiry)
            .and_then(|result: PreciseDecimal| 
                result
                .checked_div(PERIOD_SIZE)
            )
            .unwrap();
        
        let exchange_rate: PreciseDecimal = rt.exp().unwrap();

        return exchange_rate
    }
