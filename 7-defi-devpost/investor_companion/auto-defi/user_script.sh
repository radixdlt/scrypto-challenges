export account_address=account_sim1qd7h56pmha6d20v3ej9lezxeh2fah6rrtshc8sfhlw7qzsf0km
export package_address=package_sim1q98q68qdmxxy8nqryvlpep3evyamq9us0w6t6zxle4hqygzv30
export blueprint_name=Companion
export component_address=component_sim1qfjs3hyz53gsnqjxm3yquxqajhv048syvtk8d4l2gy0qn99vss
export admin_badge_address=resource_sim1qpjs3hyz53gsnqjxm3yquxqajhv048syvtk8d4l2gy0qkmah0c
export investor_badge_address=resource_sim1qr0l4ktm329e6ufvha87rzranxmyxzga65zef8fmg94syermgm

# instantiate the companion component
resim call-function $package_address $blueprint_name instantiate 0.05

# create an investment preference
resim call-method $component_address create_preference {\"finance_goal\":\"Get more bags\",\"risk_appetite\":\"Low\",\"yield_duration\":\"100\", \"min_yield\":\"10\"}

# create an investment
resim call-method $component_address invest 100,$investor_badge_address

