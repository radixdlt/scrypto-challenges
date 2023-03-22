use crate::{_comon::*, _lending_pool::*, _lending_pool_manager::*};
use scrypto::prelude::*;
use std::cmp::min;

external_component! {
    LendingPoolComponentTarget {

        fn update_interest(&mut self, interest_type: u8);

        fn move_loan_share(&mut self, from_interest_type: u8, interest_type_dest: u8, amount: Decimal);

        fn borrow(&mut self, amount: Decimal, interest_type:u8) -> (Bucket, Decimal);

        fn repay(&mut self,  payment: Bucket, interest_type:u8) -> (Bucket, Decimal);

        fn add_collateral(&mut self, assets: Bucket) -> (Decimal, ResourceAddress);

        fn remove_collateral(&mut self, amount: Decimal, get_pool_share: bool) -> Bucket;

        //

        fn get_pool_state(&mut self) -> LendingPoolState;

        fn get_pool_share_ratio(&self) -> Decimal;

        fn get_loan_share_ratio(&self, interest_type: u8) -> Decimal;

    }
}

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendendCollateralPostion {
    pub position_id: u128,
    pub resource_address: ResourceAddress,
    pub pool_share_resource_address: ResourceAddress,
    pub pool_share: Decimal,

    //
    pub cdp_id: NonFungibleLocalId,

    pub pool_component_address: ComponentAddress,

    pub pool_state: LendingPoolState,

    pub collateral_amount: Decimal,

    pub collateral_solvency_value: Decimal,

    pub collateral_healthy_value: Decimal,
}

impl ExtendendCollateralPostion {
    pub fn get_pool_share_ratio(&self) -> Decimal {
        LendingPoolComponentTarget::at(self.pool_component_address).get_pool_share_ratio()
    }
}

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendedDebtPostion {
    pub position_id: u128,
    pub resource_address: ResourceAddress,
    pub loan_share: Decimal,
    pub interest_type: u8,

    //
    pub cdp_id: NonFungibleLocalId,

    pub pool_component_address: ComponentAddress,

    pub pool_state: LendingPoolState,

    pub loan_amount: Decimal,

    pub loan_value: Decimal,
}

impl ExtendedDebtPostion {
    pub fn get_loan_share_ratio(&self) -> Decimal {
        LendingPoolComponentTarget::at(self.pool_component_address)
            .get_loan_share_ratio(self.interest_type)
    }
}

#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendedCollaterizedDebtPosition {
    pub cdp_id: NonFungibleLocalId,
    pub delegator_id: Option<NonFungibleLocalId>,
    pub delegated_ids: Vec<NonFungibleLocalId>,

    pub health_factor: Decimal,
    pub solvency_factor: Decimal,

    pub total_loan_value: Decimal,
    pub total_collateral_solvency_value: Decimal,
    pub total_collateral_healthy_value: Decimal,

    pub collateral_positions: HashMap<u128, ExtendendCollateralPostion>,
    pub debt_positions: HashMap<u128, ExtendedDebtPostion>,

    //
    cdp_resource_address: ResourceAddress,
    lending_pool_registry: ComponentAddressRegistery,
}

impl ExtendedCollaterizedDebtPosition {
    pub fn create_extended_cdp(
        cdp_id: NonFungibleLocalId,
        cdp_resource_address: ResourceAddress,
        lending_pool_registry: ComponentAddressRegistery,
    ) -> ExtendedCollaterizedDebtPosition {
        let cdp_data: CollaterizedDebtPositionData =
            borrow_resource_manager!(cdp_resource_address).get_non_fungible_data(&cdp_id);

        let mut extended_cdp: ExtendedCollaterizedDebtPosition;

        if cdp_data.delegator_id.is_none() {
            extended_cdp = ExtendedCollaterizedDebtPosition {
                cdp_id: cdp_id.clone(),
                cdp_resource_address,
                lending_pool_registry,

                delegator_id: cdp_data.clone().delegator_id,
                delegated_ids: cdp_data.clone().delegated_ids,

                health_factor: Decimal::MAX,
                solvency_factor: Decimal::MAX,
                total_loan_value: Decimal::ZERO,
                total_collateral_healthy_value: Decimal::ZERO,
                total_collateral_solvency_value: Decimal::ZERO,

                collateral_positions: HashMap::new(),
                debt_positions: HashMap::new(),
            };
            extended_cdp.hydrate(cdp_id, Some(cdp_data));

            for delegated_cdp_id in extended_cdp.clone().delegated_ids.iter() {
                extended_cdp.hydrate(delegated_cdp_id.clone(), None);
            }
        } else {
            extended_cdp = match cdp_data.delegator_id.clone() {
                Some(some_delegator_id) => Self::create_extended_cdp(
                    some_delegator_id,
                    cdp_resource_address,
                    lending_pool_registry,
                ),
                None => panic!("CDP ID Note found"),
            };

            // Restore parameters of presented CDP NFT
            extended_cdp.cdp_id = cdp_id;
            extended_cdp.delegator_id = cdp_data.clone().delegator_id;
            extended_cdp.delegated_ids = cdp_data.clone().delegated_ids;
        }

        extended_cdp
    }

