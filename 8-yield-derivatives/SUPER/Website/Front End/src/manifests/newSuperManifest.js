export const newManifest = (
    ownerAddress,
    DappDefinition,
    PackageAddy
) => `
CALL_FUNCTION
Address("${PackageAddy}")
"Super"
"new"
Address("${DappDefinition}")
;
CALL_METHOD
Address("${ownerAddress}")
"try_deposit_batch_or_refund"
Expression("ENTIRE_WORKTOP")
Enum<0u8>()
;
`;

