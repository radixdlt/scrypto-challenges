import decode_position, { Position } from "../../decoder/decode_position.js";
import { BadRequestError } from "../errors.js";

export default async function decodePosition(url:URL):Promise<Position>{
    const mutable_data_hex = url.searchParams.get('mutable_data_hex')
    const immutable_data_hex = url.searchParams.get('immutable_data_hex')

    if (mutable_data_hex == undefined) {
      throw new BadRequestError('decode_position require parameter: mutable_data_hex')
    }

    if (immutable_data_hex == undefined) {
        throw new BadRequestError('decode_position require parameter: immutable_data_hex')
    }

    const result = await decode_position(mutable_data_hex,immutable_data_hex);
    console.log('decode_position:',mutable_data_hex,immutable_data_hex,":", result);
    return result;

}