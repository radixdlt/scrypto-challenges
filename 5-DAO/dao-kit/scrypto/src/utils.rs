use scrypto::prelude::*;

pub trait IntoComponent {
    fn into_component<T: Decode + From<ComponentAddress>>(self) -> T;
}

impl IntoComponent for ComponentAddress {
    fn into_component<T: Decode + From<ComponentAddress>>(self) -> T {
        self.into()
    }
}
