
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
   pub fn new(args: &[String])-> Result<Config, &'static  str>{
        //  &'static str is the type of string literals
        if args.len() < 2 {
            return Err("not enough argument");
        }

        let query = args[0].clone() ;
        let filename = args[1].clone() ;

       let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(
            Config{
                query,
                filename,
                case_sensitive,
            }
        )
    }
}

pub fn run0(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    for line in search(&config.query , &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
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
    // 'a => connect arguments to return values in the signature
    let mut results =   Vec::new() ;
    for line in contents.lines() {
        // do something with line
//        Searching Each Line for the Query
        if line.contains(query){
            results.push(line);
        }

    }
//    vec![]
    results
}
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str) -> Vec<&'a str> {
    // 'a => connect arguments to return values in the signature
    let mut results =   Vec::new() ;

    let query = query.to_lowercase() ;

    for line in contents.lines() {
        // do something with line
//        Searching Each Line for the Query
        if line.to_lowercase().contains(&query){
            results.push(line);
        }

    }
//    vec![]
    results
}

#[cfg(test)]
mod tests {
    use super::* ;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
//    fn one_result(){
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

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
}
