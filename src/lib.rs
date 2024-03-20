mod magic;
mod parse;
mod scan;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    scan::print_summary(scan::scan(parse::get_args())?);
    Ok(())
}
