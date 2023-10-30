
mod data;
mod parsing;
mod acquisition;

use denest::*;

use structuralize::data::*;
use structuralize::pattern::data::*;
use structuralize::pattern::check::*;
use structuralize::pattern::matcher::*;

fn main() {

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: matches <pattern>+");
        return;
    }

    // TODO: add in index such that the error handling for pattern parser can indicate which 
    // pattern had the problem.
    let patterns : Result<Vec<_>, _> = args[1..].iter().map(|arg| to_pattern(arg)).collect();

    if patterns.is_err() {
        eprintln!("encountered problem parsing pattern: {}", patterns.unwrap_err());
        return;
    }

    let data = acquisition::get_data_from_dir().unwrap(); // TODO error handling?

    /*for sub in data.to_lax() {
        let results = pattern_match(&tcp, sub);

        for result in results {
            for (x, y) in result {
                pattern_match(&tcp, y);
                println!("{} {}", x, y);
            }
            println!("");
        }
    }*/
}

fn to_pattern(arg : &str) -> Result<TypeChecked<SymStr>, Box<dyn std::error::Error>> {
    let p : Pattern<SymStr> = arg.parse()?;
    let tcp = check_pattern(p)?; 
    Ok(tcp)
}