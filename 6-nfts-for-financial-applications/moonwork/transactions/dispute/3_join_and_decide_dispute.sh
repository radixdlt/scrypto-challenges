resim set-default-account $address3 $priv_key3
export dispute_id=0a0100000000000000
export account=$address3
export badge=$contractor_badge
export side=Contractor
resim run ./transactions/dispute/join_and_decide_dispute.rtm

resim set-default-account $address4 $priv_key4
export dispute_id=0a0100000000000000
export account=$address4
export badge=$client_badge
export side=Contractor
resim run ./transactions/dispute/join_and_decide_dispute.rtm

resim set-default-account $address1 $priv_key1
