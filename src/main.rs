use std::env;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::process;
use minigrep::{Config, run};

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // dbg!(config);
    if let Err(e) = run(config){
        println!("Application Error {e}");
        process::exit(1);
    }

}



