mod magic;
mod parse;
mod scan;

use parse::Args;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse_args();
    let scan_summary = scan::scan(&args)?;
    if !args.no_summary {
        scan::print_summary(scan_summary);
    }
    Ok(())
}
