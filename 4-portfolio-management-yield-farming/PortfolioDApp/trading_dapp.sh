set -e
clear
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
output=`resim call-function $tradingapp_package LendingApp instantiate_pool 1000,$xrd 1000 10 7 | awk '/Component: |Resource: / {print $NF}'`
export lending_component=`echo $output | cut -d " " -f1`
export lending_admin_badge=`echo $output | cut -d " " -f2`
export lend_nft=`echo $output | cut -d " " -f3`
export borrow_nft=`echo $output | cut -d " " -f4`
export lnd=`echo $output | cut -d " " -f5`
echo 'lending component = '$lending_component
echo 'lending admin badge = '$lending_admin_badge
echo 'lending_nft = '$lend_nft
echo 'borrow_nft = '$borrow_nft
echo 'lnd = ' $lnd

echo '====== Ready to create Portfolio component ======'
output=`resim call-function $tradingapp_package Portfolio new $xrd $btc $lending_component $component $lend_nft $borrow_nft $lnd | awk '/Component: |Resource: / {print $NF}'`
export portfolio_component=`echo $output | cut -d " " -f1`
export ADMIN_BADGE=`echo $output | cut -d " " -f2`
export user_account_history_nft=`echo $output | cut -d " " -f3`
export user_account_funding_nft=`echo $output | cut -d " " -f4`

echo "portfolio component = " $portfolio_component
echo 'portfolio admin badge = '$ADMIN_BADGE
echo 'user_account_history_nft = '$user_account_history_nft
echo 'user_account_funding_nft = '$user_account_funding_nft

echo '====== ACCOUNT ======'
resim show $account

echo '================== FUND TRADING APP ======'
resim call-method $component fund_market 100000,$xrd 20,$btc 1000,$eth 1000,$leo

echo '====== TRADING COMPONENT ======'
resim show $component
echo '====== LENDING COMPONENT ======'
resim show $lending_component
echo '====== PORTFOLIO COMPONENT ======'
resim show $portfolio_component

echo '===================================='
echo '====== REGISTER ON PORTFOLIO ======'
resim call-method $portfolio_component register $account
echo '====== REGISTER ON PORTFOLIO FOR LENDING ======'
resim call-method $portfolio_component register_for_lending
echo '====== REGISTER ON PORTFOLIO FOR BORROWING ======'
resim call-method $portfolio_component register_for_borrowing

echo '====== BUY GENERIC DIRECTLY WITH TRADING APP ======'
resim call-method $component buy_generic 500,$xrd  $eth

echo '====== BUY DIRECTLY WITH TRADING APP ======'
resim call-method $component buy 500,$xrd 
echo '====== SELL DIRECTLY WITH TRADING APP ======'
resim call-method $component sell 12.5,$btc


echo '====== LENDING WITH PORTFOLIO APP ======'
resim call-method $portfolio_component lend 100,$xrd
resim call-method $portfolio_component take_back 107,$lnd

echo '====== ACCOUNT ======'
resim show $account
echo '====== PORTFOLIO COMPONENT ======'
resim show $portfolio_component

echo '===================================='
echo '====== FUND PORTFOLIO APP ======'
resim call-method $portfolio_component fund_portfolio 10000,$xrd 
echo '====== WITHDRAW PORTFOLIO APP ======'
resim call-method $portfolio_component withdraw_portfolio 1,$user_account_funding_nft
echo '====== FUND AGAIN PORTFOLIO APP ======'
resim call-method $portfolio_component fund_portfolio 10000,$xrd 


echo '====== BUY by USING PORTFOLIO ======'
resim call-method $portfolio_component buy 500 $account $btc
echo '====== SELL by USING PORTFOLIO ======'
resim call-method $portfolio_component sell 12.5 $ADMIN_BADGE


echo '===================================='
echo '====== PORTFOLIO COMPONENT after buy/sell ======'
resim show $portfolio_component
echo '====== COMPONENT ======'
resim show $component
echo '====== ACCOUNT ======'
resim show $account




echo '===================================='
echo '====== BUY for later checking AUTO CLOSE ======'
resim call-method $portfolio_component buy 100 $account $btc


echo '===================================='
echo '====== N. RANDOM ======'
resim call-method $component current_price $xrd $btc 

epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc

echo '====== BUY for later checking AUTO CLOSE ======'
resim call-method $portfolio_component buy 100 $account $btc

epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc

echo '====== BUY for later checking AUTO CLOSE ======'
resim call-method $portfolio_component buy 100 $account $btc


epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc 

echo '====== BUY for later checking AUTO CLOSE ======'
resim call-method $portfolio_component buy 100 $account $btc


epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc 

echo '====== BUY for later checking AUTO CLOSE ======'
resim call-method $portfolio_component buy 100 $account $btc

echo '===================================='
echo '====== RUNNING OPERATION ======'
resim call-method $portfolio_component position

echo '====== ACCOUNT ======'
resim show $account

echo '====== WITHDRAW PORTFOLIO APP ======'
resim call-method $portfolio_component withdraw_portfolio 1,$user_account_funding_nft

echo '====== CLOSE ALL POSITIONS OPERATION ======'
export fake_id=12345
resim call-method $portfolio_component close_position $position_id