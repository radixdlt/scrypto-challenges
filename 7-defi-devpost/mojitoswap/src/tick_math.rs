use scrypto::prelude::*;

//smallest decimal: 0.000000000000000001 (10 ^ -18) -> smallest tick: -828972, but effectively for us smallest price is 0.00000000000001985 with min tick -631042
//as we can't have enough precision under this values: e.g. for tick -631043 the price would be 0.000000000000019849 as we don't have enough decimal places to represent it
//we stop for now at decimal: 170141183460469231731.687303715884105727 (2^127 - 1) * 10 ^ -18 -> largest tick: 931709 -> real max 170134484377190040957.155711420855095752, but this limit can be increased
const MIN_TICK: i32 = -631042;
const MAX_TICK: i32 = 931709;

// todo, maybe use constants somehow instead of lazy_static
lazy_static! {
    static ref MIN_PRICE: Decimal = dec!("0.00000000000001985");
    static ref PRICE_0X1: Decimal = dec!("1.000049998750062496");     //(sqrt(1.0001) ^ (2 ^ 0))
    static ref PRICE_0X2: Decimal = dec!("1.0001");                 //(sqrt(1.0001) ^ (2 ^ 1))
    static ref PRICE_0X4: Decimal = dec!("1.00020001");             //(sqrt(1.0001) ^ (2 ^ 2))
    static ref PRICE_0X8: Decimal = dec!("1.000400060004000093");         //(sqrt(1.0001) ^ (2 ^ 3))
    static ref PRICE_0X10: Decimal = dec!("1.000800280056006986");     //(sqrt(1.0001) ^ (2 ^ 4))
    static ref PRICE_0X20: Decimal = dec!("1.001601200560182014");    //(sqrt(1.0001) ^ (2 ^ 5))
    static ref PRICE_0X40: Decimal = dec!("1.003204964963597955");    //(sqrt(1.0001) ^ (2 ^ 6))
    static ref PRICE_0X80: Decimal = dec!("1.0064202017276138");    //(sqrt(1.0001) ^ (2 ^ 7))
    static ref PRICE_0X100: Decimal = dec!("1.012881622445450855");   //(sqrt(1.0001) ^ (2 ^ 8))
    static ref PRICE_0X200: Decimal = dec!("1.025929181087728853");   //(sqrt(1.0001) ^ (2 ^ 9))
    static ref PRICE_0X400: Decimal = dec!("1.052530684607337941");    //(sqrt(1.0001) ^ (2 ^ 10))
    static ref PRICE_0X800: Decimal = dec!("1.107820842039991493");    //(sqrt(1.0001) ^ (2 ^ 11))
    static ref PRICE_0X1000: Decimal = dec!("1.227267018058195782");  //(sqrt(1.0001) ^ (2 ^ 12))
    static ref PRICE_0X2000: Decimal = dec!("1.506184333613455851");  //(sqrt(1.0001) ^ (2 ^ 13))
    static ref PRICE_0X4000: Decimal = dec!("2.268591246822610072");    //(sqrt(1.0001) ^ (2 ^ 14))
    static ref PRICE_0X8000: Decimal = dec!("5.146506245160164533");   //(sqrt(1.0001) ^ (2 ^ 15))
    static ref PRICE_0X10000: Decimal = dec!("26.486526531472575563");  //(sqrt(1.0001) ^ (2 ^ 16))
    static ref PRICE_0X20000: Decimal = dec!("701.536087702400664335");  //(sqrt(1.0001) ^ (2 ^ 17))
    static ref PRICE_0X40000: Decimal = dec!("492152.882348790396620919"); //(sqrt(1.0001) ^ (2 ^ 18))
    static ref PRICE_0X80000: Decimal = dec!("242214459604.222321943471435452"); //(sqrt(1.0001) ^ (2 ^ 19))
    static ref MAX_PRICE: Decimal = dec!("170134484377190040957.155711420855095752");
}

/** 
 * By definition, sqrt_price = sqrt(1.0001) ^ tick, but tick is always a sum of powers of 2, e.g. 7 = 2^0 + 2^1 + 2^2,
 * So, sqrt_price = sqrt(1.0001) ^ (2 ^ a + 2 ^ b + ...) = sqrt(1.0001) ^ (2 ^ a) * sqrt(1.0001) ^ (2 ^ b) * ...
 * Where  a,b,... are uniques values in interval [0, 19], given the max tick value of 931709.
 * 
 * So, the algorithm bellow decompose the given tick in a power of 2 sum, and for each power of 2, it multiplies 
 * the sqrt_price with the corresponding pre-computed sqrt_price from the constants above. This is the sqrt_price we are looking for.
 */ 
