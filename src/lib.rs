use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    filename: String,
    case_sensetive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensetive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensetive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_insensitive() {
        let exptexted = vec!["Rust:", "Trust me."];
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(exptexted.clone(), search_insensitive("rUsT", content));
        assert_eq!(exptexted.clone(), search_insensitive("Rust", content));
    }

    #[test]
    fn case_sensitive_() {
        let content = "\
rUsT:
safe, fast, productive.
Pick three.
Rust is best
Trust me. rUsT";

        assert_eq!(vec!["rUsT:", "Trust me. rUsT"], search("rUsT", content));
        assert_eq!(vec!["Rust is best"], search("Rust", content));
    }
}
