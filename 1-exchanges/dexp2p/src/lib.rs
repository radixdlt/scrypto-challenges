use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct OrdenData {
    #[scrypto(mutable)]
    retirado: bool,
    vendido: bool
}

blueprint! {
    struct Dexp2p {
        registro_orden: HashMap<NonFungibleKey, (Address, Vault, Decimal)>,
        registro_ventas: HashMap<NonFungibleKey, Vault>,

        identification_minter: Vault,
        identification_nft_def: ResourceDef,
        identification_admin_def: ResourceDef,   

        comision: Decimal,
        caja_comision: Vault
        }

    impl Dexp2p {
        pub fn new(fee: Decimal) -> (Component, Bucket) {
           
            let identification_admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Admin DexP2p")
                .initial_supply_fungible(1);
           
            let identification_minter: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Autorización para acuñar DexP2p")
                .initial_supply_fungible(1);
          
            let identification_nft_def: ResourceDef = ResourceBuilder::new_non_fungible()
                .flags(MINTABLE|BURNABLE|INDIVIDUAL_METADATA_MUTABLE)
                .badge(identification_minter.resource_def(), MAY_MINT|MAY_BURN|MAY_CHANGE_INDIVIDUAL_METADATA)
                .no_initial_supply();
            
            let comp = Self {
                registro_orden: HashMap::new(),
                registro_ventas: HashMap::new(),
                comision: fee,
                caja_comision: Vault::new(RADIX_TOKEN),
                identification_minter: Vault::with_bucket(identification_minter),
                identification_nft_def: identification_nft_def,
                identification_admin_def: identification_admin.resource_def()
            }
            
            .instantiate();

            (comp, identification_admin)
        }

        #[auth(identification_admin_def)]
        pub fn cambiar_fee(&mut self, new_fee: Decimal) {
            self.comision = new_fee;
        }

        pub fn nueva_orden(&mut self, activo_comprar: Address, activo_vender: Bucket, precio_compra: Decimal, mut fee_xrd: Bucket) -> (Bucket, Bucket) {

            assert!(fee_xrd.amount() >= self.comision , "Comisión insuficiente");  
            self.caja_comision.put(fee_xrd.take(self.comision));

            let badge = self.identification_minter.authorize(|auth| {
                self.identification_nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), OrdenData{retirado: false, vendido: false}, auth)
            });
        
            let activo: Vault = Vault::with_bucket(activo_vender);

            self.registro_orden.insert(badge.get_non_fungible_key(),(activo_comprar, activo ,precio_compra));  
            
            (fee_xrd, badge)
        }

        pub fn eliminar_orden(&mut self, orden_badge: Bucket) -> Bucket {
            assert!(orden_badge.resource_def() == self.identification_nft_def, "No es una valida");
            assert!(orden_badge.amount() == Decimal::one(), "Solo puedes eliminar una orden");
            assert!(self.registro_orden.contains_key(&orden_badge.get_non_fungible_key()), "No existe la orden");

            let orden = self.registro_orden.get_mut(&NonFungibleKey::from(orden_badge.get_non_fungible_key())).unwrap().1.take_all();

            self.identification_minter.authorize(|auth| self.identification_nft_def.burn_with_auth(orden_badge, auth));
            
            orden
        }

        pub fn ejecutar_orden(&mut self, identificador: NonFungibleKey, mut pago: Bucket, mut fee_xrd: Bucket) -> (Bucket, Bucket, Bucket) {
            assert!(self.registro_orden.contains_key(&identificador), "No existe la orden");
            
            assert!(fee_xrd.amount() >= self.comision , "Comisión insuficiente");  
            
            let orden = self.registro_orden.get_mut(&identificador).unwrap();

            assert!(orden.1.amount() > Decimal::zero() , "Esta orden ya ha sido ejecutada");
            
            self.caja_comision.put(fee_xrd.take(self.comision));

            let cantidad: Decimal = orden.1.amount() / orden.2;
            assert_eq!(orden.0, pago.resource_address(), "El pago no coincide con el activo de venta"); 
            assert!(pago.amount() >= cantidad, "Saldo insuficiente");  
            //vendedor
            self.registro_ventas.insert(identificador.clone(), Vault::with_bucket(pago.take(cantidad)));  
            //comprador
            let badge = self.identification_minter.authorize(|auth| {
                self.identification_nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), OrdenData{retirado: false, vendido: true}, auth)
            });

            self.registro_ventas.insert(NonFungibleKey::from(badge.get_non_fungible_key()), Vault::with_bucket(orden.1.take_all()));

            self.identification_minter
                .authorize(|auth| self.identification_nft_def.update_non_fungible_data(&identificador, OrdenData{retirado: false, vendido: true} , auth));


            (pago, fee_xrd, badge)
        }

        pub fn retirar_orden(&mut self, orden_badge: BucketRef) -> Bucket {
            assert!(orden_badge.resource_def() == self.identification_nft_def, "No es una valida");
            assert!(orden_badge.amount() == Decimal::one(), "Solo puedes realizar un retiro a la vez");
            assert!(self.registro_ventas.contains_key(&orden_badge.get_non_fungible_key()), "No existe el retiro");

            let retorno = self.registro_ventas.get_mut(&orden_badge.get_non_fungible_key()).unwrap();

            assert!(retorno.amount() > Decimal::zero(), "Esta orden ya fue retirada");

            self.identification_minter.authorize(|auth| self.identification_nft_def.update_non_fungible_data(&orden_badge.get_non_fungible_key(), OrdenData{retirado: true, vendido: true}, auth));

            retorno.take_all()
        }
    }
}
