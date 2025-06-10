//! # MiniGrep 🕵️‍♂️📄
//!
//! `minigrep` is a simple command-line tool written in Rust to search for lines
//! that contain a specific query string in a given text file, similar to Unix `grep`.
//!
//! It supports both case-sensitive and case-insensitive search based on the
//! `IGNORE_CASE` environment variable.
//!
//! ## Example
//!
//! ```bash
//! cargo run -- <query> <file_path>
//! ```
//!
//! Case-insensitive search:
//!
//! ```bash
//! IGNORE_CASE=1 cargo run -- <query> <file_path>
//! ```
//!
//! ## Crate Structure
//!
//! - [`Config`] struct handles argument parsing and configuration.
//! - [`run`] function executes the main logic.
//! - [`search`] and [`search_case_insensitive`] perform line matching.

use std::env;
use std::error::Error;
use std::fs;

/// Holds the configuration parameters for the MiniGrep application.
///
/// This struct is created using the [`Config::build`] method,
/// which parses command-line arguments and reads the `IGNORE_CASE`
/// environment variable to determine if the search should be case-insensitive.
pub struct Config {
    /// The string to search for in the file.
    pub query: String,

    /// The path to the input file to search.
    pub file_path: String,

    /// A flag that determines whether the search should be case-insensitive.
    /// This is set based on the `IGNORE_CASE` environment variable.
    pub ignore_case: bool,
}

impl Config {
    /// Parses command-line arguments and builds a `Config`.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator over command-line arguments, typically from `env::args()`.
    ///
    /// # Returns
    ///
    /// * `Ok(Config)` if both query and file path are provided.
    /// * `Err(&str)` with an error message if arguments are missing.
    ///
    /// # Examples
    ///
    /// ```
    /// use minigrep::Config;
    ///
    /// let args = vec![
    ///     String::from("minigrep"), // normally the binary name
    ///     String::from("Rust"),
    ///     String::from("input.txt"),
    /// ];
    ///
    /// let config = Config::build(args.into_iter()).unwrap();
    ///
    /// assert_eq!(config.query, "Rust");
    /// assert_eq!(config.file_path, "input.txt");
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Executes the main logic of MiniGrep: reads the file, searches for the query,
/// and prints matching lines to stdout.
///
/// # Arguments
///
/// * `config` - A `Config` object containing the search parameters.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if reading the file fails.
///
/// # Examples
///
/// ```no_run
/// use minigrep::{Config, run};
///
/// let config = Config {
///     query: String::from("Rust"),
///     file_path: String::from("input.txt"),
///     ignore_case: false,
/// };
///
/// if let Err(e) = run(config) {
///     eprintln!("Application error: {e}");
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Performs a **case-sensitive** search of `query` in `contents`.
///
/// # Arguments
///
/// * `query` - The search string.
/// * `contents` - The contents of the file as a string slice.
///
/// # Returns
///
/// A vector of lines that contain the query.
///
/// # Examples
///
/// ```
/// use minigrep::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Performs a **case-insensitive** search of `query` in `contents`.
///
/// # Arguments
///
/// * `query` - The search string (case-insensitive).
/// * `contents` - The contents of the file as a string slice.
///
/// # Returns
///
/// A vector of lines that contain the query, ignoring case.
///
/// # Examples
///
/// ```
/// use minigrep::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
