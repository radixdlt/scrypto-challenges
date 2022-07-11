use sbor::*;
use scrypto::prelude::*;

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct AssetState{
    // the liquidity index
    pub supply_index: Decimal,
    // the borrow index
    pub borrow_index: Decimal,
    // the current borrow annual interest rate
    pub borrow_interest_rate: Decimal,
    // the current supply annual interest rate
    pub supply_interest_rate: Decimal,
    // the insurance funding for current asset.
    pub insurance_balance: Decimal,
    // the ratio for current asset insurance funding
    pub insurance_ratio: Decimal,
    // recipet token of asset
    pub token: ResourceAddress,
    // normalized total borrow.
    pub normalized_total_borrow: Decimal,
    // last update timestamp
    pub last_update_epoch: u64
}

#[warn(dead_code)]
impl AssetState {

    /**
    * About 15017 epochs in a year, assuming 35 minute epochs 
    * https://learn.radixdlt.com/article/how-long-is-an-epoch-on-radix
    * There is no exact timestamp at the moment, so for the time being the period of each epoch (35 minutes) is used for calculation.
    **/
    const EPOCH_OF_YEAR: u64 = 15017;

    pub fn update_index(&mut self) {
        if self.last_update_epoch < Runtime::current_epoch() {
            let (current_supply_index, current_borrow_index) = self.get_current_index();
            
            // get the total equity value
            let normalized_borrow: Decimal = self.normalized_total_borrow;
            let token_res_mgr: &ResourceManager = borrow_resource_manager!(self.token);
            let normalized_supply: Decimal = token_res_mgr.total_supply();

            // interest = equity value * (current index value - starting index value)
            let recent_borrow_interest = normalized_borrow * (current_borrow_index - self.borrow_index);
            let recent_supply_interest = normalized_supply * (current_supply_index - self.supply_index);

            // the interest rate spread goes into the insurance pool
            self.insurance_balance += recent_borrow_interest - recent_supply_interest;

            // LOG.info(asset, borrow_index, current_borrow_index, supply_index, current_supply_index);
            self.supply_index = current_supply_index;
            self.borrow_index = current_borrow_index;
            self.last_update_epoch = Runtime::current_epoch();

        }
    }


    pub fn get_current_index(&self) -> (Decimal, Decimal){
        let delta_epoch = Runtime::current_epoch() - self.last_update_epoch;
        let delta_borrow_interest_rate = self.borrow_interest_rate * delta_epoch / AssetState::EPOCH_OF_YEAR;
        let delta_supply_interest_rate = self.supply_interest_rate * delta_epoch / AssetState::EPOCH_OF_YEAR;

        (
            self.supply_index * (Decimal::ONE + delta_borrow_interest_rate),
            self.borrow_index * (Decimal::ONE + delta_supply_interest_rate)
        )

    }

    pub fn get_interest_rates(&self, extra_borrow_amount: Decimal) -> (Decimal, Decimal){
        let (current_supply_index, current_borrow_index) = self.get_current_index();

        let supply = self.get_total_supply_with_index(current_supply_index);
        if supply == Decimal::ZERO {
            return (Decimal::ZERO, Decimal::ZERO);
        }

        let borrow = self.get_total_borrow_with_index(current_borrow_index) + extra_borrow_amount;
        let borrow_ratio = borrow / supply;

        let borrow_interest_rate = self.get_borrow_interest_rate(borrow_ratio, ComponentAddress::from_str("").unwrap());
        
        let borrow_interest = borrow * borrow_interest_rate;
        let supply_interest = borrow_interest * (Decimal::ONE - self.insurance_ratio);

        let supply_interest_rate = supply_interest / supply;
        (borrow_interest_rate, supply_interest_rate)
    }

    fn get_borrow_interest_rate(&self, borrow_ratio: Decimal, component_addr: ComponentAddress) -> Decimal{
        return Decimal::from("0.05");
    }

    fn get_total_supply_with_index(&self, current_supply_index: Decimal) -> Decimal{
        self.get_total_normalized_supply() * current_supply_index
    }

    fn get_total_normalized_supply(&self) -> Decimal{
        let token_res_mgr: &ResourceManager = borrow_resource_manager!(self.token);
        token_res_mgr.total_supply()
    }

    fn get_total_borrow_with_index(&self, current_borrow_index: Decimal) -> Decimal{
        self.normalized_total_borrow * current_borrow_index
    }
    
}