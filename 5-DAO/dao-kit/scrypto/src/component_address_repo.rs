use scrypto::prelude::*;


blueprint! {

    /// Workaround for bug https://github.com/radixdlt/radixdlt-scrypto/issues/483
    ///
    /// Because of the bug component addresses cannot be stored in the metadata or non-fungible data
    /// of a resource. This component provides a systematic way of storing component addresses in a
    /// HashMap (works around the bug) and then providing an ID by which the addresses can later be
    /// looked up.
    struct ComponentAddressRepo {
        components_by_id: HashMap<NonFungibleId, ComponentAddress>,
        ids_by_component: HashMap<ComponentAddress, NonFungibleId>,
    }

    impl ComponentAddressRepo {
        pub fn instantiate_global() -> ComponentAddress {
            Self {
                components_by_id: HashMap::new(),
                ids_by_component: HashMap::new(),
            }
            .instantiate()
            .globalize()
        }

        pub fn lookup_address(&self, lookup: ComponentAddressLookup) -> ComponentAddress {
            *self.components_by_id.get(&lookup.lookup_id()).unwrap()
        }

        pub fn create_lookup(&mut self, component_address: ComponentAddress) -> ComponentAddressLookup {
            let lookup_id = if let Some(lookup_id) = self.ids_by_component.get(&component_address) {
                lookup_id.clone()
            } else {
                let lookup_id = NonFungibleId::random();
                self.components_by_id.insert(lookup_id.clone(), component_address);
                self.ids_by_component.insert(component_address, lookup_id.clone());
                lookup_id
            };
            ComponentAddressLookup(lookup_id)
        }
    }
}

#[derive(Encode, Decode, TypeId, Describe, Clone, Debug, Eq, PartialEq)]
pub struct ComponentAddressLookup(NonFungibleId);

impl ComponentAddressLookup {
    pub fn lookup_id(&self) -> NonFungibleId {
        self.0.clone()
    }
}
