export const newManifest = (
    ownerAddress,
    DappDefinition,
    PackageAddy,
    testMode
) => `
CALL_FUNCTION
Address("${PackageAddy}")
"Super"
"new"
Address("${DappDefinition}")
${testMode}u32
;
CALL_METHOD
Address("${ownerAddress}")
"deposit_batch"
Expression("ENTIRE_WORKTOP")
;`;