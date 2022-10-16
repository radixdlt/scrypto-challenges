resim new-account
SELLER=...
SELLER_PK=...

resim new-account
BUYER=...
BUYER_PK=...

resim publish .
PKG=...

resim call-function $PKG TokenSale new 10
COMP=...
BADGE=...
TOKEN=...

resim set-default-account $BUYER $BUYER_PK

resim call-method $COMP buy 99,030000000000000000000000000000000000000000000000000004

resim show $BUYER

#should fail without a badge
resim call-method $COMP withdraw_funds 100

resim call-method $COMP change_price 0.1

resim set-default-account $SELLER $SELLER_PK