pub fn sqrt_price_at_tick(tick: i32) -> Decimal {
    assert!(tick >= MIN_TICK && tick <= MAX_TICK, "Tick out of bounds.");

    let abs_tick = if tick >= 0 { tick } else { -tick };
    let mut sqrt_price = Decimal::one();

    if abs_tick & 0x1 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X1;
    }
    if abs_tick & 0x2 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X2;
    }
    if abs_tick & 0x4 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X4;
    }
    if abs_tick & 0x8 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X8;
    }
    if abs_tick & 0x10 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X10;
    }
    if abs_tick & 0x20 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X20;
    }
    if abs_tick & 0x40 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X40;
    }
    if abs_tick & 0x80 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X80;
    }
    if abs_tick & 0x100 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X100;
    }
    if abs_tick & 0x200 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X200;
    }
    if abs_tick & 0x400 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X400;
    }
    if abs_tick & 0x800 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X800;
    }
    if abs_tick & 0x1000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X1000;
    }
    if abs_tick & 0x2000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X2000;
    }
    if abs_tick & 0x4000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X4000;
    }
    if abs_tick & 0x8000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X8000;
    }
    if abs_tick & 0x10000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X10000;
    }
    if abs_tick & 0x20000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X20000;
    }
    if abs_tick & 0x40000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X40000;
    }
    if abs_tick & 0x80000 != 0 {
        sqrt_price = sqrt_price * *PRICE_0X80000;
    }

    if tick < 0 {
        sqrt_price = Decimal::one() / sqrt_price;
    }
    sqrt_price
}

/**
 * We use the same property used to compute sqrt_price_at_tick, to compute tick_at_sqrt_price, but instead of multiplying, now we
 * are dividing the given price by the pre-computed constants, in the same time adding the corresponding exponent to the target tick.
 * 
 * In the end we want to make sure the sqrt_price_at_tick and tick_at_sqrt_price return consistent values, avoiding rounding errors.
 */
