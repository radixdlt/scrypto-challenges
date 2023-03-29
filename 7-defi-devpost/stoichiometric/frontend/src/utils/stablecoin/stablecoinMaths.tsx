

function amountToLiquidate(collateral_amount: number, collateral_price: number, amount_lent: number, liquidation_threshold: number, liquidation_penalty: number, daily_interest_rate: number, loan_date: number): number {

    const collateral_value = collateral_amount * collateral_price;
    const current_time = new Date();
    const time_passed = (current_time.getSeconds() - loan_date) / (86400);
    const loan_value = amount_lent + daily_interest_rate * time_passed;

    if (collateral_value / loan_value > liquidation_threshold) { return -1; }

    // We add one to make sure to be able to liquidate even if there are computational errors
    return 1 + (loan_value * liquidation_threshold - collateral_amount * (1 - liquidation_penalty) * collateral_price) / (liquidation_threshold - 1);

}


export { amountToLiquidate }