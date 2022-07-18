

import { useEffect, useState } from 'react'
import neuracleLogo from './assets/neuracle.svg'
import './App.css'
import Notiflix from 'notiflix'
import { ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk'
import { CreditSBT, GroundCreditComponent, GroundLendingComponent, IDSBT, InstallmentCreditBadge, InstallmentCreditRequestBadge, LendingAccount, NeuRacle, StableCoin } from './assets/GROUND_ADDRESS'

function App() {
  const lightblue = { color: 'lightblue' }
  const red = { color: 'red' }
  const blanchedalmond = { color: 'blanchedalmond' }
  const [refresh, setRefresh] = useState<boolean>(false)
  const [accountAddress, setAccountAddress] = useState<string>('')
  const [tokenInfo, setTokenInfo] = useState<string>('')
  const [yourRole, setYourRole] = useState<string>('')
  const [lenderInfo, setLenderInfo] = useState<Array<Array<string>>>([])
  const [borrowerInfo, setBorrowerInfo] = useState<Array<string>>([])
  const [protocolInfo, setProtocolInfo] = useState<Array<number>>([])
  const YEAR = 60 * 60 * 24 * 365

  async function get_borrower_data(id_id: string, credit_id: string): Promise<Array<string>> {

    const response = await fetch(
      `https://pte01.radixdlt.com/non-fungible/${IDSBT}${id_id}`
    );

    const nonFungibleData = await response.json();

    const data = JSON.parse(nonFungibleData.mutable_data).fields[0].fields

    var maximum_allowance = parseFloat(data[1].value.replace(/^\D+|\D+$/g, "")) * parseFloat(data[2].value.replace(/^\D+|\D+$/g, "")) / 100

    const response2 = await fetch(
      `https://pte01.radixdlt.com/non-fungible/${CreditSBT}${credit_id}`
    );

    var allowance = maximum_allowance

    const nonFungibleData2 = await response2.json();

    const data2 = JSON.parse(nonFungibleData2.mutable_data).fields[0].fields
    
    const credit_score = parseFloat(data2[0].value.replace(/^\D+|\D+$/g, ""))

    maximum_allowance = maximum_allowance * credit_score / 100

    var credit_type = data2[1].name

    if (credit_type == "Revolving") {
      let name = data2[1].fields[0].name
      credit_type = name + ' ' + credit_type + " Credit"
      if (name == "Monthly") {
        maximum_allowance = maximum_allowance / 12
      }
    } else if (credit_type == "Installment") {
      credit_type = credit_type + " Credit"
      allowance = 0
    }

    const extra_debt = parseFloat(data2[6].value.replace(/^\D+|\D+$/g, ""))

    const current_debt = parseFloat(data2[3].value.replace(/^\D+|\D+$/g, ""))

    const total_debt = current_debt + parseFloat(data2[4].value.replace(/^\D+|\D+$/g, "")) + extra_debt

    const accumulated_repayment = parseFloat(data2[7].value.replace(/^\D+|\D+$/g, ""))

    const due_time = parseFloat(data2[5].value)

    var late = false

    if (extra_debt > 0) {
      late = true
      allowance = 0
    } else {
      if ((maximum_allowance > current_debt) && (allowance !== 0)) {
        allowance = maximum_allowance - current_debt
      } else {allowance = 0}
    }

    let info = [credit_type, credit_score, total_debt.toFixed(2), due_time, accumulated_repayment.toFixed(2), allowance.toFixed(2), maximum_allowance.toFixed(2), late]

    return info

  }

  async function data() {
    Notiflix.Loading.pulse();
    try {

      let account = await getAccountAddress()

      setAccountAddress(account)

      const response = await fetch(`https://pte01.radixdlt.com/component/${account}`)

      const account_component = await response.json()

      const my_resource = account_component.owned_resources

      const stable_coin = my_resource.find((resource: { resource_address: string} ) => {
        return resource.resource_address === StableCoin
      })

      if (stable_coin) {
        setTokenInfo(stable_coin.amount)
      } else { setTokenInfo('0')} 

      const response2 = await fetch(
        `https://pte01.radixdlt.com/component/${GroundLendingComponent}`
      );

      const component = await response2.json();

      const componentParse = JSON.parse(component.state).fields

      const lending_accounts = componentParse[7].elements;

      const account_number = lending_accounts.length / 2;

      var protocol_info = [account_number];

      const deposited = parseFloat(componentParse[4].value.replace(/^\D+|\D+$/g, ""))

      protocol_info.push(deposited)

      var protocol_vault_amount = 0

      for (const x of component.owned_resources) {
        if (x.resource_address == StableCoin) {
          protocol_vault_amount = protocol_vault_amount + parseFloat(x.amount)
        }
      }

      var risk = 0

      if (deposited !== 0) {
        risk = (1 - protocol_vault_amount / deposited) * 100
      }

      protocol_info.push(risk)

      const idx_rate = lending_accounts.findIndex((nonfgb: { value: string} ) => {
  
        return nonfgb.value === `NonFungibleId("36bc1d34f0012f58c5d7b142d6ed132d")`

      })

      let rate = lending_accounts[idx_rate + 1].fields[0].value.replace(/^\D+|\D+$/g, "")

      let current = Math.floor(Date.now() / 1000)

      const time_elapsed = current - componentParse[13].value

      const compound = YEAR / time_elapsed

      const apy = (Math.pow(rate, compound) - 1) * 100

      protocol_info.push(apy)

      const response3 = await fetch(
        `https://pte01.radixdlt.com/component/${NeuRacle}`
      );

      const neuracle = await response3.json();

      const neuracle_time = parseInt(JSON.parse(neuracle.state).fields[0].elements[1].value)

      protocol_info.push(neuracle_time)

      setProtocolInfo(protocol_info)

      if (yourRole == 'Lender') {

        const lender = my_resource.find((resource: { resource_address: string} ) => {
          return resource.resource_address === LendingAccount
        })

        if (lender) {
          const lender_infos: string[][] = []

          for (const x of lender.non_fungible_ids) {

            // const lender_info = await get_lender_data(x, LendingAccount)

            const idx = lending_accounts.findIndex((nonfgb: { value: string} ) => {
  
              return nonfgb.value === `NonFungibleId("${x}")`

            })

            let data = lending_accounts[idx + 1].fields

            lender_infos.push([x, data[0].value.replace(/^\D+|\D+$/g, ""), data[1].value])

          }

          setLenderInfo(lender_infos)

        }

      } else if (yourRole == 'Borrower') {

        const id = my_resource.find((resource: { resource_address: string} ) => {
          return resource.resource_address === IDSBT
        })

        if (id) {

          const borrower = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === CreditSBT
          })
  
          if (borrower) {
  
            let info = await get_borrower_data(id.non_fungible_ids[0], borrower.non_fungible_ids[0])
            
            setBorrowerInfo(info)
  
          } else {
            failure_big('Cannot find Credit SBT', "You don't have a Credit, please contact admin for a Credit!")
            setYourRole('')
            setRefresh(true)
          }
  

        } else {
          failure_big('Cannot find Credit SBT', "You don't have a Credit, please contact admin for a Credit!")
          setYourRole('')
          setRefresh(true)
        }

        
      }

    } catch { failure("Something wrong!")}

    Notiflix.Loading.remove()
  }

  function success_big(title: string, message: string) {
    Notiflix.Report.success(
      title,
      message,
      'Ok',
    )
  }

  // function success(message: string) {
  //   Notiflix.Notify.success(message, {
  //     position: 'right-top',
  //     borderRadius: '10px',
  //     showOnlyTheLastOne: true
  //   })
  // }

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

  function Role_name() {
    if (yourRole == 'Borrower') {
      return <><a style={blanchedalmond}>{yourRole} </a> with </>
    }
    else if (yourRole == 'Lender') { 
  
      return <><a style={lightblue}>{yourRole}</a> with </>
    }
    else return null
  }

  function Refresh_button() {
    if (yourRole == '') {
      return <><div className="role-button">
        <button type="button" onClick={() => { setYourRole('Lender'), setRefresh(true) } }>
          <a className='role-button lender'>Earn on Ground Finance</a>
        </button>&nbsp;&nbsp;&nbsp;&nbsp;<button type="button" onClick={() => { setYourRole('Borrower'), setRefresh(true) } }>
          <a className='role-button borrower'>Become a Borrower</a>
        </button>
      </div><div className="card">
          <button type="button" onClick={() => { setRefresh(true) } }>
            Refresh your data
          </button>
        </div></>
    } else {
      return <div className="card">
      <button type="button" onClick={() => { setYourRole(''), setRefresh(true)}}>
        Back to the main page
      </button>
    </div>
    }
  }

  function Show_info() {

    if (yourRole == 'Borrower') {
      if (!borrowerInfo.length) {
        return <div>Loading...</div>
      } else {
        var debt_status = null

        let unix_timestamp = parseInt(borrowerInfo![3]);

        var debt_due = null

        if (unix_timestamp == 0) {
          debt_due = null
        } else {

          let date = new Date(unix_timestamp * 1000);

          let date_format = date.toLocaleString();

          let current = protocolInfo![4]

          if (current <= unix_timestamp) {
            debt_status = null
          } else {
            debt_status = <><br/><a style={red}>You're already late on your repayment, please repay your debt!</a></>
          }

          debt_due = <div className="box-stats stats borrower"><a className='title'>Your debt due at</a><a className='borrower-info'>{date_format}</a></div>
        }

        var maximum_allowance = null

        if (borrowerInfo![0] == "Installment Credit") {
          maximum_allowance = null
        } else {
          maximum_allowance = <div className="box-stats stats borrower"><a className='title'>Your maximum allowance</a><a className='borrower-info'>{borrowerInfo![6]}</a></div>
        }

        return <div className='box-stats'>
        <div style={{paddingBottom: '20px', fontSize: '20px'}}>You're using <a className='credit-name'>{borrowerInfo![0]}</a></div>
        <div className="box-stats stats borrower"><a className='title'>Credit Score</a><a className='borrower-info'>{borrowerInfo![1]}</a></div> 
        <div className="box-stats stats borrower"><a className='title'>Total debt</a><a className='borrower-info'>{borrowerInfo![2]}</a></div> 
        {debt_due}
        <div className="box-stats stats borrower"><a className='title'>Your accumulated repayment</a><a className='borrower-info'>{borrowerInfo![4]}</a></div> 
        {maximum_allowance}
        <div className="box-stats stats borrower"><a className='title'>Your current allowance</a><a className='borrower-info'>{borrowerInfo![5]}</a></div> 
        {debt_status}
        </div>
      }
    }
    else if (yourRole == 'Lender') { 

      if (!lenderInfo.length) {
        return <div>Currently you don't have a Lending Account, please consider create a new Lending Account</div>
      } else {
        const listItems = lenderInfo.map(
          (x) =>
        <li key = {x[0]}><div className='box-stats'>
          <div className="box-stats stats lender"><a className='title'>Account NFT ID</a><a className='lender-info id'>{x[0]}</a></div>
          <div className="box-stats stats lender"><a className='title'>Account current return</a><a className='lender-info'>{parseFloat(x[1]).toFixed(2)}</a></div> 
          <div className="box-stats stats lender"><a className='title'>This account started from</a><a className='lender-info'>{new Date(parseInt(x![2]) * 1000).toLocaleString()}</a></div> 
          <button type="button" onClick={() => withdraw(x[0])}>
          <a className="">Withdraw</a>
        </button>
        </div></li>
        );
        return <div>{listItems}</div>
      }
    } else {

      var risk_percent = '0'

      var total_account = 0

      var total_deposited = 0

      var apy = '0'

      if (protocolInfo.length) {
        risk_percent = protocolInfo[2].toFixed(2)
        total_account = protocolInfo[0]
        total_deposited = protocolInfo[1]
        apy = protocolInfo[3].toFixed(2)
      }

      return <div className='box-stats'>
        <div className='box-stats stats borrower'><a className='title'>Total accounts</a><a className='info'>{total_account}</a></div>
        <div className='box-stats stats borrower'><a className='title'>Total deposited </a><a className='info'>{total_deposited}</a></div>
        <div className='box-stats stats borrower'><a className='title'>Risk percent </a><a className='info'>{risk_percent} %</a></div>
        <div className='box-stats stats borrower'><a className='title'>Current APY </a><a className='info'>{apy} %</a></div>
      </div>
      
    }
  }

  function Role_button() {
    if (yourRole == 'Lender') {
      return <div><br/><button type="button" onClick={make_lending_account}>
          <a className='role-button lender'>Make New Account</a>
          </button>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
          <button type="button" onClick={withdraw_all}>
          <a className='role-button lender'>Withdraw All</a>
          </button>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
          <button type="button" onClick={compensation}>
          <a className='role-button lender'>Take Compensation</a>
          </button></div>
    }
    else if (yourRole == 'Borrower') {
      return <div><br/><button type="button" onClick={take_loan}>
          <a className='role-button borrower'>Take Loan</a>
      </button>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<button type="button" onClick={repay_loan}>
      <a className='role-button borrower'>Make Repayment</a>
        </button>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<button type="button" onClick={change_credit}>
      <a className='role-button borrower'>Change Credit Type</a>
        </button>
        <div className='installment-credit-user'><a style={{fontSize: '25px', color: '#1a1a1a'}}>For Installment Credit user</a><br/><br/>
        <button type="button" onClick={request_installment_credit}>
      <a className='role-button borrower'>Request Installment Credit</a>
        </button>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<button type="button" onClick={use_installment_credit}>
      <a className='role-button borrower'>Use Installment Credit</a>
        </button></div></div>
    }
    else return null
  }

  async function make_lending_account() {
    const result = prompt("How much stablecoins you wish to lend on the protocol?")
    
      if (result !== null) {
        const amount = parseFloat(result)
        if (parseFloat(tokenInfo) < amount) {
          failure("Not enough token in wallet!")
          return
        } else {
          const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, StableCoin)
          .takeFromWorktop(StableCoin, 'bucket')
          .callMethod(GroundLendingComponent, 'new_lending_account', [`Bucket("bucket")`])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
          setRefresh(true)
        } else {
          failure_big("Failed", '' + receipt.logs.toString());
        }

        
        }
        
      }
      return
  }

  async function withdraw(nft_id: string) {
    const result = prompt("How much you stable coin you wish to withdraw from this lending account?")
    
    if (result !== null) {
      const amount = parseFloat(result);
      
        const manifest = new ManifestBuilder()
        .createProofFromAccountByIds(accountAddress!, [nft_id], LendingAccount)
        .popFromAuthZone("AccountProof")
        .callMethod(GroundLendingComponent, 'withdraw', [`Proof("AccountProof") Decimal("${amount}")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      if (receipt.status == 'Success') {
        success_big("Done!", '' + receipt.logs.toString());
        setRefresh(true)
      } else {
        failure_big("Failed", '' + receipt.logs.toString());
      }
      
      }
      return
  }

  async function withdraw_all() {

    const manifest = new ManifestBuilder()
          .withdrawFromAccount(accountAddress!, LendingAccount)
          .takeFromWorktop(LendingAccount, 'bucket')
          .callMethod(GroundLendingComponent, 'withdraw_all', [`Bucket("bucket")`])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);

        if (receipt.status == 'Success') {
          success_big("Done!", '' + receipt.logs.toString());
          setRefresh(true)
        } else {
          failure_big("Failed", '' + receipt.logs.toString());
        }

  }

  async function compensation() {

    const manifest = new ManifestBuilder()
      .withdrawFromAccount(accountAddress!, LendingAccount)
      .takeFromWorktop(LendingAccount, 'bucket')
      .callMethod(GroundLendingComponent, 'compensate', [`Bucket("bucket")`])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();

    const receipt = await signTransaction(manifest);

    if (receipt.status == 'Success') {
      success_big("Done!", '' + receipt.logs.toString());
      setRefresh(true)
    } else {
      failure_big("Failed", '' + receipt.logs.toString());
    }

  }

  async function take_loan() {
    const result = prompt("How much you stable coin you wish to loan?")
    
    if (result !== null) {
      const amount = parseFloat(result);
      
        const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress!, IDSBT)
        .popFromAuthZone("IDProof")
        .createProofFromAccount(accountAddress!, CreditSBT)
        .popFromAuthZone("CreditProof")
        .callMethod(GroundLendingComponent, 'revolving_credit', [`Proof("IDProof") Proof("CreditProof") Decimal("${amount}")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      if (receipt.status == 'Success') {
        success_big("Done!", '' + receipt.logs.toString());
        setRefresh(true)
      } else {
        failure_big("Failed", '' + receipt.logs.toString());
      }

     
      }
      return
  }

  async function repay_loan() {
    const result = prompt("How much you stable coin you wish to repay?")
    
    if (result !== null) {
      const amount = parseFloat(result);
      
        const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress!, IDSBT)
        .popFromAuthZone("IDProof")
        .createProofFromAccount(accountAddress!, CreditSBT)
        .popFromAuthZone("CreditProof")
        .withdrawFromAccountByAmount(accountAddress!, amount, StableCoin)
        .takeFromWorktop(StableCoin, "bucket")
        .callMethod(GroundLendingComponent, 'repay', [`Proof("IDProof") Proof("CreditProof") Bucket("bucket")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      if (receipt.status == 'Success') {
        success_big("Done!", '' + receipt.logs.toString());
        setRefresh(true)

      } else {
        failure_big("Failed", '' + receipt.logs.toString());
      }

      
      }
      return
  }

  async function change_credit() {
      
        const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress!, IDSBT)
        .popFromAuthZone("IDProof")
        .createProofFromAccount(accountAddress!, CreditSBT)
        .popFromAuthZone("CreditProof")
        .callMethod(GroundCreditComponent, 'change_credit_type', [`Proof("IDProof") Proof("CreditProof")`])
        .callMethodWithAllResources(accountAddress!, 'deposit_batch')
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      if (receipt.status == 'Success') {
        success_big("Done!", '' + receipt.logs.toString());
        setRefresh(true)
      } else {
        failure_big("Failed", '' + receipt.logs.toString());
      }
      
  }

  async function request_installment_credit() {
    const result = prompt("How much you wish to take on the installment credit?");
      if (result == null) {
        return
      } else {
        const amount = parseFloat(result);
        const result2 = prompt("How many periods you wish to pay your installment loan on (for now, each period is 1 month)");
        if (result2 == null) {
          return
        } else {
          const period = parseInt(result2);

          const manifest = new ManifestBuilder()
            .createProofFromAccount(accountAddress!, IDSBT)
            .popFromAuthZone("IDProof")
            .callMethod(GroundCreditComponent, 'request_installment_credit', [`Proof("IDProof") Decimal("${amount}") Decimal("10") Decimal("20") 2592000u64 ${period}u8`])
            .callMethodWithAllResources(accountAddress!, 'deposit_batch')
            .build()
            .toString();

            const receipt = await signTransaction(manifest);

          if (receipt.status == 'Success') {
            success_big("Done!", '' + receipt.logs.toString() + ' Now please consider contact the Credit provider to accept your installment credit!');
          } else {
            failure_big("Failed", '' + receipt.logs.toString());
          }
        }
      }
  }

  async function use_installment_credit() {
    const manifest = new ManifestBuilder()
      .createProofFromAccount(accountAddress!, IDSBT)
      .popFromAuthZone("IDProof")
      .withdrawFromAccountByAmount(accountAddress!, 1, InstallmentCreditRequestBadge)
      .takeFromWorktop(InstallmentCreditRequestBadge, "request_bucket")
      .callMethod(GroundCreditComponent, 'get_installment_credit_badge', [`Bucket("request_bucket") Proof("IDProof")`])
      .takeFromWorktop(InstallmentCreditBadge, "badge")
      .createProofFromAccount(accountAddress!, IDSBT)
      .popFromAuthZone("IDProof2")
      .createProofFromAccount(accountAddress!, CreditSBT)
      .popFromAuthZone("CreditProof")
      .callMethod(GroundLendingComponent, 'installment_credit', [`Proof("IDProof2") Proof("CreditProof") Bucket("badge")`])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();

      const receipt = await signTransaction(manifest);

    if (receipt.status == 'Success') {
      success_big("Done!", '' + receipt.logs.toString());
      setRefresh(true)
    } else {
      failure_big("Failed", '' + receipt.logs.toString());
    }
    
  }

  useEffect(() => {
    setTimeout(() => {
      setRefresh(false);
      data();
    }, 100);
  }, [accountAddress, refresh]);

  return (
    <div className="App">
      <div>
        <a href="https://github.com/unghuuduc/GroundPackages" target="_blank">
          <img src="/logo.svg" className="logo" alt="Ground logo" />
        </a>
        <a href="https://github.com/unghuuduc/NeuRacle" target="_blank">
          <img src={neuracleLogo} className="logo neuracle" alt="NeuRacle logo" />
        </a>
      </div>
      <h1><a style={blanchedalmond}>Ground</a> + <a style={lightblue}>NeuRacle</a></h1>
      <p className="read-the-docs">
        Click on the Ground and NeuRacle logos to learn more
      </p>
      <p>
          Hello <Role_name /> account: "<a style={blanchedalmond}>{accountAddress}</a>"
        </p>
        <p>
          Current Stablecoins on your wallet: <a style={blanchedalmond}>{tokenInfo}</a>
          </p>
          <Show_info />
          <Role_button /><br/>
      <Refresh_button />
      <p className="boot-straped-by">
        The Ground Web is bootstraped by Vite + React
      </p>
    </div>
  )
}

export default App
