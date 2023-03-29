import decode_hex from './decode_hex.js'

type Loan = {
    collateral_token: string,
    collateral_amount: number,
    amount_lent: number,
    loan_time: number,
    loan_to_value: number,
    interest_rate: number
}

export default async function decode_loan(mutable_data_hex:string,immutable_data_hex:string): Promise<Loan> {
    try {

      const decoded_data = await decode_hex(1,mutable_data_hex, immutable_data_hex)

      console.log(decoded_data)

      const [collateral_token,collateral_amount_str,amount_lent_str, loan_time_str,loan_to_value_str, interest_rate_str] = decoded_data.stdout.trim().split(" ")

  
      if (collateral_token == undefined
        || collateral_amount_str == undefined
        || amount_lent_str == undefined
        || loan_time_str == undefined
        || loan_to_value_str == undefined
        || interest_rate_str == undefined){
        return Promise.reject("Undefined Property")
      }

      const collateral_amount = parseFloat(collateral_amount_str)
      const amount_lent = parseFloat(amount_lent_str)
      const loan_time = parseFloat(loan_time_str)
      const loan_to_value = parseFloat(loan_to_value_str)
      const interest_rate = parseFloat(interest_rate_str)

    
      const data: Loan = {
        collateral_token,
        collateral_amount,
        amount_lent,
        loan_time,
        loan_to_value,
        interest_rate
      }
  
      return Promise.resolve(data)

    } catch (e) {
      console.log(e)
      return Promise.reject(e)
    }
  }
  
