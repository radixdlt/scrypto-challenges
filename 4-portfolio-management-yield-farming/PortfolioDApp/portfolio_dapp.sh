set -e
clear
export xrd=030000000000000000000000000000000000000000000000000004

echo "Resetting environment"
resim reset
#export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

OP1=$(resim new-account)
export PRIV_KEY1=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY1=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

echo "Publishing dapp"
export tradingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $tradingapp_package

echo "Account = " $account
echo "XRD = " $xrd

export btc=$(resim new-token-fixed --symbol btc 400 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "btc = " $btc
export eth=$(resim new-token-fixed --symbol eth 2000 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "eth = " $eth
export leo=$(resim new-token-fixed --symbol leo 10000 | sed -nr "s/└─ Resource: ([[:alnum:]_]+)/\1/p")
echo "leo = " $leo

echo '====== SHOW ACCOUNT ======'
resim show $account

echo '====== CREATE ACCOUNTS ======'
OP2=$(resim new-account)
export PRIV_KEY2=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY2=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS2=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP3=$(resim new-account)
export PRIV_KEY3=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY3=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS3=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
OP4=$(resim new-account)
export PRIV_KEY4=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export PUB_KEY4=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export ACC_ADDRESS4=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

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
output=`resim call-function $tradingapp_package Portfolio new $xrd $btc $eth $leo $lending_component $component $lend_nft $borrow_nft $lnd | awk '/Component: |Resource: / {print $NF}'`
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
resim call-method $component fund_market 100000,$xrd 400,$btc 1000,$eth 1000,$leo

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
echo '====== SELL GENERIC DIRECTLY WITH TRADING APP ======'
resim call-method $component sell_generic 40,$eth

echo '====== BUY DIRECTLY WITH TRADING APP ======'
resim call-method $component buy 500,$xrd 
echo '====== SELL DIRECTLY WITH TRADING APP ======'
resim call-method $component sell 12.5,$btc


echo '===================================='
echo '====== FUND PORTFOLIO APP ======'
resim call-method $portfolio_component fund_portfolio 10000,$xrd 1,$user_account_funding_nft
echo '====== WITHDRAW PORTFOLIO APP ======'
resim call-method $portfolio_component withdraw_portfolio 1,$user_account_funding_nft
echo '====== FUND AGAIN PORTFOLIO APP ======'
resim call-method $portfolio_component fund_portfolio 2000,$xrd 1,$user_account_funding_nft



echo '====== LENDING WITH PORTFOLIO APP ======'
resim call-method $portfolio_component lend 100
resim call-method $portfolio_component take_back 107

echo '====== ACCOUNT ======'
resim show $account


echo '====== BUY BTC by USING PORTFOLIO ======'
resim call-method $portfolio_component buy 500 $account $btc 1,$user_account_funding_nft
echo '====== SELL BTC by USING PORTFOLIO ======'
resim call-method $portfolio_component sell 12.5 $btc

echo '====== BUY ETH by USING PORTFOLIO ======'
resim call-method $portfolio_component buy 500 $account $eth 1,$user_account_funding_nft
echo '====== SELL ETH by USING PORTFOLIO ======'
resim call-method $portfolio_component sell 50 $eth

echo '====== BUY LEO by USING PORTFOLIO ======'
resim call-method $portfolio_component buy 500 $account $leo 1,$user_account_funding_nft
echo '====== SELL LEO by USING PORTFOLIO ======'
resim call-method $portfolio_component sell 100 $leo


echo '===================================='
echo '====== PORTFOLIO COMPONENT after buy/sell ======'
resim show $portfolio_component
echo '====== COMPONENT ======'
resim show $component
echo '====== ACCOUNT ======'
resim show $account




echo '===================================='
echo '====== BUY  ======'
resim call-method $portfolio_component buy 100 $account $btc 1,$user_account_funding_nft
echo '====== CHECK PRICE ======'
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc

echo '====== BUY  ======'
resim call-method $portfolio_component buy 100 $account $btc 1,$user_account_funding_nft
echo '====== CHECK PRICE ======'
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc

echo '====== BUY  ======'
resim call-method $portfolio_component buy 100 $account $btc 1,$user_account_funding_nft
echo '====== CHECK PRICE ======'
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc 

echo '====== BUY  ======'
resim call-method $portfolio_component buy 100 $account $btc 1,$user_account_funding_nft
echo '====== CHECK PRICE ======'
epoch=$(($epoch + 1))
resim set-current-epoch $epoch
resim call-method $component current_price $xrd $btc 

echo '====== BUY  ======'
resim call-method $portfolio_component buy 100 $account $btc 1,$user_account_funding_nft



echo '===================================='
echo '====== RUNNING OPERATIONS ======'
resim call-method $portfolio_component position

echo '====== ACCOUNT ======'
resim show $account

echo '====== SOME NEW ACCOUNTS ARE FUNDING IN THE PORTFOLIO APP ======'
resim set-default-account $ACC_ADDRESS2 $PRIV_KEY2
resim call-method $portfolio_component register $ACC_ADDRESS2
resim call-method $portfolio_component fund_portfolio 50000,$xrd 1,$user_account_funding_nft

resim set-default-account $ACC_ADDRESS3 $PRIV_KEY3
resim call-method $portfolio_component register $ACC_ADDRESS3
resim call-method $portfolio_component fund_portfolio 50000,$xrd 1,$user_account_funding_nft

resim set-default-account $ACC_ADDRESS4 $PRIV_KEY4
resim call-method $portfolio_component register $ACC_ADDRESS4
resim call-method $portfolio_component fund_portfolio 50000,$xrd 1,$user_account_funding_nft

echo '====== PORTFOLIO COMPONENT before leveraged buy  ======'
resim show $portfolio_component
echo '====== SET DEFAULT ACCOUNT (AMOUNT FUNDED 1000)  ======'
resim set-default-account $account $PRIV_KEY1
echo '====== BUY LEVERAGED  ======'
resim call-method $portfolio_component buy 10000 $account $btc 1,$user_account_funding_nft

echo '====== ANOTHER EPOCH ADVANCE ======'
epoch=$(($epoch + 1))
resim set-current-epoch $epoch

echo '====== RUNNING OPERATIONS UPDATE ======'
resim call-method $portfolio_component position

echo '====== CURRENT PORTFOLIO VALUE ======'
resim call-method $portfolio_component portfolio_total_value

echo '====== WITHDRAW PORTFOLIO APP ======'
resim set-default-account $account $PRIV_KEY1
resim call-method $portfolio_component withdraw_portfolio 1,$user_account_funding_nft

echo '====== CURRENT PORTFOLIO VALUE ======'
resim call-method $portfolio_component portfolio_total_value

echo '====== PORTFOLIO COMPONENT  ======'
resim show $portfolio_component
echo '====== ACCOUNT ======'
resim show $account

echo '====== CLOSE ALL POSITIONS OPERATION ======'
#export fake_id=12345
#resim call-method $portfolio_component close_position $position_id
#
# This has been tested and described in the documentation