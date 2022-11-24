resim set-default-account $address1 $priv_key1

export dispute_id=0a0100000000000000
export account=$address1
export badge=$contractor_badge
resim run ./transactions/dispute/complete_dispute.rtm
resim run ./transactions/claim_contractor_compensation.rtm

resim set-default-account $address2 $priv_key2
export account=$address2
resim run ./transactions/claim_client_work_refund.rtm
