resim set-default-account $address2 $priv_key2

export work_id=0a0100000000000000
export dispute_work_resource=$disputed_work_resource
export account=$address2
export badge=$client_badge
resim run ./transactions/dispute/create_new_dispute.rtm
