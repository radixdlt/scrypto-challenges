use scrypto::prelude::*;
use crate::hub_data::*;

blueprint! {
    struct Academy{
        // Contribution vault to store contributions, fees, gains 
        contribution_vault: Vault,
        // Vault to stock SBT Updater Badge
        sbt_updater_badge: Vault,
        // SBT Updater Badge resource address
        sbt_updater_badge_addr: ResourceAddress,
        // Teacher Badge Vault with teacher SBT resource address & ID as key.
        teacher_badge_vaults: HashMap<(ResourceAddress,NonFungibleId),Vault>,
        // Map of Test Vaults: map's key is a tuple composed by course number and test number 
        test_vaults: HashMap<(u32,u8),Vault>,
        // Protocol Minter Badge resource address
    	minter_badge: Vault,
        // Protocol Owner Badge resource address
        owner_badge: ResourceAddress,
        // Teacher Badge resource address
        teacher_badge: ResourceAddress,
        // Test NFT resource address
        academy_test: ResourceAddress,
        // TestCertificate NFT resource address
        test_certificate: ResourceAddress,
        // DegreeNFT resource address
        degree_nft: ResourceAddress,
        // Map to store an external component address and relative Caller Badge allowing it 
        // to invoke methods through component call.
    	caller_map: HashMap<ComponentAddress,ResourceAddress>,
        // Map with SBT data of teachers, TeacherBadge data.
        teacher_map: HashMap<(ResourceAddress,NonFungibleId),(ResourceAddress,NonFungibleId,DegreeNFT)>,
        // Map with SBT data of students, course number, test number & deadline, test certificate address & id, 
        // evaluation flag & test score gained.
    	test_map: HashMap<(ResourceAddress,NonFungibleId),Vec<(u32,u8,u64,ResourceAddress,NonFungibleId,bool,u8)>>,
        // Map with course number & date, certificate address & accrued title, degree certificate id and sbt data 
        // of gratuated students.
        degree_map: HashMap<(u32,u64,ResourceAddress,String),Vec<(NonFungibleId,ResourceAddress,NonFungibleId)>>,
        // User SBT Resource Address.
        user_sbt: ResourceAddress,
        // Currency accepted by Academy.
        currency: ResourceAddress,
        // Course number
        course_number: u32,
        // Course vector data with course number, start, duration, number of tests, course name, teacher id. 
        course_vec: Vec<(u32,u64,u64,u8,String,NonFungibleId)>,
        // Test vector data with course number, deadline, test number, test name, test NFT data, teacher id.
        test_vec: Vec<(u32,u64,u8,String,Test,NonFungibleId)>,
        // Claimed contribution amount.
        contribution_claimed: Decimal,
        // Set a minimum course duration.
        min_course_duration: u64,
        // Set a minimum number of tests within a single course.
        min_tests_number: u8,
        // Set a minimum test duration.
        min_test_duration: u64,
    }

    impl Academy {
        pub fn new(
            academy_name: String,                       // Protocol name instance printed on minted Badge & NFTs
            sbt_updater_badge_addr: ResourceAddress,    // Land Data protocol's SBT updater Badge resource address
            land_data_owner_badge: ResourceAddress,     // Land Data protocol's Owner Badge resource address
            user_sbt: ResourceAddress,                  // Land Data protocol's registered users SBT resource address
            currency: ResourceAddress                   // Protocol accepted currency resource address
    ) -> (ComponentAddress,Bucket) {
        	let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", academy_name.clone() + " MinterBadge ")
                .initial_supply(Decimal::one());

            let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", academy_name.clone() + " OwnerBadge ")
                .initial_supply(Decimal::one());

            let teacher_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", academy_name.clone() + " TeacherBadge ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(minter_badge.resource_address())), LOCKED)                
                .no_initial_supply();

            let academy_test = ResourceBuilder::new_non_fungible()
                .metadata("name", academy_name.clone() + " TestTemplate ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(teacher_badge)), LOCKED)                
                .no_initial_supply();

            let test_certificate = ResourceBuilder::new_non_fungible()
                .metadata("name", academy_name.clone() + " Test Certificate ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(minter_badge.resource_address())), LOCKED)                
                .no_initial_supply();

            let degree_nft = ResourceBuilder::new_non_fungible()
                .metadata("name", academy_name + " Degree NFT ")
                .mintable(rule!(require(minter_badge.resource_address())), LOCKED)                
                .no_initial_supply();

            let access_rules = AccessRules::new()
                .method("stock_sbt_updater_badge", rule!(require(land_data_owner_badge)))
            	.method("mint_caller_badge", rule!(require(owner_badge.resource_address())))   
                .method("mint_teacher_badge", rule!(require(owner_badge.resource_address())))
                .method("claim_contribution", rule!(require(owner_badge.resource_address())))
                .default(rule!(allow_all));

            let mut academy: AcademyComponent = Self {
                contribution_vault: Vault::new(currency.clone()),
                sbt_updater_badge: Vault::new(sbt_updater_badge_addr),
                sbt_updater_badge_addr,
                teacher_badge_vaults: HashMap::new(),
                test_vaults: HashMap::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                teacher_badge,
                academy_test,
                test_certificate,
                degree_nft,
                caller_map: HashMap::new(),
                teacher_map: HashMap::new(),
                test_map: HashMap::new(),
                degree_map: HashMap::new(),
                user_sbt,
                currency,
                course_number: 0,
                course_vec: Vec::new(),
                test_vec: Vec::new(),
                contribution_claimed: Decimal::zero(),
                min_course_duration: 5000,
                min_tests_number: 2,
                min_test_duration: 10,
            }
            .instantiate();
            academy.add_access_check(access_rules);
            
            (academy.globalize(),owner_badge)
        }


        // Stock tokens in Academy Contribution Vault.
        pub fn tkn_lock(&mut self, tkn_bckt: Bucket) -> Decimal {
            let locked_amount = tkn_bckt.amount();
            self.contribution_vault.put(tkn_bckt);
            info!(" Academy TKN contribution amount {} ",locked_amount);

            locked_amount
        }
    
        // Check total locked amount in Academy Contribution Vault.
        pub fn locked_amount(&self) -> Decimal {
            let locked_amount = self.contribution_vault.amount();
            info!(" Academy TKN total contribution amount {} ",locked_amount);

            locked_amount
        }

        // Stock "LandData UpdaterBadge" to update users SBT data when requested by protocol
        pub fn stock_sbt_updater_badge(&mut self, sbt_updater_badge: Bucket) {
            assert!(
                sbt_updater_badge.resource_address() == self.sbt_updater_badge_addr,
                "[stock_sbt_updater_badge]:Wrong Badge provided! "
            );
            assert!(sbt_updater_badge.amount() == dec!("1"),"[stock_sbt_updater_badge]:Just one! ");
            assert!(
                self.sbt_updater_badge.is_empty(),
                "[stock_sbt_updater_badge]:Updater Badge already present! "
            );
            self.sbt_updater_badge.put(sbt_updater_badge);
        }

        // Mint a Caller Badge to allow called from an external Component to call methods
        pub fn mint_caller_badge(&mut self, cmp_addr: ComponentAddress) -> Bucket {
            let caller_badge = self.build_badge(" ProAcademy_Caller_Badge".to_string());
            self.caller_map.insert(cmp_addr,caller_badge);
            info!(" Caller Component address added: {} ", cmp_addr);
                
            self.minter_badge
                .authorize(|| { borrow_resource_manager!(caller_badge).mint(Decimal::one()) })
        }

        // Hire a teacher specifying his SBT credentials, his Study title degrees as well as his 
        // teaching subjects. Mint a teacher badge to allow him to call protocol methods: open new 
        // teaching courses, start new tests as well as evaluate them.
        pub fn mint_teacher_badge(
            &mut self, 
            uri: String,
            teacher_sbt_addr: ResourceAddress,              // teacher SBT resource address
            teacher_sbt_id: NonFungibleId,                  // teacher SBT ID
            teacher_name: String,                           // teacher name
            degree_name: Vec<String>,                       // teacher degrees list
            teaching_subject: Vec<String>,                  // teaching subjects list
            grade_point_avg: u8,                            // teacher degree gpa
            cum_laude: bool                                 // teacher degree laude
        ) {               
            let key = NonFungibleId::random(); 

            // populate a DegreeNFT structure with relevant data
            let badge_data = DegreeNFT {
                uri: uri,
                pro_academy_address: Runtime::actor().as_component().0,
                user_sbt_address: teacher_sbt_addr,
                user_sbt_id: teacher_sbt_id.clone(),
                user_name: teacher_name,
                degree_name: degree_name,
                mint_date:  Runtime::current_epoch(),
                teaching_subject: teaching_subject,
                grade_point_avg: grade_point_avg,
                cum_laude: cum_laude
            };

            // insert teacher SBT identificative data in relative map
            self.teacher_map.insert(
                (teacher_sbt_addr,teacher_sbt_id.clone()),
                (self.teacher_badge,key.clone(),badge_data.clone())
            );
            info!(" Teacher Badge address: {} ", self.teacher_badge);
            info!(" Teacher Badge id: {} ", key.clone());
         
            // mint a teacher NFT badge 
            let teacher_badge = self.minter_badge.authorize(|| { 
                borrow_resource_manager!(self.teacher_badge).mint_non_fungible(&key,badge_data)
            }); 

            // store teacher NFT badge in related vault 
            let vault = self.teacher_badge_vaults.entry((teacher_sbt_addr,teacher_sbt_id.clone()))
                .or_insert(Vault::new(self.teacher_badge));
            vault.put(teacher_badge);
        }

        // Method callable by hired teacher to download his personal teacher badge NFT.
        // Teacher must provide his SBT credentials.
        pub fn withdrawal_teacher_badge(&mut self, teacher_sbt: Proof) -> Bucket {
            // Check teacher SBT proof provided
            let teacher_sbt: ValidatedProof = teacher_sbt.unsafe_skip_proof_validation();
            assert!(
                borrow_resource_manager!(teacher_sbt.resource_address()).resource_type()
                    == ResourceType::NonFungible,
                "[withdrawal_teacher_badge]: Wrong NFT SBT Proof detected! "
            );
            assert!(
                teacher_sbt.amount() == dec!("1"),
                "[withdrawal_teacher_badge]: Pass just one SBT Proof Pls!"
            );

            let key = teacher_sbt.non_fungible::<UserSBT>().id();
            // retrieve teacher NFT badge data from related map
            let (bdg_addr,bdg_key,bdg_data) = 
                self.teacher_map.get(&(teacher_sbt.resource_address(),key.clone())).unwrap();

            let teacher_sbt_nft = teacher_sbt.non_fungible::<UserSBT>();
            let mut teacher_data = teacher_sbt_nft.data();
            teacher_data.educational_degrees.push((*bdg_addr,bdg_key.clone(),bdg_data.clone()));

            // update teacher SBT with teacher DegreeNFT badge data
            self.sbt_updater_badge.authorize(|| {
                borrow_resource_manager!(teacher_sbt.resource_address())
                    .update_non_fungible_data(&key.clone(), teacher_data)
            });
            
            // take Teacher NFT Badge from related vault and return it 
            match self.teacher_badge_vaults.get_mut(&(teacher_sbt.resource_address(),key.clone())) {
                Some(vault) => vault.take_non_fungible(&bdg_key),
                None => {
                    info!("[withdrawal_teacher_badge]: NFT Badge unfound! ");
                    std::process::abort()
                }
            }
        } 

        // Claim accrued contribution function whom only protocol owner can succesfully call.
        pub fn claim_contribution(&mut self, amount: Decimal) -> Bucket {       
            let bckt_output: Bucket = self.contribution_vault.take(amount);
            self.contribution_claimed += bckt_output.amount();
            info!(" Academy accrued contribution claimed {} ", bckt_output.amount());
            info!(
                " Academy total accrued contribution claimed {} ${} ", 
                self.contribution_claimed, 
                self.currency
            );

            bckt_output
        }

        // Method callable by hired teacher wishing to open a new academic course of study.
        // Teacher needs to provide Teacher Badge proof to authenticate himself as well as other 
        // data like course duration, number of test within course & coursename aka final degree 
        // title achievable
        pub fn open_course(
            &mut self, 
            duration: u64,                                // course duration
            tests_number: u8,                             // expected number of tests within course
            name: String,                                 // course name 
            teacher_badge: Proof                          // teacher badge proof
        ) {
            // Check teacher badge proof provided
            let teacher_badge: ValidatedProof = teacher_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.teacher_badge,
                    dec!("1"),
                ))
                .expect("[open_course]: Invalid proof provided");
            let id = teacher_badge.non_fungible::<DegreeNFT>().id();

            // check teacher id within related map
            let mut founded = false;
            for (_key,value) in self.teacher_map.iter() {
                if value.0 == teacher_badge.resource_address() && value.1 == id.clone() {
                    founded = true;
                    break;
                } 
            }
            assert!(founded," [open_course]: Unable to find teacher id! ");

            self.course_number += 1;
            assert!(duration >= self.min_course_duration," [open_course]: Increase duration! ");
            assert!(tests_number >= self.min_tests_number," [open_course]: Add tests! ");
            let now = Runtime::current_epoch();

            // insert course data in related map
            self.course_vec.push((self.course_number,now,duration,tests_number,name.clone(),id));
            info!(" Course number: {} ", self.course_number);
            info!(" Course name: {} ", name);
            info!(" Number of tests: {} ", tests_number);
            info!(" Course duration: {} ", duration);
        }

        // Method callable to consult list of courses of study helded by academy
        pub fn course_list(&mut self) {
            for (number,start,time,tests,name,key) in self.course_vec.iter() {
                info!(" ========================================= ");
                info!(" Course number: {} ", number);
                info!(" Course name: {} ", name);
                info!(" Number of tests: {} ", tests);
                info!(" Course deadline: {} ", start+time);
                info!(" Teacher id: {} ", key);
            }
        }

        // Method callable by hired teacher wishing to publish a new test.
        // Teacher needs to provide Teacher Badge proof to authenticate himself as well as other 
        // data like test URI, course number, test duration, test number, test name aka exam title
        // achievable.
        pub fn publish_test(
            &mut self, 
            test_uri: String,                           // test URL/URI
            course_nr: u32,                             // number of course 
            duration: u64,                              // test duration
            test_nr: u8,                                // test's number  
            test_name: String,                          // test's name
            assertions: Vec<String>,                    // list of tesst's assertions
            teacher_badge: Proof                        // teacher badge proof
        ) {
            // Check teacher badge proof provided
            let teacher_badge: ValidatedProof = teacher_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.teacher_badge,
                    dec!("1"),
                ))
                .expect("[publish_test]: Invalid proof provided");
            let id = teacher_badge.non_fungible::<DegreeNFT>().id();

            // check teacher id within related map
            let mut founded = false;
            for (_key,value) in self.teacher_map.iter() {
                if value.0 == teacher_badge.resource_address() && value.1 == id.clone() {
                    founded = true;
                    break;
                } 
            }
            assert!(founded," [publish_test]: Unable to find teacher id! ");

            // retrieve course data within related map
            assert!(duration >= self.min_test_duration," [publish_test]: Increase duration! ");
            let answer_vec: Vec<bool> = Vec::new();           
            let mut course_name = "".to_string();
            founded = false;
            for tup in self.course_vec.iter() {
                if tup.0 == course_nr {
                    assert!(test_nr <= tup.3," Tests limit reached up! ");
                    assert!(duration <= tup.1+tup.2," Maximum deadline allowed: {} ", tup.1+tup.2);
                    course_name = tup.4.clone();
                    founded = true;
                    break;
                }
            }
            assert!(founded," Course unfound! ");

            // populate a Test structure with relevant data
            let test_data = Test {
                uri: test_uri,
                pro_academy_address: Runtime::actor().as_component().0,
                user_sbt_address: ResourceAddress::from(RADIX_TOKEN),
                user_sbt_id: NonFungibleId::from_u64(0),
                course_name: course_name.clone(),
                course_number: course_nr,
                test_name: test_name.clone(),
                test_number: test_nr,
                test_date: Runtime::current_epoch(),
                assertions: assertions,
                answers: answer_vec.clone(),
                right_answers: answer_vec,
                test_passed: false,
                score: 0
            };
            
            // insert test data within related map
            let end = Runtime::current_epoch()+duration;
            self.test_vec.push((self.course_number,end,test_nr,test_name.clone(),test_data,id));            
            info!(" Course number: {} ", self.course_number);
            info!(" Course name: {} ", course_name);
            info!(" Test number: {} ", test_nr);
            info!(" Test name: {} ", test_name);
            info!(" Test deadline: {} ", end);
        }

        // Method callable to consult list of tests within courses of study helded by academy
        pub fn test_list(&mut self) {
            for (course_nr,end,test_nr,test_name,_test_data,id) in self.test_vec.iter() {
                if Runtime::current_epoch() <= *end {
                    info!(" ========================================= ");
                    info!(" Course number: {} ", course_nr);
                    info!(" Test name: {} ", test_name);
                    info!(" Test number: {} ", test_nr);
                    info!(" Test deadline: {} ", end);
                    info!(" Teacher id: {} ", id);
                }
            }
        }

        // Method callable to consult test answers providing relative test number and relative
        // course number
        pub fn view_answers(&mut self, course_number: u32, test_number: u8) {
            for (course_nr,end,test_nr,test_name,data,id) in self.test_vec.iter() {
                if *course_nr == course_number && *test_nr == test_number { 
                    assert!(Runtime::current_epoch() > *end,"[view_answers]:Test still running!");
                    assert!(!data.right_answers.is_empty(),"[view_answers]:Test yet unevaluated!");
                    info!(" ========================================= ");
                    info!(" Course number: {} ", course_nr);
                    info!(" Test name: {} ", test_name);
                    info!(" Test number: {} ", test_nr);
                    info!(" Test deadline: {} ", end);
                    info!(" Teacher id: {} ", id);
                    let mut i = 1;
                    for sentence in &data.assertions {
                        info!(" ========================================= ");
                        info!(" Assertion nr {}: {} ", i, sentence);
                        info!(" Answer nr {}: {} ", i, data.right_answers[i-1]);
                        i += 1;
                        if i > data.right_answers.len() {
                            break;
                        }
                    }
                }
            }
        }

        // Method callable by student to register to academy's courses of study providing SBT proof
        // for verification purpose
        pub fn sign_up(&mut self, student_sbt: Proof) {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[sign_up]: NFT SBT Proof authorization failed! ");

            let key = student_sbt.non_fungible::<UserSBT>().id();
            let v: Vec<(u32,u8,u64,ResourceAddress,NonFungibleId,bool,u8)> = Vec::new();

            if self.test_map.contains_key(&(student_sbt.resource_address(),key.clone())) {
                info!("[sign_up]: Student already subscribed!");
            } else {
                self.test_map.insert((student_sbt.resource_address(),key),v);
                info!("[sign_up]: Student subscribed!");
            }
        }

        // Method callable by student to run a test consulting assertions list once provided 
        // relative test number and relative course number as well as own SBT proof for 
        // verification purpose
        pub fn run_test(
            &mut self, 
            course_number: u32,                           // number of course 
            test_number: u8,                              // number of test
            student_sbt: Proof                            // student SBT proof
        ) -> Vec<String> {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[run_test]: NFT SBT Proof authorization failed! ");

            let key = student_sbt.non_fungible::<UserSBT>().id();            
            let (mut assertions,mut test_vec) = (Vec::new(),Vec::new()); 
            let (mut t_name,mut uri,mut t_end) = ("".to_string(),"".to_string(), 0);
            let (addr, id) = (ResourceAddress::from(RADIX_TOKEN), NonFungibleId::from_u64(0)); 
    
            // retrieve test data from related map
            for (course_nr,end,test_nr,name,data,_id) in self.test_vec.iter_mut() {
                if course_number == *course_nr && test_number == *test_nr {
                    assert!(Runtime::current_epoch() <= *end," Test deadline already reached! ");
                    test_vec.push((*course_nr,*test_nr,*end));
                    t_name = name.to_string();
                    uri = data.uri.clone();
                    t_end = *end;
                    assertions = data.assertions.clone();
                }
            }

            // check student is registered and hasn't already run the same test
            if self.test_map.contains_key(&(student_sbt.resource_address(),key.clone())) {
                match self.test_map.get_mut(&(student_sbt.resource_address(),key.clone())) {
                    Some(v) => {
                        for (course_nr,test_nr,_deadline,_addr,_key,_flag,_score) in v.iter() {
                            if *course_nr == course_number && *test_nr == test_number {
                                info!("[run_test]: Test already ran!");
                                std::process::abort()
                            }
                        }
                        v.push((course_number,test_number,t_end,addr,id,false,0 )); 
                    }
                    _ => std::process::abort()
                }
            } else {
                info!("[run_test]: User ain't enrolled for the course, please signup first!");
                std::process::abort()
            }
            
            info!(" ========================================= ");
            info!(" Course number: {} ", course_number);
            info!(" Test name: {} ", t_name);
            info!(" Test number: {} ", test_number);
            info!(" Test deadline: {} ", t_end);
            info!(" Test URI: {} ", uri);
            info!(" ========================================= ");
            let mut index = 1;
            for sentence in assertions.iter() {
                info!(" Assertion nr {}: {} ", index, sentence);
                info!(" ========================================= ");
                index += 1;
            }

            assertions
        }
    
        // Method callable by student to answer test's assertions list once provided 
        // relative test number and relative course number as well as owned SBT proof 
        // for verification purpose
        pub fn answer_test(
            &mut self, 
            course_number: u32,                          // number of course
            test_number: u8,                             // number of test
            mut answers: Vec<bool>,                      // answers list provided by student
            student_sbt: Proof                           // student SBT proof
        ) -> bool {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[answer_test]: NFT SBT Proof authorization failed! ");

            // check student is registered and test deadline hasn't been reached already
            let key = student_sbt.non_fungible::<UserSBT>().id();
            let mut founded = false;
            match self.test_map.get_mut(&(student_sbt.resource_address(),key.clone())) {
                Some(v) => {
                    for (course_nr,test_nr,end,_addr,_key,_flag,_score) in v.iter() {
                        if *course_nr == course_number && *test_nr == test_number {
                            assert!(
                                Runtime::current_epoch() <= *end,
                                " [answer_test]: Deadline reached, test unavailable! "
                            );
                            founded = true;
                            break;
                        }
                    } 
                }
                _ => {
                    info!("[answer_test]: User ain't enrolled for the course, please signup first!");
                    std::process::abort()
                }
            }
            assert!(founded," [answer_test]: User ain't enrolled for the test! ");
            founded = false;

            // insert student's answers within related map 
            let mut data = TestZero::new();
            let mut test_data = data.test_zero.pop().unwrap();
            for (course_nr,_end,test_nr,_name,data,_id) in self.test_vec.iter_mut() {
                if course_number == *course_nr && test_number == *test_nr {
                    assert_eq!(
                        data.assertions.len(),
                        answers.len(),
                        " [answer_test]: Check answers number! "
                    );
                    data.answers.append(&mut answers);
                    test_data = data.clone();
                    test_data.user_sbt_address = student_sbt.resource_address();
                    test_data.user_sbt_id = key.clone();
                    founded = true;
                    break;
                }
            }
            assert!(founded," [answer_test]: Unable to load answers! ");

            // mint a student Test NFT
            let test_id = NonFungibleId::random();
            let academy_test = self.minter_badge.authorize(|| { 
                borrow_resource_manager!(self.academy_test).mint_non_fungible(&test_id,test_data)
            }); 

            // put student Test NFT in related vault
            let test_vault = self.test_vaults
                .entry((course_number,test_number))
                .or_insert(Vault::new(self.academy_test));
            test_vault.put(academy_test);

            founded
        }

        // Method callable by teacher to evaluate students test's answers list once provided 
        // relative course number, relative test number, right test's answers as well as owned 
        // Teacher Badge for verification purpose
        pub fn evaluate_test(
            &mut self, 
            course_nr: u32,                              // number of course
            test_nr: u8,                                 // number of test
            right_answers: Vec<bool>,                    // right answers list provided by teacher
            teacher_badge: Bucket                        // Teacher Badge 
        ) -> (Bucket,Bucket) {
            // Check Teacher Badge provided
            assert_eq!(
                teacher_badge.resource_address(),
                self.teacher_badge,
                "[evaluate_test]: Teacher badge unauthorized! "
            );
            assert_eq!(
                teacher_badge.amount(), Decimal::one(),
                "[evaluate_test]: Just one Teacher badge required! "
            );
            let key = teacher_badge.non_fungible::<DegreeNFT>().id();

            // Publish right answers on test vector for consultation purpose.
            let mut founded = false;
            for (course_nmbr,end,test_nmbr,_name,data,id) in self.test_vec.iter_mut() {
                if course_nr == *course_nmbr && test_nr == *test_nmbr && key == *id {
                    assert!(Runtime::current_epoch() > *end, " [evaluate_test]: Test still running! ");
                    assert_eq!(
                        data.assertions.len(),
                        right_answers.len(),
                        " [evaluate_test]: Check answers number! "
                    );
                    assert!(data.right_answers.len() == 0,"[evaluate_test]:Test already evaluated!");
                    data.right_answers.append(&mut right_answers.clone());
                    founded = true;
                    break;
                }
            }
            assert!(founded," [evaluate_test]: Unable to find test! ");

            // evaluate test and update data with right answers list and test evaluation response
            let mut count = 0;
            let mut nft_bckt = self.test_vaults.get_mut(&(course_nr,test_nr)).unwrap().take_all();
            loop {
                let nft = nft_bckt.take(dec!("1"));
                let mut data: Test = nft.non_fungible().data();
                data.right_answers.append(&mut right_answers.clone());
                for index in 0..data.answers.len() {
                    if data.answers[index] == data.right_answers[index] {
                        count += 1;
                    }
                }
                data.score = count;
                if usize::from(count) >= data.answers.len()*6/10 {
                    data.test_passed = true;
                }

                // update related map with test response and score
                let mut founded = false;
                match self.test_map.get_mut(&(data.user_sbt_address,data.user_sbt_id.clone())) {
                    Some(v) => {
                        for mut tuple in v {
                            if tuple.0 == course_nr && tuple.1 == test_nr { 
                                tuple.5 = data.test_passed;
                                tuple.6 = data.score;
                                founded = true;
                                break;
                            }
                        }
                    }
                    _ => {
                        info!("[evaluate_test]: User unfound!");
                        std::process::abort()
                    }
                }
                assert!(founded," [evaluate_test]:  User's test unfound! ");

                // update Test Nft data
                let id = nft.non_fungible::<Test>().id();
                teacher_badge.authorize(|| {
                    borrow_resource_manager!(nft.resource_address())
                        .update_non_fungible_data(&id,data)
                });
                
                self.test_vaults.get_mut(&(course_nr,test_nr))
                    .expect(" [evaluate_test]: Test vault error! ")
                    .put(nft);


                if nft_bckt.amount() == dec!(0) {
                    break;
                }
            }

            (teacher_badge,nft_bckt)
        } 

        // Method callable by student to check test's result once provided 
        // relative course number and relative test number as well as owned SBT proof 
        // for verification purpose. If test results as passed a TestCertificate NFT is minted and
        // returned   
        pub fn test_result(
            &mut self, 
            course_number: u32,                          // number of course
            test_number: u8,                             // number of test
            student_sbt: Proof                           // student SBT proof
        ) -> Bucket {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[test_result]: NFT SBT Proof authorization failed! ");
            let key = student_sbt.non_fungible::<UserSBT>().id();

            let mut data = TestZero::new();
            let mut test_data = data.test_zero.pop().unwrap();

            // Check test deadline has been reached and test has been evaluated
            for (course_nr,end,test_nr,_test_name,data,_id) in self.test_vec.iter() {
                if *course_nr == course_number && *test_nr == test_number { 
                    assert!(Runtime::current_epoch() > *end,"[test_result]:Test still running!");
                    assert!(!data.right_answers.is_empty(),"[test_result]:Test yet unevaluated!");
                    test_data = data.clone();
                }
            }  

            // Check test has been succesfully passed and a TestCertificate NFT hasn't 
            // been already requested by student.
            // Retrieve useful data to populate a TestCertificate structure
            let nft_id = NonFungibleId::random();
            let mut founded = false;
            let mut mint = false;
            match self.test_map.get_mut(&(student_sbt.resource_address(),key.clone())) {
                Some(v) => {
                    for tuple in v {
                        if tuple.0 == course_number && tuple.1 == test_number {
                            if tuple.5 {
                                info!(" Test passed! Score: {}",tuple.6);
                                assert!(
                                    tuple.3 != self.test_certificate && tuple.4 == NonFungibleId::from_u64(0),
                                    " Test Certificate already minted! "
                                );
                                test_data.user_sbt_address = student_sbt.resource_address();
                                test_data.user_sbt_id = key.clone();
                                test_data.test_passed = tuple.5;
                                test_data.score = tuple.6;
                                tuple.3 = self.test_certificate;
                                tuple.4 = nft_id.clone();
                                mint = true;
                            }
                            founded = true;
                            break;
                        }
                    }
                }
                _ => {
                    info!("[test_result]: User ain't enrolled for the course, please signup first!");
                    std::process::abort()
                }
            }
            assert!(founded," [test_result]: User ain't enrolled for the test! "); 

            if mint { 
                info!(" Test Certificate resource address: {} ",self.test_certificate);
                info!(" Test Certificate id: {} ",nft_id.clone());

                // populate a TestCertificate stucture with relevant data
                let cert = TestCertificate {
                    uri: test_data.uri,
                    pro_academy_address: test_data.pro_academy_address,
                    sbt_address: test_data.user_sbt_address,
                    sbt_id: test_data.user_sbt_id,
                    course_name: test_data.course_name,
                    test_name: test_data.test_name,
                    course_number: test_data.course_number,
                    test_number: test_data.test_number,
                    test_date: test_data.test_date,
                    test_passed: test_data.test_passed,
                    score: test_data.score 
                };

                // mint a TestCertificate NFT
                self.minter_badge.authorize(|| { 
                    borrow_resource_manager!(self.test_certificate).mint_non_fungible(&nft_id,cert)
                })  
            } else {
                Bucket::new(RADIX_TOKEN)
            }
        }   

        // Method callable by student to collect achieved degree once successfully passed all 
        // programmed tests within a study course. 
        // Relative course number and student name as well as bundle of all passed TestCertificate  
        // NFTs within same course are required. Owned SBT proof is also required for verification 
        // purpose. At the end of a check series to verify eligibility a Degree NFT is minted and
        // returned and student SBT data is updated to reflect study title degree assigned.
        pub fn collect_degree(
            &mut self, 
            course_nr: u32,                             // number of course
            student_name: String,                       // graduate student name
            cert_bundle: Vec<Bucket>,                   // TestCertificate NFTs bundle 
            student_sbt: Proof                          // student SBT proof 
        ) -> Bucket {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[collect_degree]: NFT SBT Proof authorization failed! ");
        
            // Retrievev course data from relative map
            let key = student_sbt.non_fungible::<UserSBT>().id();
            let mut test_sum = 0;
            let mut founded = false;
            let mut title : Vec<String> = Vec::new();
            let mut start = 0;
            let teaching_subject = title.clone();
            for (course_nmbr, end, duration, test_nr, course_name, _id) in self.course_vec.iter() {
                if *course_nmbr == course_nr { 
                    assert!(Runtime::current_epoch()>*end,"[collect_degree]:Course still running!");
                    assert!(*test_nr == cert_bundle.len() as u8,"[collect_degree]:Certificate sum issue!");
                    test_sum = *test_nr;
                    title.push(course_name.to_string());
                    if end > duration { 
                        start = end-duration;
                    }
                    founded = true;
                }
            }
            assert!(founded," [collect_degree]:Course data missing!");

            // Verify provided TestCerticate NFTs 
            let mut vec_number: Vec<u8> = Vec::new();
            let mut response_vec: Vec<bool> = Vec::new();
            let mut laude = false;
            let mut score = 0;
            let mut uri = "".to_string();
            for cert_nft in cert_bundle.into_iter() {
                let data: TestCertificate = cert_nft.non_fungible().data();
                assert_eq!(
                    Runtime::actor().as_component().0 == data.pro_academy_address,
                    course_nr == data.course_number,
                    "[collect_degree]:Test data uncorrespondence detected!"
                );
                assert_eq!(
                    student_sbt.resource_address() == data.sbt_address,
                    key.clone() == data.sbt_id,
                    "[collect_degree]:Student SBT data uncorrespondence detected!"
                );
                assert!(data.test_passed," [collect_degree]:Test evaluation issue!");
                response_vec.push(data.test_passed);
                assert!(
                    !vec_number.contains(&data.test_number),
                    " [collect_degree]:Duplicate certificate detected!"
                );
                vec_number.push(data.test_number);
                score += data.score;
                if founded {
                    uri = data.uri;
                    founded = false;
                }
                // burn TestCerticate NFT provided by student 
                self.minter_badge.authorize(|| {cert_nft.burn()});
            }
            if response_vec.iter().all(|x| *x == true ) {
                founded = true;
            } else {
                info!("[collect_degree]:Test must result all passed!");
                std::process::abort()
            }
            let gpa = score/test_sum;
            if gpa == 5 {
                laude = true;
            } 

            if founded {
                // Save accrued degree and student SBT identificative data within related map
                let nft_id = NonFungibleId::random();
                let degree = title[0].clone();
                match self.degree_map.get_mut(&(course_nr,start,self.degree_nft,degree.clone())) {
                    Some(v) => v.push((nft_id.clone(),student_sbt.resource_address(),key.clone())),
                    _ => {
                        let mut v: Vec<(NonFungibleId,ResourceAddress,NonFungibleId)> = Vec::new();
                        v.push((nft_id.clone(),student_sbt.resource_address(),key.clone()));
                        self.degree_map.insert((course_nr,start,self.degree_nft,degree),v);
                    }
                }    

                // populate DegreeNFT structure with relevant data
                let degree_nft = DegreeNFT {
                    uri: uri,
                    pro_academy_address: Runtime::actor().as_component().0,
                    user_sbt_address: student_sbt.resource_address(),
                    user_sbt_id: key.clone(),
                    user_name: student_name,
                    degree_name: title.clone(),
                    mint_date: Runtime::current_epoch(),
                    teaching_subject: teaching_subject,
                    grade_point_avg: gpa,
                    cum_laude: laude
                };

                info!(" Degree Certificate resource address: {} ",self.degree_nft);
                info!(" Degree Certificate id: {} ",nft_id.clone()); 

                // update related data within student SBT
                let student_sbt_nft = student_sbt.non_fungible::<UserSBT>();
                let mut student_data = student_sbt_nft.data();

                student_data.educational_degrees
                    .push((self.degree_nft,nft_id.clone(),degree_nft.clone()));
       
                self.sbt_updater_badge.authorize(|| {
                    borrow_resource_manager!(student_sbt.resource_address())
                        .update_non_fungible_data(&key.clone(), student_data)
                });

                // mint a DegreeNFT
                self.minter_badge.authorize(|| { 
                    borrow_resource_manager!(self.degree_nft).mint_non_fungible(&nft_id,degree_nft)
                })  
            } else {
                Bucket::new(RADIX_TOKEN)
            }
        }    

        // Method callable by an authorized external component required to provide 
        // authorization badge proof and component resource address asa index.
        // Method returns teacher's data stored within academy.
        // Teacher SBT data is required asinput.
        pub fn ask_teacher(
            &mut self, 
            teacher_sbt_addr: ResourceAddress, 
            teacher_sbt_id: NonFungibleId,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) -> Tup {
            // Verify if Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!("[ask_teacher]: Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            }
            let mut tup = Tup::new();
            match self.teacher_map.get(&(teacher_sbt_addr,teacher_sbt_id.clone())) {
                Some(v) => {
                    tup.tuple.0 = v.0;
                    tup.tuple.1 = v.1.clone();
                    tup.tuple.2 = v.2.clone();
                    info!(" Teacher NFT resource address: {} ",v.0);
                    info!(" Teacher NFT id: {} ",v.1);
                    info!(" Teacher Degree NFT data: {:?} ",v.2);
                }
                _ => {
                    info!("[ask_teacher]: Teacher Data unfound! ");
                    std::process::abort()
                }
            }

            tup
        }

        // Method callable by an authorized external component required to provide 
        // authorization badge proof and component resource address asa index.
        // Method returns student Tests data achieved within academy.
        // Student SBT data is required asinput.
        pub fn ask_test(
            &mut self, 
            student_sbt_addr: ResourceAddress, 
            student_sbt_id: NonFungibleId,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) -> Vec<(u32,u8,u64,ResourceAddress,NonFungibleId,bool,u8)> {
            // Verify if Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!("[ask_test]: Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            }
            let mut vec_data = Vec::new();
            match self.test_map.get(&(student_sbt_addr,student_sbt_id.clone())) {
                Some(v) => {
                    for tup in v {
                        info!(" Course number: {} ",tup.0);
                        info!(" Test number: {} ",tup.1);
                        info!(" Test deadline: {} ",tup.2);
                        info!(" Test Certificate resource address: {} ",tup.3);
                        info!(" Test Certificate id: {} ",tup.4);
                        info!(" Test evaluation: {} ",tup.5);
                        info!(" Test score: {} ",tup.6);
                        vec_data.push(tup.clone());
                    }
                }
                _ => {
                    info!("[ask_test]: Test Data unfound! ");
                    std::process::abort()
                }
            }

            vec_data
        }

        // Method callable by an authorized external component required to provide 
        // authorization badge proof and component resource address asa index.
        // Method returns student Degrees data achieved within academy.
        // Student SBT data is required asinput.
        pub fn ask_degree(
            &mut self, 
            student_sbt_addr: ResourceAddress, 
            student_sbt_id: NonFungibleId,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) -> Vec<(u32,u64,ResourceAddress,String,NonFungibleId)> {
            // Verify if Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!("[ask_degree]: Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            }

            let mut vec_data = Vec::new();
            for (key,value) in self.degree_map.iter() {
                for tuple in value {
                    if tuple.1 == student_sbt_addr && tuple.2 == student_sbt_id {
                        info!(" Course number: {} ",key.0);
                        info!(" Course date: {} ",key.1);
                        info!(" Degree NFT resource address: {} ",key.2);
                        info!(" Degree NFT id: {} ",tuple.0.clone());
                        info!(" Degree title: {} ",key.3.clone());
                        vec_data.push((key.0,key.1,key.2,key.3.clone(),tuple.0.clone()));
                    }
                }
            }

            vec_data
        }

        // Method callable by an authorized external component required to provide 
        // authorization badge proof and component resource address asa index.
        // Method returns student Test data referred to a determinated course number,
        // test number and student SBT data.
        pub fn get_test_nft(
            &mut self, 
            course_nr: u32, 
            test_nr: u8,
            student_sbt_addr: ResourceAddress, 
            student_sbt_id: NonFungibleId,
            caller_cmp_addr: ComponentAddress,
            auth_ref: Proof
        ) -> (TestZero,Bucket) {
            // Verify if Caller Component is authorized 
            let auth_ref: ValidatedProof = auth_ref.unsafe_skip_proof_validation();
            match self.caller_map.get(&caller_cmp_addr){
                Some(addr) => assert!(*addr == auth_ref.resource_address()),
                None => {
                    info!("[get_test_nft]: Upgrade Badge authorization failed! ");       
                    std::process::abort()     
                } 
            }

            let mut test_vec = TestZero::new();
            test_vec.test_zero.clear();
            let mut nft_bckt = self.test_vaults.get_mut(&(course_nr,test_nr)).unwrap().take_all();
            loop {
                let nft = nft_bckt.take(dec!("1"));
                let data: Test = nft.non_fungible().data();
                if student_sbt_addr == data.user_sbt_address && student_sbt_id == data.user_sbt_id {                
                    info!(" ============================================== ");
                    info!(" Test URI: {} ",data.uri);
                    info!(" Academy component address: {} ",data.pro_academy_address);
                    info!(" Course name: {} ",data.course_name);
                    info!(" Course number: {} ",data.course_number);
                    info!(" Test name: {} ",data.test_name);
                    info!(" Test number: {} ",data.test_number);
                    info!(" Test date: {} ",data.test_date);
                    info!(" Assertions: {:?} ",data.assertions);
                    info!(" Student answers: {:?} ",data.answers);
                    info!(" Right answers: {:?} ",data.right_answers);
                    info!(" Test passed: {} ",data.test_passed);
                    info!(" Test score: {} ",data.score);
                    test_vec.test_zero.push(data.clone());
                }
                self.test_vaults.get_mut(&(course_nr,test_nr))
                    .expect(" [get_test_nft]: Data extract process failed! ")
                    .put(nft);

                if nft_bckt.amount() == dec!(0) {
                    break;
                }
            }

            (test_vec,nft_bckt)
        }

        // Method callable by student to retrieve data about his achieved educational degrees
        // registered within his own SBT, required as proof. 
        pub fn ask_degree_sbt(&mut self, student_sbt: Proof) {
            // Check student SBT proof provided
            let student_sbt: ValidatedProof = student_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[ask_degree_sbt]: Invalid proof provided");

            let id = student_sbt.non_fungible::<DegreeNFT>().id();
            let data: UserSBT = student_sbt.non_fungible().data();

            info!(" Student SBT NFT resource address: {} ",student_sbt.resource_address());
            info!(" Student SBT NFT id: {} ",id);
            for tuple in data.educational_degrees.clone() {
                info!(" =============================================== "); 
                info!(" Student NFT Degree resource address: {} ",tuple.0);
                info!(" Student NFT Degree id: {} ",tuple.1);
                info!(" Student NFT Degree data: {:?} ",tuple.2);
            }
        }

        // Method callable by teacher to retrieve data about his achieved educational degrees
        // registered within his own SBT, required as proof. 
        pub fn ask_degree_sbt_teacher(&mut self, teacher_sbt: Proof) {
            // Check teacher SBT proof provided
            let teacher_sbt: ValidatedProof = teacher_sbt
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.user_sbt,
                    dec!("1"),
                ))
                .expect("[ask_degree_sbt_teacher]: Invalid proof provided");

            let id = teacher_sbt.non_fungible::<DegreeNFT>().id();
            let data: UserSBT = teacher_sbt.non_fungible().data();

            info!(" Teacher SBT NFT resource address: {} ",teacher_sbt.resource_address());
            info!(" Teacher SBT NFT id: {} ",id);
            for tuple in data.educational_degrees.clone() {
                info!(" =============================================== "); 
                info!(" Teacher NFT Degree resource address: {} ",tuple.0);
                info!(" Teacher NFT Degree id: {} ",tuple.1);
                info!(" Teacher NFT Degree data: {:?} ",tuple.2);
            }
        }

            // Build a Caller Component Badge Resource 
            fn build_badge(&mut self, name: String) -> ResourceAddress {
                ResourceBuilder::new_fungible()
                .metadata("name", format!("{}",name))
                .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                .no_initial_supply()
            }
    }
}

