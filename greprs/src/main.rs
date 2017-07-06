extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;
use greprs::Config;

fn main() {
    main_inner().unwrap_or_else(|err| {
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "{}", err).expect("Could not write to stderr");
        process::exit(1);
    });
}

fn main_inner() -> Result<(), greprs::Error> {
    let config = Config::new(env::args())?;

    greprs::run(&config)?;

    Ok(())
}
