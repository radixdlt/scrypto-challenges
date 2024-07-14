export const depositAndCreateUserManifest = (
  amount: number,
  accountAddress: string,
  tokenAddress: string,
  componentAddress: string,
) => `
CALL_METHOD
    Address("${accountAddress}")
    "withdraw"
     Address("${tokenAddress}")
    Decimal("${amount}");
TAKE_FROM_WORKTOP
     Address("${tokenAddress}")
    Decimal("${amount}")
    Bucket("bucket1");
CALL_METHOD
    Address("${componentAddress}")
    "create_user_and_deposit_principal"
    Bucket("bucket1");
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
`;

export const depositAssetManifest = (
  amount: number,
  userBadge: string | null,
  accountAddress: string,
  tokenAddress: string,
  componentAddress: string,
) => `
CALL_METHOD
    Address("${accountAddress}")
    "create_proof_of_non_fungibles"
    Address("${userBadge}")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"));
CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES
    Address("${userBadge}")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))
    Proof("proof1");
CALL_METHOD
    Address("${accountAddress}")
    "withdraw"
    Address("${tokenAddress}")
    Decimal("${amount}");
TAKE_FROM_WORKTOP
    Address("${tokenAddress}")
    Decimal("${amount}")
    Bucket("bucket1");
CALL_METHOD 
    Address("${componentAddress}") 
    "deposit_principal" 
    Bucket("bucket1")
    Proof("proof1");
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
`;

export const withdrawAssetManifest = (
  userBadge: string | undefined,
  accountAddress: string,
  tokenAddress: string,
  componentAddress: string,
) => `
CALL_METHOD
    Address("${accountAddress}")
    "create_proof_of_non_fungibles"
    Address("${userBadge}")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"));
CREATE_PROOF_FROM_AUTH_ZONE_OF_NON_FUNGIBLES
    Address("${userBadge}")
    Array<NonFungibleLocalId>(NonFungibleLocalId("#1#"))
    Proof("UserBadge");
CALL_METHOD 
    Address("${componentAddress}")
    "redeem" 
    Address("${tokenAddress}")
    Proof("UserBadge");
CALL_METHOD
    Address("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
`;
