#! ./node_modules/.bin/ts-node --esm

/**
 * run following commands for test
 *      chmod u+x run.ts    
 *      ./run.ts && source .env      
 */

import { $, question } from 'zx'
import * as _ from './const.js'
import { auto_liquidate } from './tx_manifests.js'
import { exe, load_envs, set } from './lib/utils.js'

let current_user = 0

void (async function () {

    load_envs()

    let year = 2022
    let current_date = new Date()
    await admin_set_current_time(current_date, year++)

    //
    let output = await exe($`resim new-account`)
    set(_.account, output[0], 1)
    set(_.privkey, output[2], 1)
    await exe($`resim set-default-account $account_1 $privkey_1 $owner_badge`)
    await exe($`resim call-method $${_.faucet_component} get_resource $BTC 100,$XRD`)
    await exe($`resim call-method $${_.lending_component} create_cdp`)

    //
    output = await exe($`resim new-account`)
    set(_.account, output[0], 2)
    set(_.privkey, output[2], 2)
    await exe($`resim set-default-account $account_2 $privkey_2 $owner_badge`)
    await exe($`resim call-method $${_.faucet_component} get_resource $ETH 100,$XRD`)
    await exe($`resim call-method $${_.lending_component} create_cdp`)

    // await question('Hit enter to continue ...')

    //
    output = await exe($`resim new-account`)
    set(_.account, output[0], 3)
    set(_.privkey, output[2], 3)
    await exe($`resim set-default-account $account_3 $privkey_3 $owner_badge`)
    await exe($`resim call-method $${_.faucet_component} get_resource $USDC 250,$XRD`)
    await exe($`resim call-method $${_.faucet_component} get_resource $USDT 200,$XRD`)


    //
    await user_add_liquidity(3, 'USDC', 30000)
    await user_add_liquidity(3, 'USDT', 20000)
    await user_add_collateral(1, 'BTC', 1)
    await user_add_collateral(2, 'ETH', 10)

    //
    await user_take_loan(1, 'USDC', 5000, 0) // #1

    //
    await admin_set_current_time(current_date, year++)

    //
    await user_take_loan(1, 'USDT', 3000, 1) // #2

    await user_take_loan(2, 'USDC', 4000, 1) // #3

    //
    await admin_set_current_time(current_date, year++)
    await user_take_loan(2, 'USDC', 1000, 0) // #4


    //
    await admin_set_current_time(current_date, year++)

    await user_get_resource(1, 'USDT', 20)
    await user_get_resource(1, 'USDC', 20)
    await user_repay_loan(1, 'USDC', 8000, 1)
    await user_repay_loan(1, 'USDT', 4500, 2)


    await user_get_resource(2, 'USDT', 20)
    await user_get_resource(2, 'USDC', 20)
    await user_repay_loan(2, 'USDC', 8000, 3)
    await user_repay_loan(2, 'USDC', 3000, 4)


})()


async function change_current_user(user: number) {
    if (current_user === user) return
    current_user = user
    await exe($`resim set-default-account $account_${user} $privkey_${user} $owner_badge`)
}

async function user_get_resource(user: number, asset: string, xrd_amont: number) {
    await change_current_user(user)
    let output: string[] = await exe($`resim call-method $${_.faucet_component} get_resource $${asset} ${xrd_amont},$XRD`)
    return output
}

async function user_add_liquidity(user: number, asset: string, amont: number) {
    await change_current_user(user)
    let output: string[] = await exe($`resim call-method $${_.lending_component} add_liquidity ${amont},$${asset} `)
    return output
}

async function user_take_loan(user: number, asset: string, amont: number, interest_type: number) {
    await change_current_user(user)
    return exe($`resim call-method $${_.lending_component} borrow "$${_.cdp_resource_address}:#${user}#" $${asset} ${amont} ${interest_type} `)
}

async function user_repay_loan(user: number, asset: string, amont: number, interest_type: number) {
    await change_current_user(user)
    return exe($`resim call-method $${_.lending_component} repay "$${_.cdp_resource_address}:#${user}#" ${amont},$${asset}  ${interest_type} `)

}

async function user_add_collateral(user: number, asset: string, amont: number) {
    await change_current_user(user)
    return exe($`resim call-method $${_.lending_component} new_collateral "$${_.cdp_resource_address}:#${user}#" ${amont},$${asset} `)
}

///

export async function change_to_admin_user() {
    if (current_user === 0) return
    current_user = 0
    await exe($`resim set-default-account $main_account $main_privkey $owner_badge`)
}

async function admin_set_current_time(current_date: Date, year: number) {
    current_date.setUTCFullYear(year++)
    await change_to_admin_user()
    await exe($`resim set-current-time "${current_date.toISOString().split(".")[0]}Z"`)
}

