mod magic;
mod parse;
mod scan;

use parse::Args;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    scan::print_summary(scan::scan(Args::parse_args())?);
    Ok(())
}
