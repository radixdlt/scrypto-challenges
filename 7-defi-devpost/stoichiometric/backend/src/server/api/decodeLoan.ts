import decode_loan, { Loan } from "../../decoder/decode_loan.js";
import { BadRequestError } from "../errors.js";

export default async function decodeLoan(url:URL):Promise<Loan>{
    const mutable_data_hex = url.searchParams.get('mutable_data_hex')
    const immutable_data_hex = url.searchParams.get('immutable_data_hex')

    if (mutable_data_hex == undefined) {
      throw new BadRequestError('decodeLoan require parameter: mutable_data_hex')
    }

    if (immutable_data_hex == undefined) {
        throw new BadRequestError('decodeLoan require parameter: immutable_data_hex')
    }

    
    const result = await decode_loan(mutable_data_hex,immutable_data_hex);
    return result;
}