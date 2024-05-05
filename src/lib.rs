extern crate dotenv;

use std::env::var;
use std::error::Error;
use std::fs;

use dotenv::dotenv;
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub sensible_casse: bool
}

impl Config {
    pub fn build(mut args : env::Args) -> Result<Config, &'static str> {
        dotenv().ok();
        args.next();


        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de chaîne de caractère"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Nous n'avons pas de nom de fichier"),
        };

        let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

        Ok(Config { 
            query, 
            file_path, 
            sensible_casse 
        })
    }
}

pub fn run(config: Config)->Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let resultats = if config.sensible_casse {
        search(&config.query, &contents)
    }else{
        rechercher_insensible_casse(&config.query, &contents)
    };
    
    for line in resultats{
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensible_casse() {
        let query ="duct";
        let contents = "\
        Rust:
        sécurité, rapidité, productivité.
        Obtenez les trois en même temps.
        Duck tape.";

        assert_eq!(vec!["sécurité, rapidité, productivité."], search(query, contents));
    }

    #[test]
    fn insensible_casse(){
        let query ="rUsT";
        let contents = "\
        Rust:
        sécurité, rapidité, productivité.
        Obtenez les trois en même temps.
        C'est pas rustique.";

        assert_eq!(vec!["Rust:", "C'est pas rustique."], 
        rechercher_insensible_casse(query, contents));
    }
}

pub fn search<'a>(query: &str, contents:&'a str)->Vec<&'a str>{

    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn rechercher_insensible_casse<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line.trim());
        }
    }

    result
    
}