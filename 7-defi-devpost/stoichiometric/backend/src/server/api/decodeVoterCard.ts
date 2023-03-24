import decode_voter_card, { VoterCard } from "../../decoder/decode_voter_card.js";
import { BadRequestError } from "../errors.js";

export default async function decodeVoterCard(url:URL):Promise<VoterCard>{
    const mutable_data_hex = url.searchParams.get('mutable_data_hex')

    if (mutable_data_hex == undefined) {
        throw new BadRequestError('decode_voter_card require parameter: immutable_data_hex')
    }

    const result = await decode_voter_card(mutable_data_hex);
    console.log('decode_voter_card:',mutable_data_hex,":", result);
    return result;
}