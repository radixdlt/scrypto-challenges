#[cfg(windows)]
use colored::*;
use hare::harelib::cli;

pub fn main() -> Result<(), cli::Error> {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();
    cli::run()
}
