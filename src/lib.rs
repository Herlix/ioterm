use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
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
    contents.lines().filter(|x| x.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_insensitive() {
        let expected = vec!["Rust:", "Trust me."];
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(expected.clone(), search_insensitive("rUsT", content));
        assert_eq!(expected.clone(), search_insensitive("Rust", content));
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
