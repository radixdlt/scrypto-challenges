//! Radfolio implements a crypto investment fund that automatically
//! puts invested funds into any number of configured investment
//! vehicles, harvesting profits which are then automatically
//! re-invested across those investment vehicles.
//!
//! Investors receive fungible coupon tokens in return for their
//! investment, with coupons representing shares of ownership of the
//! whole fund. Coupons can be transferred or traded in secondary
//! markets and whoever holds them can at any time redeem them for
//! their investment value. As the fund makes profits, the value of
//! each coupon increases.
//!
//! The fund's managers can dynamically add and remove investment
//! vehicles, or adjust the balancing of existing ones, to respond to
//! changing market conditions or to adjust the risk profile of the
//! fund etc.
//!
//! The fund can charge protocol fees, intended at producing profits
//! for the fund's managers. Some or all of these protocol fees can go
//! towards rewards for partners, expected to be influencers or other
//! promoters of the fund who are thus incentivized to draw customers
//! to it.
//!
//! # Overview of functions and methods
//!
//! This is a brief synopsis of all of the public functions and
//! methods provided by Radfolio. They are grouped by categories of
//! functionality.
//!
//! ## Instantiation
//!
//! [instantiate_radfolio()][blueprint::Radfolio::instantiate_radfolio]
//! Creates a new Radfolio instance.
//!
//! ## Fund Rebalancing
//!
//! [force_fund_maintenance()][blueprint::Radfolio::force_fund_maintenance]
//! Causes the fund to rebalance its funds among investment vehicles.
//!
//! ## Investor Activity
//!
//! [deposit()][blueprint::Radfolio::deposit]
//! Deposits coins into the fund, receiving coupons in return.
//!
//! [withdraw()][blueprint::Radfolio::withdraw]
//! Redeems fund coupons for the equivalent value of coins.
//!
//! [value_of_coupons()][blueprint::Radfolio::value_of_coupons]
//! Calculates the current coin value of some number of coupons.
//!
//! ## Collect Fees
//!
//! [read_partner_fees_stored()][blueprint::Radfolio::read_partner_fees_stored]
//! Reports how much uncollected fees partners have in the fund.
//!
//! [withdraw_partner_fees()][blueprint::Radfolio::withdraw_partner_fees]
//! Partners call this to collect their partner fees.
//!
//! [read_fees_stored()][blueprint::Radfolio::read_fees_stored]
//! Reports how much uncollected protocol fees are in the fund.
//!
//! [withdraw_protocol_fees()][blueprint::Radfolio::withdraw_protocol_fees]
//! Fund managers call this to collect protocol fees.
//!
//! ## Investment Vehicle Management
//!
//! [add_investment_vehicle()][blueprint::Radfolio::add_investment_vehicle]
//! Adds an investment vehicle to the fund.
//!
//! [modify_investment_vehicle()][blueprint::Radfolio::modify_investment_vehicle]
//! Changes the weight of one of our investment vehicles.
//!
//! [remove_investment_vehicles()][blueprint::Radfolio::remove_investment_vehicles]
//! Takes an investment vehicle out of the fund.
//!
//! [clear_investment_vehicles()][blueprint::Radfolio::clear_investment_vehicles]
//! Takes all investment vehicles out of the fund.
//!
//! [read_investment_vehicles()][blueprint::Radfolio::read_investment_vehicles]
//! Retrieves a list of current investment vehicles.
//!
//! [halt_investment_vehicles()][blueprint::Radfolio::halt_investment_vehicles]
//! Temporarily stops us from using some of our investment vehicles.
//!
//! [restart_investment_vehicles()][blueprint::Radfolio::restart_investment_vehicles]
//! Restarts investment vehicles that were halted.
//!
//! [read_halted_investment_vehicles()][blueprint::Radfolio::read_halted_investment_vehicles]
//! Retrieves a list of investment vehicles that are currently halted.
//!
//! ## Partner List Management
//!
//! [set_allow_any_partner()][blueprint::Radfolio::set_allow_any_partner]
//! Change whether to use a partner whitelist.
//!
//! [read_allow_any_partner()][blueprint::Radfolio::read_allow_any_partner]
//! Check whether we're currently using a partner whitelist.
//!
//! [add_approved_partners()][blueprint::Radfolio::add_approved_partners]
//! Add partners to the whitelist.
//!
//! [remove_approved_partners()][blueprint::Radfolio::remove_approved_partners]
//! Remove partners from the whitelist.
//!
//! [clear_approved_partners()][blueprint::Radfolio::clear_approved_partners]
//! Remove all partners from the whitelist.
//!
//! [read_approved_partners()][blueprint::Radfolio::read_approved_partners]
//! Retrieve the partner whitelist.
//!
//! [is_partner_approved()][blueprint::Radfolio::is_partner_approved]
//! Check if someone is currently allowed to be a partner.
//!
//! ## Other Fund Configuration
//!
//! [set_free_funds_target_percent()][blueprint::Radfolio::set_free_funds_target_percent]
//! Adjust the amount of immediate liquidity to keep around.
//!
//! [set_investment_update_interval_epochs()][blueprint::Radfolio::set_investment_update_interval_epochs]
//! Adjust the interval between forced fund maintenance cycles.
//!
//! [set_minimum_deposit()][blueprint::Radfolio::set_minimum_deposit]
//! Adjust the minimum deposit accepted.
//!
//! [set_deposit_fee_partner_bps()][blueprint::Radfolio::set_deposit_fee_partner_bps]
//! Adjust the partner deposit fee.
//!
//! [set_withdraw_fee_partner_bps()][blueprint::Radfolio::set_withdraw_fee_partner_bps]
//! Adjust the partner withdraw fee.
//!
//! ## Other Data Retrieval
//!
//! [read_total_funds()][blueprint::Radfolio::read_total_funds]
//! Returns the total current value of the fund.
//!
//! [read_total_coupons()][blueprint::Radfolio::read_total_coupons]
//! Returns how many coupon tokens are currently in existence.
//!
//! [read_investment_token()][blueprint::Radfolio::read_investment_token]
//! Returns the token that is being invested into the fund.
//!
//! [read_coupon_address()][blueprint::Radfolio::read_coupon_address]
//! Returns the address of our coupon tokens.
//!
//! [read_admin_badge_address()][blueprint::Radfolio::read_admin_badge_address]
//! Returns the address of our admin badge tokens.
//!
//! [read_mint_badge_address()][blueprint::Radfolio::read_mint_badge_address]
//! Returns the address of our coupon minting badge.
//!
//! [read_iv_control_badge_address()][blueprint::Radfolio::read_iv_control_badge_address]
//! Returns the address of our investment vehicle control badge.
//!
//! [read_participants_nft_address()][blueprint::Radfolio::read_participants_nft_address]
//! Returns the address of our Participants catalog.
//!
//! [read_investments()][blueprint::Radfolio::read_investments]
//! Returns all our investment vehicles and their weights.
//!
//! [read_free_funds()][blueprint::Radfolio::read_free_funds]
//! Returns how many free funds (i.e. immediate liquidity) are in the fund.
//!
//! [read_free_funds_target_percent()][blueprint::Radfolio::read_free_funds_target_percent]
//! Returns the free funds level we are aiming at.
//!
//! [read_investment_update_interval_epochs()][blueprint::Radfolio::read_investment_update_interval_epochs]
//! Returns the interval between forced fund maintenance cycles.
//!
//! [read_last_update_epoch()][blueprint::Radfolio::read_last_update_epoch]
//! Returns the last epoch that we had a full maintenance cycle.
//!
//! [read_minimum_deposit()][blueprint::Radfolio::read_minimum_deposit]
//! Returns the minimum deposit we acccept.
//!
//! [read_deposit_fee_bps()][blueprint::Radfolio::read_deposit_fee_bps]
//! Returns the protocol deposit fee (in basis points).
//!
//! [read_deposit_fee_partner_bps()][blueprint::Radfolio::read_deposit_fee_partner_bps]
//! Returns the partner deposit fee (in basis points).
//!
//! [read_withdraw_fee_bps()][blueprint::Radfolio::read_withdraw_fee_bps]
//! Returns the protocol withdraw fee (in basis points).
//!
//! [read_withdraw_fee_partner_bps()][blueprint::Radfolio::read_withdraw_fee_partner_bps]
//! Returns the partner withdraw fee (in basis points).
//!
//! # Main Features
//!
//! Radfolio has a number of configuration options that allow you to
//! tailor it into exactly the kind of investment fund you want to
//! run. Many configuration options accept `None` as their value,
//! allowing you to opt out of them. Others interact with eachother in
//! interesting ways, and of course in some cases you may run into
//! combinations of options that simply don't make sense.
//!
//! We will go through this in some detail in the following sections.
//!
//! ## Investment Token
//!
//! The fund accepts a single type of token for deposits, and you can
//! select which token that is when creating the fund. While we can
//! easily see this being XRD it could just as well be a different
//! token, maybe a stablecoin, etc. Rewards are expected to be in this
//! same token.
//!
//! Note that it is possible for the fund's investment vehicles to
//! invest into services that give other types of token as rewards but
//! it is then the job of the investment vehicle implementation to
//! convert those rewards into the fund's chosen token before they can
//! be passed back into Radfolio. Since investment vehicles are
//! largely black boxes to Radfolio they can shield us from all manner
//! of real world complexity when implemented well: *we* just see
//! token goes in and more token comes out.
//!
//! ## Partners and Participant Catalogs
//!
//! If you set up partner fees then it will be possible for
//! influencers and such to hook their followers up to your fund with
//! the partner's id tagging along so that they can receive a reward
//! for sending some business your way. (Exactly how to propagate the
//! partner's id from their web site or podcast or whatever to a
//! transaction manifest is a problem for front-end developers to
//! solve.)
//!
//! If you wish to use partnership programs to draw customers in this
//! way you will want to set up (or hook up with) a Participants
//! catalog that those partners use for identification.
//!
//! The Participants blueprint was introduced in the Demifi submission
//! to the Scrypto lending challenge and its source code can be found
//! there. A `demifi.wasm` has been included in the `tests` directory
//! of this source distribution and you can use it for testing
//! etc. You will find the minimal selection of transaction manifests
//! needed for this in the `rtm/participants` directory, and may refer
//! to the test code in `tests/lib.rs` for examples of how to use
//! them.
//!
//! ## Protocol Fees and Partner Fees
//!
//! Radfolio allows you to set protocol fees when instantiating it. In
//! order to provide stability for your investors it is not possible
//! to change the protocol fees after creation[^fee]. You can set one
//! fee on deposits and another fee on withdrawals; and if you're
//! feeling generous you can forego either or both.
//!
//! You can also set partner fees in both of these categories. When a
//! partner fee is active this is *not* charged on top of the protocol
//! fee, but it is taken out of the protocol fee. (The only exception
//! being when you have set the protocol fee to zero.) It follows that
//! your partner fees must be equal to or less than your protocol
//! fees.
//!
//! This ensures that the cost to the investor is the same whether he
//! has been referred to you or not, which will tend to make your
//! partnership program more attractive to influencers and such.
//!
//! Do note that if you have set zero protocol fees but put in partner
//! fees then this is handled slightly differently: referred investors
//! now *do* pay more than others, and also you will not be able to
//! change your partner fees.
//!
//! [^fee]: If we do a future version where the admin badge comes
//! under the control of a DAO (once we've figured out how to do them
//! properly) it may become more palatable to allow the protocol fee
//! to be changed.
//!
//! ## Free Funds Target
//!
//! It is healthy for a fund to have some free liquidity at all times,
//! and in the case of Radfolio this free liquidity may be the only
//! readily available source of funds for investors who want to cash
//! in their coupons. You will want to ensure that your free funds are
//! at a level not so big it impacts overall APY but not so small that
//! investors are finding it difficult to exit. You can dynamically
//! adjust this setting so you could set it high if you know many
//! investors are looking to exit, and lower in periods where you do
//! not anticipate many withdrawals.
//!
//! You set a target for your free funds in terms of how many percent
//! of the fund's total value they should be. So for example if your
//! fund has 1M XRD in it and your free funds target is 10%, it will
//! try to keep 100k XRD around at all times.
//!
//! The actual free funds will tend to vary around this number, but
//! there are mechanisms in place that automatically seek to adjust it
//! if falls below half of this, or if it grows beyond double.
//!
//! ## Fund Maintenance Cycles
//!
//! Every now and then the fund will need to throw its capital around
//! a bit to readjust its investments. You can set an interval for
//! this to happen in, and if that many epochs have passed without
//! maintenance taking place one will be run on the next `withdraw` or
//! `deposit` call to the fund.
//!
//! Additionally, a toned-down maintenance cycle is automatically run
//! after any `withdraw` call that brings free funds below half their
//! target. This toned-down cycle only attempts to free up funds, not
//! to put new funds into any investment vehicles.
//!
//! If a `deposit` call causes free funds to soar to twice or more
//! their target a full maintenance is run.
//!
//! And you can force a full maintenance cycle to run by calling
//! [force_fund_maintenance].
//!
//! A full maintenance cycle is expected to be potentially a very
//! expensive operation. Perhaps a future version of Radfolio will
//! find a way to subsidize this cost for any investor unlucky enough
//! to have his `deposit` or `withdraw` call trigger one.
//!
//! [force_fund_maintenance]: blueprint::Radfolio::force_fund_maintenance
//!
//! ## Minimum Deposit
//!
//! You can set a minimum deposit and if you do then any call to the
//! `deposit` method that is below this will be rejected.
//!
//! ## Coupons
//!
//! Instead of having internal accounting of who is owed what, the
//! fund uses a coupon system for tracking ownership of the fund: if
//! you own coupons then you own part of the fund. These coupons will
//! increase in value as the fund generates profits, or fall in value
//! as the fund incurs losses.
//!
//! Coupons are fungible tokens and can be freely traded outside of
//! the fund. Secondary markets may well develop. Maybe buying your
//! coupons becomes one of many investment strategies for someone
//! else's hedge fund.
//!
//! Radfolio automatically mints and burns coupon tokens in response
//! to investors depositing and withdrawing funds. This is authorized
//! through a separate minting badge which is held internally to the
//! Radfolio component.
//!
//! ## Investment Vehicles
//!
//! Our investment vehicles is where the magic happens: They make
//! money for us!
//!
//! Seen from the perspective of the Radfolio component, investment
//! vehicles are black boxes (albeit black boxes with a well-defined
//! interface we can use) that you can put funds into, and they
//! generate profits they give back to you.
//!
//! Underlying each investment vehicle will be some money-making
//! strategy that reaches out into the wider ledger in an attempt to
//! turn coins into more coins. One investment vehicle might be making
//! micro-finance loans, another might be staking coin towards a
//! validator, a third might be putting funds into liquidity pools, a
//! fourth has found some brilliant strategy for posting radit
//! messages that give mad returns, etc.
//!
//! Once you have found a set of investment vehicles you think is
//! perfect for your fund you need to decide how much of the fund
//! should go into each, and add them to Radfolio with weights
//! according to your priorities.
//!
//! When distributing funds to the different investment vehicles,
//! Radfolio will do so according to the weight of each. So if you
//! have four vehicles with weights 1, 5, 5 and 10 they will receive
//! (1/21), (5/21), (5/21) and (10/21) of the funding respectively.
//!
//! Your weighting should be based on the APYs and risk profiles of
//! the different investment vehicles, according to the strategy you
//! have chosen for your fund. Your weighting can be dynamic in that
//! you can change the weight of any investment vehicle at any
//! time. This allows you a great degree of control in the face of an
//! ever-changing market.
//!
//! ### Trouble-shooting Investment Vehicles
//!
//! Investment vehicles are components outside of Radfolio itself,
//! possibly developed by third parties. They may have complex
//! internal logic for handling their funds, and you may get into a
//! situation where an investment vehicle has started misbehaving.
//!
//! If you need to decouple an investment vehicle from Radfolio there
//! are two main steps to doing so:
//!
//! First you can halt the investment vehicle. This skips it in the
//! majority of Radfolio logic, most importantly it will not be
//! allocated any further funding. You can keep it halted until the
//! problem with the component has been sorted out, and then restart
//! it again.
//!
//! If halting it doesn't sufficiently insulate Radfolio from the
//! problem then you can remove the investment vehicle from the fund
//! entirely. This completely decouples it and if you cannot later
//! re-add it you will need to develop a strategy for recovering any
//! funds still left in it.
//!
//! Note that any funds that are stuck inside an investment vehicle
//! that you have removed from the fund *will not* count towards the
//! value of your fund. This means that removing a vehicle with funds
//! inside of it will reduce the value of your coupons, which might
//! cause unease among your investors.
//!
//! ### Taking an Investment Vehicle Out Of Use
//!
//! If you're discontinuing an investment vehicle you will want to
//! recover any funds still in it before you remove it from
//! Radfolio. The recommended approach for this is to start by setting
//! its weight to zero, then wait for (or force) a full maintenance
//! cycle. This will recover as much funds as is possible from it and
//! you can then remove it.
//!
//! Some investment vehicles may have long delays in recovering funds
//! (e.g. a vehicle that stakes its funds towards a validator node
//! will take two weeks to recover them) and in this case you may want
//! to keep it around at zero weight until all funds have been
//! returned before removing it.
//!
//! Note that an investment vehicle that has been removed from the
//! fund can be re-added later, either to put it back into use again
//! or perhaps to recover more funds from it.


