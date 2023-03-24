#! ./node_modules/.bin/ts-node --esm

import { $, question } from 'zx'
import * as _ from './const.js'
import { auto_liquidate, create_and_send_delegated_cdp } from './tx_manifests.js'
import { exe, get, load_envs, set } from './lib/utils.js'
import { create_user, user_get_resource, change_current_user, user_add_liquidity, user_add_collateral, user_take_loan, admin_set_current_time, user_create_cdp, user_repay_loan, change_to_admin_user } from './tx_resim.js'


void (async function () {

    load_envs()

    let output: any[]
    let year = 2023
    let current_date = new Date()
    await admin_set_current_time(current_date, year)

    //
    await create_user({ user_id: 1 })
    await user_get_resource({ asset: 'BTC', xrd_amount: 100 })
    await user_get_resource({ asset: 'ETH', xrd_amount: 100 })

    //
    await create_user({ user_id: 2 })

    //
    await create_user({ user_id: 3 })
    await user_get_resource({ asset: 'USDC', xrd_amount: 100 })
    await user_get_resource({ asset: 'USDT', xrd_amount: 100 })
    await user_add_liquidity({ asset: 'USDC', amount: 20000 })
    await user_add_liquidity({ asset: 'USDT', amount: 20000 })

    //
    await user_create_cdp({ user_id: 1 }) // CDP #1 
    await create_and_send_delegated_cdp({ from: 1, to: 2, delegator_cdp_id: "1" }) // CDP #2

    //
    await user_add_collateral({ cdp_id: 1, asset: 'BTC', amount: 1 })

    //
    await user_take_loan({ user_id: 2, cdp_id: 2, asset: 'USDC', amount: 13000, interest_type: 0 }) // #1


    // await admin_set_current_time(current_date, year++)
    // await user_take_loan({ user_id: 2, cdp_id: 2, asset: 'USDT', amount: 5000, interest_type: 0 }) // #2

    // await admin_set_current_time(current_date, year++)
    // await user_repay_loan({ user_id: 2, cdp_id: 2, asset: 'USDC', amount: 3000, position_id: 1 })

    // await admin_set_current_time(current_date, year++)
    // await user_repay_loan({ user_id: 2, cdp_id: 2, asset: 'USDC', amount: 3000, position_id: 2 })

    // await admin_set_current_time(current_date, year++)
    // await user_repay_loan({ user_id: 2, cdp_id: 2, asset: 'USDT', amount: 3000, position_id: 3 })

    await admin_set_current_time(current_date, year++)

    await admin_set_current_time(current_date, year++)

    await admin_set_current_time(current_date, year++)

    await change_to_admin_user()
    await auto_liquidate({ cdp_id: "1" })

})()


