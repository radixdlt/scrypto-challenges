// Checks the user's total tokens and deposit balance of those tokens
pub fn check_deposit_balance(&self, user_auth: Proof) -> String {
    let user_badge_data: User = user_auth.non_fungible().data();
    return info!("The user's balance information is: {:?}", user_badge_data.deposit_balance);
}

// Insert user into record hashmap
{let user_id: NonFungibleId = user_nft.non_fungible::<User>().id();
    let user: User = user_nft.non_fungible().data();
    self.user_record.insert(user_id, user);}

// Check lien - 06/01/22 - Make sure this makes sense
self.check_lien(&user_id, &address);

pub fn new_collateral_pool(&mut self, user_auth: Proof, token_address: ResourceAddress, collateral: Bucket) {

    let user_management = self.user_management_address.into();

    // Checking if a lending pool already exists for this token
    self.assert_pool_doesnt_exists(
        collateral.resource_address(), 
        String::from("New Collateral Pool")
    );

    // Checking if user exists
    let user_id = self.get_user(&user_auth);

    let deposit_amount = collateral.amount();

    let address: ResourceAddress = collateral.resource_address();
    // Sends an access badge to the collateral pool
    let access_badge_token = self.access_vault.authorize(|| borrow_resource_manager!(self.access_badge_token_address).mint(Decimal::one()));

    let (collateral_pool, transient_token): (ComponentAddress, Bucket) = CollateralPool::new(user_management, collateral, access_badge_token);

    let user_management: UserManagement = self.user_management_address.into();
    // FoldedLeverage component registers the transient token is this bad? 06/02/22
    // Is FoldedLeverage component even allowed to register resource?
    let transient_token_address = transient_token.resource_address();
    self.access_badge_token_vault.authorize(|| {user_management.register_resource(transient_token_address)});
    user_management.add_deposit_balance(user_id, token_address, deposit_amount, transient_token);

    // Inserts into lending pool hashmap
    self.collateral_pools.insert(
        address,
        collateral_pool.into()
    );

}

[ERROR] Panicked at 'called `Option::unwrap()` on a `None` value', src\user_management.rs:180:77

self.auth_vault.authorize(|| {
    let some_data = self
        .data
        .take_non_fungible(&NonFungibleId::from_u64(some_id));
                                                                                        
    let new_data = some_data.non_fungible::<SomeNFT>().data().new_data();
                                                                                        
    some_data
        .non_fungible::<SomeNFT>()
        .update_data(new_data);
                                                                                        
    self.data.put(new_data);
})

