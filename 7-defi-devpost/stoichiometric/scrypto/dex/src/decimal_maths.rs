//! Mathematical primitives implementation

use scrypto::math::BnumI256;
use scrypto::prelude::{dec, Decimal};
use std::fmt;
use std::ops::Neg;

/// A constant equal to `e = exp(1)` also known as Euler number.
pub const EULER_NUMBER: Decimal = Decimal(BnumI256::from_digits([2718281828459045235, 0, 0, 0]));

/// Returns the exponential of a [`Decimal`] using Taylor series
///
/// # Arguments
/// * `value` - The Decimal to compute the exponential for
///
/// # Examples
///
/// ```
/// use scrypto::prelude::{dec, Decimal};
/// use stoichiometric_dex::decimal_maths::exp;
///
/// let res = exp(dec!("2.5"));
/// assert_eq!(res, dec!("12.182493960703473402"));
/// ```
pub fn exp<T: TryInto<Decimal>>(value: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    let value = value.try_into().expect("Cannot convert to Decimal");
    if value.is_zero() {
        return Decimal::ONE;
    } else if value.is_negative() {
        return if value < dec!(-43) {
            // Because exp(-43)< 10^-18, the smallest representable Decimal, we return 0
            Decimal::zero()
        } else {
            Decimal::ONE / exp(value.neg())
        };
    } else {
        // The Taylor series of exp is : exp(x) = SUM x^k / fact(k)

        // outputted result
        let mut result = Decimal::one();

        // next term of the taylor expansion
        let mut added_term = value;

        // counter to remember the index of the next term to add
        let mut counter = Decimal::one();

        while added_term != Decimal::zero() {
            result = result + added_term;

            counter = counter + 1;
            // previous term was x^k/k! so the next term is:  previous*x/(k+1)
            let mut next_term = added_term / counter;
            next_term = next_term * value;

            added_term = next_term;
        }

        result
    }
}

/// Returns the natural logarithm of a [`Decimal`] using Halley's method
///
/// # Arguments
/// * `value` - The Decimal to compute the logarithm for
///
/// # Examples
///
/// ```
/// use scrypto::prelude::{dec, Decimal};
/// use stoichiometric_dex::decimal_maths::ln;
///
/// let res = ln(100);
/// assert_eq!(res, dec!("4.605170185988091375"));
/// ```
pub fn ln<T: TryInto<Decimal>>(value: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
{
    let mut value = value.try_into().expect("Cannot convert to Decimal");
    assert!(
        value.is_positive(),
        "Logarithm is only defined for positive numbers"
    );

    // We are not using the Taylor expansion of ln because it converges too slowly
    // To compute ln(y), we use Halley's method and we compute the sequence x_n defined by induction:
    // x_{n+1} = x_n + ( y - exp(x_n) )/( y + exp(x_n) )

    // Because, exp overflows very quickly, we rewrite y = a*e^n with a<e
    // Therefore, ln(y) = ln(a) + n

    if value < Decimal::ONE {
        return -ln(Decimal::ONE / value);
    }

    let mut n = 0;
    while value >= EULER_NUMBER {
        value = value / EULER_NUMBER;
        n += 1;
    }

    if value == Decimal::ONE {
        return Decimal::from(n);
    }

    // Start with an arbitrary number as the first guess
    let mut result = value / Decimal::from(2u8);

    // Too small to represent, so we start with self
    // Future iterations could actually avoid using a decimal altogether and use a buffered
    // vector, only combining back into a decimal on return
    if result.is_zero() {
        result = value;
    }
    let mut last = result + 1;

    // Keep going while last and result are not equal
    let mut circuit_breaker = 0;
    while last != result {
        circuit_breaker += 1;
        assert!(circuit_breaker < 1000, "geo mean circuit breaker");

        last = result;
        let exp_last = exp(last);
        result = last + (value - exp_last) / (value + exp_last) * 2;
    }

    result + Decimal::from(n)
}

/// Returns the power of a [`Decimal`] using `exp` and `ln`
///
/// # Arguments
/// * `value` - The Decimal to compute the exponential for
///
/// # Examples
///
/// ```
/// use scrypto::prelude::{dec, Decimal};
/// use stoichiometric_dex::decimal_maths::pow;
///
/// let res = pow::<Decimal, Decimal>(dec!("2.5"), dec!("3.4"));
/// assert_eq!(res, dec!("22.542186029800212409"));
/// ```
pub fn pow<T: TryInto<Decimal>, E: TryInto<Decimal>>(value: T, exponent: T) -> Decimal
where
    <T as TryInto<Decimal>>::Error: fmt::Debug,
    <E as TryInto<Decimal>>::Error: fmt::Debug,
{
    let value_dec = value.try_into().expect("Cannot convert into Decimal");
    let exp_dec = exponent.try_into().expect("Cannot convert into Decimal");
    assert!(
        !exp_dec.is_zero() || !value_dec.is_zero(),
        "O^O is undefined"
    );

    exp(exp_dec * ln(value_dec))
}

#[cfg(test)]
mod tests {
    use crate::decimal_maths::{exp, ln, EULER_NUMBER};
    use scrypto::math::Decimal;
    use scrypto::prelude::dec;

    #[test]
    fn test_exp_zero() {
        let res = exp(0);
        let true_res = Decimal::one();
        assert_eq!(res, true_res);
    }

    #[test]
    fn test_exp_42() {
        let res = exp(42);
        assert_eq!(res, dec!("1739274941520501047.394681299721124048"))
    }

    #[test]
    fn test_exp_minus_12() {
        let res = exp(-12);
        assert_eq!(res, dec!("0.000006144212353328"))
    }

    #[test]
    #[should_panic]
    fn test_ln_neg() {
        let _m = ln(-5);
    }

    #[test]
    #[should_panic]
    fn test_ln_zero() {
        let _m = ln(0);
    }

    #[test]
    fn test_ln_int() {
        let res = ln(exp(12));
        assert_eq!(res, dec!("12.000000000000000002"))
    }

    #[test]
    fn test_ln_smaller_than_one() {
        let res = ln("0.69");
        assert_eq!(res, dec!("-0.371063681390831991"));
    }

    #[test]
    fn test_ln_euler() {
        let res = ln(EULER_NUMBER);
        assert_eq!(res, Decimal::one())
    }
}