    pub fn hydrate(
        &mut self,
        cdp_id: NonFungibleLocalId,
        cdp_data: Option<CollaterizedDebtPositionData>,
    ) {
        let _cdp_data = match cdp_data {
            Some(c) => c,
            None => {
                borrow_resource_manager!(self.cdp_resource_address).get_non_fungible_data(&cdp_id)
            }
        };

        for (_, debpt_position) in _cdp_data.debts.clone() {
            self._new_debt_position(cdp_id.clone(), debpt_position);
        }

        for (_, collatereal_position) in _cdp_data.collaterals.clone() {
            self._new_collateral_position(cdp_id.clone(), collatereal_position);
        }

        self._update();
    }

    pub fn chek_health_factor(&self, assert_health_factor: bool) -> bool {
        let debt_state = self.health_factor > Decimal::ONE;

        debug!(
            "Loan value {}, health_factor {}, solvency_collateral_value {}",
            self.total_loan_value, self.health_factor, self.solvency_factor
        );

        if assert_health_factor {
            assert!(debt_state, "Health factor need to be lower than 1");
        }

        debt_state
    }

    pub fn can_be_liquidated(&self) {
        debug!(
            "Loan value {}, health_factor {}, solvency_collateral_value {}",
            self.total_loan_value, self.health_factor, self.solvency_factor
        );

        assert!(
            self.health_factor <= Decimal::ONE && self.solvency_factor >= Decimal::ONE,
            "Can not liquidate this CDP: factor need to be lower than 1"
        );
    }

    pub fn can_be_auto_liquidated(&self) {
        let debt_state = self.health_factor <= Decimal::ONE || self.solvency_factor <= Decimal::ONE;

        debug!(
            "Loan value {}, health_factor {}, solvency_collateral_value {}",
            self.total_loan_value, self.health_factor, self.solvency_factor
        );

        assert!(
            debt_state,
            "Can no liquidate this CDP: Health factor need to be lower than 1"
        );
    }

    pub fn get_cdp(&mut self) -> CollaterizedDebtPositionData {
        let mut cdp = CollaterizedDebtPositionData {
            delegator_id: self.delegator_id.clone(),
            delegated_ids: self.delegated_ids.clone(),
            collaterals: HashMap::new(),
            debts: HashMap::new(),
        };

        for item in &self.debt_positions {
            let (_, dp) = item;

            if dp.cdp_id != self.cdp_id {
                continue;
            }

            cdp.debts.insert(
                dp.position_id,
                DebtPostion {
                    position_id: dp.position_id,
                    resource_address: dp.resource_address,
                    loan_share: dp.loan_share,
                    interest_type: dp.interest_type,
                },
            );
        }

        for item in &self.collateral_positions {
            let (res, cp) = item;

            if cp.cdp_id != self.cdp_id {
                continue;
            }

            cdp.collaterals.insert(
                *res,
                CollateralPostion {
                    position_id: cp.position_id,
                    pool_share: cp.pool_share,
                    resource_address: cp.resource_address,
                    pool_share_resource_address: cp.pool_share_resource_address,
                },
            );
        }

        cdp
    }

