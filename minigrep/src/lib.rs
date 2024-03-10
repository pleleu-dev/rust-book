use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(query: &String, file_path: &String, ignore_case: bool) -> Config {
        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        }
    }
}

pub fn parse_arguments(args: Vec<String>, ignore_case: bool) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("Should have 2 arguments, a query and a file path");
    }
    Ok(Config::new(&args[1], &args[2], ignore_case))
}

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

pub fn search<'a, 'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}
pub fn search_case_insensitive<'a, 'b>(query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&str> = Vec::new();

    let lowercase_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitve() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Ducty, pick three.";

        assert_eq!(
            vec!["safe, fast, productive.", "Ducty, pick three."],
            search_case_insensitive(query, contents)
        );
    }
}
