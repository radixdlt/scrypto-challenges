import decode_hex from './decode_hex.js'

export interface VoterCard {
    votingPower: number;
    stablecoinsLocked: number;
    positionsLockedIds: number[];
    lastProposalVotedId: number;
    proposalsVoted: number[];
  }
  
  function parseOutput(output: string): VoterCard {
    const [votingPowerString, stablecoinsLockedString, positionsIdsString, lastProposalVotedIdString, proposalsVotedString] =
      output.split('@');

      if( votingPowerString == undefined
        || stablecoinsLockedString == undefined
        || positionsIdsString == undefined
        || lastProposalVotedIdString == undefined
        || proposalsVotedString == undefined ){
            throw new Error("Undefined property")
        }
  
    const positionsLockedIds: number[] = positionsIdsString.split(' ').map((id) => parseInt(id));
    const proposalsVoted: number[] = proposalsVotedString.split(' ').map((id) => parseInt(id));
  
    return {
      votingPower: parseFloat(votingPowerString),
      stablecoinsLocked: parseFloat(stablecoinsLockedString),
      positionsLockedIds,
      lastProposalVotedId: parseInt(lastProposalVotedIdString),
      proposalsVoted,
    };
  }
    
export default async function decode_voter_cards(mutable_data_hex:string): Promise<VoterCard> {
    try {

      const decoded_data = await decode_hex(3,mutable_data_hex, "")

      return Promise.resolve(parseOutput(decoded_data.stdout));
  
    } catch (e) {
      console.log(e)
      return Promise.reject(e)
    }
  }
  
