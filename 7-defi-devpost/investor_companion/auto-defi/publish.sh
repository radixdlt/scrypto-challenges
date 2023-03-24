reset_resim_and_publish (){
resim reset

resim new-account
resim new-account
resim publish .

}
#reset_resim_and_publish
# resim publish .

export account_1=account_sim1qw24s9xf6kqndhj5xc8qrgpjl4gxuya7xskcl0x0u8ps52twte
export account_2=account_sim1qwjg3j3ptcspw7f422ywd3zyffxvqj79r8gfmkd0eh2segzcw2
export package_address=package_sim1qygw3gt308cn2h9vmjqrmg2gw93qknpk3xkx30kms3qq0692xm
export xrd_resource_address=resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety


instantiate_companion (){
    # set the platform fee 
    resim call-function $package_address Companion instantiate 0.01 --manifest ../frontend/src/manifes.rtm
}

instantiate_companion

export component=component_sim1qf9w9rf62n2tgcmp4uddqwzqmm233cj99j7eztwmzlzqfzyuw5
export component_resource=resource_sim1qp9w9rf62n2tgcmp4uddqwzqmm233cj99j7eztwmzlzqvuu83u
export admin_badge=resource_sim1qqsnrgrlxn4uzemq8er2txddxrqhar2ut2jxrcvvkvxsaa2vr4


test_investor_access(){
    resim call-method $component create_preference {"finance_goal": "Make some dough", "risk_appetite": "High", "yield_duration" :20, "min_yield" :0.10}
}
# test_investor_access
#export investor_badge= [invest]


test_invest(){
        resim call-method $component invest 100000.0,$xrd_resource_address -p "1,$investor_badge"

}
#test_invest

test_admin_access(){
    resim call-method $component total_fees_collected --proofs "1,$admin_badge"
    resim call-method $component withdraw_fees 100.0 -p "1,$admin_badge"
    resim call-method $component change_platform_fee 0.3 -p "1,$admin_badge"
    resim call-method $component total_invested_amount  -p "1,$admin_badge"

}
#test_admin_access