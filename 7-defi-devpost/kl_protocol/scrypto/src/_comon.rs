use scrypto::prelude::*;
///

#[derive(NonFungibleData, Copy, Debug, Clone)]
pub struct AuthBadgeData {}

///

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ComponentAddressRegistery {
    component_address_lookup: HashMap<ResourceAddress, ComponentAddress>,
    resource_address_lookup: HashMap<ResourceAddress, ResourceAddress>,
    revers_resource_address_lookup: HashMap<ResourceAddress, ResourceAddress>,
}

impl ComponentAddressRegistery {
    pub fn new() -> ComponentAddressRegistery {
        ComponentAddressRegistery {
            component_address_lookup: HashMap::new(),
            resource_address_lookup: HashMap::new(),
            revers_resource_address_lookup: HashMap::new(),
        }
    }

    pub fn regester_component(
        &mut self,
        resource_address: ResourceAddress,
        component_address: ComponentAddress,
    ) {
        self.component_address_lookup
            .insert(resource_address, component_address);
    }

    pub fn regester_addresses(
        &mut self,
        resource_address: ResourceAddress,
        revers_resource_address: ResourceAddress,
    ) {
        self.resource_address_lookup
            .insert(revers_resource_address, resource_address);

        self.revers_resource_address_lookup
            .insert(resource_address, revers_resource_address);
    }

    pub fn get_component_address(&self, resource_address: ResourceAddress) -> ComponentAddress {
        let (pool_resource_address, _) = if self.resource_address_lookup.len() == 0 {
            (resource_address, resource_address)
        } else {
            self.get_resource_address(resource_address)
        };

        match self.component_address_lookup.get(&pool_resource_address) {
            Some(m) => *m,
            None => panic!("Missing resource address"),
        }
    }

    pub fn get_resource_address(
        &self,
        revers_or_resource_address: ResourceAddress,
    ) -> (ResourceAddress, ResourceAddress) {
        let revers_resource_address: ResourceAddress;

        let resource_address = match self
            .revers_resource_address_lookup
            .get(&revers_or_resource_address)
        {
            Some(m) => {
                revers_resource_address = *m;
                revers_or_resource_address
            }
            None => match self
                .resource_address_lookup
                .get(&revers_or_resource_address)
            {
                Some(m) => {
                    revers_resource_address = revers_or_resource_address;
                    *m
                }
                None => panic!("Missing resource address"),
            },
        };

        (resource_address, revers_resource_address)
    }
}
