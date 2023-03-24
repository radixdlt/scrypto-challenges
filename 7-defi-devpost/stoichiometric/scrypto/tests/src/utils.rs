use std::process::Command;

pub const ADMIN_BADGE_NAME: &str = "Stoichiometric protocol admin badge";
pub const FLASH_MINT_NAME: &str = "Stoichiometric Flash Mint";
pub const LOAN_NAME: &str = "Stoichiometric Loan";
pub const POSITION_NAME: &str = "Stoichiometric Position";
pub const PROPOSAL_RECEIPT: &str = "Stoichiometric proposal receipt";
pub const STABLECOIN_NAME: &str = "Stoichiometric USD";
pub const STABLECOIN_MINTER: &str = "Stoichiometric stablecoin minter";
pub const VOTER_CARD_NAME: &str = "Stoichiometric voter card";

pub fn run_command(command: &mut Command) -> String {
    let output = command.output().expect("Failed to run command line");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    if !output.status.success() {
        println!("stdout:\n{}", stdout);
        panic!("{}", stderr);
    }
    stdout
}
