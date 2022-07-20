mod test_runner;
mod utils;

use test_runner::TokenSaleTestRunner;

pub fn main() {
    let mut token_sale_test_runner: TokenSaleTestRunner = TokenSaleTestRunner::new("").unwrap();
}