    pub fn new_collateral(&mut self, assets: Bucket, new_position_id: u128) -> Decimal {
        let (ra, pool_share_ra) = self
            .lending_pool_registry
            .get_resource_address(assets.resource_address());

        let (pool_share_amount, _) = self
            ._get_pool(
                self.lending_pool_registry
                    .get_component_address(pool_share_ra),
            )
            .add_collateral(assets);

        self._new_collateral_position(
            self.cdp_id.clone(),
            CollateralPostion {
                position_id: new_position_id,
                resource_address: ra,
                pool_share_resource_address: pool_share_ra,
                pool_share: pool_share_amount,
            },
        );

        self._update();

        pool_share_amount
    }

    pub fn add_collateral(&mut self, assets: Bucket, position_id: u128) -> Decimal {
        let mut cp = self._remove_cp(position_id);

        let (pool_share_amount, _) = self
            ._get_pool(cp.pool_component_address)
            .add_collateral(assets);

        cp.pool_share += pool_share_amount;
        self.collateral_positions.insert(position_id, cp);

        self._update();

        pool_share_amount
    }

    pub fn remove_collateral(
        &mut self,
        position_id: u128,
        pool_shares_amount: Decimal,
        get_pool_share: bool,
    ) -> Bucket {
        let mut cp = self._remove_cp(position_id);

        cp.pool_share -= pool_shares_amount;

        let lp_tokens = self
            ._get_pool(cp.pool_component_address)
            .remove_collateral(pool_shares_amount, get_pool_share);

        self.collateral_positions.insert(position_id, cp);

        self._update();

        lp_tokens
    }

    pub fn borrow(
        &mut self,
        resource_address: ResourceAddress,
        interest_type: u8,
        amount: Decimal,
        new_position_id: u128,
    ) -> Bucket {
        let (_, pool_share_ra) = self
            .lending_pool_registry
            .get_resource_address(resource_address);

        let (borrowed_assets, loan_share) = self
            ._get_pool(self._get_pool_address(pool_share_ra))
            .borrow(amount, interest_type);

        self._new_debt_position(
            self.cdp_id.clone(),
            DebtPostion {
                position_id: new_position_id,
                resource_address,
                loan_share,
                interest_type,
            },
        );

        self._update();

        borrowed_assets
    }

    pub fn borrow_more(&mut self, amount: Decimal, position_id: u128) -> Bucket {
        let mut debt = self._remove_dp(position_id);

        let (_, pool_share_ra) = self
            .lending_pool_registry
            .get_resource_address(debt.resource_address);

        let (borrowed_assets, loan_share) = self
            ._get_pool(self._get_pool_address(pool_share_ra))
            .borrow(amount, debt.interest_type);

        debt.loan_share += loan_share;
        self.debt_positions.insert(position_id, debt);

        self._update();

        borrowed_assets
    }

    pub fn repay(&mut self, mut payment: Bucket, position_id: u128) -> Bucket {
        let mut debt = self._remove_dp(position_id);

        let max_loan_amount = min(
            payment.amount(),
            debt.loan_share / debt.get_loan_share_ratio(),
        );

        let (remainer, loan_share) = self
            ._get_pool(debt.pool_component_address)
            .repay(payment.take(max_loan_amount), debt.interest_type);

        debt.loan_share -= loan_share;

        self.debt_positions.insert(position_id, debt);

        self._update();

        payment.put(remainer);

        payment
    }

    pub fn change_interest_type(
        &mut self,
        position_id: u128,
        interest_type: u8,
        new_interest_type: u8,
    ) {
        let mut debt_position = match self.debt_positions.remove(&position_id) {
            Some(p) => p,
            None => panic!("Debt position not found"),
        };

        debt_position.interest_type = new_interest_type;

        self._get_pool(debt_position.pool_component_address)
            .move_loan_share(interest_type, new_interest_type, debt_position.loan_share);

        self.debt_positions.insert(position_id, debt_position);

        self._update();
    }

    /* INTERNAL FMETHODS */

