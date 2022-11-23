import {config} from './config.js';

export function buildTransactionNewToken(accountAddress, amount, name, symbol){
  const RDX_ENGINE = "010000000000000000000000000000000000000000000000000001";
  let metaData = [];
  if(name){
    metaData.push("name");
    metaData.push(name);
  }
  if(symbol){
    metaData.push("symbol");
    metaData.push(symbol);
  }
  const metaDataStr = metaData.map(s => `"${s}"`).join(",");
  return [
    'CALL_FUNCTION PackageAddress("' + RDX_ENGINE + '") "System" "new_resource" Enum("Fungible", 18u8) HashMap<String, String>('+metaDataStr+') HashMap<Enum, Tuple>(Enum("Withdraw"), Tuple(Enum("AllowAll"), Enum("LOCKED"))) Some(Enum("Fungible", Decimal("' + amount + '")));',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildNewOffer(accountAddress, offer){
  let args = [
    'Bucket("offerToken")',
    'Decimal("' + offer.collatAmount + '")',
    'ResourceAddress("' + offer.collatResourceAddress + '")',
    'Decimal("' + offer.costPerHour + '")',
    'ResourceAddress("' + offer.feeResourceAddress + '")',
    '' + offer.maxBorrowTime + 'u64', 
  ].join(" ");
  return [
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("' + offer.tokenAmount + '") ResourceAddress("' + offer.tokenAddress + '");',
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("' + offer.tokenAmount + '") ResourceAddress("' + offer.tokenAddress + '") Bucket("offerToken");',
    'CALL_METHOD ComponentAddress("' + config.centralComponentAddress + '") "new_offer" ' + args + ';',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildAcceptOffer(accountAddress, offer){
  return [
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("' + offer.collatAmount + '") ResourceAddress("' + offer.collatResourceAddress + '");',
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("' + offer.maxBorrowTime * offer.costPerHour + '") ResourceAddress("' + offer.feeResourceAddress + '");',
    // Place it in the worktop so it can be used 
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("' + offer.collatAmount + '") ResourceAddress("' + offer.collatResourceAddress + '") Bucket("collatBucket");',
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("' + offer.maxBorrowTime * offer.costPerHour + '") ResourceAddress("' + offer.feeResourceAddress + '") Bucket("feeBucket");',
    // Call the component method 
    'CALL_METHOD ComponentAddress("' + offer.address + '") "borrow" Bucket("collatBucket") Bucket("feeBucket");',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildCancelOffer(accountAddress, offer){
  return [
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("1") ResourceAddress("' + offer.badgeAddress + '");',
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("' + offer.badgeAddress + '") Bucket("bucket1");',
    'CALL_METHOD ComponentAddress("' + offer.address + '") "cancel" Bucket("bucket1");',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildSeizeCollateral(accountAddress, offer){
  return [
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("1") ResourceAddress("' + offer.badgeAddress + '");',
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("' + offer.badgeAddress + '") Bucket("badgeBucket");',
    'CALL_METHOD ComponentAddress("' + offer.address + '") "default" Bucket("badgeBucket");',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildSettleOffer(accountAddress, offer){
  return [
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("1") ResourceAddress("' + offer.badgeAddress + '");',
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("' + offer.badgeAddress + '") Bucket("bucket1");',
    'CALL_METHOD ComponentAddress("' + offer.address + '") "settle" Bucket("bucket1");',
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}

export function buildReturnAssets(accountAddress, offer){
  return [
    // Take the badge and the tokens that need to be returned
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("1") ResourceAddress("' + offer.badgeAddress + '");',
    'CALL_METHOD ComponentAddress("' + accountAddress + '") "withdraw_by_amount" Decimal("' + offer.tokenAmount + '") ResourceAddress("' + offer.tokenAddress + '");',
    // Place it in the worktop so it can be used 
    'TAKE_FROM_WORKTOP_BY_AMOUNT Decimal("1") ResourceAddress("' + offer.badgeAddress + '") Bucket("badgeBucket");',
    'TAKE_FROM_WORKTOP ResourceAddress("' + offer.tokenAddress + '") Bucket("assetBucket");',
    // Call the component method 
    'CALL_METHOD ComponentAddress("' + offer.address + '") "return_asset" Bucket("assetBucket") Bucket("badgeBucket");',
    // Return the collateral and in some cases a rest of the fee
    'CALL_METHOD_WITH_ALL_RESOURCES ComponentAddress("' + accountAddress + '") "deposit_batch";'
  ].join("\n");
}
