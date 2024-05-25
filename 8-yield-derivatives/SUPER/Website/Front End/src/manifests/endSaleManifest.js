export const endSaleManifest = (
    accountAddress,
    componentAddress,
    ownerBadgeAddress,
) => {
    return String.raw`
        CALL_METHOD
        Address("${accountAddress}")
        "create_proof_of_non_fungibles"
        Address("${ownerBadgeAddress}")
        Array<NonFungibleLocalId>(
            NonFungibleLocalId("#0#")
        )
        ;
        CALL_METHOD
        Address("${componentAddress}")
        "end_sale"
        ;
        CALL_METHOD
        Address("${accountAddress}")
        "try_deposit_batch_or_refund"
        Expression("ENTIRE_WORKTOP")
        Enum<0u8>()
        ;
`;}

