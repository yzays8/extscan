mod magic;
mod cli;
mod scan;

use cli::Args;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse_args();
    let scan_summary = scan::scan(&args)?;

    if !args.no_summary {
        scan::print_summary(&scan_summary);
    }

    loop {
        if args.yes {
            scan::fix_exts(&scan_summary)?;
            break;
        }
        if args.no {
            break;
        }

        let mut input = String::new();
        println!("Would you like to fix these mismatched file extensions? [y/n]");
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                scan::fix_exts(&scan_summary)?;
                break;
            }
            "n" | "no" => break,
            _ => continue,
        }
    }
    Ok(())
}
