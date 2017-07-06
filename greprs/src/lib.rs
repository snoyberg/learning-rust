use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FileOpenError(String, io::Error),
    FileReadError(String, io::Error),
    NoQueryString,
    NoFileName,
}

impl fmt::Display for Error {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FileOpenError(ref filename, ref err) =>
                write!(
                    f,
                    "Could not open file '{}' to grep: {}",
                    filename,
                    err,
                ),
            Error::FileReadError(ref filename, ref err) =>
                write!(
                    f,
                    "Could not read from file '{}' to grep: {}",
                    filename,
                    err,
                ),
            Error::NoQueryString =>
                write!(
                    f,
                    "No query string provided on command line"
                ),
            Error::NoFileName =>
                write!(
                    f,
                    "No file name provided on command line"
                ),
        }
    }
}

pub fn run(config: &Config) -> Result<(), Error> {
    let mut f =
        File::open(&config.filename)
        .map_err(|err| Error::FileOpenError(config.filename.clone(), err))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|err| Error::FileReadError(config.filename.clone(), err))?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new<Iter>(mut args: Iter) -> Result<Config, Error>
      where Iter: Iterator<Item=String> {
        args.next();
        let query = args.next().ok_or(Error::NoQueryString)?;
        let filename = args.next().ok_or(Error::NoFileName)?;

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&query)
    }).collect()
}

fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {
        line.contains(query)
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
