#! ./node_modules/.bin/ts-node --esm

import { $, question } from 'zx'
import * as _ from './const.js'
import { auto_liquidate, create_and_send_delegated_cdp } from './tx_manifests.js'
import { exe, get, load_envs, set } from './lib/utils.js'
import { create_user, user_get_resource, change_current_user, user_add_liquidity, user_add_collateral, user_take_loan, admin_set_current_time, user_create_cdp } from './tx_resim.js'


void (async function () {

    load_envs()

    let output: any[]
    let year = 2023
    let current_date = new Date()
    await admin_set_current_time(current_date, year)

    //
    await create_user({ user_id: 1 })
    await user_get_resource({ asset: 'BTC', xrd_amont: 100 })

    //
    await create_user({ user_id: 2 })
    await user_get_resource({ asset: 'ETH', xrd_amont: 100 })

    // await question('Hit enter to continue ...')

    //
    await create_user({ user_id: 3 })
    await user_get_resource({ asset: 'USDC', xrd_amont: 100 })
    await user_get_resource({ asset: 'USDT', xrd_amont: 100 })
    await user_add_liquidity({ asset: 'USDC', amont: 20000 })
    await user_add_liquidity({ asset: 'USDT', amont: 20000 })

    //
    await user_create_cdp({ user_id: 1 }) // CDP #1 
    await create_and_send_delegated_cdp({ from: 1, to: 2, delegator_cdp_id: "1" }) // CDP #2

    //
    await user_add_collateral({ cdp_id: 1, asset: 'BTC', amont: 1.1 })

    //
    await user_take_loan({ cdp_id: 2, asset: 'USDT', amont: 3000, interest_type: 0, user_id: 2, }) // #1


    await admin_set_current_time(current_date, year++)
    await user_take_loan({ cdp_id: 2, asset: 'USDC', amont: 3000, interest_type: 0 }) // #2


    await admin_set_current_time(current_date, year++)
    await user_take_loan({ cdp_id: 2, asset: 'USDT', amont: 4000, interest_type: 0 }) // #3
    // await exe($`resim call-method $${_.lending_component} repay "$${_.cdp_resource_address}:#${user_id}#" ${amont},$${asset}  ${interest_type} `)


    await admin_set_current_time(current_date, year++)
    await exe($`resim call-method $${_.lending_component} change_interest_type "$${_.cdp_resource_address}:#${2}#" 2 1 0`)


    await admin_set_current_time(current_date, year++)
    // //
    // await admin_set_current_time(current_date, year++)
    // await user_take_loan(2, 'USDC', 1000, 0) // #4


    // //
    // await admin_set_current_time(current_date, year++)

    // await user_get_resource(1, 'USDT', 20)
    // await user_get_resource(1, 'USDC', 20)
    // await user_repay_loan(1, 'USDC', 8000, 1)
    // await user_repay_loan(1, 'USDT', 4500, 2)


    // await user_get_resource(2, 'USDT', 20)
    // await user_get_resource(2, 'USDC', 20)
    // await user_repay_loan(2, 'USDC', 8000, 3)
    // await user_repay_loan(2, 'USDC', 3000, 4)

    // await auto_liquidate({ delegator_id: "1" })


})()


