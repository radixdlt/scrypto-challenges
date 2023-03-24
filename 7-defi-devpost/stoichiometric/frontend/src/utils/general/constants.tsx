
/*
    General purpose constants
*/

const dAppId = "account_tdx_b_1pq8yjqm777eg2jhla6sdye5swlkt7wyj65tqjzgav44qw9m04l";

const radix_api_url = "https://betanet.radixdlt.com"

const backend_api_url = 'https://beaker.fi:9999'


/*

    Dao constants
 */

const dao_address = "component_tdx_b_1qfge7gx8kw86jd34xvsa9lp6s3jv98jt6e7lnks983jq2n7346";

const voter_card_address = "resource_tdx_b_1qpge7gx8kw86jd34xvsa9lp6s3jv98jt6e7lnks983jqqjvlsp";

const proposal_receipt_address = "resource_tdx_b_1qqqujxnr6vt2gxkj6acge3qmp6wzv7lf32xx3dze86xqa7fvpl";


/*
    Dex constants
 */

const router_address = "component_tdx_b_1q2f2qy0qv5t2r4rx9jag5p6shms8es57e7wqjc7cjgnqmh62fg";

const position_address = "resource_tdx_b_1qru8j38866335wdr7xwj09u95rwllscrqkfcvk8pzfwslvav9g";


/*
    Stablecoin constants
 */

const issuer_address = "component_tdx_b_1qtnssm8g0zwev20ynn7578l3parucku6x6x7nudj7jzqf6xvsu";

const stablecoin_address = "resource_tdx_b_1qzs2upr69pc4djesgpl864ajmhqsmf58jxqsv5ftuphsc4gt7k";

const loan_address = "resource_tdx_b_1qr2g063eurussjcajx3585684tz8m0lszzz298dh0lqshms03l";

const flash_mint_address = "resource_tdx_b_1qq0k3ggft9demr2k874xta398dj7j0p984efu3wl08dsx0grle";

const stable_coin = { name: "Stoichiometric USD", symb: "SUSD", address: stablecoin_address, icon_url: "https://cdn-icons-png.flaticon.com/512/3215/3215346.png" }

const token_default = { name: 'Wrapped Bitcoin', symb: 'WBTC', address: 'resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r', icon_url: 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1200px-Bitcoin.svg.png' };





export { dAppId, radix_api_url, backend_api_url, dao_address, voter_card_address, proposal_receipt_address, router_address, position_address, issuer_address, stablecoin_address, loan_address, flash_mint_address, stable_coin, token_default }