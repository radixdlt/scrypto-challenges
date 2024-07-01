export const startSaleManifest = (
    accountAddress,
    componentAddress,
    xrdAddress,
    ownerBadgeAddress,
) => String.raw` 
CALL_METHOD
Address("${accountAddress}")
"create_proof_of_non_fungibles"
Address("${ownerBadgeAddress}")
Array<NonFungibleLocalId>(
    NonFungibleLocalId("#0#")
)
;
CALL_METHOD
Address("${accountAddress}")
"withdraw"
Address("${xrdAddress}")
Decimal("1")
;
TAKE_FROM_WORKTOP
Address("${xrdAddress}")
Decimal("1")
Bucket("bucket1")
;
CALL_METHOD
Address("${componentAddress}")
"start_sale"
Bucket("bucket1")
;
CALL_METHOD
Address("${accountAddress}")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>()
;
`;


