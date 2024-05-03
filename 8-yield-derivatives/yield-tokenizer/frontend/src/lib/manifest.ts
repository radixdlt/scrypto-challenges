import type Decimal from "decimal.js"

export function callMethod(component: string, method: string, args: string[]) {
    return ` 
  
CALL_METHOD
    Address("${component}")
    "${method}"
${args.map((arg) => `    ${arg}`).join('\n')}
;
    
`
}

export function withdrawNonFungibles(component: string, nft_address: string, ids: string[]) {
    return `

CALL_METHOD
    Address("${component}")
    "withdraw_non_fungibles"
    Address("${nft_address}")
    Array<NonFungibleLocalId>(
${ids.map((id) => `     NonFungibleLocalId("${id}")`).join(',\n')}
    )
;

`
}

export function withdrawFungible(account: string, res_address: string, amount: Decimal) {
    return `
    
CALL_METHOD
    Address("${account}")
    "withdraw"
    Address("${res_address}")
    Decimal("${amount.toString()}")

;

`
}


export function takeAllFromWorktop(address: string, bucket: string) {
    return `

TAKE_ALL_FROM_WORKTOP
    Address("${address}")
    Bucket("${bucket}")
;

`
}


export function depositBatch(account: string) {

    return `

CALL_METHOD
    Address("${account}")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;

`
}

