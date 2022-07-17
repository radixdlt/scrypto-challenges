//! A fast test environment for the ground packages
//! 
//! For now, the GroundTestEngine can only full test the ground finance package.

use scrypto::prelude::*;
use neuracle::{neuracle::*, validator::*};
use ground_id::*;
use ground_business::{ground_business_dao::*, ground_cv::*};
use ground_finance::{ground_credit::*, ground_lending::*};

blueprint! {
    struct GroundTestEngine {

        dao_badge: Vault,
        dao_share_token: Vault,
        stable_coin: Vault,
        admin_badge: Vault,
        neura: Vault,
        neuracle: ComponentAddress,
        ground_id: ComponentAddress,
        ground_credit: ComponentAddress,
        ground_cv: ComponentAddress,
        ground_lending: Option<ComponentAddress>,
        dao: Option<ComponentAddress>,
        validators: Vec<(ComponentAddress, Vault)>,
        stakers: Vec<Vault>,
        etc_vault: LazyMap<ResourceAddress, Vault>,

    }

    impl GroundTestEngine {

        /// This function is to create a new test environment inside a testing component.
        /// 
        /// The function will also create some new testing components:
        /// 
        /// ### NeuRacle:
        /// - Validator capacity: 100,
        /// - Round length: 1 epoch,
        /// - Pay rate: 1 Neura,
        /// - Stablecoin fee: 0 (can be any cause we won't test NeuRacle's algo-stable coin mechanism),
        /// - Unstake delay: 500 epoch,
        /// - Reward rate each round: 0.0015%
        /// - Punishment mutiply: 5 times.
        /// 
        /// ### GroundId:
        /// - name: Test Component
        /// - admin_badge: The test component admin badge
        /// 
        /// ### GroundCredit:
        /// - name: Test Component
        /// - admin_badge: The test component admin badge
        /// - test_credit_scoring_rates: yearly degrade rate: 20, restore rate: 10; monthly degrade rate: 10, restore rate: 5
        /// - id_sbt: The resource address get from when instantiate the GroundID component.
        /// 
        /// ### GroundCV:
        /// - name: Test Component
        /// - id_sbt: The resource address get from when instantiate the GroundID component.
        /// - admin_badge: The test component admin badge
        pub fn new() -> ComponentAddress {

            let dao_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Test Dao Badge")
                .initial_supply(dec!(1isize));

            let dao_share_token: Bucket = ResourceBuilder::new_fungible()
                .mintable(rule!(require(dao_badge.resource_address())), LOCKED)
                .burnable(rule!(require(dao_badge.resource_address())), LOCKED)
                .metadata("name", " Test DAO Token")
                .metadata("symbol", "GRD")
                .initial_supply(12000000isize);

            let stable_coin: Bucket = ResourceBuilder::new_fungible()
                .mintable(rule!(require(dao_badge.resource_address())), LOCKED)
                .burnable(rule!(require(dao_badge.resource_address())), LOCKED)
                .metadata("name", " Test Stable Coin")
                .metadata("symbol", "GRD")
                .initial_supply(120000000isize);

            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name",  "Test Admin Badge")
                .initial_supply(dec!("1"));

            let mint_controller_badge = ResourceBuilder::new_fungible()
                .metadata("name", "NeuRacle Mint Controller Badge")
                .initial_supply(dec!("1"));

            let controller_badge = ResourceBuilder::new_fungible()
                .mintable(rule!(require(mint_controller_badge.resource_address())), LOCKED)
                .metadata("name", "NeuRacle Controller Badge")
                .no_initial_supply();

            let token_bucket: Bucket = ResourceBuilder::new_fungible()
                .updateable_metadata(rule!(require(admin_badge.resource_address())), MUTABLE(rule!(require(admin_badge.resource_address()))))
                .mintable(rule!(require(controller_badge)), LOCKED)
                .burnable(rule!(require(controller_badge)), LOCKED)
                .metadata("name", "Neura")
                .metadata("symbol", "NAR")
                .initial_supply(10000000isize);

            let neuracle = NeuRacle::new(

                    token_bucket.resource_address(),
                    admin_badge.resource_address(),
                    mint_controller_badge,
                    controller_badge,
                    100usize,
                    1u64,
                    Decimal::ONE,
                    Decimal::ZERO,
                    500u64,
                    dec!("0.0015"),
                    dec!("5"),

                );

            info!("Neuracle test component address: {}", neuracle);

            let ground_id = GroundID::new(

                String::from("GroundID Test Component"),
                admin_badge.resource_address()

            );

            info!("GroundID test component address: {}", ground_id);

            let test_credit_scoring_rates = CreditScoringRates {

                yearly: CreditScoring {
                    degrade_rate: dec!("20"),
                    restore_rate: dec!("10"),
                },
                monthly: CreditScoring {
                    degrade_rate: dec!("10"),
                    restore_rate: dec!("5"),
                }

            };

            let ground_credit = GroundCredit::new(

                String::from("GroundCredit Test Component"),
                admin_badge.resource_address(),
                test_credit_scoring_rates,
                ground_id

            );

            info!("Ground Credit test component address: {}", ground_credit);

            let ground_cv = GroundCV::new(

                String::from("GroundCV Test Component"),
                admin_badge.resource_address()
                
            );
                
            Self {

                dao_badge: Vault::with_bucket(dao_badge),
                dao_share_token: Vault::with_bucket(dao_share_token),
                stable_coin: Vault::with_bucket(stable_coin),
                admin_badge: Vault::with_bucket(admin_badge),
                neura: Vault::with_bucket(token_bucket),
                neuracle: neuracle,
                ground_id: ground_id,
                ground_credit: ground_credit,
                ground_cv: ground_cv,
                ground_lending: None,
                dao: None,
                validators: Vec::new(),
                stakers: Vec::new(),
                etc_vault: LazyMap::new()

            }
            .instantiate()
            .globalize()

        }

        /// ## The method will do the following:
        /// 
        /// ### Create new validator node to validate data into the NeuRacle. 
        /// In practice, this should done by many users but for a simple test, we just get one validator node on this test component.
        /// 
        /// ### Call the "become_new_user" method on the NeuRacle and get 2 unix time badges
        /// 
        /// ### Create new Ground Lending test component with the follow params:
        /// - name: "Ground Lending Test Component"
        /// - admin_badge: the DAO badge.
        /// - interest_rates: yearly interest rate: 10%, interest rate late: 20%; monthly interest rate: 0.5%, interest rate late: 2%
        /// - stable_coin: the stable coin resource address from when instantiate the test component.
        /// - fee: lender withdrawal fee is 0.2%.
        /// - tolerance_threshold: the protocol's vault tolerance threshold is 60%.
        /// - credit_service: the ground credit component address from when instantiate the test component.
        /// - oracle: The oracle component address and the unix time oracle badge.
        /// - dao: None > will update through the lending protocol's method later.
        /// - compensate_rate: 50%
        /// 
        /// ### List the lending protocol on the ground credit component.
        /// 
        /// ### Create new DAO test component with the follow params:
        /// - name: "Test DAO component"
        /// - dao_badge: The dao_badge created from when instantiate the test component.
        /// - dao_share_token: 2 mils dao share tokens created from when instantiate the test component.
        /// - stable_coin: 20 mils stable coins created from when instantiate the test component.
        /// - swap_fee: 0.5%
        /// - ground_cv: GroundCV component address created from when instantiate the test component.
        /// - entry_requirement: The DAO entry require ECONOMIC_UNDERSTANDING at level 5 and COMMUNICATION at level 6 (check the cv const table on the Ground Business folder).
        /// - proposal requirement: The DAO concept proposer require SCRYPTO_PROGRAMMING at level 5.
        /// - dividend_rate: DAO dividend rate for each concept voted is 0.01%.
        /// - dev_expo_reward: The extra reward rate when an concept is accepted for proposer is 0.02%
        /// - year_commited_rate: The extra voting power for each year commited on the DAO is 10%
        /// - voting_pool_fee: fee for DAO member's voting pool is 1%
        /// - year_cap: DAO capped year allow a member commit on is 10 years.
        /// - unstake_delay: DAO's unstake delay is 2.592.000 seconds (1 month).
        /// - oracle: The oracle component address and the unix time oracle badge.
        /// - protocols: The Ground Finance protocol's controller badge address.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn init(&mut self) -> (ComponentAddress, ComponentAddress) {

            let neuracle: NeuRacle = self.neuracle.into();

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let (address, badge) = neuracle.create_new_validator_node(
                String::from("Test NeuRacle Validator Node"),
                String::from("Test Environment"),
                String::from("Test WEB 3"),
                dec!("0.5")
            );

            self.validators.push((address, Vault::with_bucket(badge)));

            info!("Created new NeuRacle validator with component address: {}", address);

            let (unix_time_badge, repayment) = neuracle.become_new_user(self.neura.take(dec!("1000000")), String::from("https://showcase.api.linx.twenty57.net/UnixTime/tounix?date=now"));

            self.neura.put(repayment);

            let interest_rates = RevolvingCreditInterestRates {
                yearly: Interest {
                    interest_rate: dec!("10"),
                    interest_rate_late: dec!("15")
                },
                monthly: Interest {
                    interest_rate: dec!("0.5"),
                    interest_rate_late: dec!("2")
                },
            };

            let (ground_lending, ground_lending_controller) = GroundLending::new(
                String::from("Ground Lending Test Component"), 
                self.dao_badge.resource_address(),
                interest_rates,
                self.stable_coin.resource_address(), 
                dec!("0.2"), 
                dec!("60"),
                self.ground_credit,
                (self.neuracle, unix_time_badge),
                None,
                dec!("50")
            );

            info!("Ground Lending test component address: {}", ground_lending);

            self.ground_lending = Some(ground_lending);

            let (unix_time_badge2, repayment) = neuracle.become_new_user(self.neura.take(dec!("1000000")), String::from("https://showcase.api.linx.twenty57.net/UnixTime/tounix?date=now"));

            self.neura.put(repayment);

            let ground_credit: GroundCredit = self.ground_credit.into();

            ground_credit.list_protocol(ground_lending_controller);

            let mut entry_requirement = HashMap::new();
            entry_requirement.insert(3011, 5);
            entry_requirement.insert(3014, 6);

            let dao = GroundBusinessDAO::new(
                String::from("Test DAO component"), 
                self.dao_badge.take_all(), 
                self.dao_share_token.take(dec!("2000000")),
                self.stable_coin.take(dec!("20000000")),
                dec!("0.5"),
                self.ground_cv,
                self.ground_id,
                entry_requirement,
                5u8,
                dec!("0.01"),
                dec!("0.02"),
                dec!("10"),
                dec!("1"),
                10u8,
                2592000u64,
                (self.neuracle, unix_time_badge2),
                Vec::from([ground_lending_controller])
            );

            info!("Ground Business DAO test component address: {}", dao);

            self.dao = Some(dao);

            ComponentAuthZone::pop().drop();

            (ground_lending, dao)

        }

        /// The method will stake some neura into the validator nodes.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn init_stake_for_validator_node(&mut self) {
            for (address, _) in &self.validators {
                let validator: Validator = address.clone().into();
                let staker = validator.stake(self.neura.take(dec!("100")));
                self.stakers.push(Vault::with_bucket(staker))
            }
            info!("Staked 100 neura into each validator components.");
        }

        /// The method will run through a NeuRacle data updating round to update new unix time data.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn update_neuracle_data(&mut self, data: String) {

            let unix_time: u64 = data.parse().expect("Wrong data!");

            let neuracle: NeuRacle = self.neuracle.into();
            self.neura.put(neuracle.new_round());
            let apis = neuracle.get_apis();
            let mut datas = BTreeMap::new();

            for api in apis {
                datas.insert(api, data.clone());
            }

            for (address, badge) in &self.validators {

                let validator: Validator = address.clone().into();

                let proof = badge.create_proof();

                ComponentAuthZone::push(proof);

                validator.update_data(datas.clone());

                ComponentAuthZone::pop().drop();

            }

            self.neura.put(neuracle.end_round());

            info!("Updated the NeuRacle data, new unix time: {}.", unix_time);
        }

        /// Same as the account component
        pub fn deposit(&mut self, bucket: Bucket) {
            let resource_address = bucket.resource_address();
            match self.etc_vault.get(&resource_address) {
                Some(mut v) => {
                    v.put(bucket);
                }
                None => {
                    let v = Vault::with_bucket(bucket);
                    self.etc_vault.insert(resource_address, v);
                }
            }
        }


        /// The method will get all resource of a new DAO member and go through a Ground Business Dao's 
        /// concept consensus to let the lending protocol use the dao treasury.
        ///
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn lending_use_dao(&mut self) {

            let ([id_sbt, credit_sbt, stable_coin, dao_share_token, cv, member_sbt], move_proof) = self.init_dao_member(8u8);

            self.stable_coin.put(stable_coin); self.dao_share_token.put(dao_share_token);

            self.deposit(id_sbt); self.deposit(credit_sbt);

            let dao_address = self.dao.unwrap();

            let lending_address = self.ground_lending.unwrap();

            let dao: GroundBusinessDAO = dao_address.into();

            ComponentAuthZone::push(member_sbt.create_proof());

            let propose_badge = dao.propose_concept(cv.create_proof(), Methods { methods: vec![Method {component: lending_address, method: String::from("use_dao"), args: vec![scrypto_encode(&dao_address)] }]}, dec!("100"));

            ComponentAuthZone::pop().drop();

            let id = NonFungibleId::from_u64(dao.proposal_id_counter() - 1);

            dao.vote(member_sbt.create_proof(), id, true);

            let return_reward = dao.execute_concept(propose_badge);

            self.deposit(return_reward.unwrap());

            ComponentAuthZone::push(move_proof);

            self.deposit(member_sbt); self.deposit(cv); 

            ComponentAuthZone::pop().drop();

        }

        /// The method will get all the needed resources needed to become a DAO member and test all the Ground Packages. The resources included:
        /// - An unique Person ID SBT with income: 1000 stablecoins, trust factor score: 80
        /// - A credit SBT with the credit score same as the ID SBT.
        /// - 100000 stable coins
        /// - 1000 dao share tokens
        /// - CV component SBTs: ECONOMIC_UNDERSTANDING (skill id 11) at level 6, COMMUNICATION (skill id 14) at level 7 and SCRYPTO_PROGRAMMING (skill id 2) at level 6.
        /// - The DAO Member SBT with the input commited year and the associate voting power calculated by the Quartic Voting Mechanism.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn init_dao_member(&mut self, commit_year: u8) -> ([Bucket; 6], Proof) {

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let ground_id: GroundID = self.ground_id.into();
            let id_sbt = ground_id.issue_new_id_sbt(IdentityType::Person, dec!("3000"), dec!("80"));

            let ground_credit: GroundCredit = self.ground_credit.into();
            let credit_sbt = ground_credit.get_new_credit_sbt(id_sbt.create_proof());

            let ground_cv: GroundCV = self.ground_cv.into();
            let mut cv1 = ground_cv.issue_new_cv_sbt(CVdata::Skill(Skill {skill_id: 11, level: 6}));
            info!("You got new ECONOMIC UNDERSTANDING CV component level 6 SBT with id: {}", cv1.non_fungible::<CiV>().id());
            let cv2 = ground_cv.issue_new_cv_sbt(CVdata::Skill(Skill {skill_id: 14, level: 7}));
            info!("You got new COMMUNICATION CV component level 7 SBT with id: {}", cv2.non_fungible::<CiV>().id());
            let cv3 = ground_cv.issue_new_cv_sbt(CVdata::Skill(Skill {skill_id: 2, level: 6}));
            info!("You got new SCRYPTO_PROGRAMMING CV component level 6 SBT with id: {}", cv3.non_fungible::<CiV>().id());

            cv1.put(cv2);

            cv1.put(cv3);

            let dao: GroundBusinessDAO = self.dao.unwrap().into();

            let (member_sbt, move_proof) = dao.become_dao_member(id_sbt.create_proof(), cv1.create_proof(), self.dao_share_token.take(dec!("1000")), commit_year);

            ComponentAuthZone::pop().drop();

            ([id_sbt, credit_sbt, self.stable_coin.take(dec!("100000")), self.dao_share_token.take(dec!("1000")), cv1, member_sbt], move_proof)

        }

        /// The method will get all the needed resources to get a credit or become delegator but not meet the DAO member requirement.
        /// 
        /// The resources are enough to test the Ground Finance package, included:
        /// - A unique Person ID SBT with income: 100 stablecoins, trust factor score: 60
        /// - A credit SBT with the credit score same as the ID SBT.
        /// - 10000 stable coins
        /// - 100 dao share tokens
        /// - CV component SBTs: COMMUNICATION (skill id 14) at level 3 and SCRYPTO_PROGRAMMING (skill id 2) at level 2.
        /// - The DAO's delegator badge.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn init_credit_user(&mut self, maximum_monthly_allowance: Decimal) -> [Bucket; 6] {

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let income = (maximum_monthly_allowance * dec!("12") / dec!("0.6") / dec!("0.6")).ceiling();

            let ground_id: GroundID = self.ground_id.into();
            let id_sbt = ground_id.issue_new_id_sbt(IdentityType::Person, income, dec!("60"));

            let ground_credit: GroundCredit = self.ground_credit.into();
            let credit_sbt = ground_credit.get_new_credit_sbt(id_sbt.create_proof());

            let ground_cv: GroundCV = self.ground_cv.into();
            let mut cv1 = ground_cv.issue_new_cv_sbt(CVdata::Skill(Skill {skill_id: 14, level: 3}));
            info!("You got new COMMUNICATION CV component level 3 SBT with the resource address: {}, id: {}", cv1.resource_address(), cv1.non_fungible::<CiV>().id());
            let cv2 = ground_cv.issue_new_cv_sbt(CVdata::Skill(Skill {skill_id: 2, level: 2}));
            info!("You got new SCRYPTO_PROGRAMMING CV component level 2 SBT with the resource address: {}, id: {}", cv2.resource_address(), cv2.non_fungible::<CiV>().id());

            cv1.put(cv2);

            let dao: GroundBusinessDAO = self.dao.unwrap().into();

            let delegator_badge = dao.delegate_dao_share(Some(id_sbt.create_proof()), Some(cv1.create_proof()), self.dao_share_token.take(dec!("1000")), 0u8);

            ComponentAuthZone::pop().drop();

            [id_sbt, credit_sbt, self.stable_coin.take(dec!("10000")), self.dao_share_token.take(dec!("100")), cv1, delegator_badge]

        }

        /// The method will just let user update their ID SBT data.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn review_id_sbt_update_data_request(&self, id: u64) {

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let ground_id: GroundID = self.ground_id.into();
            ground_id.review_update_data(id, true);

            ComponentAuthZone::pop().drop();

        }

        /// The method will go through the whole installment credit process 
        /// from the Ground Credit component and just let user get an installment credit badge to try on the lending protocol.
        /// 
        /// The Installment Credit data is as follow:
        /// - total_loan: tester's input.
        /// - interest_rate: 10%
        /// - interest_rate_late: 25%
        /// - period_length: 2.592.000 seconds (1 month).
        /// - period_max: tester's input.
        /// 
        /// ***This is just for test purpose and totally not the practial use of these packages!***
        pub fn review_installment_credit(&self, id: u64) {

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let ground_credit: GroundCredit = self.ground_credit.into();
            ground_credit.review_installment_credit_request(id, true);

            ComponentAuthZone::pop().drop();

        }

        /// The method will blacklist and prevent an unique Identity SBT ID from using the Ground Finance components.
        pub fn blacklist(&self, id: NonFungibleId) {

            let proof = self.admin_badge.create_proof();

            ComponentAuthZone::push(proof);

            let ground_credit: GroundCredit = self.ground_credit.into();
            ground_credit.blacklist(id);

            ComponentAuthZone::pop().drop();

        }
    }
}