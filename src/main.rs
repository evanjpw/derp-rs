//! # derp-rs
//!
//! A crappy knockoff of "grep" that exists only as a way to teach myself Rust.
//!
//! I have been paying attention to Rust for years, and have seen it
//! evolve over time, but never actually tried to write any Rust code.
//!
//! For personal reasons, it has become of more interest recently, so
//! this seemed an excellent first program, since it had many dimensions
//! for a little program.
//!
//! (Amusingly, [one of the Rust books/online resources][1]
//! uses building `grep` as an exercise. When I saw that, I avoided
//! reading that book further, so as to not contaminate my experience.
//! But it is entertaining that they decided on the same little program
//! to teach Rust. I must be onto something.)
//!
//! ## Usage
//! ```
//! USAGE:
//! derp [FLAGS] <PATTERN> [PATH]...
//!
//! FLAGS:
//! -d               Sets the level of debugging information
//! -h, --help       Prints help information
//! -V, --version    Prints version information
//!
//! ARGS:
//! <PATTERN>    The pattern (regular expression) to search for
//! <PATH>...    The path to search for the pattern
//! ```
//!
//! [1]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html

#[macro_use]
extern crate clap;
extern crate derp;

use derp::{find_all_lines, Context, SearchResult};
use regex::Regex;
use std::io::{Error, ErrorKind, Result};

/// The function called by search to process search results.
fn search_result_processor(search_result: &SearchResult, debug: bool) {
    if debug {
        eprintln!("Naïve SearchResult is {}", search_result);
    }
    if search_result.new_file {
        println!("{}:", search_result.path.to_string_lossy());
    }
    println!("{}: {}", search_result.line_number, search_result.text)
}

/// The main function that parses arguments and runs the search
fn main() -> Result<()> {
    let matches = clap_app!(myapp =>
       (author: "Evan Williams <evan.williams@pathai.com>")
        // Is version: "1.0" (or any version) a useful concept
        // for something no one is ever supposed to use?
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
        let context = Context::new(re, &search_result_processor, debug);
        find_all_lines(paths, &context)?;
    } else {
        // It would be nice if I could use the same string twice. There is probably a superior way.
        eprint!("Invalid pattern \"{}\"", pattern);
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid pattern {}", pattern),
        ));
    }
    Ok(())
}