use scrypto::prelude::*;

blueprint! {
    struct Radfolio {
        /// This is the token we're investing.
        investment_token: ResourceAddress,

        /// These are the coupon tokens we use.
        coupon_address: ResourceAddress,

        /// Admin badges are returned to whoever created us and are
        /// used for calling restricted methods.
        admin_badge_address: ResourceAddress,

        /// This is the Participants catalog we use to track partners.
        participants_nft_address: Option<ResourceAddress>,

        /// Our investment vehicles are listed here, each with the
        /// investment weight to apply to that vehicle.
        investments: HashMap<ComponentAddress, Decimal>,

        /// These investment vehicles are temporarily taken out of
        /// use. They remain in `investments` but get skipped in most
        /// of our logic.
        halted_investments: HashSet<ComponentAddress>,

        /// Available funds for investing or returning to investors.
        free_funds: Vault,

        /// We try to keep our `free_funds` at this level relative to
        /// the total funds we have under management.
        free_funds_target_percent: Decimal,

        /// Every this often we automatically run a full maintenance
        /// cycle of the fund, in response to a deposit or withdraw
        /// action.
        investment_update_interval_epochs: u64,

        /// The last time we ran a full maintenance cycle, whether
        /// forced or automatic.
        last_update_epoch: u64,

        /// We don't accept deposits below this value.
        minimum_deposit: Decimal,

        /// The protocol fee to charge on deposits.
        deposit_fee_bps: Option<Decimal>,

        /// The partner fee to offer on deposits.
        deposit_fee_partner_bps: Option<Decimal>,

        /// The protocol fee to charge on withdrawals.
        withdraw_fee_bps: Option<Decimal>,

        /// The partner fee to offer on withdrawals.
        withdraw_fee_partner_bps: Option<Decimal>,

        /// The badge we use to mint and burn our coupons. It only
        /// exists in this vault.
        mint_badge: Vault,

        /// The badge we use to control the investment vehicles. It
        /// only lives in this vault.
        ///
        /// (We can't use the `mint_badge` for this since we don't
        /// want to give investment vehicles the power to mint and
        /// burn coupons.)
        iv_control_badge: Vault,

        /// Protocol fees collected.
        fees: Vault,

        /// Partner fees collected.
        partner_fees: HashMap<NonFungibleId, Vault>,

        /// Whether to allow anyone to be a partern.
        allow_any_partner: bool,

        /// The partners to allow (if we don't allow everyone).
        approved_partners: HashSet<NonFungibleId>,
    }

    impl Radfolio {

        /// Creates a new Radfolio, returning to you any admin badges
        /// that were created in the process.
        ///
        /// Will panic if it detects errors in the input parameters.
        ///
        /// Refer to [main module documentation][crate::radfolio] for
        /// an overview of parameters not explained here and how they
        /// relate to each other.
        ///
        /// `investment_token` is the token we're managing investments
        /// of, e.g. XRD.
        ///
        /// `participants_nft_address` is the Participants catalog we
        /// use to manage our partners. If you will not be using
        /// partners you can set it to `None`.
        ///
        /// You can specify your own strings for `admin_badge_name`,
        /// `coupon_name`, `mint_badge_name` and
        /// `iv_control_badge_name` if you do not like the
        /// defaults. Note that of these `coupon_name` is the one that
        /// is most visible to your investors since they will all have
        /// coupons.
        ///
        /// You can control the quantity of admin badges you will
        /// receive with `admin_badge_quantity`. You may want several
        /// if you have a large admin team etc. It does not seem
        /// useful to ask for zero badges but it is supported.
        ///
        /// (If you study the function signature carefully you may
        /// notice that it returns two buckets. One contains the
        /// admin badges and the other will always be empty.)
        ///
        /// ---
        ///
        /// **Access control:** Anyone can instantiate radfolio.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/instantiate_radfolio.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/instantiate_radfolio.rtm")]
        /// ```
        pub fn instantiate_radfolio(
            investment_token: ResourceAddress,
            participants_nft_address: Option<ResourceAddress>,
            free_funds_target_percent: Decimal,
            investment_update_interval_epochs: u64,
            minimum_deposit: Decimal,
            admin_badge_name: Option<String>,
            admin_badge_quantity: u64,
            coupon_name: Option<String>,
            deposit_fee_bps: Option<Decimal>,
            deposit_fee_partner_bps: Option<Decimal>,
            withdraw_fee_bps: Option<Decimal>,
            withdraw_fee_partner_bps: Option<Decimal>,
            mint_badge_name: Option<String>,
            iv_control_badge_name: Option<String>,
        ) -> (ComponentAddress, ResourceAddress, Bucket, ResourceAddress, Bucket)
        {
            Radfolio::assert_minimum_deposit(minimum_deposit);
            assert!(participants_nft_address.is_some()
                    || deposit_fee_partner_bps.is_none() && withdraw_fee_partner_bps.is_none(),
                    "A Participants catalog must be specified when using partner fees");
            Radfolio::assert_fee_and_partner_fee(deposit_fee_bps, deposit_fee_partner_bps,
                                                 "deposit");
            Radfolio::assert_fee_and_partner_fee(withdraw_fee_bps, withdraw_fee_partner_bps,
                                                 "withdraw");
            Radfolio::assert_free_funds_target_percent(free_funds_target_percent);

            // The admin_badge is used for controlling our investment
            // portfolio and parameters, and for triggering fund
            // management
            let admin_badges = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", admin_badge_name.unwrap_or(
                    "Radfolio admin badge".to_string()))
                .initial_supply(admin_badge_quantity);
            let admin_res = admin_badges.resource_address();

            // This is kept in a bucket in self, for automatic minting
            // and burning of coupon tokens.
            let mint_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", mint_badge_name.unwrap_or(
                    "Radfolio coupon mint badge".to_string()))
                .initial_supply(1);

            // This is kept in a bucket in self, for manipulating our
            // investment vehicles.
            let iv_control_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", iv_control_badge_name.unwrap_or(
                    "Radfolio investment vehicle control badge".to_string()))
                .initial_supply(1);

            // These coupons represent one's investment into the
            // service and also any gains or losses accrued.
            let coupons = ResourceBuilder::new_fungible()
                .mintable(rule!(require(mint_badge.resource_address())), LOCKED)
                .burnable(rule!(require(mint_badge.resource_address())), LOCKED)
                .metadata("name", coupon_name.unwrap_or(
                    "Radfolio coupon".to_string()))
                .initial_supply(0);

            let radfolio = 
                Self {
                    investment_token,
                    coupon_address: coupons.resource_address(),
                    admin_badge_address: admin_badges.resource_address(),
                    participants_nft_address,
                    investments: HashMap::new(),
                    halted_investments: HashSet::new(),
                    free_funds: Vault::new(investment_token),
                    free_funds_target_percent,
                    investment_update_interval_epochs,
                    last_update_epoch: 0,
                    minimum_deposit,
                    deposit_fee_bps,
                    deposit_fee_partner_bps,
                    withdraw_fee_bps,
                    withdraw_fee_partner_bps,
                    mint_badge: Vault::with_bucket(mint_badge),
                    iv_control_badge: Vault::with_bucket(iv_control_badge),
                    fees: Vault::new(investment_token),
                    partner_fees: HashMap::new(),
                    allow_any_partner: false,
                    approved_partners: HashSet::new(),
                }
            .instantiate()
                .add_access_check(
                    AccessRules::new()
                    // In order to stay on the safe side we default to
                    // requiring the admin badge, and individually
                    // specify those methods that are either available
                    // to all or have custom access control. This way
                    // we won't accidentally leave sensitive methods
                    // open.
                        .default(rule!(require(admin_res)))
                        .method("read_approved_partners", rule!(allow_all))
                        .method("is_partner_approved", rule!(allow_all))
                        .method("read_allow_any_partner", rule!(allow_all))
                        .method("deposit", rule!(allow_all))
                        // withdraw requires you to provide coupon
                        // tokens
                        .method("withdraw", rule!(allow_all))
                        .method("value_of_coupons", rule!(allow_all))
                        .method("read_total_funds", rule!(allow_all))
                        .method("read_total_coupons", rule!(allow_all))
                        .method("read_investment_token", rule!(allow_all))
                        .method("read_coupon_address", rule!(allow_all))
                        .method("read_admin_badge_address", rule!(allow_all))
                        .method("read_participants_nft_address", rule!(allow_all))
                        .method("read_investments", rule!(allow_all))
                        .method("read_free_funds", rule!(allow_all))
                        .method("read_free_funds_target_percent", rule!(allow_all))
                        .method("read_investment_update_interval_epochs", rule!(allow_all))
                        .method("read_last_update_epoch", rule!(allow_all))
                        .method("read_minimum_deposit", rule!(allow_all))
                        .method("read_deposit_fee_bps", rule!(allow_all))
                        .method("read_deposit_fee_partner_bps", rule!(allow_all))
                        .method("read_withdraw_fee_bps", rule!(allow_all))
                        .method("read_withdraw_fee_partner_bps", rule!(allow_all))
                        .method("read_mint_badge_address", rule!(allow_all))
                        .method("read_iv_control_badge_address", rule!(allow_all))
                        .method("read_fees_stored", rule!(allow_all))
                        .method("read_partner_fees_stored", rule!(allow_all))
                        .method("read_investment_vehicles", rule!(allow_all))
                        .method("read_halted_investment_vehicles", rule!(allow_all))
                        // withdraw_partner_fees requires you to
                        // provide a Proof
                        .method("withdraw_partner_fees", rule!(allow_all))
                ).globalize();

            (
                radfolio,
                admin_badges.resource_address(),
                admin_badges,
                coupons.resource_address(),
                coupons,
            )
        }

        /// Sets whether to allow everyone to be a partner, or to
        /// restrict it to a list of approved partners. Pass `true` to
        /// this method to allow everyone.
        ///
        /// So long as everyone is allowed our `approved_partners`
        /// list is effectively not in use. You may however still add
        /// and remove partners on it, e.g. in expectation of wanting
        /// to turn off `allow_any_partner` at some point.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_allow_any_partner.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_allow_any_partner.rtm")]
        /// ```
        pub fn set_allow_any_partner(&mut self, allow: bool) {
            self.allow_any_partner = allow;
        }

        /// Returns our `allow_any_partner` setting.
        ///
        /// Note that this list can be temporarily overridden by the
        /// `allow_any_partner` setting.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_allow_any_partner.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_allow_any_partner.rtm")]
        /// ```
        pub fn read_allow_any_partner(&self) -> bool {
            self.allow_any_partner
        }

        /// Adds one or more partners to our list of approved partners.
        ///
        /// Note that this list can be temporarily overridden by the
        /// `allow_any_partner` setting.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/add_approved_partners.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/add_approved_partners.rtm")]
        /// ```
        pub fn add_approved_partners(&mut self, add: HashSet<NonFungibleId>) {
            self.approved_partners.extend(add);
        }

        /// Removes one or more partners from our list of approved
        /// partners.
        ///
        /// Note that this list can be temporarily overridden by the
        /// `allow_any_partner` setting.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/remove_approved_partners.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/remove_approved_partners.rtm")]
        /// ```
        pub fn remove_approved_partners(&mut self, remove: HashSet<NonFungibleId>) {
            self.approved_partners.retain(|p| !remove.contains(p));
        }

        /// Removes everyone from our list of approved partners.
        ///
        /// Note that this list can be temporarily overridden by the
        /// `allow_any_partner` setting.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/clear_approved_partners.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/clear_approved_partners.rtm")]
        /// ```
        pub fn clear_approved_partners(&mut self) {
            self.approved_partners.clear();
        }

        /// Retrieves our list of approved partners.
        ///
        /// Note that this list can be temporarily overridden by the
        /// `allow_any_partner` setting.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_approved_partners.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_approved_partners.rtm")]
        /// ```
        pub fn read_approved_partners(&self) -> HashSet<NonFungibleId> {
            self.approved_partners.clone()
        }

        /// Determines if a given partner is currently approved by
        /// Radfolio.
        ///
        /// If the `allow_any_partner` setting is `true` then all
        /// partners are considered to be approved. Otherwise a
        /// partner is only approved if it is in our
        /// `approved_partners` list.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/is_partner_approved.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/is_partner_approved.rtm")]
        /// ```
        pub fn is_partner_approved(&self, candidate: NonFungibleId) -> bool {
            self.allow_any_partner || self.approved_partners.contains(&candidate)
        }

        /// Deposits funds into Radfolio, returning a number of
        /// coupons representing the resulting share of ownership of
        /// the fund.
        ///
        /// Will panic if you try to deposit the wrong type of token,
        /// if you're depositing too few tokens, and under various
        /// other error conditions.
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone sending us
        /// tokens of the correct type
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/deposit.rtm")]
        /// ```
        pub fn deposit(&mut self, mut funds: Bucket, partner: Option<NonFungibleId>) -> Bucket  {
            assert!(funds.resource_address() == self.investment_token,
                    "Wrong token type sent");
            assert!(funds.amount() >= self.minimum_deposit,
                    "Send at least the minimum {} token deposit", self.minimum_deposit);
            assert!(partner.is_none()
                    || self.allow_any_partner
                    || self.approved_partners.contains(partner.as_ref().unwrap()),
                    "The partner is not approved");

            self.charge_fees(self.deposit_fee_bps, self.deposit_fee_partner_bps, &mut funds, &partner);
            
            let cmgr: &ResourceManager = borrow_resource_manager!(self.coupon_address);

            // We mint a number of new coupons equal to the value of
            // the deposit. The value of each coupon is set to the
            // total value of the investment fund divided by the
            // number of coupons already minted - all prior to this
            // new deposit.
            let total = self.calc_total_funds();
            let mint_q = if total.is_zero()
            { funds.amount() } else { (cmgr.total_supply() / total ) * funds.amount()};
            let coupons: Bucket = self.mint_badge.authorize(|| cmgr.mint(mint_q));

            self.free_funds.put(funds);
            self.maintain_fund(false); 

            coupons
        }

        /// Withdraws funds from Radfolio by depositing ownership
        /// coupons. The coupons will be burnt and funds returned.
        ///
        /// Will panic if insufficient free funds are available. In
        /// this case, try with a smaller number of coupons and/or
        /// wait for more free funds to become available.
        ///
        /// Will also panic if you try to deposit the wrong coupon
        /// type and under various error conditions.
        ///
        /// ---
        ///
        /// **Access control:** Can be called by anyone sending us
        /// coupons of the correct type
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/withdraw.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/withdraw.rtm")]
        /// ```
        pub fn withdraw(&mut self, coupons: Bucket, partner: Option<NonFungibleId>) -> Bucket {
            assert!(coupons.resource_address() == self.coupon_address,
                    "Wrong coupon type");
            assert!(partner.is_none()
                    || self.allow_any_partner
                    || self.approved_partners.contains(partner.as_ref().unwrap()),
                    "The partner is not approved");

            self.recover_profits_from_ivs();
            let cmgr: &ResourceManager = borrow_resource_manager!(self.coupon_address);
            // We receive a number of tokens proportional to our
            // ownership% in the coupon tokens.
            //
            // Note that if free_funds does not have sufficient tokens
            // then this call fails and the user needs to wait for
            // free_funds to refill, possibly making a smaller
            // withdrawal in the meantime.
            let mut bucket_out = self.free_funds.take(self.value_of(coupons.amount(), cmgr));
            self.charge_fees(self.withdraw_fee_bps, self.withdraw_fee_partner_bps,
                             &mut bucket_out, &partner);
            self.mint_badge.authorize(||  {
                coupons.burn();
            });

            self.maintain_fund(false);

            bucket_out
        }

        /// Can be called by a partner to withdraw the partner fees
        /// accrued by them.
        ///
        /// This can be called even if a partner is not currently in
        /// our `approved_partners` list. (Presumably they were in
        /// that list back when they earned the fees.)
        ///
        /// Note that the proof passed in can be for multiple partners
        /// in which case the fees of them all will be returned.
        ///
        /// Will panic if no `participants_nft_address` has been set
        /// or if the partner proof isn't valid.
        ///
        /// ---
        ///
        /// **Access control:** Will withdraw any fees accrued for the
        /// partners in the Proof.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/withdraw_partner_fees.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/withdraw_partner_fees.rtm")]
        /// ```
        pub fn withdraw_partner_fees(&mut self, partner: Proof) -> Vec<Bucket> {
            assert_eq!(
                partner.resource_address(),
                self.participants_nft_address.unwrap(),
                "Unsupported participant NFT"
            );

            let mut fees : Vec<Bucket> = Vec::new();
            for nfid in partner.non_fungible_ids() {
                if let Some(vault) = self.partner_fees.get_mut(&nfid) { fees.push(vault.take_all()) }
            }

            fees
        }

        /// Withdraws any accrued protocol fees.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/withdraw_protocol_fees.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/withdraw_protocol_fees.rtm")]
        /// ```
        pub fn withdraw_protocol_fees(&mut self) -> Bucket {
            self.fees.take_all()
        }

        /// Calculates the current value of some number of coupons,
        /// measured in units of the `investment_token`.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/value_of_coupons.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/value_of_coupons.rtm")]
        /// ```
        pub fn value_of_coupons(&self, amount: Decimal) -> Decimal {
            let cmgr: &ResourceManager = borrow_resource_manager!(self.coupon_address);
            self.value_of(amount, cmgr)
        }

        /// Calculates the current total funding within Radfolio,
        /// including free funds and the investment values reported by
        /// all our investment vehicles.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_total_funds.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_total_funds.rtm")]
        /// ```
        pub fn read_total_funds(&self) -> Decimal {
            self.calc_total_funds()
        }

        /// Calculates how many coupons are currently in existence.
        /// Each coupon represents a share of ownership of the fund.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_total_coupons.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_total_coupons.rtm")]
        /// ```
        pub fn read_total_coupons(&self) -> Decimal {
            let cmgr: &ResourceManager = borrow_resource_manager!(self.coupon_address);
            cmgr.total_supply()
        }

        /// Returns our `investment_token`.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_investment_token.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_investment_token.rtm")]
        /// ```
        pub fn read_investment_token(&self) -> ResourceAddress {
            self.investment_token
        }

        /// Returns the address of our ownership coupons.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_coupon_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_coupon_address.rtm")]
        /// ```
        pub fn read_coupon_address(&self) -> ResourceAddress {
            self.coupon_address
        }

        /// Returns the address of our admin badges.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_admin_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_admin_badge_address.rtm")]
        /// ```
        pub fn read_admin_badge_address(&self) -> ResourceAddress {
            self.admin_badge_address
        }

        /// Returns the address of or Participants catalog, if any.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_participants_nft_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_participants_nft_address.rtm")]
        /// ```
        pub fn read_participants_nft_address(&self) -> Option<ResourceAddress> {
            self.participants_nft_address
        }

        /// Returns a map with investment vehicle addresses as keys
        /// and the investment value (measured in units of the
        /// `investment_token`) of each vehicle as values.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_investments.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_investments.rtm")]
        /// ```
        pub fn read_investments(&self) -> HashMap<ComponentAddress, Decimal> {
            let mut investments = HashMap::new();
            for iv in self.investments.keys() {
                investments.insert(iv.clone(),
                                   self.iv_read_investment_value(iv));
            }

            investments
        }

        /// Returns the amount of free funds currently held. These are
        /// funds that aren't bound into any particular investment,
        /// but instead are available for withdrawals and which may be
        /// used for future investments.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_free_funds.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_free_funds.rtm")]
        /// ```
        pub fn read_free_funds(&self) -> Decimal {
            self.free_funds.amount()
        }

        /// Returns the desired percentage of free funds to total fund
        /// value. Actual free funds will typically fluctuate around
        /// this target value.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_funds_target_percent.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_free_funds_target_percent.rtm")]
        /// ```
        pub fn read_free_funds_target_percent(&self) -> Decimal {
            self.free_funds_target_percent
        }

        /// Returns how many epochs must pass after a full maintenance
        /// cycle before a new full maintenance will be forced.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_investment_update_interval_epochs.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_investment_update_interval_epochs.rtm")]
        /// ```
        pub fn read_investment_update_interval_epochs(&self) -> u64 {
            self.investment_update_interval_epochs
        }

        /// Returns the last epoch on which a full maintenance cycle
        /// was performed.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_last_update_epoch.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_last_update_epoch.rtm")]
        /// ```
        pub fn read_last_update_epoch(&self) -> u64 {
            self.last_update_epoch
        }

        /// Returns the minimum deposit we accept from investors.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_minimum_deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_minimum_deposit.rtm")]
        /// ```
        pub fn read_minimum_deposit(&self) -> Decimal {
            self.minimum_deposit
        }

        /// Returns the deposit fee, measured in basis points.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_deposit_fee_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_deposit_fee_bps.rtm")]
        /// ```
        pub fn read_deposit_fee_bps(&self) -> Option<Decimal> {
            self.deposit_fee_bps
        }

        /// Returns the partner deposit fee, measured in basis points.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_deposit_fee_partner_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_deposit_fee_partner_bps.rtm")]
        /// ```
        pub fn read_deposit_fee_partner_bps(&self) -> Option<Decimal> {
            self.deposit_fee_partner_bps
        }

        /// Returns the withdraw fee, measured in basis points.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_withdraw_fee_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_withdraw_fee_bps.rtm")]
        /// ```
        pub fn read_withdraw_fee_bps(&self) -> Option<Decimal> {
            self.withdraw_fee_bps
        }

        /// Returns the partner withdraw fee, measured in basis points.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_withdraw_fee_partner_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_withdraw_fee_partner_bps.rtm")]
        /// ```
        pub fn read_withdraw_fee_partner_bps(&self) -> Option<Decimal> {
            self.withdraw_fee_partner_bps
        }

        /// Returns the address of the badge we use for minting and
        /// burning coupons. The only such badge in existence is held
        /// within the component itself.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_mint_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_mint_badge_address.rtm")]
        /// ```
        pub fn read_mint_badge_address(&self) -> ResourceAddress {
            self.mint_badge.resource_address()
        }

        /// Returns the address of the badge we use for controlling
        /// our investment vehicles. The only such badge in existence
        /// is held within the component itself.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_iv_control_badge_address.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_iv_control_badge_address.rtm")]
        /// ```
        pub fn read_iv_control_badge_address(&self) -> ResourceAddress {
            self.iv_control_badge.resource_address()
        }

        /// Reports how many protocol fees are currently sitting in
        /// the component, waiting to be collected.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_fees_stored.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_fees_stored.rtm")]
        /// ```
        pub fn read_fees_stored(&self) -> Decimal {
            self.fees.amount()
        }

        /// Reports how many partner fees are sitting in the component
        /// waiting to be collected.
        ///
        /// The returned map maps from partner nfid to amount of fees
        /// accrued to that partner.
        ///
        /// Partners that have zero fees accrued are not included in
        /// the map.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_partner_fees_stored.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_partner_fees_stored.rtm")]
        /// ```
        pub fn read_partner_fees_stored(&self, nfids: Option<HashSet<NonFungibleId>>)
                                        -> HashMap<NonFungibleId, Decimal>
        {
            let mut amounts: HashMap<NonFungibleId, Decimal> = HashMap::new();
            let nfids = nfids.as_ref();
            for (nfid, vault) in &self.partner_fees {
                if !vault.amount().is_zero()
                    && (nfids.is_none() || nfids.unwrap().contains(nfid))
                {
                    amounts.insert(nfid.clone(), vault.amount());
                }
            }
            amounts
        }

        /// Adds an investment vehicle to the fund, with the weight given.
        ///
        /// Will panic if we already have that exact investment vehicle.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/add_investment_vehicle.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/add_investment_vehicle.rtm")]
        /// ```
        pub fn add_investment_vehicle(&mut self,
                                      vehicle: ComponentAddress, weight: Decimal) {
            assert!(self.investments.insert(vehicle, weight).is_none(),
                    "We already use this investment vehicle");
        }

        /// Changes the weight of an investment vehicle.
        ///
        /// Will panic if we do not currently have this investment vehicle.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/modify_investment_vehicle.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/modify_investment_vehicle.rtm")]
        /// ```
        pub fn modify_investment_vehicle(&mut self,
                                         vehicle: ComponentAddress, weight: Decimal) {
            assert!(self.investments.insert(vehicle, weight).is_some(),
                    "We do not use this investment vehicle");
        }

        /// Removes one or more investment vehicles from the fund.
        ///
        /// Note that if there are funds still inside those investment
        /// vehicles, such funds will no longer be collected by
        /// Radfolio. You need to have an alternative strategy for
        /// rescuing such funds.
        ///
        /// Investment vehicles that are removed will also be removed
        /// from the list of halted investment vehicles, if they were
        /// there.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/remove_investment_vehicles.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/remove_investment_vehicles.rtm")]
        /// ```
        pub fn remove_investment_vehicles(&mut self, vehicles: HashSet<ComponentAddress>) {
            self.investments.retain(|k, _| !vehicles.contains(k));
            self.halted_investments.retain(|k| !vehicles.contains(k));
        }
        
        /// Removes ALL investment vehicles from the fund.
        ///
        /// Note that if there are funds still inside those investment
        /// vehicles, such funds will no longer be collected by
        /// Radfolio. You need to have an alternative strategy for
        /// rescuing such funds.
        ///
        /// The list of halted investment vehicles will also be
        /// cleared.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/clear_investment_vehicles.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/clear_investment_vehicles.rtm")]
        /// ```
        pub fn clear_investment_vehicles(&mut self) {
            self.investments.clear();
            self.halted_investments.clear();
        }

        /// Retrieves a list of our current investment vhicles. This
        /// includes vehicles that are currently halted.
        ///
        /// The return value maps from investment vehicle address to
        /// the vehicle's desired weight.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_investment_vehicles.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_investment_vehicles.rtm")]
        /// ```
        pub fn read_investment_vehicles(&self) -> HashMap<ComponentAddress, Decimal> {
            self.investments.clone()
        }

        /// Halts one or more investment vehicles. A halted vehicle is
        /// excluded from most of the logic in Radfolio and will not
        /// receive any new funding.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/halt_investment_vehicles.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/halt_investment_vehicles.rtm")]
        /// ```
        pub fn halt_investment_vehicles(&mut self, vehicles: HashSet<ComponentAddress>) {
            self.halted_investments.extend(vehicles);
        }

        /// Restarts one or more investment vehicles that have been halted.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/restart_investment_vehicles.rtm.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/restart_investment_vehicles.rtm")]
        /// ```
        pub fn restart_investment_vehicles(&mut self, vehicles: HashSet<ComponentAddress>) {
            self.halted_investments.retain(|v| !vehicles.contains(v));
        }

        /// Retrieves a list of halted investment vehicles.
        ///
        /// ---
        ///
        /// **Access control:** Read only, can be called by anyone.
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/read_halted_investment_vehicles.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/read_halted_investment_vehicles.rtm")]
        /// ```
        pub fn read_halted_investment_vehicles(&self) -> HashSet<ComponentAddress> {
            self.halted_investments.clone()
        }

        /// Triggers a full fund maintenance cycle. All non-halted
        /// investment vehicles will have profits collected from them,
        /// and they will then be rebalanced towards their desired
        /// weights.
        ///
        /// This is likely to be a computationally expensive
        /// operation.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/force_fund_maintenance.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/force_fund_maintenance.rtm")]
        /// ```
        pub fn force_fund_maintenance(&mut self) {
            self.maintain_fund(true);
        }

        /// Changes the target free funds percentage. The fund will
        /// start moving towards the new target on its next
        /// maintenance cycle.
        ///
        /// Call [Radfolio::force_fund_maintenance] if you want an immediate
        /// move towards the new target.
        ///
        /// Will panic if the new percentage isn't between 0 and 100.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_free_funds_target_percent.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_free_funds_target_percent.rtm")]
        /// ```
        pub fn set_free_funds_target_percent(&mut self, target: Decimal) {
            Radfolio::assert_free_funds_target_percent(target);
            self.free_funds_target_percent = target;
        }

        /// Changes the interval between forced full maintenance cycles.
        ///
        /// This will not cause an immediate update even if the new
        /// value makes us overdue.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_investment_update_interval_epochs.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_investment_update_interval_epochs.rtm")]
        /// ```
        pub fn set_investment_update_interval_epochs(&mut self, interval: u64) {
            self.investment_update_interval_epochs = interval;
        }

        /// Changes the minimum deposit accepted by the fund.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_minimum_deposit.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_minimum_deposit.rtm")]
        /// ```
        pub fn set_minimum_deposit(&mut self, minimum_deposit: Decimal) {
            Radfolio::assert_minimum_deposit(minimum_deposit);
            self.minimum_deposit = minimum_deposit;
        }

        /// Changes the deposit fee given to our partners.
        ///
        /// Will panic if the new fee is out of bounds, or if we don't
        /// have a Participants catalog set.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_deposit_fee_partner_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_deposit_fee_partner_bps.rtm")]
        /// ```
        pub fn set_deposit_fee_partner_bps(&mut self, partner_fee: Option<Decimal>) {
            assert!(self.participants_nft_address.is_some(),
                    "A Participants catalog must be specified when using partner fees");
            assert!(self.deposit_fee_bps.is_some(),
                    "Cannot alter partner fee with no protocol fee");
            Radfolio::assert_fee_and_partner_fee(self.deposit_fee_bps, partner_fee, "deposit");
            self.deposit_fee_partner_bps = partner_fee;
        }

        /// Changes the withdraw fee given to our partners.
        ///
        /// Will panic if the new fee is out of bounds, or if we don't
        /// have a Participants catalog set.
        ///
        /// ---
        ///
        /// **Access control:** Can only be called with admin badge in auth zone
        ///
        /// **Transaction manifest:**
        /// `rtm/radfolio/set_withdraw_fee_partner_bps.rtm`
        /// ```text
        #[doc = include_str!("../rtm/radfolio/set_withdraw_fee_partner_bps.rtm")]
        /// ```
        pub fn set_withdraw_fee_partner_bps(&mut self, partner_fee: Option<Decimal>) {
            assert!(self.participants_nft_address.is_some(),
                    "A Participants catalog must be specified when using partner fees");
            assert!(self.withdraw_fee_bps.is_some(),
                    "Cannot alter partner fee with no protocol fee");
            Radfolio::assert_fee_and_partner_fee(self.withdraw_fee_bps, partner_fee, "withdraw");
            self.withdraw_fee_partner_bps = partner_fee;
        }
        

        //  ---
        //  Non-public methods follow

        /// Panics if the input value is an invalid minimum deposit.
        fn assert_minimum_deposit(minimum_deposit: Decimal) {
            assert!(!minimum_deposit.is_negative(),
                    "Minimum deposit can't be negative");
        }

        /// Panics if the input value is Some(fee) and the fee has an
        /// invalid value. The strings are used in the panic message.
        fn assert_fee_bps(fee: Option<Decimal>, actor: &str, direction: &str) {
            if fee.is_some() {
                assert!(!fee.unwrap().is_negative(),
                        "{} {} fee can't be negative", direction, actor);
                assert!(fee.unwrap() <= dec!("10000"),
                        "{} {} fee can't exceed 100%", direction, actor);
            }
        }

        /// Panics if the input values aren't valid for fee and
        /// partner fee settings. The string is used in the panic
        /// message.
        fn assert_fee_and_partner_fee(fee: Option<Decimal>, partner_fee: Option<Decimal>,
                                      direction: &str) {
            Radfolio::assert_fee_bps(fee, "", direction);
            Radfolio::assert_fee_bps(partner_fee, "partner", direction);
            if partner_fee.is_some() && fee.is_some() {
                assert!(partner_fee <= fee,
                        "Partner {0} fee cannot be larger than protocol {0} fee", direction);
            }
        }

        /// Panics if the input value isn't valid for
        /// `free_funds_target_percent`.
        fn assert_free_funds_target_percent(target: Decimal) {
            assert!(!target.is_negative(),
                    "Target free funds cannot be negative");
            assert!(target <= dec!("100"),
                    "Target free funds cannot be greater than 100%");
        }

        /// Calculates the value of a given number of coupons.
        fn value_of(&self, amount: Decimal, manager: &ResourceManager) -> Decimal {
            let total = manager.total_supply();
            if total.is_zero() { return amount; }
            else { return amount * (self.calc_total_funds() / total); }
        }
        

        /// Charges protocol fees and/or partner fees for a
        /// transaction. Fees will be taken out of `moneybag` and
        /// placed into the protocol fee vault and/or the partner's
        /// fee vault.
        ///
        /// Will panic if the partner nfid doesn't exist in our
        /// Participants catalog.
        fn charge_fees(&mut self,
                     fee: Option<Decimal>, partner_fee: Option<Decimal>,
                     moneybag: &mut Bucket,
                     partner: &Option<NonFungibleId>)
        {
            if partner.is_some() {
                self.check_participant_nfid(partner.as_ref().unwrap());
            }
            let mut fee_taken = Decimal::ZERO;
            let total_pot = moneybag.amount();
            if partner.is_some() {
                if let Some(partner_fee) = partner_fee {
                    if !self.partner_fees.contains_key(partner.as_ref().unwrap()) {
                        self.partner_fees.insert(partner.as_ref().unwrap().clone(),
                                                 Vault::new(self.investment_token));
                    }
                    fee_taken = (partner_fee / dec!("10000")) * total_pot;
                    self.partner_fees.get_mut(&partner.as_ref().unwrap()).unwrap().put(
                        moneybag.take(fee_taken));
                }
            }
            if let Some(fee) = fee {
                let my_fee: Decimal = (fee / dec!("10000")) * total_pot - fee_taken;
                if my_fee.is_positive() {
                    self.fees.put(moneybag.take(my_fee));
                }
            }
        }

        /// Calls an investment vehicle's `add_funds` method.
        ///
        /// That method is outside of the Radfolio component. Its job
        /// is to add the funds we give it to the investment that it
        /// interfaces towards.
        ///
        /// If it cannot invest everything we give it the excess funds
        /// will be returned.
        fn iv_add_funds(&self, iv: &ComponentAddress, new_funds: Bucket) -> Option<Bucket> {
            self.iv_control_badge.authorize(
                ||
                    borrow_component!(*iv).call::<Option<Bucket>>(
                        "add_funds",
                        args!(new_funds)))
        }

        /// Calls an investment vehicle's `reduce_funds` method.
        ///
        /// That method is outside of the Radfolio component. Its job
        /// is to take funds out of the investment that it interfaces
        /// towards and return them to us.
        ///
        /// We must expect that it may not always be able to do this.
        fn iv_reduce_funds(&self, iv: &ComponentAddress, by_amount: Decimal) -> Option<Bucket> {
            self.iv_control_badge.authorize(
                ||
                    borrow_component!(*iv).call::<Option<Bucket>>(
                        "reduce_funds",
                        args!(by_amount)))
        }
        
        /// Calls an investment vehicle's `withdraw_profits` method.
        ///
        /// That method is outside of the Radfolio component. Its job
        /// is to take any profits that have accrued in the investment
        /// it interfaces towards, and return them to us.
        ///
        /// Note that some types of investment may not have any
        /// accruing profits as such, but instead just automatically
        /// re-invest profits into the main investment itself.
        fn iv_withdraw_profits(&self, iv: &ComponentAddress) -> Option<Bucket> {
            self.iv_control_badge.authorize(
                ||
                    borrow_component!(*iv).call::<Option<Bucket>>(
                        "withdraw_profits",
                        Vec::new()))
        }

        /// Calls an investment vehicle's `read_investment_value` method.
        ///
        /// That method is outside of the Radfolio component. Its job
        /// is to give us a conservative estimate of how much its
        /// investment is worth in the current moment.
        fn iv_read_investment_value(&self, iv: &ComponentAddress) -> Decimal {
            borrow_component!(*iv).call::<Decimal>(
                "read_investment_value",
                Vec::new())
        }

        /// Fetches outstanding profits from all our non-halted
        /// investment vehicles and puts them into our free funds.
        fn recover_profits_from_ivs(&mut self) {
            for v in self.investments.keys() {
                if !self.halted_investments.contains(v) {
                    if let Some(profits) = self.iv_withdraw_profits(v) {
                        self.free_funds.put(profits);
                    }
                }
            }
        }

        /// Runs through a fund maintenance cycle. If a full cycle is
        /// forced or if certain criteria are met then a full
        /// maintenance is done. Otherwise we do a toned-down
        /// maintenance.
        ///
        /// A full maintenance will recover profits from investment
        /// vehicles and then rebalance them towards their configured
        /// weights.
        ///
        /// Ideally we would do this every time a deposit or withdraw
        /// happens but these are potentially very computationally
        /// expensive operations and it doesn't make sense to take
        /// that much transaction cost every single time. For this
        /// reason we only do so when at least one of these is true:
        ///
        /// - The forced flag is set (this only happens when
        /// explicitly called by an administrator).
        ///
        /// - It has been more than
        /// `investment_update_interval_epochs` epochs since last time
        /// a full maintenance was run.
        ///
        /// - Our free funds are more than twice their target number.
        ///
        /// NOTE: This strategy will need to be revisited once more is
        /// known about the transaction fee model.
        ///
        /// A toned-down maintenance will merely try to recover funds,
        /// and even this only if the current free funds are getting
        /// low.
        fn maintain_fund(&mut self, forced: bool) {
            if self.investments.len() == 0 {
                return;
            }
            // We only include the funds that are in non-halted
            // investment vehicles
            let (total_funds, iv_invested) = self.calc_iv_funds(None);
            if total_funds.is_zero() || iv_invested.len() == 0 {
                return;
            }

            let free_percent = (self.free_funds.amount() / total_funds) * 100;
            let target_free_funds = total_funds * self.free_funds_target_percent / 100;

            let mut total_weight = self.calc_total_weight();
            
            if forced
                || (Runtime::current_epoch() >=
                self.investment_update_interval_epochs + self.last_update_epoch)
                || free_percent > self.free_funds_target_percent * 2
            {
                // Run a full fund update
                // Free up profits and excess funds
                self.recover_profits_from_ivs();
                let (total_funds, iv_invested) = self.calc_iv_funds(None);

                self.trim_vehicles_to_weight(iv_invested, total_funds, total_weight);
                let (total_funds, iv_invested) = self.calc_iv_funds(None);
                
                // Add funds to bring up to weight
                let filled_ivs =
                    self.fund_vehicles(iv_invested, total_funds, total_weight, target_free_funds);
                if !filled_ivs.is_empty() {
                    // Some vehicles didn't accept the full funds
                    // passed to them, so do a second pass among the
                    // vehicles that are still accepting funds
                    for iv in &filled_ivs { total_weight -= *self.investments.get(&iv).unwrap(); }
                    if total_weight.is_positive() {
                        let (total_funds, iv_invested) = self.calc_iv_funds(Some(filled_ivs));
                        self.fund_vehicles(iv_invested, total_funds, total_weight, target_free_funds);
                    }
                    // Further passes would have seriously diminishing
                    // returns so we stop here.
                }
                
                self.last_update_epoch = Runtime::current_epoch();
            } else {
                let minimum_percent = self.free_funds_target_percent / 2;
                if free_percent < minimum_percent {
                    // Run a toned-down update attempting to free up
                    // some funds

                    let missing_free_funds = (target_free_funds / 2) - self.free_funds.amount();
                    let old_free_funds = self.free_funds.amount();

                    // First just collect any outstanding profits
                    self.recover_profits_from_ivs();
                    if total_funds.is_positive()
                        && (self.free_funds.amount() - old_free_funds) < missing_free_funds
                    {
                        // If that wasn't enough, ask investment
                        // vehicles to give up some of their funds
                        self.trim_vehicles_to_weight(iv_invested, total_funds, total_weight);
                    }
                }
            }
        }

        /// Will try to reduce the funding level of any investment
        /// vehicles that are currently above their configured weight.
        ///
        /// Do not call this method with `total_invested <= 0`.
        fn trim_vehicles_to_weight(&mut self,
                                   iv_invested: HashMap<ComponentAddress, Decimal>,
                                   total_funds: Decimal,
                                   total_weight: Decimal) {
            // Remove funds to bring down to weight
            for iv in self.investments.keys() {
                if !self.halted_investments.contains(iv) {
                    let free_funds_target = self.free_funds_target_percent * total_funds / 100;
                    let myweight = *self.investments.get(iv).unwrap();
                    let invested = *iv_invested.get(iv).unwrap();
                    let investment_target = if myweight.is_zero() { Decimal::ZERO } else {
                        (total_funds - free_funds_target)
                            * (myweight / total_weight)
                    };
                    let investment_overshoot = invested - investment_target;

                    if investment_overshoot.is_positive() {
                        if let Some(returns) = self.iv_reduce_funds(iv, investment_overshoot) {
                            self.free_funds.put(returns);
                        }
                    }
                }
            }
        }

        /// Will try to distribute current excess funding to our
        /// investment vehicles.
        ///
        /// Returns the set of investment vehicles that refused to
        /// receive the full funding we sent them. This can be used to
        /// do a second pass with only the vehicles that still accept
        /// new funding.
        ///
        /// Do not call this method with `total_weight <= 0`.
        fn fund_vehicles(&mut self,
                         iv_invested: HashMap<ComponentAddress, Decimal>,
                         total_funds: Decimal,
                         total_weight : Decimal,
                         free_funds_target: Decimal)
                         -> HashSet<ComponentAddress> {
            let mut filled_ivs = HashSet::new();

            // Figure out if we have any spare funds to invest
            let free_funds_amount = self.free_funds.amount();
            let excess_funds = free_funds_amount - free_funds_target;
            if !excess_funds.is_positive() {
                return filled_ivs;
            }

            // Now distribute excess_funds out to our vehicles
            for iv in iv_invested.keys() {
                let myweight = *self.investments.get(iv).unwrap();
                let investment_target = if myweight.is_zero() { Decimal::ZERO } else {
                    (total_funds - free_funds_amount + excess_funds)
                        * (myweight / total_weight)
                };
                let currently_invested = *iv_invested.get(iv).unwrap();
                let investment_undershoot = investment_target - currently_invested;
                if investment_undershoot.is_positive() {
                    let addition = self.free_funds.take(investment_undershoot);
                    if let Some(returns) = self.iv_add_funds(iv, addition) {
                        if !returns.amount().is_zero() {
                            filled_ivs.insert(iv.clone());
                        }
                        self.free_funds.put(returns);
                    }
                }
            }

            filled_ivs
        }

        /// Calculates the total weights configured for non-halted
        /// investment vehicles.
        fn calc_total_weight(&self) -> Decimal {
            let mut total = Decimal::ZERO;
            for (iv, weight) in &self.investments {
                if !self.halted_investments.contains(iv) {
                    total += *weight;
                }
            }
            total
        }

        /// Calculates the total funds we have across all investment
        /// vehicles. We use a conservative estimate for this.
        fn calc_total_funds(&self) -> Decimal {
            let mut total: Decimal = self.free_funds.amount();
            for iv in self.investments.keys() {
                total += self.iv_read_investment_value(iv);
            }
            total
        }

        /// Calculates the funds placed into non-halted investment
        /// vehicles.
        ///
        /// Will skip any vehicles that are present in the `exclude`
        /// set.
        ///
        /// Returns a tuple with:
        ///
        /// - The total funds in these investment vehicles.
        ///
        /// - A per-vehicle map from vehicle to funds in that vehicle.
        fn calc_iv_funds(&self, exclude: Option<HashSet<ComponentAddress>>)
                         -> (Decimal, HashMap<ComponentAddress, Decimal>) {
            let mut total: Decimal = self.free_funds.amount();
            let mut map = HashMap::new();
            for iv in self.investments.keys() {
                if !self.halted_investments.contains(iv)
                    && (exclude.is_none()
                        || !exclude.as_ref().unwrap().contains(iv))
                {
                    let value = self.iv_read_investment_value(iv);
                    total += value;
                    map.insert(iv.clone(), value);
                }
            }
            (total, map)
        }

        /// Checks that a given non-fungible id exists in our
        /// Participants catalog.
        ///
        /// Panics if it doesn't.
        ///
        /// Also panics if `participants_nft_address` hasn't been set.
        fn check_participant_nfid(&self, nfid: &NonFungibleId) {
            let nft_manager = borrow_resource_manager!(self.participants_nft_address.unwrap());
            assert!(nft_manager.non_fungible_exists(&nfid),
                    "The supplied participant id doesn't exist");
        }
        
    }
}
