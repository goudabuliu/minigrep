use std::{error, fs};

pub fn run(cfg: &Cfg) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(&cfg.file)?;

    let results = if cfg.ignore_case {
        search_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub struct Cfg {
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}
impl Cfg {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Cfg, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // };
        args.next(); //跳过程序名
        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get a query"),
        };
        let file = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a file"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Cfg {
            query,
            file,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn it_works2() {
        let query = "rust";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["Rust:"], search_insensitive(query, contents));
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
