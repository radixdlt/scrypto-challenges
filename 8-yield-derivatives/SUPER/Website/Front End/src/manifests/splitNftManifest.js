export const splitNftManifest = (
    accountAddress,
    componentAddress,
    yieldNftAddress,
    NftId, // WITH HASHTAGS
    numSplits,
) => {
    return String.raw`
        CALL_METHOD
            Address("${accountAddress}")
            "withdraw_non_fungibles"
            Address("${yieldNftAddress}")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("${NftId}")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("${yieldNftAddress}")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("${NftId}")
            )
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("${componentAddress}")
            "split_yield_nft"
            Bucket("bucket1")
            ${numSplits}u64
        ;
        CALL_METHOD
            Address("${accountAddress}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP")
        ;
`;}

