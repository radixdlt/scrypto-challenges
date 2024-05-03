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
1u32
;
CALL_METHOD
Address("${ownerAddress}")
"deposit_batch"
Expression("ENTIRE_WORKTOP")
;`;