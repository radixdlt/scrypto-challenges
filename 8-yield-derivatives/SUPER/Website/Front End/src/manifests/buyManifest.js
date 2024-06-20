export const buyManifest = (
    accountAddress,
    componentAddy,
    xrdAddy,
    xrdAmount,
) => String.raw`
    CALL_METHOD
        Address("${accountAddress}")
        "withdraw"
        Address("${xrdAddy}")
        Decimal("${xrdAmount}");
    
    TAKE_FROM_WORKTOP
        Address("${xrdAddy}")
        Decimal("${xrdAmount}")
        Bucket("bucket1");
    
    CALL_METHOD
        Address("${componentAddy}")
        "deposit"
        Bucket("bucket1");
    
    CALL_METHOD
        Address("${accountAddress}")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP");
`;
