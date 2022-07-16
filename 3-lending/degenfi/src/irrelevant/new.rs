blueprint!{
    struct ResourceUser {
        allowed_resources: Vec<ResourceAddress>
    }

    pub fn new(allowed: ResourceAddress) -> ComponentAddress {
        let access_rules: AccessRules = AccessRules::new()
            .method("add_resource", rule!(require(allowed()));
        
        Self {
            allowed_resources: Vec::from([allowed])
        }
        .instantiate()
        .add_access_check(access_rules)
        .globalize()
    }

    pub fn add_resource(&mut self, resource_address: ResourceAddress) {
        self.allowed_resources.push(resource_address);
    }
}
