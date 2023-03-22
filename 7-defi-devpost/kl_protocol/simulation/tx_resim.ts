import { $ } from 'zx'
import * as _ from './const.js'
import { exe, set } from './lib/utils.js'

let current_user = 0

export async function user_create_cdp({ user_id }: { user_id?: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.lending_component} create_cdp`)
}


export async function user_get_resource({ user_id, asset, xrd_amont }: { user_id?: number; asset: string; xrd_amont: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.faucet_component} get_resource $${asset} ${xrd_amont},$XRD`)
}

export async function user_add_liquidity({ user_id, asset, amont }: { user_id?: number; asset: string; amont: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.lending_component} add_liquidity ${amont},$${asset} `)
}

export async function user_take_loan({ cdp_id, asset, amont, interest_type, user_id }: { cdp_id: number; asset: string; amont: number; interest_type: number; user_id?: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.lending_component} borrow "$${_.cdp_resource_address}:#${cdp_id}#" $${asset} ${amont} ${interest_type} `)
}

export async function user_repay_loan({ cdp_id, user_id, asset, amont, interest_type }: { cdp_id: number; user_id?: number; asset: string; amont: number; interest_type: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.lending_component} repay "$${_.cdp_resource_address}:#${cdp_id}#" ${amont},$${asset}  ${interest_type} `)
}

export async function user_add_collateral({ cdp_id, user_id, asset, amont }: { cdp_id: number; user_id?: number; asset: string; amont: number }) {
    if (user_id !== undefined) {
        await change_current_user({ user_id })
    }
    return exe($`resim call-method $${_.lending_component} new_collateral "$${_.cdp_resource_address}:#${cdp_id}#" ${amont},$${asset} `)
}

/// ADMIN FUNCTIONS 

export async function change_to_admin_user() {
    if (current_user === 0) return
    current_user = 0
    await exe($`resim set-default-account $main_account $main_privkey $owner_badge`)
}

export async function change_current_user({ user_id }: { user_id: number }): Promise<number> {
    if (current_user === user_id) return current_user;
    current_user = user_id
    await exe($`resim set-default-account $account_${user_id} $privkey_${user_id} $owner_badge`)
    return user_id;
}


export async function create_user({ user_id }: { user_id: number }) {
    let output = await exe($`resim new-account`)
    set(_.account, output[0], user_id)
    set(_.privkey, output[2], user_id)
    await change_current_user({ user_id })
}

export async function admin_set_current_time(current_date: Date, year: number) {
    current_date.setUTCFullYear(year++)
    await exe($`resim set-current-time "${current_date.toISOString().split(".")[0]}Z"`)
}

export async function admin_set_time_and_prices({ date_time, btc, eth }: { date_time?: Date; btc?: number; eth?: number } = {}) {

    await change_to_admin_user()

    if (date_time != undefined) {
        await exe($`resim set-current-time "${date_time.toISOString().split(".")[0]}Z"`)
    }

    if (btc != undefined) {
        await exe($`resim call-method $${_.faucet_component} update_price $BTC ${btc} --proofs 1,$${_.faucet_admin_badge}`);
    }

    if (eth != undefined) {
        await exe($`resim call-method $${_.faucet_component} update_price $ETH ${eth} --proofs 1,$${_.faucet_admin_badge}`);
    }

}