use std::env;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::process;
use minigrep::{Config, run};

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // dbg!(config);
    if let Err(e) = run(config){
        eprintln!("Application Error {e}");
        process::exit(1);
    }

}



