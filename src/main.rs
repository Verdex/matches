
use denest::*;

use structuralize::data::*;

use structuralize::pattern::*;
use structuralize::pattern::check::*;

fn main() {

    let mut args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: matches [--row --sub --cut pattern] <pattern>");
        return;
    }

    let pattern = args.pop().unwrap();
    let pattern = pattern.parse::<Pattern>();

    let mut display_as_row = false;
    let mut sub = false;
    let mut ignore = None;

    args.reverse();
    while args.len() > 1 {
        let a = args.pop().unwrap();
        if a == "--row" {
            display_as_row = true;
        }
        else if a == "--sub" {
            sub = true;
        }
        else if a == "--cut" {
            if let Some(p) = args.pop() {
                ignore = Some(p.parse::<Pattern>());
            }
            else {
                eprintln!("Pattern is required after --cut");
                return;
            }
        }
    }

    if pattern.is_err() {
        eprintln!("Encountered Pattern Parsing error: {}", pattern.unwrap_err());
        return;
    }

    let pattern = check_pattern(pattern.unwrap());

    if pattern.is_err() {
        eprintln!("Encountered Invalid Pattern error: {}", pattern.unwrap_err());
        return;
    }

    let ignore = if let Some(p) = ignore {
        if p.is_err() {
            eprintln!("Encountered Invalid Ignore Pattern error: {}", p.unwrap_err());
            return;
        }
        let p = check_pattern(p.unwrap());
        
        if p.is_err() {
            eprintln!("Encountered Invalid Ignore Pattern error: {}", p.unwrap_err());
            return;
        }

        Some(p.unwrap())
    }
    else {
        None
    };


    let pattern = pattern.unwrap();

    let input = std::io::read_to_string(std::io::stdin());

    if input.is_err() {
        eprintln!("Encountered error reading stdin: {}", input.unwrap_err());
        return;
    }

    let data = input.unwrap().parse::<Data>();

    if data.is_err() {
        eprintln!("Encountered Data Parsing error: {}", data.unwrap_err());
        return;
    }

    let data = data.unwrap();

    let results : Vec<MatchMap> = if sub {
        let mut rs = vec![];
        for d in data.to_lax() {
            rs.push(pattern_match(&pattern, &d));
        }
        rs.into_iter().flatten().collect()
    }
    else {
        pattern_match(&pattern, &data)
    };

    if display_as_row {
        display_results_in_row(results);
    }
    else {
        display_results_in_data(results);
    }
}

fn display_results_in_data(results : Vec<MatchMap>) {
    let o = results.iter().map(|x| 
            format!( "result([{}])", x.iter().map(|(s, d)| format!("slot( \"{}\", {} )", s, d)).collect::<Vec<_>>().join(", ") )
        ).collect::<Vec<_>>().join(", ");
    println!("results([{}])", o);
}

fn display_results_in_row(results : Vec<MatchMap>) {
    for result in results {
        for (slot, value) in result {
            print!("{} = {} ;", slot, value);
        }
        println!("");
    }
}
