use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    for value in search(&config.query, &contents) {
        println!("{}", value);
    }

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
Safe, fast, productive.
Pick three.
        ";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust: 
Safe, fast, productive.
Pick three.
        ";
    }
}
