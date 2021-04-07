use std::fs;
use std::error::Error;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "safe";
        let content = "\
            Rust:
            safe, fast, productive.
            Pick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "safe";
        let content = "\
            Rust:
            safe, fast, productive.
            Pick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_insensitive(query, content)
        )
    }
}

fn search<'a>(query: & str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            ret.push(line.trim());
        }
    }

    ret
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            ret.push(line.trim());
        }
    }

    ret
}


pub struct Config {
    query: String,
    filename: String,
    sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let sensitive = env::var("INSENSITIVE").is_err();

        Ok(Config { query, filename, sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    
    if config.sensitive {
        for line in search(&config.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in search_insensitive(&config.query, &content) {
            println!("{}", line);
        }
    }

    

    Ok(())
}
