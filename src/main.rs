
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

    let patterns = patterns.unwrap();

    let data = acquisition::get_data_from_dir(); 

    if data.is_err() {
        eprintln!("encountered problem getting data: {}", data.unwrap_err());
        return;
    }

    let data = data.unwrap();

    match_patterns(&patterns[..], &data, 0);
}

fn match_patterns(ps : &[TypeChecked<SymStr>], data : &Data, indent : usize) {
    if ps.len() == 0 {
        return;
    }
    for d in data.to_lax() {
        let results = pattern_match(&ps[0], d);
        for result in results {
            for (var, data) in result {
                println!("{}{}={}", "  ".repeat(indent), var, data);
                match_patterns(&ps[1..], data, indent + 1);
            }
            println!("======================");
        }
    }
}

fn to_pattern(arg : &str) -> Result<TypeChecked<SymStr>, Box<dyn std::error::Error>> {
    let p : Pattern<SymStr> = arg.parse()?;
    let tcp = check_pattern(p)?; 
    Ok(tcp)
}