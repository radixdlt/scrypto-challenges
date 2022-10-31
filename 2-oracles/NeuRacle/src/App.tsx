import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction} from 'pte-browser-extension-sdk'
import Notiflix from 'notiflix'
import { TESTER, ADMINBADGE, PACKAGE, NEURA, COMPONENT, VALIDATOR_BADGE, USER_BADGE, MINT_CONTROLLER_BADGE, CONTROLLER_BADGE} from './NEURACLE'

function App() {
  const [accountAddress, setAccountAddress] = useState<string>('')
  const lightgreen = { color: 'lightgreen' }
  const lightblue = { color: 'lightblue' }
  const [yourRole, setYourRole] = useState<string>("Visitor")
  const [validatorInfo, setValidatorInfo] = useState<Array<string>>([])
  const [userInfo, setUserInfo] = useState<Array<Array<string>>>([])
  const [tokenInfo, setTokenInfo] = useState<string>('')
  const [validator, setValidator] = useState<string>('')
  const [showStaker, setShowStaker] = useState<boolean>(false)
  const [stakerInfo, setStakerInfo] = useState<Array<string>>([])
  const [stakerBadge, setStakerBadge] = useState<string>('')
  const [refresh, setRefresh] = useState<boolean>(false)

  function Show_each_staked_info(): JSX.Element {
    if (!stakerInfo.length) {
      return <div>
        You haven't staked in this validator
      </div>
    }
    else {
        return (<div style={{ border: '3px solid cyan' }}><br />
          Validator Name: {stakerInfo[1]}
          <br /><br />
          Current staked: {stakerInfo[5] + " NAR"}
          <br /><br />
          Current unstaking: {stakerInfo[2].replace(/^\D+|\D+$/g, "")}
          <br /><br />
          Estimated unstaking done in epoch: {stakerInfo[3]}
          <br /><br />
          Avaiable for withdraw: {stakerInfo[4].replace(/^\D+|\D+$/g, "")}
          <br /><br /></div>)
    }
  }

  function Show_stake_info() {
    if (showStaker == false) {
      return <div><button type="button" onClick={async function () {

        const Valid = prompt("Validator Address");
        Notiflix.Loading.pulse();
        if (Valid == null) {
          Notiflix.Loading.remove();
          return}
        else {
          const response = await fetch(

            `https://pte01.radixdlt.com/component/${COMPONENT}`

          )

          const parseData = await response.json()

          const parseValidatorAddress = JSON.parse(parseData.state).fields[2].elements

          const result = parseValidatorAddress.find((x: { elements: { value: string }[] }): boolean => x.elements[0].value === `ComponentAddress(\"${Valid}\")`)

          if (!result) {
            failure("Wrong validator address.")
          }
          else {
            
            success("Success!")

            const response2 = await fetch(`https://pte01.radixdlt.com/component/${Valid}`);

            const parseData = await response2.json();

            const parseStakerBadge = JSON.parse(parseData.state).fields[7].value;

            const my_badge: string = parseStakerBadge.substring(
              parseStakerBadge.indexOf('"') + 1, 
              parseStakerBadge.lastIndexOf('"')
            );

            setValidator(Valid!)
            setStakerBadge(my_badge)
            setShowStaker(true)
          }
          Notiflix.Loading.remove();
        }
      }}>
      Show staked amount
    </button></div>
    }
    else {
      return <div><button type="button" onClick={function () {
        setShowStaker(false);
        return 
      }}>
      Hide staked amount
    </button><br/><br/>
      <Stake_button /> | <button type="button" onClick={unstake}>
        Unstake
      </button> | <button type="button" onClick={stop_unstake}>
        Stop Unstake
      </button> | <button type="button" onClick={withdraw}>
        Withdraw
      </button>
      <br /><br />
      <Show_each_staked_info />
      <br /><br />
      </div>
    }
  }

  function Stake_button() {
    if (!stakerInfo.length) {
      return <button type="button" onClick={stake}>
      Stake
    </button>
    }
    else {
      return <button type="button" onClick={addstake}>
      Add Stake
    </button>
    }
  }

  function Show_info() {
    if (yourRole == 'NeuRacle Validator') {
      return <div><div style={{ border: '3px solid lightgreen', padding: '10px', margin: '10px auto'}}>
        Name: {validatorInfo![0]} <br /> Country: {validatorInfo![1]} <br /> Website: {validatorInfo![2]} <br /> Address: {validatorInfo![3]} 
      </div><br/></div>
    }
    else if (yourRole == 'NeuRacle User') { 
  
      const listItems = userInfo.map((x) =>
      <li key = {x[3]}><div style={{ border: '3px solid lightblue', maxWidth: '1000px', padding: '10px', margin: '10px auto', overflowWrap: 'anywhere' }}>
          Your data source: {x[0]} 
          <br /> This account have access until epoch {x[1]} 
          <br /> Your off-chain data: {x[2]}
          </div><br/></li>);
      return <div>{listItems}</div>
    }
    else return null
  }

  function Role_button() {
    if (yourRole == 'NeuRacle Admin') {
      return <div style={{ border: '3px solid lightgreen', padding: '20px', margin: '20px auto', textAlign: 'center' }}><a style={{color: 'lightblue'}}>ROLE BUTTON</a><br/><div style={{lineHeight: '10px'}}><br/></div>
      <button type="button" onClick={instantiate_new_neuracle}>
          Instantiate new NeuRacle
        </button><br/><div style={{lineHeight: '10px'}}><br/></div><button type="button" onClick={assign_validators}>
          Assign a validator
        </button>
      </div>
    }
    // else if (yourRole == 'NeuRacle Validator') {
    //   return <div><br/>
    //     <button type="button" onClick={function () { }}>
    //       Change fee
    //     </button><br/><button type="button" onClick={function () { }}>
    //       Withdraw fee
    //     </button>
    //   </div>
    // }
    else if (yourRole == 'TESTER') {
      return <div style={{ border: '3px solid lightgreen', padding: '20px', margin: '20px auto', textAlign: 'center' }}><a style={{color: 'lightblue'}}>ROLE BUTTON</a><br/><div style={{lineHeight: '10px'}}><br/></div><button type="button" onClick={publish_package}>
          Publish package
      </button><br/><div style={{lineHeight: '10px'}}><br/></div><button type="button" onClick={new_token}>
          Create new NeuRacle medium token
        </button></div>
    }
    else return null
  }

  function Visitor_info() {
    if (tokenInfo == '') { return <div> Loading... </div> }
    return <div style={{ textAlign: "center" }}>
      Current NAR on your wallet: {tokenInfo}
      <br /><br />
      <button type="button" onClick={become_user}>
        Become User
      </button>
      <br /><br />
      <Show_stake_info />
      <br /><br />
    </div>
  }

  async function get_nft_data(non_fungible_ids: string, resource: string): Promise<Array<string>> {

    const response = await fetch(
      `https://pte01.radixdlt.com/non-fungible/${resource}${non_fungible_ids}`
    );

    let info: Array<string> = [];
    const nonFungibleData = await response.json();

    const data = JSON.parse(nonFungibleData.immutable_data).fields;

    data.forEach((x: { value: string }) => {
      info.push(x.value);
    });

    const data2 = JSON.parse(nonFungibleData.mutable_data).fields
    data2.forEach((x: { value: string }) => {
      info.push(x.value);
    });
    return info

  }

  async function data() {
    Notiflix.Loading.pulse();
      try {

          let account = await getAccountAddress()

          setAccountAddress(account)

          const response = await fetch(`https://pte01.radixdlt.com/component/${account}`)

          const component = await response.json()

          const my_resource = component.owned_resources

          const token_nar = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === NEURA
          })

          if (token_nar) {
            setTokenInfo(token_nar.amount)
          } else { setTokenInfo('0')} ;

          const admin = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === ADMINBADGE
          })
          const validator = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === VALIDATOR_BADGE
          })
          const user = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === USER_BADGE
          })

          if (admin) {
            setYourRole("NeuRacle Admin")
          }

          else if (validator) {
            

            setYourRole("NeuRacle Validator")

            setValidatorInfo(await get_nft_data(validator.non_fungible_ids[0], VALIDATOR_BADGE))

          }

          else if (user) {
            
            const user_infos: string[][] = []
            setYourRole("NeuRacle User")
            
            for (const x of user.non_fungible_ids) {

              const user_info = await get_nft_data(x, USER_BADGE)

              try {
                const response = await fetch(user_info[0], {method: 'GET'})

              if (response.ok) {

                const your_data = await response.json()

                const result = JSON.stringify(your_data)

                user_info.push(result, x)

              } else {

                user_info.push("This data source is inaccessible", x)
                
              }

              } catch {

                user_info.push("This data source is inaccessible", x)

              }

            user_infos.push(user_info)

            }
            setUserInfo(user_infos)
            } 

          else if (accountAddress == TESTER) {
            setYourRole("TESTER")
          } else { setYourRole("Visitor")} 

          if (stakerBadge !== undefined) {

            const staker = my_resource.find((resource: { resource_address: string} ) => {
              return resource.resource_address === stakerBadge
            })

            if (staker) {

              const staker_info = await get_nft_data(staker.non_fungible_ids[0], stakerBadge);

              const response = await fetch(

                `https://pte01.radixdlt.com/component/${staker_info[0]}`
  
              )
              
              const parseData = await response.json()
  
              const parseNonFungibleId = JSON.parse(parseData.state).fields[0].elements
  
              const idx = parseNonFungibleId.findIndex((nonfgb: { value: string} ) => {
  
                return nonfgb.value === `NonFungibleId("${staker.non_fungible_ids[0]}")`
  
              })
  
              const staked_amount = parseNonFungibleId[idx + 1].value.replace(/^\D+|\D+$/g, "")

              staker_info.push(staked_amount)
  
              setStakerInfo(staker_info)

            } else {
              setStakerInfo([])
            }
          } 
      } catch { failure("Something wrong!")
      }
    
    Notiflix.Loading.remove()
  }

  async function publish_package() {

    const response = await fetch('./neuracle.wasm');
    const wasm = new Uint8Array(await response.arrayBuffer());

    const manifest = new ManifestBuilder()
      .publishPackage(wasm)
      .build()
      .toString();

    const receipt = await signTransaction(manifest);
  
    const newpack: string = receipt.newPackages[0];
    success("Done!");
    info("Change the value", "New package address: " + newpack + ". " + "\nPlease add this on NEURACLE.tsx")
    setRefresh(true);
    
  }

  async function new_token() {
    const manifest = new ManifestBuilder()
      .callFunction(PACKAGE, 'NeuraToken', 'new_token', ['"Neura"', '"NAR"', 'Decimal("10000000")', '18u8'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString()

    const receipt = await signTransaction(manifest)

    let log = ''
    for (const x of receipt.logs) {
      log += x + '. '
    }
 
    if (receipt.status == 'Success') {
      success("Done!")
      info("Change the value", 'You have created new NeuRacle medium token, please check your wallet detail in Pouch. You must edit the NEURACLE.tsx file. ' + log)
    }
    else {
      failure_big("Failed", "Please try again: " + receipt.status)
    }
    setRefresh(true);
  }

  async function instantiate_new_neuracle() {

    const manifest = new ManifestBuilder()
      .withdrawFromAccountByAmount(accountAddress!, 1, MINT_CONTROLLER_BADGE)
      .takeFromWorktop(MINT_CONTROLLER_BADGE, 'mint_badge')
      .callFunction(PACKAGE, 'NeuRacle', 'new', [`ResourceAddress("${NEURA}")`, `ResourceAddress("${ADMINBADGE}")`, 'Bucket("mint_badge")', `ResourceAddress("${CONTROLLER_BADGE}")`, '100u32', '1u64', 'Decimal("1")', 'Decimal("0.3")', '500u64', 'Decimal("0.0015")', 'Decimal("10")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString()

    const receipt = await signTransaction(manifest)

    var log = '\n'

    for (const x of receipt.logs) {
      log += x + '. \n'
        }
 
    if (receipt.status == 'Success') {
      success("Done!");
      info("Change the value", 'You have instantiated new NeuRacle Component. You must edit the NEURACLE.tsx file. ' + log)
    }
    else {
      failure_big("Failed", "Please try again: " + receipt.logs.toString())
    }
    setRefresh(true);
   
  }


  async function assign_validators() {

    if (yourRole == "NeuRacle Admin") {

      const result = prompt("Validator Account Address");
      if (result == null) {
        return
      } else {
        const validator_account_address: string = result;
        const result2 = prompt("Validator Name");
        if (result2 == null) {
          return
        } else {
          const validator_name: string = result2;
          const result3 = prompt("Validator Country");
          if (result3 == null) {
            return
          } else {
            const validator_country: string = result3;
            const result4 = prompt("Validator Website");
            if (result4 == null) {
              return
            } else {
              const validator_website: string = result4;
              const result5 = prompt("Validator Fee");
              if (result5 == null) {
                return
              } else {
                const validator_fee: string = result5;
                const manifest = new ManifestBuilder()
                  .withdrawFromAccountByAmount(accountAddress!, 1, ADMINBADGE)
                  .takeFromWorktop(ADMINBADGE, 'bucket')
                  .createProofFromBucket('bucket', 'admin_proof')
                  .pushToAuthZone('admin_proof')
                  .callMethod(COMPONENT, 'create_new_validator_node', [`"${validator_name}" "${validator_country}" "${validator_website}" Decimal("${validator_fee}")`])
                  .takeFromWorktopByAmount(1, VALIDATOR_BADGE, 'val1')
                  .callMethod(validator_account_address, 'deposit', ['Bucket("val1")'])
                  .callMethodWithAllResources(accountAddress!, 'deposit_batch')
                  .build()
                  .toString();

                const receipt = await signTransaction(manifest);

                if (receipt.status == 'Success') {
                  success_big("Done!", "The address you provided has been assigned as NeuRacle Validator.");
                } else {
                  failure_big("Failed", "Please try again: " + receipt.status);
                }
              }
            }
          }
        }
      }
    }
    else {
      failure_big("Failed", "You are not NeuRacle Admin")
    }
    setRefresh(true);
  }

  async function become_user() {
    const result = prompt("Your data source")
    if (result !== null) {
      const result2 = prompt("Payment amount")
      if (result2 !== null) {
        const amount = parseFloat(result2)
        if (parseFloat(tokenInfo) < amount) {
          failure("Not enough token in wallet!")
          return
        } else {
          const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NEURA)
          .takeFromWorktop(NEURA, 'bucket')
          .callMethod(COMPONENT, 'become_new_user', [`Bucket("bucket") "${result}"`])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", '' + receipt.logs.toString());
        }
        setRefresh(true)
        }
        
      }
      return
    }
    return
  } 

  async function stake() {
      const result = prompt("How much NAR want to stake to this validator?");
      if (result == null) {
        return
      } else {
        const amount: number = parseFloat(result);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NEURA)
          .takeFromWorktop(NEURA, 'bucket')
          .callMethod(validator!, 'stake', ['Bucket("bucket")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", '' + receipt.logs.toString());
        }
        setRefresh(true)
      }
  }

  async function addstake() {
    const result = prompt("How much NAR want to add stake to this validator?");
      if (result == null) {
        return
      } else {
        const amount: number = parseFloat(result);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NEURA)
          .takeFromWorktop(NEURA, 'bucket')
          .withdrawFromAccountByAmount(accountAddress!, 1, stakerBadge!)
          .takeFromWorktop(stakerBadge!, 'bucket1')
          .callMethod(validator!, 'add_stake', ['Bucket("bucket") Bucket("bucket1")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", "Please try again: " + receipt.logs.toString());
        }
        setRefresh(true)
      }
  }

  async function unstake() {
      const result = prompt("How much NAR want to unstake from this validator?");
      if (result == null) {
        return
      } else {
        const amount: number = parseFloat(result);
        if (parseFloat(stakerInfo![5]) < amount){
          failure("You don't have enough NAR staked.")
        } else {

          const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, 1, stakerBadge!)
          .takeFromWorktop(stakerBadge!, 'bucket1')
          .callMethod(validator!, 'unstake', [`Decimal("${amount}") Bucket("bucket1")`])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", "Please try again: " + receipt.logs.toString());
        }
        }
        setRefresh(true)
      }
  }

  async function stop_unstake() {

        const manifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(accountAddress!, 1, stakerBadge!)
        .takeFromWorktop(stakerBadge!, 'bucket1')
        .callMethod(validator!, 'stop_unstake', [`Bucket("bucket1")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", "Please try again: " + receipt.logs.toString());
        }
        setRefresh(true)
  }

  async function withdraw() {
      const result = prompt("How much NAR want to withdraw from this validator?");
      if (result == null) {
        return
      } else {
        const amount: number = parseFloat(result);
        const manifest = new ManifestBuilder()
        .withdrawFromAccountByAmount(accountAddress!, 1, stakerBadge!)
        .takeFromWorktop(stakerBadge!, 'bucket1')
        .callMethod(validator!, 'withdraw', [`Decimal("${amount}") Bucket("bucket1")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
        } else {
          failure_big("Failed", "Please try again: " + receipt.logs.toString());
        }
        setRefresh(true) 
      }
  }

  function success_big(title: string, message: string) {
    Notiflix.Report.success(
      title,
      message,
      'Ok',
    )
  }

  function success(message: string) {
    Notiflix.Notify.success(message, {
      position: 'right-top',
      borderRadius: '10px',
      showOnlyTheLastOne: true
    })
  }

  function info(title: string, message: string) {
    Notiflix.Report.info(
      title,
      message,
      'Ok',
      function () {
      },
      {
        width: "525px",
        messageMaxLength: 1000,
      }
    )
  }

  function failure(message: string) {
    Notiflix.Notify.failure(message, {
      position: 'right-top',
      borderRadius: '10px',
      showOnlyTheLastOne: true
    })
  }

  function failure_big(title: string, message: string) {
    Notiflix.Report.failure(
      title,
      message,
      'Ok',
    )
  }

  useEffect(() => {
    setTimeout(() => {
      setRefresh(false);
      data();
    }, 100);
  }, [accountAddress, refresh]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Welcome to NeuRacle! (Incompleted)</p>
        <div>
          Install <a
            className="App-link"
            href="https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Radix Babylon PTE
          </a> PTE to getting started.
        </div>
        <p>
          Send your NAR token through <a
            className="App-link"
            href="https://plymth.github.io/pouch/"
            target="_blank"
            rel="noopener noreferrer"
          >Pouch</a>
        </p>
        <p>
          Hello <a style={lightblue}>{yourRole}</a> with account: "<a style={lightgreen}>{accountAddress}</a>"
        </p>
        <div >
          <Show_info />
        </div>
        <button type="button" onClick={() => { setShowStaker(false); setRefresh(true)}}>
          Refresh your data
        </button>
        <div>
          <Role_button />
        </div>
        <br />
        <Visitor_info />
      <p></p><p></p>
      </header>
    </div>
  )
}

export default App