    fn _new_collateral_position(
        &mut self,
        cdp_id: NonFungibleLocalId,
        collateral_position: CollateralPostion,
    ) -> &ExtendendCollateralPostion {
        let pool_component_address =
            self._get_pool_address(collateral_position.pool_share_resource_address);
        let pool_state = self._get_pool(pool_component_address).get_pool_state();

        let cp = ExtendendCollateralPostion {
            position_id: collateral_position.position_id,
            resource_address: collateral_position.resource_address,
            pool_share_resource_address: collateral_position.pool_share_resource_address,
            pool_share: collateral_position.pool_share,

            //
            cdp_id,
            pool_state,
            collateral_amount: Decimal::ZERO,
            collateral_solvency_value: Decimal::ZERO,
            collateral_healthy_value: Decimal::ZERO,
            pool_component_address: self
                ._get_pool_address(collateral_position.pool_share_resource_address),
        };

        self.collateral_positions
            .insert(collateral_position.position_id, cp);

        match self
            .collateral_positions
            .get(&collateral_position.position_id)
        {
            Some(c) => c,
            None => panic!(""),
        }
    }

    fn _new_debt_position(
        &mut self,
        cdp_id: NonFungibleLocalId,
        debt_position: DebtPostion,
    ) -> &ExtendedDebtPostion {
        let pool_component_address = self._get_pool_address(debt_position.resource_address);
        let mut pool_component = self._get_pool(pool_component_address);

        pool_component.update_interest(debt_position.interest_type);

        let pool_state = pool_component.get_pool_state();

        let dp = ExtendedDebtPostion {
            cdp_id,
            resource_address: debt_position.resource_address,
            loan_share: debt_position.loan_share,
            interest_type: debt_position.interest_type,
            position_id: debt_position.position_id,

            //
            pool_component_address: self._get_pool_address(debt_position.resource_address),
            pool_state,
            loan_amount: Decimal::ZERO,
            loan_value: Decimal::ZERO,
        };

        debug!("position_id: {}", debt_position.position_id);

        self.debt_positions.insert(debt_position.position_id, dp);

        match self.debt_positions.get(&debt_position.position_id) {
            Some(d) => d,
            None => panic!(""),
        }
    }

    fn _update(&mut self) {
        let mut total_debt: Decimal = Decimal::ZERO;

        for (_, mut dp) in &mut self.debt_positions {
            //  let  = item;

            dp.loan_amount = dp.loan_share / dp.get_loan_share_ratio();
            dp.loan_value = dp.loan_amount * dp.pool_state.last_price;

            total_debt += dp.loan_value;
        }

        let mut total_healthy_collateral = Decimal::ZERO;
        let mut total_solvency_collateral = Decimal::ZERO;

        for item in &mut self.collateral_positions {
            let (_, cp) = item;

            cp.collateral_amount = cp.pool_share / cp.get_pool_share_ratio();

            cp.collateral_solvency_value = cp.collateral_amount * cp.pool_state.last_price
                / (dec!(1) + cp.pool_state.liquidation_spread);

            cp.collateral_healthy_value = cp.collateral_amount
                * cp.pool_state.last_price
                * cp.pool_state.liquidation_threshold;

            total_healthy_collateral += cp.collateral_healthy_value;
            total_solvency_collateral += cp.collateral_solvency_value;
        }

        let hf: Decimal;
        let solvency: Decimal;

        if total_debt == Decimal::ZERO {
            hf = Decimal::MAX;
            solvency = Decimal::MAX
        } else {
            hf = total_healthy_collateral / total_debt;
            solvency = total_solvency_collateral / total_debt;
        }

        self.health_factor = hf;
        self.solvency_factor = solvency;
        self.total_loan_value = total_debt;
        self.total_collateral_solvency_value = total_solvency_collateral;
        self.total_collateral_healthy_value = total_healthy_collateral;
    }

    /* INTERNAL FMETHODS */

    fn _remove_cp(&mut self, position_id: u128) -> ExtendendCollateralPostion {
        match self.collateral_positions.clone().remove(&position_id) {
            Some(c) => c,
            None => panic!("Collateral position not found"),
        }
    }

    fn _remove_dp(&mut self, position_id: u128) -> ExtendedDebtPostion {
        match self.debt_positions.clone().remove(&position_id) {
            Some(p) => p,
            None => panic!("Debt position not found"),
        }
    }

    fn _get_pool(&self, component_address: ComponentAddress) -> LendingPoolComponentTarget {
        LendingPoolComponentTarget::at(component_address)
    }

    fn _get_pool_address(&self, resource_address: ResourceAddress) -> ComponentAddress {
        self.lending_pool_registry
            .get_component_address(resource_address)
    }
}
