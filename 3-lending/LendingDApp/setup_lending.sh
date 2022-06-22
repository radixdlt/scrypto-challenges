export xrd=030000000000000000000000000000000000000000000000000004

echo "Reseting environment"
resim reset
export account=$(resim new-account | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Account = " $account

echo "Publishing dapp"
export lendingapp_package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
PACKAGE_RESULT=$(resim call-function $lendingapp_package LendingApp instantiate_pool 100,$xrd 100 10 7)
echo "Package = " $PACKAGE_RESULT
export component=$(echo $PACKAGE_RESULT | sed -nr "s/^.*Component: \([0-9a-zA-Z]*\).*/\1/")
export lnd_token=$(echo $PACKAGE_RESULT | sed -nr "s/^.*Resource: \([0-9a-zA-Z]*\).*/\1/")

echo "Component = " $component
echo "Lnd = " $lnd_token