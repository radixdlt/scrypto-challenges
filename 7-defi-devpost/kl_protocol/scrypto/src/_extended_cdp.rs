use crate::{_comon::*, _lending_pool::*, _lending_pool_manager::*};
use scrypto::prelude::*;
use std::cmp::min;

// Reference to a lending pool. will be use for method calls from the Extended CDP
external_component! {
    LendingPoolComponentTarget {

        fn update_interest(&mut self, interest_type: u8);

        fn move_loan_share(&mut self, from_interest_type: u8, interest_type_dest: u8, amount: Decimal);

        fn borrow(&mut self, amount: Decimal, interest_type:u8) -> (Bucket, Decimal);

        fn repay(&mut self,  payment: Bucket, interest_type:u8) -> (Bucket, Decimal);

        fn add_collateral(&mut self, assets: Bucket) -> Decimal;

        fn remove_collateral(&mut self, amount: Decimal, get_pool_share: bool) -> Bucket;

        //

        fn get_pool_state(&mut self) -> LendingPoolState;

        fn get_pool_share_ratio(&self) -> Decimal;

        fn get_loan_share_ratio(&self, interest_type: u8) -> Decimal;

    }
}

// Extends the CollateraPosition with necessery information for the CDP health check and call method of the related lending pool
#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendendCollateralPostion {
    pub position_id: u128,
    pub pool_resource_address: ResourceAddress,
    pub pool_share_resource_address: ResourceAddress,
    pub pool_component_address: ComponentAddress,
    pub pool_share: Decimal,

    // As multiple CDP cant mi combinedd in one extended CDP, we need to track the cdp each position billong
    pub cdp_id: NonFungibleLocalId,

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

// Extends the DebtPosition with necessery information for the CDP health check and call method of the related lending pool
#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendedDebtPostion {
    pub position_id: u128,
    pub pool_resource_address: ResourceAddress,
    pub pool_component_address: ComponentAddress,
    pub loan_share: Decimal,
    pub interest_type: u8,

    // As multiple CDP cant mi combinedd in one extended CDP, we need to track the cdp each position billong
    pub cdp_id: NonFungibleLocalId,

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

// Extends the CDP with necessery information for the CDP health check and call method of the related lending pool
// In addition the Extended CDP can combine mulitiple CDP and perform health check on the batch. this is usefull for delegated CDP
#[derive(LegacyDescribe, ScryptoEncode, ScryptoDecode, ScryptoCategorize, Debug, Clone)]
pub struct ExtendedCollaterizedDebtPosition {
    /// As multiple CDP cant mi combinedd in one extended CDP, we need to track the cdp each position billong to and save all "non-combinable" infos in
    /// a the header field
    pub header: HashMap<NonFungibleLocalId, (Option<NonFungibleLocalId>, Vec<NonFungibleLocalId>)>,
    pub cdp_id: NonFungibleLocalId,

    pub health_factor: Decimal,
    pub solvency_factor: Decimal,

    pub total_loan_value: Decimal,
    pub total_collateral_solvency_value: Decimal,
    pub total_collateral_healthy_value: Decimal,

    pub collateral_positions: HashMap<u128, ExtendendCollateralPostion>,
    pub debt_positions: HashMap<u128, ExtendedDebtPostion>,

    //
    cdp_resource_address: ResourceAddress,
}

impl ExtendedCollaterizedDebtPosition {
    /// This function create an ExtendedCDP based on the provided CDP localId. the function startby analysing the cdp data. if we have a cdp with several
    /// delegated CDP, We load alson all the delegated CDP. on the other hand if the provided CDP is a delegated CDP, We load the delegator. the presented CDP will
    /// certainly be load during the delagator loading.
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
                header: HashMap::new(),

                cdp_resource_address,

                cdp_id: cdp_id.clone(),
                health_factor: Decimal::MAX,
                solvency_factor: Decimal::MAX,
                total_loan_value: Decimal::ZERO,
                total_collateral_healthy_value: Decimal::ZERO,
                total_collateral_solvency_value: Decimal::ZERO,

