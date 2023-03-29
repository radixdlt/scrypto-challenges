import decode_hex from './decode_hex.js'

type proposal_id = {proposal_id:string}
  
    
export default async function decode_voter_cards(immutable_data_hex:string): Promise<proposal_id> {
    try {

      const proposal_id = await (await decode_hex(4,"", immutable_data_hex)).stdout

      if (proposal_id == undefined) {throw new Error("Undefined property")}

      return {proposal_id}
  
    } catch (e) {
      console.log(e)
      return Promise.reject(e)
    }
  }
  
