resim set-default-account $address1 $priv_key1
export account=$address1

resim set-current-epoch 481

resim run ./transactions/promotion/remove_expired_promotions.rtm