                collateral_positions: HashMap::new(),
                debt_positions: HashMap::new(),
            };
            extended_cdp.extend(cdp_id, Some(cdp_data.clone()));

            for delegated_cdp_id in cdp_data.clone().delegated_ids.iter() {
                extended_cdp.extend(delegated_cdp_id.clone(), None);
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

            // Restore parameters of presented CDP_ID
            extended_cdp.cdp_id = cdp_id;
        }

        extended_cdp
    }

    /// This is where each position is extend and load in the ExtendedCDP. not combined details are stor in the header
    fn extend(
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

        self.header.insert(
            cdp_id.clone(),
            (_cdp_data.delegator_id, _cdp_data.delegated_ids),
        );

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

        if assert_health_factor {
            assert!(debt_state, "Health factor need to be higher than 1");
        }

        debt_state
    }

    pub fn can_be_liquidated(&self) {
        assert!(
            self.health_factor <= Decimal::ONE && self.solvency_factor >= Decimal::ONE,
            "Can not liquidate this CDP: factor need to be lower than 1"
        );
    }

    pub fn can_be_auto_liquidated(&self) {
        let debt_state = self.health_factor <= Decimal::ONE || self.solvency_factor <= Decimal::ONE;

        assert!(
            debt_state,
            "Can no liquidate this CDP: Health factor need to be lower than 1"
        );
    }

    /// Return ALL CDP loaded during the CDP extention with updated data. in most cases, it will contain one CDP but for delegated CDP, all delegated CDP
    /// Will be returned allong the delagator.

    pub fn get_cdps(&mut self) -> HashMap<NonFungibleLocalId, CollaterizedDebtPositionData> {
        let mut cdps: HashMap<NonFungibleLocalId, CollaterizedDebtPositionData> = HashMap::new();

        for (_, dp) in &self.debt_positions {
            let cdp_id = dp.cdp_id.clone();

            let mut _current_cdp = match cdps.remove(&cdp_id) {
                Some(_c) => _c,
                None => {
                    let (delegator_id, delegated_ids) = match self.header.remove(&cdp_id) {
                        Some(_d) => _d,
                        None => panic!("CDP Header not found"),
                    };

                    CollaterizedDebtPositionData {
                        delegator_id,
                        delegated_ids,
                        collaterals: HashMap::new(),
                        debts: HashMap::new(),
                    }
                }
            };

            _current_cdp.debts.insert(
                dp.position_id,
                DebtPostion {
                    position_id: dp.position_id,
                    pool_resource_address: dp.pool_resource_address,
                    pool_component_address: dp.pool_component_address,
                    loan_share: dp.loan_share,
                    interest_type: dp.interest_type,
                },
            );

            cdps.insert(cdp_id, _current_cdp);
        }

        for (res, cp) in &self.collateral_positions {
            let cdp_id = cp.cdp_id.clone();

            let mut _current_cdp = match cdps.remove(&cdp_id) {
                Some(_c) => _c,
                None => {
                    let (delegator_id, delegated_ids) = match self.header.remove(&cdp_id) {
                        Some(_d) => _d,
                        None => panic!("CDP Header not found"),
                    };

                    CollaterizedDebtPositionData {
                        delegator_id,
                        delegated_ids,
                        collaterals: HashMap::new(),
                        debts: HashMap::new(),
                    }
                }
            };

            _current_cdp.collaterals.insert(
                *res,
                CollateralPostion {
                    position_id: cp.position_id,
                    pool_share: cp.pool_share,
                    pool_resource_address: cp.pool_resource_address,
                    pool_component_address: cp.pool_component_address,
                    pool_share_resource_address: cp.pool_share_resource_address,
                },
            );

            cdps.insert(cdp_id, _current_cdp);
        }

        cdps
    }

    /*

        PROXY METHODS TO UNDERLYING LENDING POOL
    The role of the extended CDP is to performe all user related verification an also all update on the CDP data. once done, it call the methods from the liquidity pool
    for asset related action and takes input from the result.

         */

    pub fn new_collateral(
        &mut self,
        assets: Bucket,
        mut new_collateral_position: CollateralPostion,
    ) -> Decimal {
        let pool_share_amount = self
            ._get_pool(new_collateral_position.pool_component_address)
            .add_collateral(assets);

        new_collateral_position.pool_share = pool_share_amount;

        self._new_collateral_position(self.cdp_id.clone(), new_collateral_position);

        self._update();

        pool_share_amount
    }

    pub fn add_collateral(&mut self, assets: Bucket, position_id: u128) -> Decimal {
        let mut cp = self._remove_cp(position_id);

        let pool_share_amount = self
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
        pool_share_amount: Decimal,
        get_pool_share: bool,
    ) -> Bucket {
        let mut cp = self._remove_cp(position_id);

        let max_pool_share_amount = min(pool_share_amount, cp.pool_share);

        let pool_shares = self
            ._get_pool(cp.pool_component_address)
            .remove_collateral(max_pool_share_amount, get_pool_share);

        cp.pool_share -= max_pool_share_amount;

        self.collateral_positions.insert(position_id, cp);

        self._update();

        pool_shares
    }

    pub fn borrow(
        &mut self,
        interest_type: u8,
        amount: Decimal,
        mut new_debt_position: DebtPostion,
    ) -> Bucket {
        let (borrowed_assets, loan_share) = self
            ._get_pool(new_debt_position.pool_component_address)
            .borrow(amount, interest_type);

        new_debt_position.loan_share = loan_share;

        self._new_debt_position(self.cdp_id.clone(), new_debt_position);

        self._update();

        borrowed_assets
    }

    pub fn borrow_more(&mut self, amount: Decimal, position_id: u128) -> Bucket {
        let mut debt = self._remove_dp(position_id);

        let (borrowed_assets, loan_share) = self
            ._get_pool(debt.pool_component_address)
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
        let pool_state = self
            ._get_pool(collateral_position.pool_component_address)
            .get_pool_state();

        let cp = ExtendendCollateralPostion {
            position_id: collateral_position.position_id,
            pool_resource_address: collateral_position.pool_resource_address,
            pool_share_resource_address: collateral_position.pool_share_resource_address,
            pool_share: collateral_position.pool_share,

            //
            cdp_id,
            pool_state,
            collateral_amount: Decimal::ZERO,
            collateral_solvency_value: Decimal::ZERO,
            collateral_healthy_value: Decimal::ZERO,
            pool_component_address: collateral_position.pool_component_address,
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
        let mut pool_component = self._get_pool(debt_position.pool_component_address);

        pool_component.update_interest(debt_position.interest_type);

        let pool_state = pool_component.get_pool_state();

        let dp = ExtendedDebtPostion {
            cdp_id,
            pool_resource_address: debt_position.pool_resource_address,
            loan_share: debt_position.loan_share,
            interest_type: debt_position.interest_type,
            position_id: debt_position.position_id,

            //
            pool_component_address: debt_position.pool_component_address,
            pool_state,
            loan_amount: Decimal::ZERO,
            loan_value: Decimal::ZERO,
        };

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
}
