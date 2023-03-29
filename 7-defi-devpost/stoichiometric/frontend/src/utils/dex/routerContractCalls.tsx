import { position_address, router_address, stablecoin_address } from "../general/constants";
import { rdt } from "../connectToWallet";

import { step } from "types";

async function swap_direct(account: string, token1Address: string, token2Address: string, amount: string) {
    const manifest = `
                    CALL_METHOD
                      ComponentAddress("${account}")
                      "withdraw_by_amount"
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}")
                      Bucket("0");
                    
                    CALL_METHOD
                      ComponentAddress("${router_address}")
                      "swap"
                      Bucket("0")
                      ResourceAddress("${token2Address}");
                    
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

async function swap_indirect(account: string, token1Address: string, token2Address: string, amount: string) {
    const manifest = `
                    CALL_METHOD
                     ComponentAddress("${account}")
                      "withdraw_by_amount"
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}")
                      Bucket("0");
                    
                    CALL_METHOD
                      ComponentAddress("${router_address}")
                      "swap"
                      Bucket("0")
                      ResourceAddress("${stablecoin_address}");
                    
                    TAKE_FROM_WORKTOP
                      ResourceAddress(${stablecoin_address})
                      Bucket("1");
                    
                    CALL_METHOD
                      ComponentAddress("${router_address}")
                      "swap"
                      Bucket("1")
                      ResourceAddress("${token2Address}");
                      
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

async function addLiquidityNoPosition(account: string, other_token: string, stablecoin_amount: number, other_token_amount: number, steps: number[][]) {

    let steps_string = "";

    for (const step of steps) {
        let step_string = `Tuple(${step[0]}u16, Decimal("${step[1].toFixed(10)}"), Decimal("${step[2].toFixed(10)}")), `;
        steps_string += step_string;
    }
    steps_string = steps_string.slice(0, -2);

    let manifest = `
    CALL_METHOD
        ComponentAddress("${account}")
        "withdraw_by_amount"
        Decimal("${stablecoin_amount.toFixed(10)}")
        ResourceAddress("${stablecoin_address}");

    TAKE_FROM_WORKTOP_BY_AMOUNT
        Decimal("${stablecoin_amount.toFixed(10)}")
        ResourceAddress("${stablecoin_address}")
        Bucket("0");

    CALL_METHOD
        ComponentAddress("${account}")
        "withdraw_by_amount"
        Decimal("${other_token_amount.toFixed(10)}")
        ResourceAddress("${other_token}");

    TAKE_FROM_WORKTOP_BY_AMOUNT
        Decimal("${other_token_amount.toFixed(10)}")
        ResourceAddress("${other_token}")
        Bucket("1");

    CALL_METHOD
        ComponentAddress("${router_address}")
        "add_liquidity_at_steps"
        Bucket("0")
        Bucket("1")
        Array<Tuple>(${steps_string})
        None;

    CALL_METHOD
        ComponentAddress("${account}")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP");

    `;

    console.log(manifest);

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function addLiquidityToPosition(account: string, other_token: string, stablecoin_amount: number, other_token_amount: number, steps: number[][], position_id: string) {

    let steps_string = "";

    for (const step of steps) {
        let step_string = `Tuple(${step[0]}u16, Decimal("${step[1]}"), Decimal("${step[2]}")), `;
        steps_string += step_string;
    }

    steps_string = steps_string.slice(0, -2);

    let manifest = `
                CALL_METHOD
                    ComponentAddress("${account}")
                    "withdraw_by_amount"
                    Decimal("${stablecoin_amount}")
                    ResourceAddress("${stablecoin_address}");
                
                TAKE_FROM_WORKTOP_BY_AMOUNT
                    Decimal("${stablecoin_amount}")
                    ResourceAddress("${stablecoin_address}")
                    Bucket("0");
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "withdraw_by_amount"
                    Decimal("${other_token_amount}")
                    ResourceAddress("${other_token}");
                
                TAKE_FROM_WORKTOP_BY_AMOUNT
                    Decimal("${other_token_amount}")
                    ResourceAddress("${other_token}")
                    Bucket("1");
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "create_proof_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}");
                
                CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}")
                    Proof("2");

                CALL_METHOD
                    ComponentAddress("${router_address}")
                    "add_liquidity_at_steps"
                    Bucket("0")
                    Bucket("1")
                    Array<Tuple>(${steps_string})
                    Some(Proof("2"));
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "deposit_batch"
                    Expression("ENTIRE_WORKTOP");`;

    console.log(manifest);

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function removeAllLiquidity(account: string, position_id: string) {

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
                    ComponentAddress("${router_address}")
                    "remove_all_liquidity"
                    Bucket("0");
                
                CALL_METHOD
                    ComponentAddress("${account}")
                    "deposit_batch"
                    Expression("ENTIRE_WORKTOP");
    `;

    console.log(manifest);

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

async function claimFees(account: string, position_id: string) {

    let manifest = `
                CALL_METHOD
                    ComponentAddress("${account}")
                    "create_proof_by_ids"
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}");
                
                CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                    Array<NonFungibleLocalId>(NonFungibleLocalId("${position_id}"))
                    ResourceAddress("${position_address}")
                    Proof("0");
                
                CALL_METHOD
                    ComponentAddress("${router_address}")
                    "claim_fees"
                    Proof("0");
                
                DROP_ALL_PROOFS;
                
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

export { swap_direct, swap_indirect, addLiquidityNoPosition, addLiquidityToPosition, removeAllLiquidity, claimFees }