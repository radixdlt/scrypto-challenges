# MojitoSwap

MojitoSwap is a concentrated liquidity pool inspired by the [Uniswap v3 whitepaper](https://uniswap.org/whitepaper-v3.pdf). This whitepaper explains quite well how concentrated liquidity pools work. Thus it represents a prerequiste to read before diving into the MojitoSwap implementation. Implementation details are provided though as comments in the code.

## Contents
For now, the MojitoSwap repository contains only the scrypto blueprint implementation, without the Oracle functionality described in the whitepaper. The blueprint represents a pool of 2 fungible resources and provides the following operations:
 - Add a new liquidity position to the pool
 - Remove a liquidity position a LP holds
 - Add more liquidity to a position a LP already holds
 - Collect the fees for a position a LP holds
 - Add position fees to liquidity
 - Add positions acting as limit orders (implicit)

## How to run and test the component
The repository contains an utility that allows to define test scenarios as integration tests. These tests scenarios create transaction manifests that are against the component. For example, a test scenario that can be created (tests/pool.rs file) looks as following:

    /**
    * Limit order & multiple range swap.
    * 
    * Given a pool with fee=0, sqrt_price=1 and a position=[10000 MOJ + 10000 USDT, -1000, 1000]
    * 
    * Test that if: 
    * - an account adds a position=[1000 MOJ, 199, 200], this can act as limit order saying: sell 1000MOJ at price 1.02. 
    * The range [199, 200] is corresponding to sqrt_price range [1.009999163397141405, 1.010049662092875426] which 
    * aproximates a price of 1.02
    * - the price moves past the position range
    * - the account holding the limit order position remove it
    * 
    * Then the account gets the expected amount of USDT: ~1020 USDT
    */
    #[test]
    fn scenario_6() {
        let mut context = Context::new(Decimal::zero(), Decimal::one());
        let account = context.new_account_with_moj_and_usdt(dec!("20000"), dec!("20000"));
        context.add_position(&account, dec!("10000"), dec!("10000"), -1000, 1000);

        let account2 = context.new_account_with_moj_and_usdt(dec!("20000"), dec!("20000"));
        context.add_position(&account2, dec!("1000"), Decimal::zero(), 199, 200);

        context.swap_usdt_for_moj(&account, dec!("8000"), dec!("7750.081465536550594191"));
        context.remove_pos(&account2, Decimal::zero(), dec!("1020.149313703371480602"));
    }

After the test scenario is created, it can be run as a scrypto/cargo test from the project directory:

    scrypto test - scenario_6  -- --nocapture 

Look in tests/pool.rs for more scenarios and documentation on the test utility methods.

The component also displays a detailed debug log for each executed transaction. It has also some unit tests for the math computations with explanations of the formulas.




## License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

    Copyright 2024 Radix Publishing Ltd

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software for non-production informational and educational purposes without
    restriction, including without limitation the rights to use, copy, modify,
    merge, publish, distribute, sublicense, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    This notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
    AND EDUCATIONAL PURPOSES ONLY.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
    EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
    COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
    OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.

