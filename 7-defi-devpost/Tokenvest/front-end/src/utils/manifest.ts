/*
createProductManifest is a function which takes four arguments: 
pakcageId(the package address of the deployed package)
raiseAmount(the amount to raise for the product)
title(product title)
accountAddress(the address of the account that creates the product)
createProductManifest returns a manifest string using all of the arguments mentioned
*/
export const createProductManifest = (packageId: string, raiseAmount: string, title: string, accountAddress: string) => {
    return `
CALL_FUNCTION
    PackageAddress("${packageId}")
    "Investment"
    "new"
    Decimal("${raiseAmount}")
    "${title}";
CALL_METHOD
    ComponentAddress("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");    
`
}

/*
investManifest is a function which takes four arguments: 
accountAddress(the address of the account that creates the product)
investAmount(the amount to invest)
radixAddress(radix resource address)
componentId(the adress of the component)
investManifest returns a manifest string using all of the arguments mentioned
*/
export const investManifest = (accountAddress:string,investAmount:string,radixAddress:string,componentId:string) => {
    return `
CALL_METHOD
    ComponentAddress("${accountAddress}")
    "withdraw_by_amount"
    Decimal("${investAmount}")
    ResourceAddress("${radixAddress}");
TAKE_FROM_WORKTOP_BY_AMOUNT
    Decimal("${investAmount}")
    ResourceAddress("${radixAddress}")
    Bucket("bucket1");
CALL_METHOD
    ComponentAddress("${componentId}")
    "invest"
    Bucket("bucket1")
    "walter";
CALL_METHOD
    ComponentAddress("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");                  
`
}

/*
withdrawManifest is a function which takes three arguments: 
accountAddress(the address of the account that creates the product)
resourceAddress(the address of the 'OWNER' resource)
componentId(the adress of the component)
withdrawManifest returns a manifest string using all of the arguments mentioned
*/
export const withdrawManifest = (accountAddress:string,resourceAddress:string, componentId:string) =>{
    return `
CALL_METHOD 
    ComponentAddress("${accountAddress}")
    "create_proof"
    ResourceAddress("${resourceAddress}");
CALL_METHOD
    ComponentAddress("${componentId}")
    "withdraw";
CALL_METHOD
    ComponentAddress("${accountAddress}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
`
}