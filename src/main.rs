
mod data;
mod parsing;
mod acquisition;

use denest::*;

use structuralize::data::*;
use structuralize::pattern::data::*;
use structuralize::pattern::check::*;
use structuralize::pattern::matcher::*;

fn main() {

    let mut args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("usage: matches <pattern>+");
        return;
    }

    args.reverse();
    args.pop();

    let p = args.pop().unwrap();

    let pattern : Pattern<SymStr> = p.parse().unwrap(); // TODO error handling?

    let tcp = check_pattern(pattern).unwrap(); // TODO error handling?

    let data = acquisition::get_data_from_dir().unwrap(); // TODO error handling?

    for sub in data.to_lax() {
        let results = pattern_match(&tcp, sub);

        for result in results {
            for (x, y) in result {
                println!("{} {}", x, y);
            }
            println!("");
        }
    }
}
