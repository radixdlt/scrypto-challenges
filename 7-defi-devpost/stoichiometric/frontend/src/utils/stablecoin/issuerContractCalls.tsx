import { issuer_address, loan_address, stablecoin_address } from "../general/constants";
import { rdt } from "../connectToWallet";

async function takeLoan(account: string, collateral_token: string, collateral_amount: string, amount_to_loan: string) {
    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "withdraw_by_amount"
                        Decimal("${collateral_amount}")
                        ResourceAddress("${collateral_token}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                        Decimal("${collateral_amount}")
                        ResourceAddress("${collateral_token}")
                        Bucket("0");
                    
                    CALL_METHOD
                        ComponentAddress("${issuer_address}")
                        "take_loan"
                        Bucket("0")
                        Decimal("${amount_to_loan}");
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
    `;

    console.log('call');
    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    })

    return !result.isErr()
}

async function repayLoan(account: string, stablecoin_amount: string, loan_id: string) {
    const manifest = `
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
            "withdraw_by_ids"
            Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
            ResourceAddress("${loan_address}");

        TAKE_FROM_WORKTOP_BY_IDS
            Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
            ResourceAddress("${loan_address}")
            Bucket("1");

        CALL_METHOD
            ComponentAddress("${issuer_address}")
            "repay_loans"
            Bucket("0")
            Bucket("1");

        CALL_METHOD
            ComponentAddress("${account}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");
    `;

    console.log(manifest)

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    })

    return !result.isErr();
}

async function addCollateral(account: string, collateral_token: string, collateral_amount: string, loan_id: string) {
    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "withdraw_by_amount"
                        Decimal("${collateral_amount}")
                        ResourceAddress("${collateral_token}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                        Decimal("${collateral_amount}")
                        ResourceAddress("${collateral_token}")
                        Bucket("0");
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "create_proof_by_ids"
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
                        ResourceAddress("${loan_address}");
                    
                    CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
                        ResourceAddress("${loan_address}")
                        Proof("1");
                    
                    CALL_METHOD
                        ComponentAddress("${issuer_address}")
                        "add_collateral"
                        Bucket("0")
                        Proof("1");
                    
                    DROP_ALL_PROOFS;
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    })

    return result.isOk();
}

async function removeCollateral(account: string, amount_to_remove: string, loan_id: string) {
    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "create_proof_by_ids"
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
                        ResourceAddress("${loan_address}");
                    
                    CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
                        Array<NonFungibleLocalId>(NonFungibleLocalId("${loan_id}"))
                        ResourceAddress("${loan_address}")
                        Proof("0");
                    
                    CALL_METHOD
                        ComponentAddress("${issuer_address}")
                        "remove_collateral"
                        Decimal("${amount_to_remove}")
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
    })

    return result.isOk();
}

async function liquidate(account: string, stablecoins_to_withdraw: string, loan_id: string) {

    const manifest = `
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "withdraw_by_amount"
                        Decimal("${stablecoins_to_withdraw}")
                        ResourceAddress("${stablecoin_address}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                        Decimal("${stablecoins_to_withdraw}")
                        ResourceAddress("${stablecoin_address}")
                        Bucket("0");
                    
                    CALL_METHOD
                        ComponentAddress("${issuer_address}")
                        "liquidate"
                        Bucket("0")
                        NonFungibleLocalId("${loan_id}");
                    
                    CALL_METHOD
                        ComponentAddress("${account}")
                        "deposit_batch"
                        Expression("ENTIRE_WORKTOP");
        
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    })

    return result.isOk();
}

export { takeLoan, repayLoan, addCollateral, removeCollateral, liquidate }