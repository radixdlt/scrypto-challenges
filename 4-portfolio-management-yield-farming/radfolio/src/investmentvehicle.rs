//! This blueprint demonstrates the interface you need to provide when
//! implementing an investment vehicle of your own. These methods are
//! used by Radfolio when managing its investments.
//!
//! The code in this file is provided for documentation purposes
//! only. It performs no useful function and can be removed from the
//! project with no ill effect.
//!
//! Note that [read_max_investable] is not used by Radfolio
//! directly but should still be implemented for the benefit of
//! front-end software which will want to display such information
//! to the user.
//!
//! Radfolio uses the [read_investment_value] method extensively when
//! rebalancing its investments and a great deal of care should go
//! into its design. A few rules of thumb:
//!
//! - It should take known losses into account as early as possible.

//! - It should take profits into account only when they have been
//! realized.

//! - If no better information is available, it can assume that funds
//! that have been deposited into an investment opportunity still
//! retain their original value.
//!
//! You will need to also write an `instantiate_` type function
//! for your component. This function should take as parameter the
//! `iv_control_badge` that Radfolio uses to control its
//! investment vehicles. The following methods always get called
//! with that badge in auth zone and you should implement them to
//! only be callable that way (i.e. with
//! `rule!(require(iv_control_badge))`).
//!
//! - [add_funds]
//! - [reduce_funds]
//! - [withdraw_profits]
//!
//! [add_funds]: blueprint::InvestmentVehicle::add_funds
//! [reduce_funds]: blueprint::InvestmentVehicle::reduce_funds
//! [withdraw_profits]: blueprint::InvestmentVehicle::withdraw_profits
//! [read_investment_value]: blueprint::InvestmentVehicle::read_investment_value
//! [read_max_investable]: blueprint::InvestmentVehicle::read_max_investable

use scrypto::prelude::*;

blueprint! {

    struct InvestmentVehicle {}

    impl InvestmentVehicle {

        /// This method adds funds to the investment vehicle.
        ///
        /// Some vehicles may be able to accept any amount of funds
        /// and will swallow everything they receive.
        ///
        /// Others may only be able to accept funds in specific
        /// increments, e.g. they must receive funds in whole
        /// thousands and cannot use fractions of a thousand.
        ///
        /// Maybe some can only be increased on Mondays.
        ///
        /// Some may have a maximum or minimum increase from current
        /// investment, or a max overall investment etc.
        ///
        /// Weirder things than this are always possible.
        ///
        /// Whatever the case may be for the one you are implementing,
        /// put as much as is possible into the underlying investment
        /// opportunity and return whatever is left over.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn add_funds(&mut self, mut _new_funds: Bucket) -> Option<Bucket>
        { None }

        /// This method is called in an attempt to pull funds out of
        /// the investment vehicle.
        ///
        /// Different investment opportunities have different profiles
        /// with regard to how much can be pulled out and how fast it
        /// can be done. For example if your investment is staking XRD
        /// towards a validator node it will take two weeks to reduce
        /// that stake and get the funds back.
        ///
        /// Your implementation needs to provide a façade that hides
        /// the details of this from Radfolio and does its best to
        /// abide by the instructions received. Feel free to be
        /// creative in order to adapt this to the particular nature
        /// of the underlying investment.
        ///
        /// It is perfectly fine for example for your implementation
        /// to not return funds right away, but to instead start a
        /// fund retrieval process that only completes at some later
        /// time. Radfolio is patient. It will wait.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn reduce_funds(&mut self, _by_amount: Decimal) -> Option<Bucket>
        { None }

        /// Pulls out any profits that have accrued in this investment
        /// vehicle.
        ///
        /// Some types of investment will regularly produce rewards
        /// that do not compound but instead get put into a big bag of
        /// profits. Radfolio calls this method to empty out that bag.
        ///
        /// Some types of investment never put funds into the profits
        /// bag, for example because they auto-compound the investment
        /// instead.
        ///
        /// Some types of investment convert their full invested value
        /// into profit upon maturity (for example, a loan that gets
        /// repaid -- hopefully with some interest on top).
        ///
        /// You should implement a profits model that fits your
        /// underlying investment opportunity: Maybe you will use this
        /// method extensively, or maybe not at all.
        ///
        /// Note that if the investment generates rewards in other
        /// token types than our investment token you will either need
        /// to find a way to convert those tokens into the investment
        /// token for return to Radfolio, or else provide additional
        /// methods that an admin user can call direct to pull out
        /// those tokens outside of the Radfolio logic.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with `iv_control_badge` in auth zone
        ///
        /// **Transaction manifest:** Not user callable, so no manifest is provided
        pub fn withdraw_profits(&mut self) -> Option<Bucket>
        { None }

        /// Radfolio calls this method to estimate the current value
        /// of the investment. This is then used to rebalance its
        /// investments. It is important that this method returns a
        /// good estimate of value since this value is also used to
        /// calculate the value of Radfolio's coupons when users
        /// deposit and withdraw funds.
        ///
        /// It is recommended not to include speculative future
        /// profits into this value, but to realize them only when
        /// they physically arrive.
        ///
        /// A typical implementation might return the original
        /// investment value plus the current uncollected profits that
        /// are held by the component, but every investment vehicle is
        /// unique and may have its own quirks in this regard.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/investmentvehicle/read_investment_value.rtm`
        /// ```text
        #[doc = include_str!("../rtm/investmentvehicle/read_investment_value.rtm")]
        /// ```
        pub fn read_investment_value(&self) -> Decimal
        { Decimal::ZERO }

        /// Front-ends and other support systems will call this method
        /// to discover if we have a maximum possible investment level
        /// and if so what it is. Radfolio itself doesn't use this
        /// method.
        ///
        /// If the maximum is unlimited, return `None`.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/investmentvehicle/read_max_investable.rtm`
        /// ```text
        #[doc = include_str!("../rtm/investmentvehicle/read_max_investable.rtm")]
        /// ```
        pub fn read_max_investable(&self) -> Option<Decimal>
        { None }
    }
}
