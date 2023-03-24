import {
    dao_address,
    position_address,
    proposal_receipt_address,
    stablecoin_address,
    voter_card_address
} from "../general/constants";
import { rdt } from "../connectToWallet";

async function executeProposal(account: string, proposal_id: string) {
    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "withdraw_by_ids"
                        Array<NonFungibleLocalId>("#${proposal_id}#")
                        ResourceAddress("${proposal_receipt_address}");
                    
                    TAKE_FROM_WORKTOP_BY_IDS
                        Array<NonFungibleLocalId>("#${proposal_id}#")
                        ResourceAddress("${proposal_receipt_address}")
                        Bucket("0");
                    
                    CALL_METHOD
                        ComponentAddress("${dao_address}")
                        "execute_proposal"
                        Bucket("0");
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function allowClaimProposal(account: string, token_address: string, amount: number) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${dao_address}")
                        "make_proposal"
                        Enum(4u8, Array<Tuple>(Tuple(ResourceAddress("${token_address}"), Decimal("${amount}"))));
                        
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
`;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function minimumVoteThreshold(account: string, new_threshold: number) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${dao_address}")
                        "make_proposal"
                        Enum(1u8, Decimal("${new_threshold}"));
                        
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
`;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function changeVotePeriod(account: string, new_vote_period: number) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${dao_address}")
                        "make_proposal"
                        Enum(1u8, ${new_vote_period}i64);
                        
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
`;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function voteForProposal(account: string, voter_card_id: string, proposal_address: string) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "create_proof_by_ids"
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                        ResourceAddress("${voter_card_address}");
                    
                    CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                        ResourceAddress("${voter_card_address}")
                        Proof("0");
                    
                    CALL_METHOD
                        ComponentAddress("${proposal_address}")
                        "vote_for"
                        Proof("0");
                    
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();

}

async function voteAgainstProposal(account: string, voter_card_id: string, proposal_address: string) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "create_proof_by_ids"
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                        ResourceAddress("${voter_card_address}");
                    
                    CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                        ResourceAddress("${voter_card_address}")
                        Proof("0");
                    
                    CALL_METHOD
                        ComponentAddress("${proposal_address}")
                        "vote_against"
                        Proof("0");
                    
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();

}

async function lockStablecoinsNoVoterCard(account: string, amount: number) {

    const manifest = `
                   CALL_METHOD
                        ComponentAddress("${account}")
                        "withdraw_by_amount"
                        Decimal("${amount}")
                        ResourceAddress("${stablecoin_address}");
                
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                        Decimal("${amount}")
                        ResourceAddress("${stablecoin_address}")
                        Bucket("0");
                
                    CALL_METHOD
                        ComponentAddress("${dao_address}")
                        "lock_stablecoins"
                        Bucket("0")
                        None;
                
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
   `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function lockStablecoinsToVoterCard(account: string, amount: number, voter_card_id: string) {
    const manifest = `
                CALL_METHOD
                    ComponentAddress("${account}")
                    "withdraw_by_amount"
                    Decimal("${amount}")
                    ResourceAddress("${stablecoin_address}");
                
                TAKE_FROM_WORKTOP_BY_AMOUNT
                    Decimal("${amount}")
                    ResourceAddress("${stablecoin_address}")
                    Bucket("0");
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "create_proof_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                    ResourceAddress("${voter_card_address}");
                
                CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                    ResourceAddress("${voter_card_address}")
                    Proof("1");
                
                CALL_METHOD
                    ComponentAddress("${dao_address}")
                    "lock_stablecoins"
                    Bucket("0")
                    Some(Proof("1"));
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "deposit_batch"
                    Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function lockPositionNoVoterCard(account: string, position_id: string) {


    let manifest = `
                CALL_METHOD
                    ComponentAddress("${account}")
                    "withdraw_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}");
                
                TAKE_FROM_WORKTOP_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}")
                    Bucket("0");
                
                CALL_METHOD
                    ComponentAddress("${dao_address}")
                    "lock_positions"
                    Bucket("0")
                    None;
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "deposit_batch"
                    Expression("ENTIRE_WORKTOP");
        
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function lockPositionToVoterCard(account: string, position_id: string, voter_card_id: string) {

    let manifest = `
                CALL_METHOD
                    ComponentAddress("${account}")
                    "withdraw_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}");
                
                TAKE_FROM_WORKTOP_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}")
                    Bucket("0");
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "create_proof_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                    ResourceAddress("${voter_card_address}");
                
                CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${voter_card_id}"))
                    ResourceAddress("${voter_card_address}")
                    Proof("1");
                
                CALL_METHOD
                    ComponentAddress("${dao_address}")
                    "lock_positions"
                    Bucket("0")
                    Some(Proof("1"));
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "deposit_batch"
                    Expression("ENTIRE_WORKTOP");
`;
}


export { executeProposal, allowClaimProposal, minimumVoteThreshold, changeVotePeriod, voteAgainstProposal, voteForProposal, lockPositionNoVoterCard, lockPositionToVoterCard, lockStablecoinsNoVoterCard, lockStablecoinsToVoterCard };