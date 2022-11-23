--------------------------------------------------------------------------------------------------
# Academy Test  
--------------------------------------------------------------------------------------------------

Purpose of test is illustrate how SBT and NFT can be used in educational field.
Specifically this test will graduate two students:
first one will achieve a General Construction Degree while second one will achieve a Master Architecture.
Both Degrees title will be used later on HouseHub protocol to submit house projects and the build them up. 

N.B."Instructions" data tabs within Transactions output as well as other empty fields have been intentionally obmitted.

-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	
> [Part_1](#part_1) . Neverland Academy Component and resource addresses and active list
>
> [Part_2](#part_2) . Hire a teacher specifying his SBT credentials, his Study title degrees as well as his teaching subjects. Start first course and test.
>
> [Part_3](#part_3) . General Construction course. Open course & published Test list consultation. Student sign up. Student run first test and submit answers. 
>
> [Part_4](#part_4) . Teacher evaluate first test and publish second one.
>
> [Part_5](#part_5) . View test answers. View test result. Test list consultation. Student run second test and provide answers. 
>
> [Part_6](#part_6) . Teacher evaluate second test. Student views test answers. View test result. Collect Degree and check it within student SBT.
>
> [Part_7](#part_7) . Architecture course. Teacher open study course in Architecture. Publish first test.   
>
> [Part_8](#part_8) . Open courses & published Test list consultation. Student sign up. Student run first test and submit answers.
>
> [Part_9](#part_9) . Teacher evaluate first test and publish second one.
>
> [Part_10](#part_10) . View test answers. View test result. Test list consultation. Student run second test and provide answers.
>
> [Part_11](#part_11) . Teacher evaluate second test. Student views test answers. View test result. Collect Degree and check it within student SBT.
>


#
### Part_1 
# Neverland Academy Component and resource addresses and active accounts list
-------------------------------------------------------------------------------------------

>```Neverland Academy Component```
```
└─ Component: component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y		Neverland Academy Component address
├─ Resource: resource_sim1qrpkt9wlm08n4z00ffe65ku4a62j485g0nlxnna99jesdd46c2		Neverland Academy MinterBadge resource
├─ Resource: resource_sim1qr7s9ayr765ce2llvd55ucpre7ggudxz6ayk5cludfus28zuyl		Neverland OwnerBadge resource address
├─ Resource: resource_sim1qqcstxhymrd6h4crsm5g878uqszxh3552gn8lw8u6mxqng98jf		Neverland TeacherBadge MinterBadge resource address
├─ Resource: resource_sim1qrzvr23pw2w4n6s7fj8hxwmvw6mpss3pr9tr68pla5vqryc65l		Neverland Academy Test Template resource address
├─ Resource: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu		Neverland Academy Test Certificate resource address
└─ Resource: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370		Neverland Academy Degree NFT resource address
```

>
> protocol's owner account
>
```
Account component address: account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3	
Public key: 0383ffb219b35c04f26db0a1e8efb9efec16fdd931aef837512bd60aa172342fa4
Private key: 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28
No configuration found on system. will use the above account as default.
```

>
> neverland's teacher account with SBT resource address & ID
>
```
Account component address: account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h	
Public key: 037a622c7d7f3730c1b1ec5b340f8ea78d040aba3af74e5df7bcf45284a1f01126
Private key: 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2

User SBT address added: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
User SBT id: 300710000000b7b0771cabf0e0cee8b06b94a47490e1
```

>
> neverland's general construction student account 
>
```
Account component address: account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra	
Public key: 035d9746555d52eab4484c63b6b4df25b2186d44969bbacfb923f5de99e00df733
Private key: 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2
```

>
> neverland's architecture student account 
>
```
Account component address: account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg	
Public key: 03e4d1962763c01d07d9432a22bd41c7c6685487d7aa0b0efba9ba8163435fb4d9
Private key: 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c
```

#
### Part_2 
## Hire a teacher specifying his SBT credentials, his Study title degrees as well as his teaching subjects. Mint a teacher badge to allow
## him to open new teaching courses, start new tests as well as evaluate them.
## Withdrawal teacher badge providing right SBT credentials. 
## Open first study course in General Construction.
## Publish first test.
-------------------------------------------------------------------------------------------

> Switch default account to protocol owner

```resim set-default-account account_sim1qwk73ye3gfmnxnw42jgpv3gey9jj8a50se753pvnccfquqkgk3 49b84fbf2a1e326872162f577133cc61d7886d084b48de3303300c0faafc7b28```


>cd academy_transaction_manifest

> update data on ```mint_teacher_badge```

>```mint_teacher_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.068127 XRD burned, 0.00340635 XRD tipped to validators
Cost Units: 100000000 limit, 681270 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Teacher Badge address: resource_sim1qqcstxhymrd6h4crsm5g878uqszxh3552gn8lw8u6mxqng98jf
└─ [INFO ]  Teacher Badge id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
```

>Withdrawal teacher badge
>Method callable by hired teacher to download his personal teacher badge NFT.
>Teacher must provide his SBT credentials.

> Switch default account  to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


>update data on ```withdrawal_teacher_badge```

>```withdrawal_teacher_badge.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.114731 XRD burned, 0.00573655 XRD tipped to validators
Cost Units: 100000000 limit, 1147310 consumed, 0.0000001 XRD per cost unit

```

> Check Degree Certificate within teacher SBT 

> update data on ```ask_degree_sbt_teacher```

>```ask_degree_sbt_teacher.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2293877 XRD burned, 0.011469385 XRD tipped to validators
Cost Units: 100000000 limit, 2293877 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Teacher SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Teacher SBT NFT id: 300710000000b7b0771cabf0e0cee8b06b94a47490e1
├─ [INFO ]  ====================================================================================================
├─ [INFO ]  Teacher NFT Degree resource address: resource_sim1qqcstxhymrd6h4crsm5g878uqszxh3552gn8lw8u6mxqng98jf
├─ [INFO ]  Teacher NFT Degree id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
└─ [INFO ]  Teacher NFT Degree data: DegreeNFT { uri: "https://teacher_badge_nft_url_pointer.com", 
	    pro_academy_address: component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y, 
	    user_sbt_address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f, 
	    user_sbt_id: 300710000000b7b0771cabf0e0cee8b06b94a47490e1, user_name: "Gamma Prof", 
	    degree_name: ["PhD Building Construction", "PhD Building Phisycs", "PhD Architecture"], mint_date: 0, 
	    teaching_subject: ["Bachelor General Contractor", "Bachelor Architecture", "Master Architecture"], 
	    grade_point_avg: 5, cum_laude: true }

```

> Open first study course

> update data on ```open_course```

>```open_course.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0843463 XRD burned, 0.004217315 XRD tipped to validators
Cost Units: 100000000 limit, 843463 consumed, 0.0000001 XRD per cost unit
Logs: 4
├─ [INFO ]  Course number: 1
├─ [INFO ]  Course name: General Construction
├─ [INFO ]  Number of tests: 2
└─ [INFO ]  Course duration: 10000

```


> Publish first test

>update data on ```publish_test```

>```publish_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0940084 XRD burned, 0.00470042 XRD tipped to validators
Cost Units: 100000000 limit, 940084 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  Course number: 1
├─ [INFO ]  Course name: General Construction
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test name: General electric
└─ [INFO ]  Test deadline: 1000

```


[Back Up](#index)
#
### Part_3 
## Open courses & published Test list consultation. Student sign up.  
## Student run first test and submit answers.  
-------------------------------------------------------------------------------------------

> Switch default account to general construction student account	

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2```


> Open course list consultation

>update data on ```course_list```

>```course_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.059423 XRD burned, 0.00297115 XRD tipped to validators
Cost Units: 100000000 limit, 594230 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Course name: General Construction
├─ [INFO ]  Number of tests: 2
├─ [INFO ]  Course deadline: 10000
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37


```

> Published Test list consultation

> update data on ```test_list```

>```test_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0601478 XRD burned, 0.00300739 XRD tipped to validators
Cost Units: 100000000 limit, 601478 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: General electric
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 1000
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37

```

> Student sign up

>update data on ```sign_up```

>```sign_up.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0937476 XRD burned, 0.00468738 XRD tipped to validators
Cost Units: 100000000 limit, 937476 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ] [sign_up]: Student subscribed!


```

> Student run first test

> update data on ```run_test```

>```run_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.111448 XRD burned, 0.0055724 XRD tipped to validators
Cost Units: 100000000 limit, 1114480 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ]  =========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: General electric
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 1000
├─ [INFO ]  Test URI: https://www.general_contractor_test_pointer.com
├─ [INFO ]  =========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  =========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  =========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  =========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  =========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  =========================================================

```

> Student submit first test's answers

>update data on ```answer_test```

>```answer_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1181624 XRD burned, 0.00590812 XRD tipped to validators
Cost Units: 100000000 limit, 1181624 consumed, 0.0000001 XRD per cost unit



```

[Back Up](#index)
#
### Part_4 
# Teacher evaluate first test and publish second test
-------------------------------------------------------------------------------------------

> Switch default account to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


> Overtake test deadline

> ```resim set-current-epoch 1001```


> Evaluate first test

> update data on ```evaluate_test```		

>```evaluate_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1463436 XRD burned, 0.00731718 XRD tipped to validators
Cost Units: 100000000 limit, 1463436 consumed, 0.0000001 XRD per cost unit


```

> Publish second test

> update data on ```publish_test```		

>```publish_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1087723 XRD burned, 0.005438615 XRD tipped to validators
Cost Units: 100000000 limit, 1087723 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  Course number: 1
├─ [INFO ]  Course name: General Construction
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test name: materials science
└─ [INFO ]  Test deadline: 2001

```

[Back Up](#index)
#
### Part_5
# View test answers. View test result. Test list consultation. Student run second test and provide answers. 
-------------------------------------------------------------------------------------------

> Switch default account to general construction student account 	

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2```


> view first test answers

> update data on ```view_answers```

>```view_answers.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0878819 XRD burned, 0.004394095 XRD tipped to validators
Cost Units: 100000000 limit, 878819 consumed, 0.0000001 XRD per cost unit
Logs: 21
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: General electric
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 1000
├─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  Answer nr 1: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  Answer nr 2: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  Answer nr 3: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  Answer nr 4: false
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  Answer nr 5: false

```


> Student views test result

> update data on ```test_result```

>```test_result.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1396454 XRD burned, 0.00698227 XRD tipped to validators
Cost Units: 100000000 limit, 1396454 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Test passed! Score: 5
├─ [INFO ]  Test Certificate resource address: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu
└─ [INFO ]  Test Certificate id: 300710000000b4260a792f7ccaba240680d21b696c32

```


> Published Test list consultation

>```test_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0755379 XRD burned, 0.003776895 XRD tipped to validators
Cost Units: 100000000 limit, 755379 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: materials science
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 2001
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37

```

> Student run second test

> update data on ```run_test```

>```run_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1221352 XRD burned, 0.00610676 XRD tipped to validators
Cost Units: 100000000 limit, 1221352 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: materials science
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 2001
├─ [INFO ]  Test URI: https://www.general_contractor_test_pointer.com
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  ========================================================

```

> Student submit second test's answers

>update data on ```answer_test```

>```answer_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1282938 XRD burned, 0.00641469 XRD tipped to validators
Cost Units: 100000000 limit, 1282938 consumed, 0.0000001 XRD per cost unit



```

[Back Up](#index)
#
### Part_6 
# Teacher evaluate second test. Student views test answers. View test result. Collect Degree and check it within student SBT.
-------------------------------------------------------------------------------------------

> Switch default account to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


> Overtake test deadline

> ```resim set-current-epoch 2002```


> Evaluate second test

> update data on ```evaluate_test```		

>```evaluate_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1578609 XRD burned, 0.007893045 XRD tipped to validators
Cost Units: 100000000 limit, 1578609 consumed, 0.0000001 XRD per cost unit


```

> Switch default account to general construction student account 	

```resim set-default-account account_sim1qwa6y5h0nqzmuh8thmj4epllg86svxchhqp9ck3hr9sqxq0yra 74c859f5f25c098c96d0f4d961ab023b0e8fb931cb5a436865a394e4dff119f2```


> view second test answers

> update data on ```view_answers```

>```view_answers.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0895424 XRD burned, 0.00447712 XRD tipped to validators
Cost Units: 100000000 limit, 895424 consumed, 0.0000001 XRD per cost unit
Logs: 21
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Test name: materials science
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 2001
├─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  Answer nr 1: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  Answer nr 2: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  Answer nr 3: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  Answer nr 4: false
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  Answer nr 5: false

```


> Student views second test result

> update data on ```test_result```

>```test_result.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1399362 XRD burned, 0.00699681 XRD tipped to validators
Cost Units: 100000000 limit, 1399362 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Test passed! Score: 5
├─ [INFO ]  Test Certificate resource address: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu
└─ [INFO ]  Test Certificate id: 300710000000764a9b56c0b600ca5699d71a57e3e081

```

> Student collect his achieved degree providing his Test Certificate NFTs

> update data on ```collect_degree```

>```collect_degree.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2171223 XRD burned, 0.010856115 XRD tipped to validators
Cost Units: 100000000 limit, 2171223 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Degree Certificate resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370	
└─ [INFO ]  Degree Certificate id: 300710000000d379d1ad6cedb9c2e17e8a4263ba8bee
```


> Check collected Degree Certificate within student SBT 

> update data on ```ask_degree_sbt```

>```ask_degree_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2254694 XRD burned, 0.01127347 XRD tipped to validators
Cost Units: 100000000 limit, 2254694 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Student SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Student SBT NFT id: 300710000000985f0bf0a4fbceed30b544a167858b01
├─ [INFO ]  ====================================================================================================
├─ [INFO ]  Student NFT Degree resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370
├─ [INFO ]  Student NFT Degree id: 300710000000d379d1ad6cedb9c2e17e8a4263ba8bee
└─ [INFO ]  Student NFT Degree data: DegreeNFT { uri: "https://www.general_contractor_test_pointer.com", 
            pro_academy_address: component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y, 
            user_sbt_address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f, 
	    user_sbt_id: 300710000000985f0bf0a4fbceed30b544a167858b01, user_name: "gamma.xrd", 
            degree_name: ["General Construction"], mint_date: 2002, 
	    teaching_subject: [], grade_point_avg: 10, cum_laude: true }
```


[Back Up](#index)
#
### Part_7 
# Architecture course. Teacher open study course in Architecture. Publish first test.  
----------------------------------------------------------------------------------------------------------------------

> Switch default account to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


> Open a study course

> update data on ```open_course```

>```open_course.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1558412 XRD burned, 0.00779206 XRD tipped to validators
Cost Units: 100000000 limit, 1558412 consumed, 0.0000001 XRD per cost unit
Logs: 4
├─ [INFO ]  Course number: 3
├─ [INFO ]  Course name: Master Architecture
├─ [INFO ]  Number of tests: 2
└─ [INFO ]  Course duration: 10000

```


> Publish first test

>update data on ```publish_test```

>```publish_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.162069 XRD burned, 0.00810345 XRD tipped to validators
Cost Units: 100000000 limit, 1620690 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  Course number: 3
├─ [INFO ]  Course name: Master Architecture
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test name: Phisics and Mathematics
└─ [INFO ]  Test deadline: 4003

```


[Back Up](#index)
#
### Part_8 
# Open courses & published Test list consultation. Student sign up. Student run first test and submit answers. 
-------------------------------------------------------------------------------------------

> Switch default account to architecture student account 	

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```


> Open course list consultation

>update data on ```course_list```

>```course_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1371712 XRD burned, 0.00685856 XRD tipped to validators
Cost Units: 100000000 limit, 1371712 consumed, 0.0000001 XRD per cost unit
Logs: 12
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 1
├─ [INFO ]  Course name: General Construction
├─ [INFO ]  Number of tests: 2
├─ [INFO ]  Course deadline: 10000
├─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Course name: Master Architecture
├─ [INFO ]  Number of tests: 2
├─ [INFO ]  Course deadline: 13003
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37


```

> Published Test list consultation

> update data on ```test_list```

>```test_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1380375 XRD burned, 0.006901875 XRD tipped to validators
Cost Units: 100000000 limit, 1380375 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Phisics and Mathematics
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 4003
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37

```

> Student sign up

>update data on ```sign_up```

>```sign_up.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1376446 XRD burned, 0.00688223 XRD tipped to validators
Cost Units: 100000000 limit, 1376446 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ] [sign_up]: Student subscribed!


```

> Student run first test

> update data on ```run_test```

>```run_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1579479 XRD burned, 0.007897395 XRD tipped to validators
Cost Units: 100000000 limit, 1579479 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Phisics and Mathematics
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 4003
├─ [INFO ]  Test URI: https://www.architecture_test_pointer.com
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  ========================================================

```

> Student submit first test's answers

>update data on ```answer_test```

>```answer_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1607787 XRD burned, 0.008038935 XRD tipped to validators
Cost Units: 100000000 limit, 1607787 consumed, 0.0000001 XRD per cost unit



```


[Back Up](#index)
#
### Part_9 
# Teacher evaluate first test and publish second one. 
-------------------------------------------------------------------------------------------

> Switch default account to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


> Overtake test deadline

> ```resim set-current-epoch 4004```


> Evaluate first test

> update data on ```evaluate_test```		

>```evaluate_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1905578 XRD burned, 0.00952789 XRD tipped to validators
Cost Units: 100000000 limit, 1905578 consumed, 0.0000001 XRD per cost unit


```

> Publish second test		

> update data on ```publish_test```		

>```publish_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1536694 XRD burned, 0.00768347 XRD tipped to validators
Cost Units: 100000000 limit, 1536694 consumed, 0.0000001 XRD per cost unit
Logs: 5
├─ [INFO ]  Course number: 3
├─ [INFO ]  Course name: Master Architecture
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test name: Drawing and Representation
└─ [INFO ]  Test deadline: 5004

```

[Back Up](#index)
#
### Part_10
# View test answers. View test result. Test list consultation. Student run second test and provide answers. 
-------------------------------------------------------------------------------------------

> Switch default account to architecture student account 	

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```


> view first test answers

> update data on ```view_answers```

>```view_answers.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1504858 XRD burned, 0.00752429 XRD tipped to validators
Cost Units: 100000000 limit, 1504858 consumed, 0.0000001 XRD per cost unit
Logs: 21
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Phisics and Mathematics
├─ [INFO ]  Test number: 1
├─ [INFO ]  Test deadline: 4003
├─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  Answer nr 1: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  Answer nr 2: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  Answer nr 3: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  Answer nr 4: false
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  Answer nr 5: false

```


> Student views test result

> update data on ```test_result```

>```test_result.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1998825 XRD burned, 0.009994125 XRD tipped to validators
Cost Units: 100000000 limit, 1998825 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Test passed! Score: 5
├─ [INFO ]  Test Certificate resource address: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu
└─ [INFO ]  Test Certificate id: 3007100000002bc0588fa7abbe731d9d5ad9b5d9c499

```


> Published Test list consultation

>```test_list.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0755379 XRD burned, 0.003776895 XRD tipped to validators
Cost Units: 100000000 limit, 755379 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Drawing and Representation
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 5004
└─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37

```

> Student run second test

> update data on ```run_test```

>```run_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.18884 XRD burned, 0.009442 XRD tipped to validators
Cost Units: 100000000 limit, 1888400 consumed, 0.0000001 XRD per cost unit
Logs: 17
├─ [INFO ]  ===================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Drawing and Representation
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 5004
├─ [INFO ]  Test URI: https://www.architecture_test_pointer.com
├─ [INFO ]  ===================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  ===================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  ===================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  ===================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  ===================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  ===================================================

```

> Student submit second test's answers

>update data on ```answer_test```

>```answer_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1954408 XRD burned, 0.00977204 XRD tipped to validators
Cost Units: 100000000 limit, 1954408 consumed, 0.0000001 XRD per cost unit



```


[Back Up](#index)
#
### Part_11 
# Teacher evaluate second test. Student views test answers. View test result. Collect Degree and check it within student SBT.
-------------------------------------------------------------------------------------------

> Switch default account to teacher account

```resim set-default-account account_sim1qw5hcjf6uya9en5z42k0jwvse3ew9pja6cft5esuhscq7dr54h 130d864897157b02d29ad98af94fca0215bc556336bf18d5994a58ad469fd1b2```


> Overtake test deadline

> ```resim set-current-epoch 5005```


> Evaluate second test

> update data on ```evaluate_test```		

>```evaluate_test.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2242073 XRD burned, 0.011210365 XRD tipped to validators
Cost Units: 100000000 limit, 2242073 consumed, 0.0000001 XRD per cost unit


```

> Switch default account to architecture student account 	

```resim set-default-account account_sim1qv2hppdw4cpdd5008uqznx4ttu8kcdetenzpqhl7j78sem5wxg 069704ddcdef50d6535fbb7b216f1bb64b97e072417654882ed6067a7dd2122c```


> view second test answers

> update data on ```view_answers```

>```view_answers.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.15528 XRD burned, 0.007764 XRD tipped to validators
Cost Units: 100000000 limit, 1552800 consumed, 0.0000001 XRD per cost unit
Logs: 21
├─ [INFO ]  ========================================================
├─ [INFO ]  Course number: 3
├─ [INFO ]  Test name: Drawing and Representation
├─ [INFO ]  Test number: 2
├─ [INFO ]  Test deadline: 5004
├─ [INFO ]  Teacher id: 300710000000b68dfedae4eadf9d7616ea754bafbb37
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 1: Green wire is Earth
├─ [INFO ]  Answer nr 1: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 2: Blue wire is Neutral
├─ [INFO ]  Answer nr 2: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 3: Brown wire is Line
├─ [INFO ]  Answer nr 3: true
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 4: Green & Yellow wire is Line
├─ [INFO ]  Answer nr 4: false
├─ [INFO ]  ========================================================
├─ [INFO ]  Assertion nr 5: Black wire is Earth
└─ [INFO ]  Answer nr 5: false

```


> Student views second test result

> update data on ```test_result```

>```test_result.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2031235 XRD burned, 0.010156175 XRD tipped to validators
Cost Units: 100000000 limit, 2031235 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Test passed! Score: 5
├─ [INFO ]  Test Certificate resource address: resource_sim1qqh9sk68463hn6l2x46f68paefxk28ss2lqeag4mxnmqdrgefu
└─ [INFO ]  Test Certificate id: 300710000000afeeab940d07b18949ef63b9065de0ad

```

> Student collect his achieved degree providing his Test Certificate NFTs

> update data on ```collect_degree```

>```collect_degree.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2825687 XRD burned, 0.014128435 XRD tipped to validators
Cost Units: 100000000 limit, 2825687 consumed, 0.0000001 XRD per cost unit
Logs: 2
├─ [INFO ]  Degree Certificate resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370
└─ [INFO ]  Degree Certificate id: 300710000000b64e9392d358c2388481975e9ef7124d

```

> Check collected Degree Certificate within student SBT 

> update data on ```ask_degree_sbt```

>```ask_degree_sbt.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.2234145 XRD burned, 0.011170725 XRD tipped to validators
Cost Units: 100000000 limit, 2234145 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Student SBT NFT resource address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f
├─ [INFO ]  Student SBT NFT id: 300710000000e492d1a29f9c41e86f8ba19215d0c46c
├─ [INFO ]  ====================================================================================================
├─ [INFO ]  Student NFT Degree resource address: resource_sim1qpyc88dfmeuvly4vu5ssdqnwzrpyzqrcp0juettpvttqp25370
├─ [INFO ]  Student NFT Degree id: 300710000000b64e9392d358c2388481975e9ef7124d
└─ [INFO ]  Student NFT Degree data: DegreeNFT { uri: "https://www.architecture_test_pointer.com", 
            pro_academy_address: component_sim1qtfvyec6hzfuvfrzr08yyd3ts4j5tavzeuzl7u0sj9qsdpls0y, 
            user_sbt_address: resource_sim1qz7wnl65aq9432pagchqjh4g56gssjzuath032qa0mvs6uw55f, 
            user_sbt_id: 300710000000e492d1a29f9c41e86f8ba19215d0c46c, 
            user_name: "Delta.xrd", 
            degree_name: ["Master Architecture"], mint_date: 5005, 
            teaching_subject: [], grade_point_avg: 5, cum_laude: true }

```


[Back Up](#index)





