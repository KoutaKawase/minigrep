use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_sensitive: bool,
    pub exists_ignore_option: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        //セットされてるかどうかだけ分かればいい。セットされてたら区別なしで検索してほしいのでis_errがfalseを返し都合が良い
        let is_sensitive = if args[3..].contains(&String::from("--insensitive")) {
            let is_sensitive = false;
            is_sensitive
        } else {
            env::var("IS_INSENSITIVE").is_err()
        };
        let exists_ignore_option = args[3..].contains(&String::from("--ignore"));

        Ok(Config {
            query,
            filename,
            is_sensitive,
            exists_ignore_option,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.exists_ignore_option {
        search_without_query(&config.query, &contents)
    } else if config.exists_ignore_option && !config.is_sensitive {
        search_without_query_insensitive(&config.query, &contents)
    } else if config.is_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_without_query<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_without_query_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    //クエリーを含まない行を全て検索するテスト
    #[test]
    fn test_search_without_query_sensitive() {
        let query = "hello";

        let contents = "\
hello
world from rust.
rust is the best language.
hello from rust.
Hello World";

        assert_eq!(
            vec![
                "world from rust.",
                "rust is the best language.",
                "Hello World"
            ],
            search_without_query(query, contents)
        );
    }

    #[test]
    fn test_search_without_query_insensitive() {
        let query = "rust";

        let contents = "\
rust is great.
C++ is good.
C is my father.
Rust is future.";

        assert_eq!(
            vec!["C++ is good.", "C is my father."],
            search_without_query_insensitive(query, contents)
        );
    }
}
