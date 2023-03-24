resim set-default-account $address1 $priv_key1
export account=$address1
resim run ./transactions/claim_contractor_compensation.rtm

resim set-default-account $address3 $priv_key3
export account=$address3
resim run ./transactions/claim_contractor_compensation.rtm
