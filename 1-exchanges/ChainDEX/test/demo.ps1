$account = '0293c502780e23621475989d707cd8128e4506362e5fed6ac0c00a'
$package = '01ff3eae9463d913a0dba37b78896414eadf59ce144a9143c8018f'
$bitcoin = '03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542'
$tether = '0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d'
$chain_book = '02b47568f61ad9596570b52e114e5151dd9ea4df982fb940eb0b6b'
$order = '03bb52f6cf513295fdd13dfa70a7bbd49aa8a860a304b4ff00c1f0'

# setup
resim reset
resim new-account
resim publish .
resim new-token-fixed 21000000 --name Bitcoin
resim new-token-fixed 1000000000 --name Tether
resim call-function $package ChainBook instantiate_chain_book BTC/USDT $bitcoin $tether

# limit sell 
resim call-method $chain_book create_order 1,$bitcoin 39000
resim call-method $chain_book create_order 1,$bitcoin 40000
resim call-method $chain_book create_order 1,$bitcoin 42000
resim call-method $chain_book create_order 1,$bitcoin 38000
resim call-method $chain_book create_order 1,$bitcoin 43000
resim call-method $chain_book create_order 1,$bitcoin 41000
resim call-method $chain_book create_order 1,$bitcoin 40000
resim call-method $chain_book create_order 1,$bitcoin 40000

# limit buy
resim call-method $chain_book create_order 10000,$tether 36000
resim call-method $chain_book create_order 10000,$tether 35000
resim call-method $chain_book create_order 10000,$tether 33000
resim call-method $chain_book create_order 10000,$tether 37000
resim call-method $chain_book create_order 10000,$tether 32000
resim call-method $chain_book create_order 10000,$tether 34000
resim call-method $chain_book create_order 10000,$tether 35000
resim call-method $chain_book create_order 10000,$tether 35000

# market sell
resim call-method $chain_book create_order 1,$bitcoin 0

# market buy
resim call-method $chain_book create_order 150000,$tether 10000000

# claim tokens from completed order
resim call-method $chain_book claim_tokens `#00000000000000000000000000000001,$order

# claim tokens from part filled order
resim call-method $chain_book claim_tokens `#0000000000000000000000000000000f,$order

# part market part limit sell
resim call-method $chain_book create_order 1,$bitcoin 35000

# part market part limit buy
resim call-method $chain_book create_order 150000,$tether 40000

# cancel order
resim call-method $chain_book cancel_order `#00000000000000000000000000000003,$order

resim show $chain_book
resim show $account