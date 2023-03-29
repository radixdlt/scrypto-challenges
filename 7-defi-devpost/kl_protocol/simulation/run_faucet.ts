#! ./node_modules/.bin/ts-node --esm

/**
 * run following commands for test
 *      chmod u+x run.ts    
 *      ./run.ts && source .env      
 */

import { $ } from 'zx';
import * as _ from './const.js';
import { _XRD_ } from './const.js';
import { exe, reset_env_vars, set } from './lib/utils.js';

void (async function () {

    reset_env_vars()

    set("XRD", _XRD_)


    await $`resim reset`


    let output: string[] = await exe($`resim new-account`)
    set(_.main_account, output[0])
    set(_.main_privkey, output[2])
    set(_.owner_badge, output[3])


    output = await exe($`resim publish ../scrypto`)
    set(_.main_package, output[0])

    output = await exe($`resim call-function $${_.main_package} Faucet new`)
    set(_.faucet_component, output[0])
    set(_.faucet_admin_badge, output[1])

    output = await exe($`resim call-function $${_.main_package} LendingPoolManager instantiate $${_.faucet_component}`)
    set(_.lending_component, output[0])
    set(_.lending_admin_badge, output[2])
    set(_.cdp_resource_address, output[3])

    //

    await create_lending_pool(_.btc_pool, "BTC", "Bitcoin", 21000, 0.7);
    await create_lending_pool(_.eth_pool, "ETH", "Ethereum", 1600, 0.6);
    await create_lending_pool(_.usdc_pool, "USDC", "Circle_USD", 1, 0.4);
    await create_lending_pool(_.usdt_pool, "USDT", "Tether_USD", 1.001, 0);


})();

async function create_lending_pool(
    pool_name: string, symbol: string, name: string, initial_price: number, liquidation_threshold: number,
    flashloan_fee_rate: number = 0.005, liquidation_spread: number = 0.05, liquidation_closing_factor: number = 1
) {
    let output = await exe($`resim call-method $${_.faucet_component} create_resource "${symbol}" "${name}" "" ${initial_price} --proofs 1,$${_.faucet_admin_badge}`);
    set(symbol, output[0]);

    output = await exe($`resim call-method $${_.lending_component} create_lending_pool $${symbol} "LEND-${symbol}" "Lended ${symbol}" "" ${flashloan_fee_rate} ${liquidation_threshold} ${liquidation_spread} ${liquidation_closing_factor} $${_.faucet_component} $${_.faucet_component} "${symbol}" --proofs 1,$${_.lending_admin_badge}`)
    set("LEN_" + symbol, output[2]);
    set(pool_name, output[0]);
}

