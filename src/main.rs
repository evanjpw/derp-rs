#[macro_use]
extern crate clap;
extern crate derp;

use derp::Context;
use derp::{find_all_lines, SearchResult}; // To be fixed
use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};

fn result_processor(search_result: &SearchResult, debug: bool) {
    if debug {
        eprintln!("Naïve SearchResult is {}", search_result);
    }
    if search_result.new_file {
        println!("{}:", search_result.path.to_string_lossy());
    }
    println!("{}: {}", search_result.line_number, search_result.text)
}

fn main() -> io::Result<()> {
    let matches = clap_app!(myapp =>
       (version: "1.0")
       (author: "Evan Williams <evan.williams@pathai.com>")
       (version: "0.1.0")
       (about: "Redundant, boring, recursive regular expression search")
       (@arg debug: -d "Sets the level of debugging information")
       (@arg PATTERN: +required "The pattern (regular expression) to search for")
       (@arg PATH: ... "The path to search for the pattern")
    )
    .get_matches();

    let debug = matches.is_present("debug");
    let pattern = matches.value_of("PATTERN").unwrap();
    let paths = matches.values_of_os("PATH");
    if debug {
        eprintln!("Debug is {}, Pattern is \"{}\"", debug, pattern);
    }
    if let Ok(re) = Regex::new(pattern) {
        if debug {
            eprintln!("Re is {:?}", re)
        }
        let context = Context { debug, re };
        find_all_lines(paths, &result_processor, &context)?;
    } else {
        eprint!("Invalid pattern \"{}\"", pattern);
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid pattern {}", pattern),
        ));
    }
    Ok(())
}
