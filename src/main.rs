
use structuralize::data::*;

use structuralize::pattern::*;
use structuralize::pattern::check::*;

fn main() {

    let mut args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: matches [-c] <pattern>");
        return;
    }

    let pattern = args.pop().unwrap();
    let pattern = pattern.parse::<Pattern>();

    let mut display_as_column = false;
    while args.len() != 1 {
        let a = args.pop().unwrap();
        if a == "-c" {
            display_as_column = true;
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

    let results = pattern_match(&pattern, &data);

    if display_as_column {
        display_results_in_column(results);
    }
    else {

    }
}

fn display_results_in_column(results : Vec<MatchMap<Slot, &Data>>) {
    for result in results {
        for (slot, value) in result {
            print!("{}:{};", slot, value);
        }
        println!("");
    }
}
