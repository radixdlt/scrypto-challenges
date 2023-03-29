import decode_proposal_receipt, { Proposal_id } from "../../decoder/decode_proposal_receipt.js";
import { BadRequestError } from "../errors.js";

export default async function decodeProposalReceipt(url:URL):Promise<Proposal_id>{
    const mutable_data_hex = url.searchParams.get('mutable_data_hex')

    if (mutable_data_hex == undefined) {
      throw new BadRequestError('decode_proposal_receipt require parameter: mutable_data_hex')
    }

    const result = await decode_proposal_receipt(mutable_data_hex);
    console.log('decode_proposal_receipt:',mutable_data_hex,":", result);
    return result;

}