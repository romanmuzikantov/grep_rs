use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = if args.len() == 4 && args[3].eq(&String::from("-i")) {
            false
        } else if env::var("CASE_INSENSITIVE").is_ok() {
            false
        } else {
            true
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lowercase = query.to_ascii_lowercase();
    for line in content.lines() {
        if line.to_ascii_lowercase().contains(&query_lowercase) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_case_sensitive() {
        let query = "his";
        let content = "\
His palms are sweaty, knees weak, arms are heavy
There's vomit on his sweater already, mom's spaghetti
He's nervous, but on the surface he looks calm and ready";

        assert_eq!(vec!["There's vomit on his sweater already, mom's spaghetti"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "his";
        let content = "\
His palms are sweaty, knees weak, arms are heavy
There's vomit on his sweater already, mom's spaghetti
He's nervous, but on the surface he looks calm and ready";

        assert_eq!(vec!["His palms are sweaty, knees weak, arms are heavy", "There's vomit on his sweater already, mom's spaghetti"],
                        search_case_insensitive(query, content));
    }
}