pub fn tick_at_sqrt_price(sqrt_price_in: Decimal) -> i32 {
    assert!(sqrt_price_in >= *MIN_PRICE && sqrt_price_in <= *MAX_PRICE, "Sqrt price out of bounds.");

    let is_negative_tick = sqrt_price_in < Decimal::one();
    let mut sqrt_price = if is_negative_tick { Decimal::one() / sqrt_price_in } else {sqrt_price_in};
    let mut tick = 0;
    if sqrt_price >= *PRICE_0X80000 {
        sqrt_price = sqrt_price / *PRICE_0X80000;
        tick += 0x80000; //2^19
    } 
    if sqrt_price >= *PRICE_0X40000 {
        sqrt_price = sqrt_price / *PRICE_0X40000;
        tick += 0x40000 //2^18
    }
    if sqrt_price >= *PRICE_0X20000 {
        sqrt_price = sqrt_price / *PRICE_0X20000;
        tick += 0x20000; //2^17
    }
    if sqrt_price >= *PRICE_0X10000 {
        sqrt_price = sqrt_price / *PRICE_0X10000;
        tick += 0x10000; //2^16
    }
    if sqrt_price >= *PRICE_0X8000 {
        sqrt_price = sqrt_price / *PRICE_0X8000;
        tick += 0x8000; //2^15
    }
    if sqrt_price >= *PRICE_0X4000 {
        sqrt_price = sqrt_price / *PRICE_0X4000;
        tick += 0x4000; //2^14
    }
    if sqrt_price >= *PRICE_0X2000 {
        sqrt_price = sqrt_price / *PRICE_0X2000;
        tick += 0x2000; //2^13
    }
    if sqrt_price >= *PRICE_0X1000 {
        sqrt_price = sqrt_price / *PRICE_0X1000;
        tick += 0x1000; //2^12
    }
    if sqrt_price >= *PRICE_0X800 {
        sqrt_price = sqrt_price / *PRICE_0X800;
        tick += 0x800; //2^11
    }
    if sqrt_price >= *PRICE_0X400 {
        sqrt_price = sqrt_price / *PRICE_0X400;
        tick += 0x400; //2^10
    }
    if sqrt_price >= *PRICE_0X200 {
        sqrt_price = sqrt_price / *PRICE_0X200;
        tick += 0x200; //2^9
    }
    if sqrt_price >= *PRICE_0X100 {
        sqrt_price = sqrt_price / *PRICE_0X100;
        tick += 0x100; //2^8
    }
    if sqrt_price >= *PRICE_0X80 {
        sqrt_price = sqrt_price / *PRICE_0X80;
        tick += 0x80; //2^7
    }
    if sqrt_price >= *PRICE_0X40 {
        sqrt_price = sqrt_price / *PRICE_0X40;
        tick += 0x40; //2^6
    }
    if sqrt_price >= *PRICE_0X20 {
        sqrt_price = sqrt_price / *PRICE_0X20;
        tick += 0x20; //2^5
    }
    if sqrt_price >= *PRICE_0X10 {
        sqrt_price = sqrt_price / *PRICE_0X10;
        tick += 0x10; //2^4
    }
    if sqrt_price >= *PRICE_0X8 {
        sqrt_price = sqrt_price / *PRICE_0X8;
        tick += 0x8; //2^3
    }
    if sqrt_price >= *PRICE_0X4 {
        sqrt_price = sqrt_price / *PRICE_0X4;
        tick += 0x4; //2^2
    }
    if sqrt_price >= *PRICE_0X2 {
        sqrt_price = sqrt_price / *PRICE_0X2;
        tick += 0x2; //2^1
    }
    if sqrt_price >= *PRICE_0X1 {
        tick += 0x1; //2^0
    }

    let tick_candidate = if is_negative_tick { -tick } else { tick };
    let sqrt_price_tick_candidate = sqrt_price_at_tick(tick_candidate);
    if sqrt_price_tick_candidate == sqrt_price_in { tick_candidate }
    else if sqrt_price_tick_candidate > sqrt_price_in { tick_candidate - 1 }
    else { tick_candidate + 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_price_at_tick() {
        assert_eq!(*MIN_PRICE, sqrt_price_at_tick(MIN_TICK));
        assert_eq!(dec!("0.000000000276890319"), sqrt_price_at_tick(-440170));
        assert_eq!(dec!("0.999850018747812746"), sqrt_price_at_tick(-3));
        assert_eq!(dec!("0.999950003749687527"), sqrt_price_at_tick(-1));
        assert_eq!(Decimal::one(), sqrt_price_at_tick(0));
        assert_eq!(dec!("1.000150003749937502"), sqrt_price_at_tick(3));
        assert_eq!(dec!("1.000250018750312495"), sqrt_price_at_tick(5));
        assert_eq!(dec!("1.451912069310684182"), sqrt_price_at_tick(7458));
        assert_eq!(dec!("13.043260825728760908"), sqrt_price_at_tick(51368));
        assert_eq!(dec!("3611718901.096063128233128884"), sqrt_price_at_tick(440171));
        assert_eq!(*MAX_PRICE, sqrt_price_at_tick(MAX_TICK));
    }

    #[test]
    fn test_tick_at_sqrt_price() {
        assert_eq!(0, tick_at_sqrt_price(Decimal::one()));
        assert_eq!(1, tick_at_sqrt_price(dec!("1.0000499987500624")));
        assert_eq!(2, tick_at_sqrt_price( dec!("1.0001")));
        assert_eq!(3, tick_at_sqrt_price(dec!("1.000150003749937406")));
        assert_eq!(-3, tick_at_sqrt_price(dec!("0.999850018747812842")));
        assert_eq!(-440170, tick_at_sqrt_price(dec!("0.000000000276890319")));
        assert_eq!(-440170, tick_at_sqrt_price(dec!("0.00000000027689032")));
        assert_eq!(440171, tick_at_sqrt_price(dec!("3611718901.08879675118568791")));
        assert_eq!(440171, tick_at_sqrt_price(dec!("3611718901.08879675118568792")));
        assert_eq!(MAX_TICK, tick_at_sqrt_price(*MAX_PRICE));
        assert_eq!(MIN_TICK, tick_at_sqrt_price(*MIN_PRICE));
    }

    /**
     * Tests that we are consistent with all the ticks between MIN_TICK and MAX_TICK. Depending on the machine, this test can take 1-2 minutes to run, so by default is disabled.
     */
    #[test]
    #[ignore]
    fn tick_at_sqrt_price_equals_original_tick() {
        for tick in (MIN_TICK..MAX_TICK + 1).rev() {
            assert_eq!(tick, tick_at_sqrt_price(sqrt_price_at_tick(tick)));
        }
    }
}
