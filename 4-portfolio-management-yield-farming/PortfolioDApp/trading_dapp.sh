set -e

export xrd=030000000000000000000000000000000000000000000000000004

echo "Resetting environment"
resim reset
export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo "Publishing dapp"
export tradingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $tradingapp_package

echo "Account = " $account
echo "XRD = " $xrd

export btc=$(resim new-token-fixed --symbol btc 100 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "btc = " $btc
export eth=$(resim new-token-fixed --symbol eth 2000 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "eth = " $eth
export leo=$(resim new-token-fixed --symbol leo 10000 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "leo = " $leo

resim show $account

echo '====== Ready to create Trading component ======'
export component=$(resim call-function $tradingapp_package TradingApp create_market $xrd $btc $eth $leo | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo "trading component = " $component

echo '====== Ready to create Lending component ======'
export lending_component=$(resim call-function $tradingapp_package LendingApp instantiate_pool 100,$xrd 1000 10 7 | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo "lending component = " $lending_component

echo '====== Ready to create Portfolio component ======'
export portfolio_component=$(resim call-function $tradingapp_package Portfolio new $xrd $btc $lending_component $component | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
echo "portfolio component = " $portfolio_component

#export component=$(resim call-function $tradingapp_package TradingApp create_market 1000,$xrd 10,$btc 1000,$eth 10000,$leo | sed -nr "s/Component: ([[:alnum:]_]+)/\1/p")
#echo "COMPONENT = " $component
#output=`resim call-function $tradingapp_package TradingApp create_market 1000,$xrd 10,$btc 1000,$eth 1000,$leo | awk '/Component: / {print $NF}'`
#export component=`echo $output | cut -d " " -f1`
#export ADMIN_BADGE=`echo $output | cut -d " " -f2`
#export lend_nft=`echo $output | cut -d " " -f3`
#export borrow_nft=`echo $output | cut -d " " -f4`
#export lnd=`echo $output | cut -d " " -f5`

#echo 'ADMIN_BADGE = '$ADMIN_BADGE
#echo 'LEND_NFT = '$lend_nft
#echo 'BORROW_NFT = '$borrow_nft
#echo 'LND = ' $lnd

echo '====== ACCOUNT ======'
resim show $account

echo '================== FUND MARKET ======'
resim call-method $component fund_market 1000,$xrd 20,$btc 1000,$eth 1000,$leo

echo '====== TRADING COMPONENT ======'
resim show $component
echo '====== LENDING COMPONENT ======'
resim show $lending_component
echo '====== PORTFOLIO COMPONENT ======'
resim show $portfolio_component

echo '====== BUY ======'
resim call-method $component buy 500,$xrd 

echo '====== SELL ======'
resim call-method $component sell 12.5,$btc

echo '====== ACCOUNT AFTER BUY/SELL ======'
resim show $account

echo '====== COMPONENT ======'
resim show $component

echo '====== PORTFOLIO COMPONENT before buy/sell ======'
resim show $portfolio_component

echo '====== FUND by USING PORTFOLIO ======'
resim call-method $portfolio_component fund_portfolio 10000,$xrd

echo '====== BUY by USING PORTFOLIO ======'
resim call-method $portfolio_component buy 500
echo '====== SELL by USING PORTFOLIO ======'
resim call-method $portfolio_component sell 12.5

echo '====== PORTFOLIO COMPONENT after buy/sell ======'
resim show $portfolio_component

echo '====== COMPONENT ======'
resim show $component



echo '====== N. RANDOM ======'
resim call-method $component current_price $xrd $btc 


# logc "Advance epoch by 1."
# epoch=$(($epoch + 1))
# resim set-current-epoch $epoch