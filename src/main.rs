
mod parsing;

use denest::*;

use structuralize::data::*;

use structuralize::pattern::Pattern;
use structuralize::pattern::check::*;
use structuralize::pattern::lazy_matcher::*;


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
    args.pop(); // Get rid of exe name
    while args.len() > 0 {
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

    if sub {
        if let Some(ignore) = ignore {
            let mut results = vec![];
            for d in data.to_lax_cut(|d| pattern_match(&ignore, d).next().is_none()) {
                results.push(pattern_match(&pattern, &d));
            }
            display(results.into_iter().flatten(), display_as_row);
        }
        else {
            let mut results = vec![];
            for d in data.to_lax() {
                results.push(pattern_match(&pattern, &d));
            }
            display(results.into_iter().flatten(), display_as_row);
        };
    }
    else {
        let results = pattern_match(&pattern, &data);
        display(results, display_as_row);
    };
}

fn display<'a>(results : impl Iterator<Item = MatchMap<'a>>, display_as_row : bool ) {
    if display_as_row {
        display_results_in_row(results);
    }
    else {
        display_results_in_data(results);
    }
}

fn display_results_in_data<'a>(results : impl Iterator<Item = MatchMap<'a>>) {
    let o = results.map(|x| 
            format!( "result([{}])", x.iter().map(|(s, d)| format!("slot( \"{}\", {} )", s, d)).collect::<Vec<_>>().join(", ") )
        ).collect::<Vec<_>>().join(", ");
    println!("results([{}])", o);
}

fn display_results_in_row<'a>(results : impl Iterator<Item = MatchMap<'a>>) {
    for result in results {
        for (slot, value) in result {
            print!("{} = {} ;", slot, value);
        }
        println!("");
    }
}
