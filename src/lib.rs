pub mod parse;

use parse::Args;

pub fn run(args: Args) {
    println!("Args: {:#?}", args);
    for file in args.files {
        println!("Scanning {}", file);
    }
}
