extern crate walkdir;

use clap::OsValues;
use regex::Regex;
use std::ffi::OsStr;
use std::fmt::{Error, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fmt, io};
use walkdir::WalkDir;

type SearchResultProcessor = dyn Fn(&SearchResult, bool);

/// The holder of state for the search operation, which contains all of the needed parameters.
pub struct Context<'a> {
    debug: bool,
    re: Regex,
    search_result_processor: &'a SearchResultProcessor,
}

impl Context<'_> {
    /// Allow the creation of the holder of search parameters without making them public fields.
    pub fn new(re: Regex, search_result_processor: &SearchResultProcessor, debug: bool) -> Context {
        Context {
            debug,
            re,
            search_result_processor,
        }
    }
}

// can't #[derive] because the SearchResultProcessor function reference doesn't implement Debug
impl fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("debug", &self.debug)
            .field("re", &self.re)
            .field(
                "search_result_processor",
                &format_args!("{:p}", self.search_result_processor),
            )
            .finish()
    }
}

impl fmt::Display for Context<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "( debug: {}, re: {} search_result_processor: {:p} )",
            self.debug, self.re, self.search_result_processor
        )
    }
}

type LineNumber = u64;

/// The structure used to return search results for processing.
#[derive(Clone, Debug)]
pub struct SearchResult<'a> {
    pub path: &'a OsStr,
    pub line_number: LineNumber,
    pub text: &'a str,
    pub new_file: bool,
    _private: (), // to make creation only be possible within this crate, but allow pub fields
}

impl fmt::Display for SearchResult<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "({}, {}, {}, \"{}\")",
            self.path.to_string_lossy(),
            self.line_number,
            self.new_file,
            self.text
        )
    }
}

/// The main function to perform a search.
pub fn find_all_lines<'a>(paths: Option<OsValues<'a>>, context: &Context) -> io::Result<()> {
    walk_all_paths(paths, context)
}

fn do_search(the_path: &PathBuf, _context: &Context) -> io::Result<()> {
    let search_result_processor: &SearchResultProcessor = _context.search_result_processor;
    // I always hate using a "first" sentinel, but sometimes it is needed
    let mut new_file = true;
    let re = &_context.re;
    if _context.debug {
        eprintln!("About to open file {:?}", the_path)
    }
    let f = File::open(the_path)?;
    let f = BufReader::new(f);
    for (_index, line) in f.lines().enumerate() {
        if _context.debug {
            eprintln!("About to look for {:?} in {:?}", re, line);
        }
        match line {
            Ok(l) => {
                if re.is_match(l.as_str()) {
                    let search_result: SearchResult = SearchResult {
                        path: the_path.as_os_str(),
                        line_number: (_index + 1) as LineNumber,
                        text: l.as_str(),
                        new_file,
                        _private: (),
                    };
                    if new_file {
                        new_file = false
                    }
                    search_result_processor(&search_result, _context.debug);
                }
            }
            Err(e) => {
                if _context.debug {
                    eprintln!("Error reading file {:?}: {:?}", the_path, e)
                }
            }
        }
    }
    Ok(())
}

fn walk_a_dir<'a>(the_dir: &'a OsStr, context: &Context) -> io::Result<()> {
    for entry in WalkDir::new(the_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path: PathBuf = entry.into_path();
        let path_ref: &PathBuf = &path;
        do_search(path_ref, context)?;
    }
    Ok(())
}

fn walk_all_paths<'a>(paths: Option<OsValues<'a>>, context: &Context) -> io::Result<()> {
    let dot_as_os_str = OsStr::new(".");

    let the_iterators: OsValues<'a> = match paths {
        Some(path_values) => path_values,
        None => OsValues::default(),
    };

    if the_iterators.clone().count() == 0 {
        walk_a_dir(dot_as_os_str, context)?;
    } else {
        for the_iterator in the_iterators {
            walk_a_dir(the_iterator, context)?;
        }
    }
    Ok(())
}
