use derive_new::new;
use scrypto::prelude::*;

/// A helper struct to streamline the creation and minting of the various RadSense resources
#[derive(TypeId, Describe, Encode, Decode)]
pub struct Minter {
    /// The minting authority
    authority: Vault,
}

impl Minter {
    /// Creates a new minting authority setting the given bucket as the minter's authority
    pub fn new(authority: Bucket) -> Self {
        Self { authority: Vault::with_bucket(authority) }
    }

    /// Creates a new fungible resource with the given `name` (written to the resource's metadata) and the given
    /// `divisibility`. There is no initial supply. Returns the resource address.
    pub fn new_fungible_resource(&self, name: &str, divisibility: u8) -> ResourceAddress {
        ResourceBuilder::new_fungible()
            .divisibility(divisibility)
            .metadata("name", name)
            .mintable(rule!(require(self.authority.resource_address())), LOCKED)
            .no_initial_supply()
    }

    /// Creates a new non fungible resource with the given `name` (written to the resource's metadata) and the given
    /// config. There is no initial supply. Returns the resource address.
    pub fn new_non_fungible_resource(&self, name: &str, config: NonFungibleConfig) -> ResourceAddress {
        let mut rb = ResourceBuilder::new_non_fungible();
        rb.mintable(rule!(require(self.authority.resource_address())), LOCKED);

        if config.updatable_non_fungible_data {
            rb.updateable_non_fungible_data(rule!(require(self.authority.resource_address())), LOCKED);
        }
        rb.metadata("name", name);
        for (key, value) in config.metadata {
            rb.metadata(key, value);
        }

        rb.no_initial_supply()
    }

    /// Mints the given `amount` of the given fungible `resource` and returns it.
    pub fn mint(&self, resource: ResourceAddress, amount: Decimal) -> Bucket {
        self.authority.authorize(|| {
            let rm = borrow_resource_manager!(resource);
            rm.mint(amount)
        })
    }

    /// Mints a non fungible of the given `resource` with the given `data` and returns it. The non fungible is assigned
    /// a random ID.
    pub fn mint_non_fungible<T: NonFungibleData>(&self, resource: ResourceAddress, data: T) -> Bucket {
        self.mint_non_fungible_with_id(resource, data, &NonFungibleId::random())
    }

    /// Mints a non fungible of the given `resource` with the given `data` and returns it. The non fungibles is assigned
    /// the given `id`.
    pub fn mint_non_fungible_with_id<T: NonFungibleData>(
        &self,
        resource: ResourceAddress,
        data: T,
        id: &NonFungibleId,
    ) -> Bucket {
        self.authority.authorize(|| {
            let rm = borrow_resource_manager!(resource);
            rm.mint_non_fungible(id, data)
        })
    }

    /// Updates the non fungible identified by the given `resource` and `id` with the given `data`.
    pub fn update_non_fungible<T: NonFungibleData>(&self, resource: ResourceAddress, id: &NonFungibleId, data: T) {
        self.authority.authorize(|| {
            let rm = borrow_resource_manager!(resource);
            rm.update_non_fungible_data(id, data)
        })
    }

    /// Modifies the non fungible identified by the given resource` and `id` by applying the given code `block` to it.
    ///
    /// The non fungible is loaded and passed to the code block which must mutate it. The mutated non funguible is then
    /// saved back to the ledger.
    pub fn modify_non_fungible_data<T, F>(&self, resource: ResourceAddress, id: &NonFungibleId, block: F)
    where
        T: NonFungibleData,
        F: FnOnce(&mut T),
    {
        let rm = borrow_resource_manager!(resource);
        let mut data: T = rm.get_non_fungible_data(id);
        block(&mut data);
        rm.update_non_fungible_data(id, data);
    }
}

/// A config describing how a new non fungible resource should be created.
#[derive(new)]
pub struct NonFungibleConfig {
    /// Whether the non fungible's data should be updatable
    #[new(default)]
    updatable_non_fungible_data: bool,

    /// The metadata of the non fungible. These metadata are applied in addition to the name metadata field, possibly
    /// overwriting it.
    #[new(default)]
    metadata: HashMap<&'static str, String>,
}

impl NonFungibleConfig {
    pub fn updatable_non_fungible_data(mut self, updatable: bool) -> Self {
        self.updatable_non_fungible_data = updatable;
        self
    }

    pub fn metadata(mut self, key: &'static str, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
