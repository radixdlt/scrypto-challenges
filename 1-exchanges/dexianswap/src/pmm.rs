use scrypto::prelude::*;
use sbor::*;
use crate::util::*;

#[derive(TypeId, Clone, Copy, Encode, Decode, Describe, Debug)]
pub enum RState {
    One,
    AboveOne,
    BelowOne
}

blueprint! {

    struct PMMPool {
        base_vault: Vault,
        quote_vault: Vault,
        lp_minter_badge: Vault,
        base0_amnt: Decimal,
        quote0_amnt: Decimal,
        _r_state: RState,
        _i: Decimal,
        _k: Decimal,
        _fee: Decimal,
        lp_token_def: ResourceDef
    }

    impl PMMPool{
        
        pub fn new(
            base_tokens: Bucket,
            quote_tokens: Bucket,
            k: Decimal,
            fee: Decimal,
            lp_name: String,
            lp_url: String,
            lp_initial_supply: Decimal
        ) -> (Component, Bucket){
            assert!(
                !base_tokens.is_empty() && !quote_tokens.is_empty(),
                "You must pass in an initial supply of each token."
            );

            assert!(
                fee >= Decimal::zero() && fee <= Decimal::one(),
                "Invalid fee in thousandths"
            );

            let mid_price = base_tokens.amount() / quote_tokens.amount();

            let lp_minter_badge = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "LP Token Mint Auth")
                .metadata("symbol", "LP")
                .initial_supply_fungible(Decimal::one());
            
            let lp_token_symbol = get_pool_token_pair(base_tokens.resource_address(), quote_tokens.resource_address());
            let mut lp_token_def = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("symbol", lp_token_symbol)
                .metadata("name", lp_name)
                .metadata("url", lp_url)
                .flags(MINTABLE | BURNABLE)
                .badge(lp_minter_badge.resource_def(), MAY_MINT | MAY_BURN)
                .no_initial_supply();
            
            let lp_tokens = lp_token_def.mint(lp_initial_supply, lp_minter_badge.present());
            let b0 = base_tokens.amount();
            let q0 = quote_tokens.amount();

            let pmm_pool = Self {
                base_vault: Vault::with_bucket(base_tokens),
                quote_vault: Vault::with_bucket(quote_tokens),
                lp_minter_badge: Vault::with_bucket(lp_minter_badge),
                base0_amnt: b0,
                quote0_amnt: q0,
                _r_state: RState::One,
                _i:mid_price,
                _k:k,
                _fee:fee,
                lp_token_def
            }
            .instantiate();

            (pmm_pool, lp_tokens)
        }

        pub fn sell_base(
            &mut self,
            pay_base_amnt: Decimal
        ) -> Bucket {
            let (quote_amnt, new_r_state) = PMMPool::sell_base_token(
                pay_base_amnt,
                self._r_state,
                self.base0_amnt,
                self.base_vault.amount(),
                self.quote0_amnt,
                self.quote_vault.amount(),
                self._i,
                self._k
            );
            self._r_state = new_r_state;
            let received_quote_amnt = quote_amnt * (Decimal::one()-self._fee);
            self.quote_vault.take(received_quote_amnt)
        }

        fn sell_base_token(
            pay_base_amnt: Decimal,
            r_state: RState,
            b0: Decimal,
            b: Decimal,
            q0: Decimal,
            q: Decimal,
            i: Decimal,
            k: Decimal
        ) -> (Decimal, RState) {
            match r_state{
                RState::One => {
                    let received_quote_amnt = PMMPool::_r_one_sell_base(
                        q0,
                        pay_base_amnt,
                        i,
                        k
                    );
                    (received_quote_amnt, RState::AboveOne)
                },
                RState::BelowOne => {
                    let received_quote_amnt = Decimal::zero();
                    (received_quote_amnt, RState::One)
                },
                RState::AboveOne => {
                    // case 3: R > 1
                    let back_to_one_pay_base = b0 - b;
                    let back_to_one_receive_quote = q - q0;

                    // complex case, R status depends on trading amount
                    if pay_base_amnt < back_to_one_pay_base {
                        // case 3.1 R status do not change
                        let mut received_quote_amnt = PMMPool::_r_above_sell_base(
                            b0,
                            pay_base_amnt,
                            b,
                            i,
                            k
                        );
                        if received_quote_amnt > back_to_one_receive_quote {
                            // [Important corner case!] may enter this branch when some precision problem happens. And consequently contribute to negative spare quote amount
                            // to make sure spare quote>=0, mannually set receiveQuote=backToOneReceiveQuote
                            received_quote_amnt = back_to_one_receive_quote;
                        }
                        (received_quote_amnt, RState::AboveOne)
                    }
                    else if pay_base_amnt == back_to_one_pay_base {
                        // case 3.2 R state changes to ONE
                        let received_quote_amnt = back_to_one_receive_quote;
                        (received_quote_amnt, RState::One)
                    }
                    else{
                        // case 3.3 R status changes to below_one
                        let received_quote_amnt = back_to_one_receive_quote + PMMPool::_r_one_sell_base(
                            q0 + back_to_one_receive_quote,
                            pay_base_amnt - back_to_one_pay_base,
                            i,
                            k
                        );
                        (received_quote_amnt, RState::BelowOne)
                    }
                }
            }
        }

        fn _r_above_sell_base(
            b0: Decimal,
            pay_base_amnt: Decimal,
            b: Decimal,
            i: Decimal,
            k: Decimal
        ) -> Decimal{
            return PMMPool::general_integrate(
                b0,
                b + pay_base_amnt,
                b,
                i,
                k
            );
        }

        fn _r_one_sell_base(
            q0: Decimal,
            pay_base_amnt: Decimal,
            i: Decimal,
            k: Decimal
        ) -> Decimal{
            return PMMPool::solve_quadratic_function_for_trade(q0, q0, pay_base_amnt, i, k);
        }

        fn general_integrate(
            v0: Decimal,
            v1: Decimal,
            v2: Decimal,
            i: Decimal,
            k: Decimal
        ) -> Decimal {
            //asset v0 > 0
            let fair_amnt = i * (v1 - v2);   // i*delta
            if k == Decimal::zero(){
                return fair_amnt;
            }

            let v0v0v1v2 = v0 * v0 / (v1 * v2);
            let penalty = k * v0v0v1v2;
            return (Decimal::one() - k + penalty) * fair_amnt;
        }

        fn solve_quadratic_function_for_target(
            v1: Decimal,
            delta: Decimal,
            i: Decimal,
            k: Decimal
        ) -> Decimal{
            if k == Decimal::zero(){
                return v1 + i * delta;
            }

            // v0 = v1 * (1 + (sqrt - 1) / 2k )
            // sqrt = √(1+4kidelta/V1)
            // premium = 1 + (sqrt - 1) / 2k
            let mut sqrt = Decimal::zero();
            let ki = Decimal(4) * k * i;

            if ki == Decimal::zero() {
                sqrt = Decimal::one();
            }
            else {
                sqrt = Decimal::one();  // sqrt( Decimal::one() + ki * delta / v1  )
            }

            return v1 * (Decimal::one() + sqrt - Decimal::one() / (Decimal(2) * k))
        }

        fn solve_quadratic_function_for_trade(
            v0: Decimal,
            v1: Decimal,
            delta: Decimal,
            i: Decimal,
            k: Decimal
        ) -> Decimal{
            //asset v0 > 0
            if delta == Decimal::zero(){
                return Decimal::zero();
            }

            if k == Decimal::zero(){
                if i * delta > v1 {
                    return v1;
                }
                else{
                    return i * delta;
                }
            }

            if k == Decimal::one() {
                let mut tmp = Decimal::zero();
                let idelta = i * delta;
                if idelta == Decimal::zero() {
                    tmp = Decimal::zero();
                }
                else{
                    tmp = delta * v1 / (v0 * v0)
                }
                return v1 * tmp / (tmp + Decimal::one())
            }

            let part2 = k * v0 * v0 / v1 + i * delta;
            let mut b_abs = (Decimal::one() - k) * v1;

            let b_sig: bool = b_abs < part2;
            if b_sig {
                b_abs = part2 - b_abs;
            }
            else{
                b_abs = b_abs - part2;
            }

            let square_root = Decimal::one(); //sqrt(b_abs * b_abs  + Decimal(4) * (Decimal::one() - k) * v0 * v0 * k);

            let denominator = Decimal(2) * (Decimal::one() - k);
            let mut numerator:Decimal = Decimal::zero();
            if b_sig {
                numerator = square_root - b_abs;
            }
            else{
                numerator = b_abs + square_root;
            }

            let v2 = numerator / denominator;
            if v2 > v1 {
                return Decimal::zero();
            }
            return v1 - v2;
        }
        
    }
}